use std::{env, path::PathBuf};

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};

use dotkoke::{
    config::Config,
    executor::{DryExecutor, RealExecutor},
    install::install,
    remove::remove,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long = "config", global = true)]
    config_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init {},
    Install {
        #[arg(long)]
        dry_run: bool,
    },
    Add {},
    Remove {
        #[arg(long)]
        dry_run: bool,

        path: PathBuf,
    },
    Clean {},
    List {},
    Status {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 以下の優先順位でconfigを探す．
    // 1. コマンドオプション`--config`で指定されたファイル
    // 2. 環境変数`DOTKOKE_CONFIG`で指定されたファイル
    // 3. `$HOME/.config/dotkoke_config.toml`
    let config_file_path = cli
        .config_file
        .or_else(|| env::var("DOTKOKE_CONFIG").ok().map(PathBuf::from))
        .or_else(|| {
            env::var("HOME")
                .ok()
                .map(PathBuf::from)
                .map(|p| p.join(".config/dotkoke_config.toml"))
        })
        .ok_or_else(|| anyhow!("config file not found."))?;

    let config = Config::read(config_file_path)?;

    match cli.command {
        Command::Init {} => {
            unimplemented!();
        }
        Command::Install { dry_run } => {
            if dry_run {
                install(&DryExecutor::new(config))?;
            } else {
                install(&RealExecutor::new(config))?;
            }
        }
        Command::Add {} => {
            unimplemented!();
        }
        Command::Remove { path, dry_run } => {
            if dry_run {
                remove(&DryExecutor::new(config), path)?;
            } else {
                remove(&RealExecutor::new(config), path)?;
            }
        }
        Command::Clean {} => {
            unimplemented!();
        }
        Command::List {} => {
            unimplemented!();
        }
        Command::Status {} => {
            unimplemented!();
        }
    }

    Ok(())
}
