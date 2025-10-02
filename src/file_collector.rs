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
/// - 引数で指定したパスが`files`または`links`に入る可能性がある．
/// - シンボリックリンク，通常ファイル，ディレクトリのどれでもないパスは`Unkown file`として警告を出力して無視する．
pub fn collect_files_and_links(path: impl AsRef<Path>) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut files = vec![];
    let mut links = vec![];

    let mut stack = vec![path.as_ref().to_path_buf()];

    while let Some(path) = stack.pop() {
        match file_kind(&path)? {
            FileKind::Symlink => {
                links.push(path);
            }
            FileKind::Dir => {
                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            stack.push(entry.path());
                        }
                    }
                }
            }
            FileKind::File => {
                files.push(path);
            }
            FileKind::Unknown => {
                eprintln!("[warning] unknown file type: {}", path.display());
            }
            FileKind::NotFound => {
                // ここには到達しない想定．
                eprintln!("[warning]not found: {}", path.display());
            }
        }
    }

    files.sort_unstable();
    links.sort_unstable();

    Ok((files, links))
}
