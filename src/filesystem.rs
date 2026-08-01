use std::{fs, io, path::Path};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FileKind {
    RegularFile,
    Directory,
    Symlink,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PathState {
    Missing,
    Existing(FileKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FollowedPathState {
    Missing,
    BrokenSymlink,
    Existing(FileKind),
}

pub(crate) fn inspect_path(path: impl AsRef<Path>) -> Result<PathState> {
    let path = path.as_ref();

    match fs::symlink_metadata(path) {
        Ok(metadata) => Ok(PathState::Existing(classify(metadata.file_type()))),
        Err(error) if is_definitely_missing(&error) => Ok(PathState::Missing),
        Err(error) => Err(error).with_context(|| {
            format!(
                "failed to inspect path without following symlinks: {}",
                path.display()
            )
        }),
    }
}

pub(crate) fn inspect_path_following(path: impl AsRef<Path>) -> Result<FollowedPathState> {
    let path = path.as_ref();

    match inspect_path(path)? {
        PathState::Missing => Ok(FollowedPathState::Missing),
        PathState::Existing(FileKind::Symlink) => match fs::metadata(path) {
            Ok(metadata) => Ok(FollowedPathState::Existing(classify(metadata.file_type()))),
            Err(error) if is_unresolvable_symlink(&error) => Ok(FollowedPathState::BrokenSymlink),
            Err(error) => Err(error)
                .with_context(|| format!("failed to inspect symlink target: {}", path.display())),
        },
        PathState::Existing(kind) => Ok(FollowedPathState::Existing(kind)),
    }
}

fn classify(file_type: fs::FileType) -> FileKind {
    if file_type.is_file() {
        FileKind::RegularFile
    } else if file_type.is_dir() {
        FileKind::Directory
    } else if file_type.is_symlink() {
        FileKind::Symlink
    } else {
        FileKind::Unknown
    }
}

fn is_definitely_missing(error: &io::Error) -> bool {
    matches!(
        error.kind(),
        io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
    )
}

fn is_unresolvable_symlink(error: &io::Error) -> bool {
    is_definitely_missing(error) || error.raw_os_error() == Some(libc::ELOOP)
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::CString,
        fs,
        os::unix::{ffi::OsStrExt, fs::symlink},
    };

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn classifies_entries_without_following_symlinks() {
        let root = TempDir::new().unwrap();
        let file = root.path().join("file");
        let directory = root.path().join("directory");
        let link = root.path().join("link");
        let fifo = root.path().join("fifo");

        fs::write(&file, "content").unwrap();
        fs::create_dir(&directory).unwrap();
        symlink(&file, &link).unwrap();
        create_fifo(&fifo);

        assert_eq!(
            inspect_path(&file).unwrap(),
            PathState::Existing(FileKind::RegularFile)
        );
        assert_eq!(
            inspect_path(&directory).unwrap(),
            PathState::Existing(FileKind::Directory)
        );
        assert_eq!(
            inspect_path(&link).unwrap(),
            PathState::Existing(FileKind::Symlink)
        );
        assert_eq!(
            inspect_path(&fifo).unwrap(),
            PathState::Existing(FileKind::Unknown)
        );
        assert_eq!(
            inspect_path(root.path().join("missing")).unwrap(),
            PathState::Missing
        );
    }

    #[test]
    fn follows_absolute_and_relative_symlinks() {
        let root = TempDir::new().unwrap();
        let target = root.path().join("target");
        let absolute_link = root.path().join("absolute-link");
        let links = root.path().join("links");
        let relative_link = links.join("relative-link");

        fs::write(&target, "content").unwrap();
        fs::create_dir(&links).unwrap();
        symlink(&target, &absolute_link).unwrap();
        symlink("../target", &relative_link).unwrap();

        assert_eq!(
            inspect_path_following(&absolute_link).unwrap(),
            FollowedPathState::Existing(FileKind::RegularFile)
        );
        assert_eq!(
            inspect_path_following(&relative_link).unwrap(),
            FollowedPathState::Existing(FileKind::RegularFile)
        );
    }

    #[test]
    fn identifies_unresolvable_symlinks_as_broken() {
        let root = TempDir::new().unwrap();
        let missing_link = root.path().join("missing-link");
        let blocking_file = root.path().join("blocking-file");
        let not_directory_link = root.path().join("not-directory-link");
        let loop_a = root.path().join("loop-a");
        let loop_b = root.path().join("loop-b");

        symlink("missing", &missing_link).unwrap();
        fs::write(&blocking_file, "content").unwrap();
        symlink(blocking_file.join("child"), &not_directory_link).unwrap();
        symlink("loop-b", &loop_a).unwrap();
        symlink("loop-a", &loop_b).unwrap();

        for link in [&missing_link, &not_directory_link, &loop_a] {
            assert_eq!(
                inspect_path_following(link).unwrap(),
                FollowedPathState::BrokenSymlink
            );
        }
    }

    #[test]
    fn reports_permission_errors_as_indeterminate() {
        use std::os::unix::fs::PermissionsExt;

        let root = TempDir::new().unwrap();
        let locked = root.path().join("locked");
        let child = locked.join("child");

        fs::create_dir(&locked).unwrap();
        fs::write(&child, "content").unwrap();
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
        let result = inspect_path(&child);
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o700)).unwrap();

        let error = result.unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains("failed to inspect path without following symlinks"));
        assert!(message.contains(&child.display().to_string()));
    }

    fn create_fifo(path: &Path) {
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();
        // SAFETY: `path` is a valid, NUL-terminated C string and remains alive for the call.
        let result = unsafe { libc::mkfifo(path.as_ptr(), 0o600) };
        assert_eq!(result, 0, "{}", io::Error::last_os_error());
    }
}
