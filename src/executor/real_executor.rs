use std::path::Path;

use anyhow::Result;

use crate::{
    config::Config,
    executor::{Executor, HasConfig},
    file_operations::*,
};

pub struct RealExecutor {
    config: Config,
}

impl RealExecutor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl HasConfig for RealExecutor {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl Executor for RealExecutor {
    /// `from`のリンクを`to`につくる．
    fn create_symlink(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
        println!(
            "created link: {} -> {}",
            from.as_ref().display(),
            to.as_ref().display()
        );
        create_symlink(from, to)
    }

    fn remove_symlink_from_home(&self, path: impl AsRef<Path>) -> Result<()> {
        remove_symlink(path)
    }

    // remove(rename)
    fn remove_file_from_home(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let suffix = path.strip_prefix(self.home_dir())?;
        let backup = self.backup_dir().join(suffix);
        rename(path, &backup)?;
        Ok(())
    }

    fn remove_dir_all_from_home(&self, path: impl AsRef<Path>) -> Result<()> {
        remove_dir_all(path)
    }

    fn remove_unknown_path_from_home(&self, path: impl AsRef<Path>) -> Result<()> {
        remove_unknown_path(path)
    }

    fn remove_file_from_dotfiles_home(&self, path: impl AsRef<Path>) -> Result<()> {
        remove_file(path)
    }

    fn copy(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
        copy(from, to)
    }
}
