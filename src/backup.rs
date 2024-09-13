use std::{
    fs,
    path::{Path, PathBuf},
};

const CHECK_LIST: [&str; 1] = [".config/nvim"];

pub fn backup<P, Q>(dotfiles_dir: P, home_dir: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let files = collect_files(&home_dir);

    if files.is_empty() {
        println!("No files to backup.");
        return;
    }

    for file in files {
        copy_file(&dotfiles_dir, &home_dir, &file);
    }
}

fn copy_file<P, Q>(dotfiles_dir: P, home_dir: Q, file: &PathBuf)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let file_name = file
        .strip_prefix(home_dir.as_ref())
        .unwrap_or_else(|_| panic!("failed to strip_prefix: {:?}", file));

    let to = dotfiles_dir.as_ref().join(file_name);

    if file.is_symlink() {
        return;
    }

    let parent = to
        .parent()
        .unwrap_or_else(|| panic!("failed to get parent: {:?}", to));
    fs::create_dir_all(&parent)
        .unwrap_or_else(|_| panic!("failed to create directory: {:?}", parent));
    fs::copy(file, &to).unwrap_or_else(|_| panic!("failed to copy: {:?} -> {:?}", file, to));
    println!("backup: {:?} -> {:?}", file, to);
}

fn collect_files<P: AsRef<Path>>(home_dir: P) -> Vec<PathBuf> {
    let mut files = vec![];

    let mut dirs = CHECK_LIST
        .iter()
        .map(|s| home_dir.as_ref().join(s))
        .filter(|p| p.exists())
        .collect::<Vec<_>>();

    while let Some(dir) = dirs.pop() {
        for path in fs::read_dir(&dir)
            .unwrap_or_else(|_| panic!("failed to read directory: {:?}", dir))
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
        {
            if path.is_dir() {
                dirs.push(path);
            } else if path.is_symlink() {
            } else if path.is_file() {
                files.push(path);
            }
        }
    }

    files
}
