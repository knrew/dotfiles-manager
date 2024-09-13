use std::{
    fs,
    os::unix,
    path::{Path, PathBuf},
};

const IGNORES: [&str; 4] = [".git", ".gitignore", "README.md", "ex"];

pub fn install<P, Q, R>(dotfiles_dir: P, install_dir: Q, backup_dir: R)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
{
    let files = collect_dotfiles(&dotfiles_dir);
    for file in &files {
        make_link(&dotfiles_dir, &install_dir, &backup_dir, &file);
    }
}

/// シンボリックリンクを作成する
/// もともとファイルが存在する場合はバックアップディレクトリに移動
fn make_link<P, Q, R>(dotfiles_dir: P, install_dir: Q, backup_dir: R, file: &PathBuf)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
{
    let file_name = file
        .strip_prefix(dotfiles_dir.as_ref())
        .unwrap_or_else(|_| panic!("failed to strip_prefix: {:?}", file));

    let to = install_dir.as_ref().join(file_name);

    if to.exists() {
        if !to.is_symlink() {
            let backup_path = backup_dir.as_ref().join(file_name);
            let backup_parent = backup_path
                .parent()
                .unwrap_or_else(|| panic!("failed to get parent: {:?}", backup_path));

            fs::create_dir_all(backup_parent)
                .unwrap_or_else(|_| panic!("failed to create directory: {:?}", backup_parent));

            fs::copy(&to, &backup_path)
                .unwrap_or_else(|_| panic!("failed to copy: {:?} -> {:?}", to, backup_path));
        }

        fs::remove_file(&to).unwrap_or_else(|_| panic!("failed to remove: {:?}", to));
    }

    let parent = to
        .parent()
        .unwrap_or_else(|| panic!("failed to get parent: {:?}", to));
    fs::create_dir_all(parent)
        .unwrap_or_else(|_| panic!("failed to create directory: {:?}", parent));
    unix::fs::symlink(file, &to)
        .unwrap_or_else(|_| panic!("failed to create symlink: {:?} -> {:?}", file, to));

    println!("install: {:?} -> {:?}", file, to);
}

/// dotfiles_dir以下のファイルをリストアップする
/// IGNOREで指定したファイルは除外する
fn collect_dotfiles<P: AsRef<Path>>(dotfiles_dir: P) -> Vec<PathBuf> {
    let ignores = IGNORES
        .iter()
        .map(|s| dotfiles_dir.as_ref().join(s))
        .filter(|p| p.exists())
        .collect::<Vec<_>>();

    let mut files = vec![];

    let mut dirs = vec![PathBuf::from(dotfiles_dir.as_ref())];
    while let Some(dir) = dirs.pop() {
        for path in fs::read_dir(&dir)
            .unwrap_or_else(|_| panic!("failed to read directory: {:?}", dir))
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| !ignores.contains(path))
        {
            if path.is_dir() {
                dirs.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }

    files
}
