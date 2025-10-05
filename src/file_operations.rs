use std::{fs, os::unix, path::Path};

use anyhow::{Context, Result, anyhow};

use crate::file_kind::*;

/// `path`の親ディレクトリを作成する．
pub fn create_parent_dir(path: impl AsRef<Path>) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create parent directory: {}", parent.display()))?;
    }
    Ok(())
}

/// pathがファイルである場合に，それを削除する．
/// 引数がファイル以外ならエラー．
pub fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if !is_file(path) {
        return Err(anyhow!("{} is not a file.", path.display()));
    }

    fs::remove_file(path).with_context(|| format!("failed to remove file: {}", path.display()))?;

    Ok(())
}

/// pathがリンクである場合に，それを削除する．
/// 引数がリンク以外ならエラー．
pub fn remove_symlink(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if !is_symlink(path) {
        return Err(anyhow!("{} is not a symlink.", path.display()));
    }

    fs::remove_file(path)
        .with_context(|| format!("failed to remove symlink: {}", path.display()))?;

    Ok(())
}

/// pathが不明なパスである場合に，それを削除する．
/// 引数が不明なパス以外ならエラー．
pub fn remove_unknown_path(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if !is_unknown(path) {
        return Err(anyhow!("{} is not an unknown path.", path.display()));
    }

    fs::remove_file(path)
        .with_context(|| format!("failed to remove unknown path: {}", path.display()))?;

    Ok(())
}

/// pathがディレクトリである場合に，それをすべて削除．
/// 引数がディレクトリ以外ならエラー．
pub fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if !is_dir(path) {
        return Err(anyhow!("{} is not a directory.", path.display()));
    }

    fs::remove_dir_all(path)
        .with_context(|| format!("failed to remove dir: {}", path.display()))?;

    Ok(())
}

/// `from`を`to`にrename(mv)する．
/// `to`に既存ファイルがあるかどうかは確認しない．
pub fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    create_parent_dir(to)?;

    fs::rename(from, to)
        .with_context(|| format!("failed to rename: {} -> {}", from.display(), to.display()))?;

    Ok(())
}

/// `from`を`to`にcopyする．
/// `to`に既存ファイルがあるかどうかは確認しない．
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    create_parent_dir(to)?;

    fs::copy(from, to)
        .with_context(|| format!("failed to copy: {} -> {}", from.display(), to.display()))?;

    Ok(())
}

/// `from`へのリンクを`to`に貼る．
/// `to`の参照先が`from`になるようにする．
pub fn create_symlink(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    create_parent_dir(to)?;
    unix::fs::symlink(from, to).with_context(|| {
        format!(
            "failed to create link: {} -> {}",
            from.display(),
            to.display()
        )
    })?;

    Ok(())
}
