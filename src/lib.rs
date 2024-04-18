use std::{
    collections::VecDeque,
    error::Error,
    fs,
    os::unix,
    path::{Path, PathBuf},
};

pub fn link_all(
    dotfiles_dir: &Path,
    install_dir: &Path,
    backup_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    println!("Dotfiles installer launched.");

    println!("dotfiles directory: {}", dotfiles_dir.to_str().unwrap());
    println!("install directory: {}", install_dir.to_str().unwrap());
    println!("backup directory: {}", backup_dir.to_str().unwrap());

    let mut scanning_dirs: VecDeque<PathBuf> = VecDeque::new();
    scanning_dirs.push_front(PathBuf::from(dotfiles_dir));

    while !scanning_dirs.is_empty() {
        let dir = scanning_dirs.front().unwrap().clone();
        scanning_dirs.pop_front().unwrap();

        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();

            let filename = path.file_name().unwrap();

            if dir.eq(dotfiles_dir) && filename.to_str().unwrap().chars().nth(0).unwrap() != '.' {
                continue;
            }

            if filename == ".git" || filename == ".gitignore" {
                continue;
            }

            if path.is_dir() {
                scanning_dirs.push_front(path);
            } else {
                link(&path, &dotfiles_dir, &install_dir, &backup_dir)?;
            }
        }
    }

    println!("Install completed!");

    Ok(())
}

fn link(
    file: &Path,
    dotfiles_dir: &Path,
    home_dir: &Path,
    backup_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let file_name = file.strip_prefix(dotfiles_dir).unwrap();

    let original = dotfiles_dir.join(file_name);
    let link = home_dir.join(file_name);

    if link.exists() {
        if !link.is_symlink() {
            let backup_file = backup_dir.join(file_name);
            fs::create_dir_all(&backup_file.parent().unwrap())?;
            fs::copy(&link, &backup_file)?;
        }
        fs::remove_file(&link)?;
    }

    fs::create_dir_all(&link.parent().unwrap())?;
    unix::fs::symlink(&original, &link)?;

    println!("installed: {} ", file_name.to_str().unwrap());

    Ok(())
}
