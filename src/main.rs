use std::{env, path::PathBuf};

use clap::{arg, command, value_parser, Command};
use dotfiles_manager::{backup::Backup, install::Installer};

fn main() {
    let command = command!()
        .subcommand(
            Command::new("install")
                .arg(
                    arg!([dotfiles_dir] "dotfiles directory [default: ~/.dotfiles/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(
                    arg!([home_dir] "home directory [default: ~/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(
                    arg!([backup_dir] "backup directory [default: ~/.backup_dotfiles/]")
                        .value_parser(value_parser!(PathBuf))
                        .short('b')
                        .long("backup")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("backup")
                .arg(
                    arg!([dotfiles_dir] "dotfiles [default: ~/.dotfiles/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                )
                .arg(
                    arg!([home_dir] "home directory [default: ~/]")
                        .value_parser(value_parser!(PathBuf))
                        .required(false),
                ),
        );

    // for printing help
    let mut cmd = command.clone();

    match command.get_matches().subcommand() {
        Some(("install", args)) => {
            let home_dir = args.get_one::<PathBuf>("home_dir");
            let dotfiles_dir = args.get_one::<PathBuf>("dotfiles_dir");
            let bacup_dir = args.get_one::<PathBuf>("backup_dir");
            let installer = Installer::new(&home_dir, &dotfiles_dir, &bacup_dir);
            installer.install();
        }
        Some(("backup", args)) => {
            let home_dir = args.get_one::<PathBuf>("home_dir");
            let dotfiles_dir = args.get_one::<PathBuf>("dotfiles_dir");
            let backup = Backup::new(&home_dir, &dotfiles_dir);
            backup.backup();
        }
        _ => {
            cmd.print_help().expect("failed to print help.");
            panic!();
        }
    }
}
