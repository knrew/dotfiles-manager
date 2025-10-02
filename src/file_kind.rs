use std::{fs, path::Path};

use anyhow::Result;

pub enum FileKind {
    File,
    Dir,
    Symlink,
    Unknown,
    NotFound,
}

pub fn file_kind(path: &Path) -> Result<FileKind> {
    match fs::symlink_metadata(path) {
        Ok(meta) => {
            let ft = meta.file_type();
            if ft.is_symlink() {
                Ok(FileKind::Symlink)
            } else if ft.is_dir() {
                Ok(FileKind::Dir)
            } else if ft.is_file() {
                Ok(FileKind::File)
            } else {
                Ok(FileKind::Unknown)
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(FileKind::NotFound),
        Err(e) => Err(e.into()),
    }
}

pub fn is_file(path: &Path) -> Result<bool> {
    Ok(matches!(file_kind(path)?, FileKind::File))
}

pub fn is_dir(path: &Path) -> Result<bool> {
    Ok(matches!(file_kind(path)?, FileKind::Dir))
}

pub fn is_symlink(path: &Path) -> Result<bool> {
    Ok(matches!(file_kind(path)?, FileKind::Symlink))
}

pub fn is_unknown(path: &Path) -> Result<bool> {
    Ok(matches!(file_kind(path)?, FileKind::Unknown))
}

/// linkがpathを指しているか判定する．
pub fn is_symlink_pointing_to(link: &Path, path: &Path) -> Result<bool> {
    let meta = match fs::symlink_metadata(link) {
        Ok(m) => m,
        Err(_) => return Ok(false),
    };

    if !meta.file_type().is_symlink() {
        return Ok(false);
    }

    let target = fs::read_link(link)?;

    let target_abs = if target.is_absolute() {
        target
    } else {
        link.parent().unwrap_or(Path::new("")).join(target)
    };

    let lhs = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let rhs = target_abs.canonicalize().unwrap_or(target_abs);

    Ok(lhs == rhs)
}
