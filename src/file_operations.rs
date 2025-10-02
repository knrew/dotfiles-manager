use std::{fs, os::unix, path::Path};

use anyhow::{Context, Result};

use crate::file_kind::{is_dir, is_file, is_symlink, is_unknown};

/// 親ディレクトリを作成する(pathに対する操作を行えるようにするため)．
pub fn create_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create parent directory: {}", parent.display()))?;
    }
    Ok(())
}

pub fn remove_dir_all(path: &Path) -> Result<()> {
    assert!(is_dir(path)?);

    fs::remove_dir_all(&path)
        .with_context(|| format!("failed to remove dir: {}", path.display()))?;

    Ok(())
}

pub fn remove_link(path: &Path) -> Result<()> {
    assert!(is_symlink(path)?);

    fs::remove_file(&path)
        .with_context(|| format!("failed to remove symlink: {}", path.display()))?;

    Ok(())
}

pub fn remove_file(path: &Path) -> Result<()> {
    assert!(is_file(path)?);
    fs::remove_file(&path).with_context(|| format!("failed to remove file: {}", path.display()))?;
    Ok(())
}

pub fn remove_unknown(path: &Path) -> Result<()> {
    assert!(is_unknown(path)?);
    fs::remove_file(&path)
        .with_context(|| format!("failed to remove unknown: {}", path.display()))?;
    Ok(())
}

pub fn rename(from: &Path, to: &Path) -> Result<()> {
    create_parent_dir(&to)?;
    fs::rename(from, to)
        .with_context(|| format!("failed to rename: {} -> {}", from.display(), to.display()))?;
    Ok(())
}

pub fn create_symlink(from: &Path, to: &Path) -> Result<()> {
    create_parent_dir(&to)?;
    unix::fs::symlink(&from, &to).with_context(|| {
        format!(
            "failed to create link: {} -> {}",
            from.display(),
            to.display()
        )
    })?;
    Ok(())
}
