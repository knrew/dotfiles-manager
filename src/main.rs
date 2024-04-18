use std::{env, error::Error, path::PathBuf, str::FromStr};

use chrono::Local;
use clap::Parser;

use dotfiles_installer::link_all;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let install_dir_path =
        PathBuf::from_str(&args.install_dir_path.unwrap_or(env::var("HOME").unwrap()))?
            .canonicalize()?;

    let dotfiles_dir_path = match args.dotfiles_dir_path {
        Some(s) => PathBuf::from_str(&s)?,
        None => install_dir_path.join("dotfiles/"),
    }
    .canonicalize()?;

    let backup_dir_path = match args.backup_dir_path {
        Some(ref s) => PathBuf::from_str(s)?,
        None => install_dir_path.join(".backup_dotfiles/"),
    }
    .join(Local::now().format("%Y%m%d_%H%M").to_string());

    link_all(&dotfiles_dir_path, &install_dir_path, &backup_dir_path)?;

    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    /// default: $HOME/dotfiles/
    #[arg(short, long = "dotfiles", default_value = None)]
    dotfiles_dir_path: Option<String>,

    /// default: $HOME/
    #[arg(short, long = "install", default_value = None)]
    install_dir_path: Option<String>,

    /// default: $HOME/.backup_dotfiles/
    #[arg(short, long ="backup", default_value=None)]
    backup_dir_path: Option<String>,
}
