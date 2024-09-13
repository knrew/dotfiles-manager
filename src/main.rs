use std::{env, path::PathBuf};

use chrono::Local;
use clap::{arg, command, value_parser, Command};

use dotfiles_manager::{backup::backup, install::install};

fn main() {
    let home_dir = if let Ok(home_dir) = env::var("HOME") {
        if let Ok(home_dir) = PathBuf::from(home_dir).canonicalize() {
            Some(home_dir)
        } else {
            None
        }
    } else {
        None
    };

    let default_dotfiles_dir = if let Some(ref home_dir) = home_dir {
        if let Ok(dotfiles_dir) = home_dir.join(".dotfiles").canonicalize() {
            Some(dotfiles_dir)
        } else {
            None
        }
    } else {
        None
    };

    let default_install_dir = home_dir.clone();

    let default_backup_dir = if let Some(ref home_dir) = home_dir {
        Some(home_dir.join(".backup_dotfiles"))
    } else {
        None
    };

    let command = command!()
        .subcommand(
            Command::new("install")
                .arg(
                    arg!([dotfiles_dir] "dotfiles [default: $HOME/.dotfiles/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(
                    arg!([install_dir] "install directory [default: $HOME/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(
                    arg!([backup_dir] "backup directory [default: $HOME/.backup_dotfiles/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("backup")
                .arg(
                    arg!([dotfiles_dir] "dotfiles [default: $HOME/.dotfiles/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(
                    arg!([home_dir] "home directory [default: $HOME/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                ),
        );

    // for printing help
    let mut cmd = command.clone();

    match command.get_matches().subcommand() {
        Some(("install", args)) => {
            println!("installing dotfiles...");

            let dotfiles_dir = if let Some(path) = args.get_one::<PathBuf>("dotfiles_dir") {
                path.canonicalize()
                    .unwrap_or_else(|_| panic!("not found: {:?}", path))
            } else {
                default_dotfiles_dir.unwrap_or_else(|| panic!("specify dotfiles directory"))
            };

            let install_dir = if let Some(path) = args.get_one::<PathBuf>("install_dir") {
                path.canonicalize()
                    .unwrap_or_else(|_| panic!("not found: {:?}", path))
            } else {
                default_install_dir.unwrap_or_else(|| panic!("specify install directory."))
            };

            let now = Local::now().format("%Y%m%d_%H%M").to_string();
            let backup_dir = if let Some(path) = args.get_one::<PathBuf>("backup_dir") {
                path.clone()
            } else {
                default_backup_dir.unwrap_or_else(|| panic!("specify backup directory."))
            }
            .join(now);

            println!("dotfiles directory: {:?}", dotfiles_dir);
            println!("install directory: {:?}", install_dir);
            println!("backup directory: {:?}", backup_dir);

            install(dotfiles_dir, install_dir, backup_dir);
        }
        Some(("backup", args)) => {
            println!("backing up new files...");

            let dotfiles_dir = if let Some(path) = args.get_one::<PathBuf>("dotfiles_dir") {
                path.canonicalize()
                    .unwrap_or_else(|_| panic!("not found: {:?}", path))
            } else {
                default_dotfiles_dir.unwrap_or_else(|| panic!("specify dotfiles directory."))
            };

            let home_dir = if let Some(path) = args.get_one::<PathBuf>("home_dir") {
                path.canonicalize()
                    .unwrap_or_else(|_| panic!("not found: {:?}", path))
            } else {
                home_dir.unwrap_or_else(|| panic!("specify home directory."))
            };

            println!("dotfiles directory: {:?}", dotfiles_dir);
            println!("home directory: {:?}", home_dir);

            backup(dotfiles_dir, home_dir);
        }
        _ => {
            cmd.print_help().unwrap();
            panic!();
        }
    }
}
