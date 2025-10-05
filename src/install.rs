use anyhow::Result;

use crate::{executor::Executor, file_collector::*, file_kind::*};

pub fn install(executor: impl Executor) -> Result<()> {
    let (files, links) = collect_files_and_links(executor.dotfiles_home_dir())?;

    if !links.is_empty() {
        eprintln!(
            "[warning] symlink(s) exist in {} (they will be ignored).",
            executor.dotfiles_home_dir().display()
        );
    }
    drop(links);

    for from in files {
        assert!(!is_symlink(&from));

        let to = executor.install_path(&from)?;

        // fromのリンクをtoにつくる．

        if is_symlink_pointing_to(&to, &from) {
            executor.skip_link_creating(&from, &to)?;
            continue;
        }

        match file_kind(&to) {
            FileKind::Symlink => {
                executor.remove_symlink_from_home(&to)?;
            }
            FileKind::File => {
                executor.remove_file_from_home(&to)?;
            }
            FileKind::Dir => {
                // TODO: 将来的にはバックアップをとるよう修正予定．
                executor.remove_dir_all_from_home(&to)?;
            }
            FileKind::Unknown => {
                executor.remove_unknown_path_from_home(&to)?;
            }
            FileKind::NotFound => {}
            FileKind::Error => {
                executor.warn_cannot_determine(&to)?;
                continue;
            }
        }

        executor.create_symlink(&from, &to)?;
    }

    Ok(())
}
