use std::{fs, os::unix, path::Path};

use anyhow::{Context, Result, anyhow};

use crate::{config::Config, file_collector::collect_files_and_links, file_kind::*};

pub fn install(config: Config) -> Result<()> {
    let Config {
        dotfiles_dir,
        home_dir,
        backup_dir,
    } = config;

    // dotfiles/home/
    let dotfiles_home = dotfiles_dir
        .join("home")
        .canonicalize()
        .with_context(|| format!("invalid path: {}/home/", dotfiles_dir.display()))?;

    if !dotfiles_home.is_dir() {
        return Err(anyhow!("{} is not a directory.", dotfiles_home.display()));
    }

    fs::create_dir_all(&backup_dir)
        .with_context(|| format!("failed to create backup dir: {}", backup_dir.display()))?;

    let (files, links) = collect_files_and_links(&dotfiles_home)?;

    if !links.is_empty() {
        eprintln!(
            "[warning] symlink(s) exist in {} (they will be ignored).",
            dotfiles_home.display()
        );
    }
    let _ = links;

    for from in files {
        assert!(!from.is_symlink());

        let suffix = from.strip_prefix(&dotfiles_home)?;
        let to = home_dir.join(suffix);

        // fromのリンクをtoにつくる．

        // すでに正しいリンクが貼られていたらスキップ．
        if is_symlink_pointing_to(&to, &from)? {
            println!(
                "skipped (already linked): {} -> {}",
                from.display(),
                to.display()
            );

            continue;
        }

        match file_kind(&to)? {
            FileKind::Dir => {
                // ディレクトリは削除．
                // TODO: 将来的にはバックアップをとるよう修正．
                fs::remove_dir_all(&to)
                    .with_context(|| format!("failed to remove dir: {}", to.display()))?;
            }
            FileKind::Symlink => {
                // 既存のリンクは削除．
                fs::remove_file(&to)
                    .with_context(|| format!("failed to remove symlink: {}", to.display()))?;
            }
            FileKind::File => {
                // 既存のファイルはバックアップを作成．
                let backup = backup_dir.join(suffix);
                create_parent_dir(&backup)?;
                fs::rename(&to, &backup).with_context(|| {
                    format!("failed to rename: {} -> {}", to.display(), backup.display())
                })?;
            }
            FileKind::Unknown => {
                eprintln!("[warning] unknown file type: {}", to.display());
                fs::remove_file(&to)
                    .with_context(|| format!("failed to remove path: {}", to.display()))?;
            }
            FileKind::NotFound => {}
        }

        create_parent_dir(&to)?;
        unix::fs::symlink(&from, &to).with_context(|| {
            format!(
                "failed to create link: {} -> {}",
                from.display(),
                to.display()
            )
        })?;

        println!("created link: {} -> {}", from.display(), to.display());
    }

    // バックアップディレクトリが空なら削除する．
    if is_empty_dir(&backup_dir)? {
        fs::remove_dir(backup_dir)?
    }

    Ok(())
}

/// 親ディレクトリを作成する(pathに対する操作を行えるようにするため)．
fn create_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create parent directory: {}", parent.display()))?;
    }
    Ok(())
}

fn is_symlink_pointing_to(link: &Path, path: &Path) -> Result<bool> {
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

fn is_empty_dir(path: &Path) -> Result<bool> {
    Ok(fs::read_dir(path)?.next().is_none())
}
