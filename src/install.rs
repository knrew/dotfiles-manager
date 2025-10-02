use anyhow::Result;

use crate::{
    config::Config, file_collector::collect_files_and_links, file_kind::*, file_operations::*,
};

pub fn install(config: Config) -> Result<()> {
    let Config {
        dotfiles_dir: _dotfiles_dir,
        home_dir,
        backup_dir,
        dotfiles_home_dir,
    } = config;

    let (files, links) = collect_files_and_links(&dotfiles_home_dir)?;

    if !links.is_empty() {
        eprintln!(
            "[warning] symlink(s) exist in {} (they will be ignored).",
            dotfiles_home_dir.display()
        );
    }
    let _ = links;

    for from in files {
        assert!(!from.is_symlink());

        let suffix = from.strip_prefix(&dotfiles_home_dir)?;
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
                remove_dir_all(&to)?;
            }
            FileKind::Symlink => {
                // 既存のリンクは削除．
                remove_link(&to)?;
            }
            FileKind::File => {
                // 既存のファイルはバックアップを作成．
                let backup = backup_dir.join(suffix);
                rename(&to, &backup)?;
            }
            FileKind::Unknown => {
                eprintln!("[warning] unknown file type: {}", to.display());
                remove_unknown(&to)?
            }
            FileKind::NotFound => {}
        }

        create_symlink(&from, &to)?;
        println!("created link: {} -> {}", from.display(), to.display());
    }

    Ok(())
}
