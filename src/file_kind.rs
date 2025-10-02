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

