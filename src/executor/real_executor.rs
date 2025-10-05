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

impl HasConfig for RealExecutor {
    fn config(&self) -> &Config {
        &self.config
    }
}

impl RealExecutor {
    pub fn new(config: Config) -> Self {
        Self { config }
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

    fn remove_symlink(&self, path: impl AsRef<Path>) -> Result<()> {
        remove_symlink(path)
    }

    // remove(rename)
    fn remove_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let suffix = path.strip_prefix(self.home_dir())?;
        let backup = self.backup_dir().join(suffix);
        self.rename(path, &backup)?;
        Ok(())
    }

    fn remove_dir_all(&self, path: impl AsRef<Path>) -> Result<()> {
        remove_dir_all(path)
    }

    fn remove_unknown_path(&self, path: impl AsRef<Path>) -> Result<()> {
        remove_unknown_path(path)
    }

    /// `from`を`to`にrename(move)する．
    fn rename(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
        rename(from, to)
    }
}
