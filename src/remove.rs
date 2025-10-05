use std::path::Path;

use anyhow::{Context, Result, anyhow};

use crate::{executor::Executor, file_kind::*};

pub fn remove(executor: impl Executor, path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let path = path
        .canonicalize()
        .with_context(|| format!("invalid path: {}", path.display()))?;

    if !path.starts_with(executor.dotfiles_home_dir()) {
        return Err(anyhow!(
            "{} is not in {}.",
            path.display(),
            executor.dotfiles_home_dir().display()
        ));
    }

    let to = executor.install_path(&path)?;

    if is_symlink_pointing_to(&to, &path) || is_broken_link(&to) {
        executor.remove_symlink_from_home(&to)?;
    }

    executor.remove_file_from_dotfiles_home(path)?;

    Ok(())
}
