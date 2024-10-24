use std::{fs, path::PathBuf};

use crate::{default_dotfiles_dir, default_home_dir};

const CHECK_LIST: [&str; 1] = [".config/nvim"];

/// チェックリストで指定されているディレクトリ内のシンボリックリンクでないファイルをdotfilesにコピーする
pub struct Backup {
    /// ドットファイルたちが展開されているディレクトリ
    /// default: ~/
    home_dir: PathBuf,

    /// dotfilesがあるディレクトリ
    /// default: ~/.dotfiles
    dotfiles_dir: PathBuf,
}

impl Backup {
    pub fn new(home_dir: &Option<&PathBuf>, dotfiles_dir: &Option<&PathBuf>) -> Self {
        // TODO: error処理

        let home_dir = if let Some(home_dir) = home_dir {
            home_dir.canonicalize().unwrap()
        } else {
            default_home_dir().unwrap()
        };

        let dotfiles_dir = if let Some(dotfiles_dir) = dotfiles_dir {
            dotfiles_dir.canonicalize().unwrap()
        } else {
            default_dotfiles_dir().unwrap()
        };

        Self {
            home_dir,
            dotfiles_dir,
        }
    }

    pub fn backup(&self) {
        println!("backing up new files...");

        println!("dotfiles_dir: {:?}", self.dotfiles_dir);
        println!("home_dir: {:?}", self.home_dir);

        let files = self.collect_files();

        if files.is_empty() {
            println!("No files to backup.");
            return;
        }

        for file in files {
            self.copy_file(&file);
        }
    }

    fn copy_file(&self, file: &PathBuf) {
        // ホームディレクトリからの相対パス
        let file_name = file
            .strip_prefix(&self.home_dir)
            .unwrap_or_else(|_| panic!("failed to strip_prefix: {:?}", file));

        // dotfilesからの相対パス
        let to = self.dotfiles_dir.join(file_name);

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

    fn collect_files(&self) -> Vec<PathBuf> {
        let mut res = vec![];

        let mut dirs = CHECK_LIST
            .iter()
            .map(|s| self.home_dir.join(s))
            .filter(|p| p.exists())
            .collect::<Vec<_>>();

        while let Some(dir) = dirs.pop() {
            for path in fs::read_dir(&dir)
                .unwrap_or_else(|_| panic!("failed to read directory: {:?}", dir))
                .map(|entry| entry.expect("failed to unwrap entry").path())
                .filter(|path| !path.is_symlink())
            {
                if path.is_dir() {
                    dirs.push(path);
                } else if path.is_file() {
                    res.push(path.canonicalize().expect("failed to canonicalize"))
                } else {
                    eprintln!("unknown file type: {:?}", path);
                }
            }
        }

        res
    }
}
