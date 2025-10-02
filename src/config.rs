//! 設定ファイル(toml)から設定を読み込む．

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct General {
    #[serde(rename = "dotfiles")]
    dotfiles_dir: PathBuf,

    #[serde(rename = "home")]
    home_dir: PathBuf,

    #[serde(rename = "backup_dir")]
    backup_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Toml {
    general: General,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub home_dir: PathBuf,
    pub dotfiles_dir: PathBuf,

    // バックアップは`backup_dir/YYYYmmdd_HHMM`以下に保存する．
    pub backup_dir: PathBuf,

    // dotfiles/home/
    pub dotfiles_home_dir: PathBuf,
}

impl Config {
    pub fn read(config_toml_path: impl AsRef<Path>) -> Result<Self> {
        let config_toml_path = config_toml_path.as_ref();

        let toml_str = fs::read_to_string(config_toml_path).with_context(|| {
            format!("failed to read config file: {}", config_toml_path.display())
        })?;

        let Toml {
            general:
                General {
                    dotfiles_dir,
                    home_dir,
                    backup_dir,
                },
        } = toml::from_str(&toml_str).with_context(|| {
            format!(
                "failed to parse config file: {}",
                config_toml_path.display()
            )
        })?;

        let dotfiles_dir = dotfiles_dir.canonicalize().with_context(|| {
            format!(
                "invalid dotfiles directory in config: {}",
                dotfiles_dir.display()
            )
        })?;

        if !dotfiles_dir.is_dir() {
            return Err(anyhow!("{} is not directory.", dotfiles_dir.display()));
        }

        let home_dir = home_dir
            .canonicalize()
            .with_context(|| format!("invalid home directory in config: {}", home_dir.display()))?;

        if !home_dir.is_dir() {
            return Err(anyhow!("{} is not directory.", home_dir.display()));
        }

        let backup_dir = backup_dir
            .canonicalize()
            .with_context(|| {
                format!(
                    "invalid backup directory in config: {}",
                    backup_dir.display()
                )
            })?
            .join(Local::now().format("%Y%m%d_%H%M").to_string());

        if backup_dir.is_dir() {
            eprintln!("[warning] {} already exists.", backup_dir.display());
        }

        let dotfiles_home_dir = dotfiles_dir
            .join("home")
            .canonicalize()
            .with_context(|| format!("invalid path: {}/home", dotfiles_dir.display()))?;

        if !dotfiles_home_dir.is_dir() {
            return Err(anyhow!("{} is not directory.", dotfiles_home_dir.display()));
        }

        let config = Config {
            dotfiles_dir,
            home_dir,
            backup_dir,
            dotfiles_home_dir,
        };

        Ok(config)
    }
}
