use core::panic;
use std::{fs, os::unix, path::PathBuf};

use chrono::Local;

use crate::{default_backup_dir, default_dotfiles_dir, default_home_dir};

pub struct Installer {
    /// ドットファイルたちを展開するディレクトリ
    /// default: ~/
    home_dir: PathBuf,

    /// インストールするドットファイルがあるディレクトリ
    /// default: ~/.dotfiles
    dotfiles_dir: PathBuf,

    /// バックアップディレクトリ
    /// インストールする際，もともとファイルが存在すればここに移動される
    /// default: ~/.backup_dotfiles/
    backup_dir: PathBuf,

    ignores: Vec<PathBuf>,
}

impl Installer {
    const IGNORES: [&str; 5] = [".git", ".gitignore", "README.md", "readme.md", "ex"];

    pub fn new(
        home_dir: &Option<&PathBuf>,
        dotfiles_dir: &Option<&PathBuf>,
        bacup_dir: &Option<&PathBuf>,
    ) -> Self {
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

        let now = Local::now().format("%Y%m%d_%H%M").to_string();
        let backup_dir = if let Some(backup_dir) = bacup_dir {
            PathBuf::from(backup_dir)
        } else {
            default_backup_dir().unwrap()
        }
        .join(now);

        let ignores = Self::IGNORES
            .iter()
            .map(|s| dotfiles_dir.join(s))
            .filter(|p| p.exists())
            .collect::<Vec<_>>();

        Self {
            home_dir,
            dotfiles_dir,
            backup_dir,
            ignores,
        }
    }

    pub fn install(&self) {
        println!("installing dotfiles...");

        println!("dotfiles_dir: {:?}", self.dotfiles_dir);
        println!("home_dir: {:?}", self.home_dir);
        println!("backup_dir: {:?}", self.backup_dir);

        let files = self.collect_dotfiles(&self.dotfiles_dir);
        for file in &files {
            self.make_link(&file);
        }
    }

    /// dotfiles_dir以下のファイルをリストアップする
    /// IGNOREで指定したファイルは除外する
    fn collect_dotfiles(&self, dotfiles_dir: &PathBuf) -> Vec<PathBuf> {
        let mut res = vec![];

        let mut dirs = vec![dotfiles_dir.clone()];

        while let Some(dir) = dirs.pop() {
            for path in fs::read_dir(&dir)
                .unwrap_or_else(|_| panic!("failed to read directory: {:?}", dir))
                .map(|entry| entry.expect("failed to unwrap entry").path())
                .filter(|path| !self.ignores.contains(path))
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

        res.sort_unstable();
        res.dedup();

        res
    }

    /// シンボリックリンクを作成する
    /// もともとファイルが存在する場合はバックアップディレクトリに移動
    fn make_link(&self, from: &PathBuf) {
        // dotfiles/からの相対パス
        // cf. ~/.dotfies/config/poyo.cfg -> config/poyo.cfg
        let file_name = from
            .strip_prefix(&self.dotfiles_dir)
            .unwrap_or_else(|_| panic!("failed to strip_prefix: {:?}", from));

        // インストール先となるファイル
        // ホームディレクトリからの相対パス
        // cf. config/poyo.cfg -> ~/.config/poyo.cfg
        let to = self.home_dir.join(file_name);

        if to.exists() {
            // インストール先にリンクでないファイルが存在すればバックアップする
            if !to.is_symlink() {
                // バックアップ先となるパス
                // バックアップディレクトリからの相対パス
                // cf. config/poyo.cfg -> ~/.backup_dotfiles/YYYYmmdd_HHMM/.config/poyo.cfg
                let backup_path = self.backup_dir.join(file_name);

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

        let to_parent = to
            .parent()
            .unwrap_or_else(|| panic!("failed to get parent: {:?}", to));
        fs::create_dir_all(to_parent)
            .unwrap_or_else(|_| panic!("failed to create directory: {:?}", to_parent));

        unix::fs::symlink(from, &to)
            .unwrap_or_else(|_| panic!("failed to create symlink: {:?} -> {:?}", from, to));

        println!("install: {:?} -> {:?}", from, to);
    }
}
