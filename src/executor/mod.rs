use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::config::Config;

trait HasConfig {
    fn config(&self) -> &Config;
}

#[allow(private_bounds)]
pub trait Executor: HasConfig {
    fn home_dir(&self) -> &Path {
        &self.config().home_dir
    }
    fn dotfiles_home_dir(&self) -> &Path {
        &self.config().dotfiles_home_dir
    }
    fn backup_dir(&self) -> &Path {
        &self.config().backup_dir
    }

    /// dotfiles レポジトリからの相対パス(例: ~/.dotfiles/foo/bar -> foo/bar)
    fn repo_rel_from_dotfiles_home(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        Ok(path
            .as_ref()
            .strip_prefix(self.dotfiles_home_dir())?
            .to_path_buf())
    }

    fn repo_rel_from_home(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        Ok(path.as_ref().strip_prefix(self.home_dir())?.to_path_buf())
    }

    /// レポジトリ内の`path`を$HOME に"インストール"した場合の絶対パス
    fn install_path(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        Ok(self
            .home_dir()
            .join(self.repo_rel_from_dotfiles_home(path)?))
    }

    /// $HOME以下のpathに対応するdotfiles/home以下の絶対パス
    fn entity_path(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        Ok(self
            .dotfiles_home_dir()
            .join(self.repo_rel_from_home(path)?))
    }

    /// `from`のリンクを`to`につくる．
    fn create_symlink(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()>;

    /// `from`のリンクを`to`につくる処理をスキップする．
    fn skip_link_creating(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
        println!(
            "skipped (already linked): {} -> {}",
            from.as_ref().display(),
            to.as_ref().display()
        );
        Ok(())
    }

    fn remove_symlink_from_home(&self, path: impl AsRef<Path>) -> Result<()>;

    // renameを含む．
    fn remove_file_from_home(&self, path: impl AsRef<Path>) -> Result<()>;

    fn remove_dir_all_from_home(&self, path: impl AsRef<Path>) -> Result<()>;

    fn remove_unknown_path_from_home(&self, path: impl AsRef<Path>) -> Result<()>;

    fn remove_file_from_dotfiles_home(&self, path: impl AsRef<Path>) -> Result<()>;

    fn warn_cannot_determine(&self, path: impl AsRef<Path>) -> Result<()> {
        eprintln!(
            "[warning] cannot determine file kind of {} (skipped)",
            path.as_ref().display()
        );
        Ok(())
    }

    fn copy(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()>;
}

pub mod dry_executor;
pub mod real_executor;

pub use dry_executor::*;
pub use real_executor::*;
