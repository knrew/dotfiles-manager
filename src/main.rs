use core::panic;
use std::{env, path::PathBuf};

use chrono::Local;
use clap::{arg, command, value_parser, Command};

use dotfiles_manager::{backup::backup, install::install};

fn main() {
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

    let mut cmd = command.clone();

    match command.get_matches().subcommand() {
        Some(("install", args)) => {
            println!("installing dotfiles...");

            let dotfiles_dir = if let Some(path) = args.get_one::<PathBuf>("dotfiles_dir") {
                path.clone()
            } else {
                let home_dir = env::var("HOME").expect("HOME not found");
                PathBuf::from(home_dir).join("dotfiles")
            }
            .canonicalize()
            .unwrap_or_else(|_| panic!("dotfiles directory not found"));

            let install_dir = if let Some(path) = args.get_one::<PathBuf>("install_dir") {
                path.clone()
            } else {
                let home_dir = env::var("HOME").expect("HOME not found");
                PathBuf::from(home_dir)
            }
            .canonicalize()
            .unwrap_or_else(|_| panic!("dotfiles directory not found"));

            let today = Local::now().format("%Y%m%d_%H%M").to_string();
            let backup_dir = if let Some(path) = args.get_one::<PathBuf>("backup_dir") {
                path.join(today)
            } else {
                let home_dir = env::var("HOME").expect("HOME not found");
                PathBuf::from(home_dir).join("backup").join(today)
            };

            println!("dotfiles directory: {:?}", dotfiles_dir);
            println!(
                "install directory: {:?}",
                install_dir.canonicalize().unwrap()
            );
            println!("backup directory: {:?}", backup_dir);

            install(dotfiles_dir, install_dir, backup_dir);
        }
        Some(("backup", args)) => {
            let dotfiles_dir = if let Some(path) = args.get_one::<PathBuf>("dotfiles_dir") {
                path.clone()
            } else {
                let home_dir = env::var("HOME").expect("HOME not found");
                PathBuf::from(home_dir).join("dotfiles")
            };

            let home_dir = if let Some(path) = args.get_one::<PathBuf>("home_dir") {
                path.clone()
            } else {
                let home_dir = env::var("HOME").expect("HOME not found");
                PathBuf::from(home_dir)
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
