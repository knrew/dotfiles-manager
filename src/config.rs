//! 設定ファイル(toml)から設定を読み込む．

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
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
    pub backup_dir: PathBuf,
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

        let config = Config {
            dotfiles_dir: dotfiles_dir.canonicalize().with_context(|| {
                format!(
                    "invalid dotfiles directory in config: {}",
                    dotfiles_dir.display()
                )
            })?,

            home_dir: home_dir.canonicalize().with_context(|| {
                format!("invalid home directory in config: {}", home_dir.display())
            })?,

            // バックアップは`backup_dir/YYYYmmdd_HHMM`以下に保存する．
            backup_dir: backup_dir
                .canonicalize()
                .with_context(|| {
                    format!(
                        "invalid backup directory in config: {}",
                        backup_dir.display()
                    )
                })?
                .join(Local::now().format("%Y%m%d_%H%M").to_string()),
        };

        Ok(config)
    }
}
