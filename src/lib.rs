pub mod backup;
pub mod install;

use std::{env, path::PathBuf};

/// ~/
pub fn default_home_dir() -> Option<PathBuf> {
    if let Ok(home_dir) = env::var("HOME") {
        if let Ok(home_dir) = PathBuf::from(home_dir).canonicalize() {
            return Some(home_dir);
        }
    }
    None
}

/// ~/.dotfiles/
pub fn default_dotfiles_dir() -> Option<PathBuf> {
    if let Some(ref home_dir) = default_home_dir() {
        if let Ok(dotfiles_dir) = home_dir.join(".dotfiles").canonicalize() {
            return Some(dotfiles_dir);
        }
    }
    None
}

/// ~/.backup_dotfiles/
/// for install command
pub fn default_backup_dir() -> Option<PathBuf> {
    if let Some(ref home_dir) = default_home_dir() {
        Some(home_dir.join(".backup_dotfiles"))
    } else {
        None
    }
}
