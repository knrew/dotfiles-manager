use std::path::Path;

use anyhow::{Context, Result, anyhow};

use crate::{
    executor::Executor,
    file_kind::{exists, is_symlink},
};

pub fn add(executor: impl Executor, path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if is_symlink(path) {
        eprintln!("[warning] {} is a symlink. skipped.", path.display());
        return Ok(());
    }

    let path = path
        .canonicalize()
        .with_context(|| format!("invalid path: {}", path.display()))?;

    if !path.starts_with(executor.home_dir()) {
        return Err(anyhow!(
            "{} is not in {}.",
            path.display(),
            executor.home_dir().display()
        ));
    }

    // dotfiles管理下ならスキップ．
    if path.starts_with(executor.dotfiles_home_dir()) {
        return Err(anyhow!(
            "{} is in {}.",
            path.display(),
            executor.dotfiles_home_dir().display()
        ));
    }

    let to = executor.entity_path(&path)?;

    if exists(&to) {
        eprintln!("[warning] {} already exists. skipped.", to.display());
        return Ok(());
    }

    executor.copy(path, to)?;

    Ok(())
}
