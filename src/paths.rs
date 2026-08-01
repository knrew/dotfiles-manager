use std::{
    ffi::{OsStr, OsString},
    fs,
    os::unix::ffi::OsStrExt,
    path::{Component, Path, PathBuf},
};

use anyhow::{Context, Result, ensure};

use crate::filesystem::{FileKind, PathState, inspect_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DescendantPathState {
    Existing(FileKind),
    Missing { first_missing: PathBuf },
    Blocked { component: PathBuf, kind: FileKind },
}

pub(crate) fn canonicalize_existing(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    fs::canonicalize(path)
        .with_context(|| format!("failed to canonicalize existing path: {}", path.display()))
}

pub(crate) fn canonicalize_with_missing(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    ensure!(
        path.is_absolute(),
        "path must be absolute when canonicalizing with missing components: {}",
        path.display()
    );

    let mut ancestor = path.to_path_buf();
    let mut missing_components = Vec::<OsString>::new();

    loop {
        match inspect_path(&ancestor)? {
            PathState::Existing(_) => break,
            PathState::Missing => {
                let component = ancestor.components().next_back().ok_or_else(|| {
                    anyhow::anyhow!(
                        "failed to find an existing ancestor for path: {}",
                        path.display()
                    )
                })?;
                ensure!(
                    !matches!(component, Component::Prefix(_) | Component::RootDir),
                    "failed to find an existing ancestor for path: {}",
                    path.display()
                );
                missing_components.push(component.as_os_str().to_owned());
                ensure!(
                    ancestor.pop(),
                    "failed to find an existing ancestor for path: {}",
                    path.display()
                );
            }
        }
    }

    let mut canonical = canonicalize_existing(&ancestor)?;
    if !missing_components.is_empty() {
        let metadata = fs::metadata(&canonical).with_context(|| {
            format!(
                "failed to inspect existing ancestor while canonicalizing {}: {}",
                path.display(),
                canonical.display()
            )
        })?;
        ensure!(
            metadata.is_dir(),
            "existing ancestor is not a directory while canonicalizing {}: {}",
            path.display(),
            ancestor.display()
        );
    }

    for component in missing_components.iter().rev() {
        if component == OsStr::new(".") {
            continue;
        }
        if component == OsStr::new("..") {
            canonical.pop();
        } else {
            canonical.push(component);
        }
    }
    Ok(canonical)
}

pub(crate) fn paths_are_equal(left: impl AsRef<Path>, right: impl AsRef<Path>) -> bool {
    left.as_ref() == right.as_ref()
}

pub(crate) fn is_same_or_descendant(root: impl AsRef<Path>, candidate: impl AsRef<Path>) -> bool {
    candidate.as_ref().starts_with(root.as_ref())
}

pub(crate) fn is_strict_descendant(root: impl AsRef<Path>, candidate: impl AsRef<Path>) -> bool {
    let root = root.as_ref();
    let candidate = candidate.as_ref();
    candidate != root && candidate.starts_with(root)
}

pub(crate) fn inspect_descendant_path(
    root: impl AsRef<Path>,
    relative_path: impl AsRef<Path>,
) -> Result<DescendantPathState> {
    let root = root.as_ref();
    let relative_path = relative_path.as_ref();
    ensure!(
        root.is_absolute(),
        "root must be absolute: {}",
        root.display()
    );
    ensure!(
        is_normal_relative_path(relative_path),
        "path must be a non-empty relative path without '.' or '..': {}",
        relative_path.display()
    );

    let components = relative_path.components().collect::<Vec<_>>();
    let mut current = root.to_path_buf();

    for (index, component) in components.iter().enumerate() {
        current.push(component.as_os_str());
        let is_target = index + 1 == components.len();

        match inspect_path(&current)? {
            PathState::Missing => {
                return Ok(DescendantPathState::Missing {
                    first_missing: current,
                });
            }
            PathState::Existing(kind) if is_target => {
                return Ok(DescendantPathState::Existing(kind));
            }
            PathState::Existing(FileKind::Directory) => {}
            PathState::Existing(kind) => {
                return Ok(DescendantPathState::Blocked {
                    component: current,
                    kind,
                });
            }
        }
    }

    unreachable!("a validated relative path has at least one component")
}

fn is_normal_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && !path
            .as_os_str()
            .as_bytes()
            .split(|byte| *byte == b'/')
            .any(|component| matches!(component, b"." | b".."))
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
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
    fn canonicalizes_existing_paths_and_symlink_aliases() {
        let root = TempDir::new().unwrap();
        let directory = root.path().join("directory");
        let alias = root.path().join("alias");

        fs::create_dir(&directory).unwrap();
        symlink(&directory, &alias).unwrap();

        assert_eq!(
            canonicalize_existing(&alias).unwrap(),
            canonicalize_existing(&directory).unwrap()
        );
    }

    #[test]
    fn canonicalizes_an_existing_ancestor_and_appends_missing_components() {
        let root = TempDir::new().unwrap();
        let directory = root.path().join("directory");
        let alias = root.path().join("alias");
        let path = alias.join("missing/child");

        fs::create_dir(&directory).unwrap();
        symlink(&directory, &alias).unwrap();

        assert_eq!(
            canonicalize_with_missing(&path).unwrap(),
            canonicalize_existing(&directory)
                .unwrap()
                .join("missing/child")
        );
    }

    #[test]
    fn normalizes_relative_components_after_the_existing_ancestor() {
        let root = TempDir::new().unwrap();
        let directory = root.path().join("directory");
        let path = directory.join("missing/../../sibling");

        fs::create_dir(&directory).unwrap();

        assert_eq!(
            canonicalize_with_missing(&path).unwrap(),
            canonicalize_existing(root.path()).unwrap().join("sibling")
        );
    }

    #[test]
    fn rejects_non_directory_and_broken_symlink_ancestors() {
        let root = TempDir::new().unwrap();
        let file = root.path().join("file");
        let broken_link = root.path().join("broken-link");

        fs::write(&file, "content").unwrap();
        symlink("missing", &broken_link).unwrap();

        assert!(canonicalize_with_missing(file.join("child")).is_err());
        assert!(canonicalize_with_missing(broken_link.join("child")).is_err());
    }

    #[test]
    fn compares_paths_at_component_boundaries_without_case_folding() {
        let root = Path::new("/a/b");

        assert!(paths_are_equal(root, Path::new("/a/b")));
        assert!(!paths_are_equal(root, Path::new("/a/B")));
        assert!(is_same_or_descendant(root, root));
        assert!(is_same_or_descendant(root, Path::new("/a/b/child")));
        assert!(!is_same_or_descendant(root, Path::new("/a/bc")));
        assert!(!is_strict_descendant(root, root));
        assert!(is_strict_descendant(root, Path::new("/a/b/child")));
    }

    #[test]
    fn inspects_destination_components_without_following_symlinks() {
        let root = TempDir::new().unwrap();
        let directory = root.path().join("directory");
        let target = directory.join("target");
        let file_parent = root.path().join("file-parent");
        let link_parent = root.path().join("link-parent");
        let fifo_parent = root.path().join("fifo-parent");

        fs::create_dir(&directory).unwrap();
        fs::write(&target, "content").unwrap();
        fs::write(&file_parent, "content").unwrap();
        symlink(&directory, &link_parent).unwrap();
        create_fifo(&fifo_parent);

        assert_eq!(
            inspect_descendant_path(root.path(), Path::new("directory/target")).unwrap(),
            DescendantPathState::Existing(FileKind::RegularFile)
        );
        assert_eq!(
            inspect_descendant_path(root.path(), Path::new("directory/missing/child")).unwrap(),
            DescendantPathState::Missing {
                first_missing: directory.join("missing")
            }
        );

        for (name, kind) in [
            ("file-parent", FileKind::RegularFile),
            ("link-parent", FileKind::Symlink),
            ("fifo-parent", FileKind::Unknown),
        ] {
            assert_eq!(
                inspect_descendant_path(root.path(), Path::new(name).join("child")).unwrap(),
                DescendantPathState::Blocked {
                    component: root.path().join(name),
                    kind,
                }
            );
        }
    }

    #[test]
    fn reports_component_permission_errors_as_indeterminate() {
        use std::os::unix::fs::PermissionsExt;

        let root = TempDir::new().unwrap();
        let locked = root.path().join("locked");
        fs::create_dir(&locked).unwrap();
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
        let result = inspect_descendant_path(root.path(), Path::new("locked/child"));
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o700)).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn rejects_paths_that_can_escape_the_root() {
        let root = TempDir::new().unwrap();

        for path in [
            Path::new(""),
            Path::new("."),
            Path::new("directory/./target"),
            Path::new("../outside"),
            root.path(),
        ] {
            assert!(inspect_descendant_path(root.path(), path).is_err());
        }
    }

    fn create_fifo(path: &Path) {
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();
        // SAFETY: `path` is a valid, NUL-terminated C string and remains alive for the call.
        let result = unsafe { libc::mkfifo(path.as_ptr(), 0o600) };
        assert_eq!(result, 0, "{}", std::io::Error::last_os_error());
    }
}
