use std::path::Path;

use anyhow::Result;

use crate::{
    config::Config,
    executor::{Executor, HasConfig},
};

pub struct DryExecutor {
    pub config: Config,
}

impl DryExecutor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl HasConfig for DryExecutor {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl Executor for DryExecutor {
    fn create_symlink(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
        println!(
            "[dry-run] ln -s {} -> {}",
            from.as_ref().display(),
            to.as_ref().display()
        );
        Ok(())
    }

    fn remove_symlink_from_home(&self, path: impl AsRef<Path>) -> Result<()> {
        println!("[dry-run] rm (symlink) {}", path.as_ref().display());
        Ok(())
    }

    fn remove_file_from_home(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let suffix = path.strip_prefix(self.home_dir())?;
        let backup = self.backup_dir().join(suffix);
        println!("[dry-run] mv {} -> {}", path.display(), backup.display());
        Ok(())
    }

    fn remove_dir_all_from_home(&self, path: impl AsRef<Path>) -> Result<()> {
        println!("[dry-run] rm -rf {}", path.as_ref().display());
        Ok(())
    }

    fn remove_unknown_path_from_home(&self, path: impl AsRef<Path>) -> Result<()> {
        println!("[dry-run] unlink (unknown) {}", path.as_ref().display());
        Ok(())
    }

    fn remove_file_from_dotfiles_home(&self, path: impl AsRef<Path>) -> Result<()> {
        println!("[dry-run] rm -rf {}", path.as_ref().display());
        Ok(())
    }

    fn copy(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
        println!(
            "[dry-run] cp {} -> {}",
            from.as_ref().display(),
            to.as_ref().display()
        );
        Ok(())
    }
}
