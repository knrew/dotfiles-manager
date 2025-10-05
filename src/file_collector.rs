use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::file_kind::*;

/// 指定したパス以下を再帰的に探索し，通常ファイルとシンボリックリンク(壊れたリンクを含む)を収集する．
///
/// # 引数
///
/// - `path`: 探索を開始するディレクトリまたはファイルのパス
///
/// # 返り値
///
/// `(files, links)`のタプル:
/// - `files`: 通常ファイルのパス一覧
/// - `links`: シンボリックリンクのパス一覧
///
/// # NOTE
/// - 引数で指定したパスが `files` または `links` に入る可能性がある．
/// - シンボリックリンク，通常ファイル，ディレクトリのどれでもないパスは
///   `Unknown file` として警告を出力して無視する．
/// - ディレクトリへのシンボリックリンクは辿らない．
pub fn collect_files_and_links(path: impl AsRef<Path>) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut files = vec![];
    let mut links = vec![];

    let mut stack = vec![path.as_ref().to_path_buf()];

    while let Some(path) = stack.pop() {
        match file_kind(&path) {
            FileKind::Symlink => {
                // 壊れたリンクも収集．
                links.push(path);
            }
            FileKind::File => {
                files.push(path);
            }
            FileKind::Dir => match fs::read_dir(&path) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(e) => stack.push(e.path()),
                            Err(e) => eprintln!(
                                "[warning] failed to read entry in {}: {}",
                                path.display(),
                                e
                            ),
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[warning] failed to read_dir {}: {}", path.display(), e);
                }
            },
            FileKind::Unknown => {
                eprintln!("[warning] unknown file type: {}", path.display());
            }
            FileKind::Error => {
                eprintln!("[warning] error path: {}", path.display());
            }
            FileKind::NotFound => {
                eprintln!("[warning] not found: {}", path.display());
            }
        }
    }

    files.sort_unstable();
    files.dedup();
    links.sort_unstable();
    links.dedup();

    Ok((files, links))
}
