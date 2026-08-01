# dotkoke 用語集

この用語集は、dotkoke のドキュメントで使う用語を定義します。ドキュメントを編集するときは、ここにある語を優先してください。ドキュメント全体の構成と執筆規約は [development.md](development.md#ドキュメント構成と執筆規約) を参照してください。

## 記載基準

用語の各節に載せる語は、次のいずれかに該当し、かつ複数のドキュメントで使われる語とします。

- dotkoke が導入した固有の概念・機能名。
- 一般的な用法や外部仕様の用法と意味・範囲が異なる、または dotkoke が限定した意味で使う用語。
- dotkoke 内に似た概念が併存し、混同しやすい用語。対になる語を揃えて載せ、区別が分かる定義を書く。

次の内容は載せません。

- 外部仕様の用語で、dotkoke が標準的な意味のまま使うもの(TOML、XDG など)。
- 単一の文書内でしか使わない術語。その文書内で定義します。
- 挙動の説明。各語の定義は 1〜2 文と正本へのリンクまでとし、挙動の正は [specification.md](specification.md) に置きます。

## パスとツリーの用語

- source root: `paths.dotfiles` と `source.root` から決まる、管理対象の元ファイルを置く root ディレクトリ。具体的には `{paths.dotfiles}/{source.root}` である([specification.md 3.3 節](specification.md#33-source))。
- source tree: source root 配下のファイルシステムの tree。dotkoke はこの tree を走査して managed file を見つける([specification.md 5 章](specification.md#5-managed-file))。
- source-relative path: source root から見た相対パス。managed file の識別、`source.ignore`、placement rule の照合に使う。
- destination root: managed file の配置先の root ディレクトリ。通常は利用者のホームディレクトリである。
- destination tree: destination root 配下のファイルシステムの tree。dotkoke は managed file に対応する destination path だけを扱い、destination tree 全体から未管理のファイルを探すことはしない。
- destination path: managed file の配置先のパス。存在しないこともあり、存在する場合も通常ファイル、ディレクトリ、symlink、broken symlink、unknown file type などになりうる。
- destination-relative path: destination root から見た相対パス。destination path と backup path の対応付けに使う。
- canonical path: symlink をすべて解決した絶対パス。パスの同一性判定と包含判定に使う([specification.md 4 章](specification.md#4-パス解決))。

## managed file と placement の用語

- managed file: source root 配下の通常ファイルのうち、`source.ignore` に一致しないファイル。dotkoke が `install`、`add`、`remove`、`status` で管理対象として扱う単位([specification.md 5 章](specification.md#5-managed-file))。
- excluded path: `source.ignore` によって managed file から除外される source-relative path。どのコマンドでも managed file として扱わない([specification.md 3.3 節](specification.md#33-source))。
- placement method: managed file を destination path に配置する方法。指定できる値は `symlink` と `copy` である([specification.md 6 章](specification.md#6-placement-method))。
- placement rule: managed file の source-relative path に placement method を割り当てる設定。一致する placement rule がない managed file には `placement.default_method` を使う([specification.md 3.4 節](specification.md#34-placement))。
- desired state: placement method によって定まる destination path のあるべき状態。`install` は destination path を desired state に近づける。
- permission bits: file mode のうち permission を表す bits 全体。setuid、setgid、sticky bit を含み、owner、group、xattr、ACL は含まない。

## 判定と状態の用語

- file kind: パスの種類の判定結果。通常ファイル、ディレクトリ、symlink、unknown file type、存在しない、のいずれかに分類する。
- broken symlink: target を解決できない symlink。target が存在しない場合と、解決が symlink loop になる場合を含む。
- unknown file type: 通常ファイル、ディレクトリ、symlink のいずれでもないファイル種別。FIFO、socket、device などを含む。
- 判定不能: 権限エラーなどにより、パスの存在、file kind、または一致判定を確定できない状態。パスが存在しないことが確定している状態とは区別する。
- source tree scan error: source tree の走査を不完全にする問題。読み取れないディレクトリ、エントリの読み取り失敗、file kind の判定不能を含む([specification.md 5 章](specification.md#5-managed-file))。
- drifted: destination path が存在し、desired state と一致していない状態。`install` はこの destination path を backup path へ移動してから desired state を作成し、`status` は `drifted` として表示する([specification.md 7 章](specification.md#7-destination-path-と-drifted))。
- status state: `status` が destination path ごとに表示する状態。`ok`、`missing`、`drifted`、`blocked`、`unsupported` の 5 値がある([specification.md 2.6 節](specification.md#26-status))。

## backup の用語

- backup root: backup を保存する root ディレクトリ。`paths.backup` で指定する([specification.md 8 章](specification.md#8-backup))。
- backup set directory: 1 回の実行で作成される backup 用のディレクトリ。backup root 配下に作成され、その実行で backup されるパスをまとめて保持する。
- backup path: backup される destination path や managed file の移動先のパス。destination-relative path または source-relative path を backup set directory 配下に維持する([specification.md 8 章](specification.md#8-backup))。

## plan と設定の用語

- plan: 実行のたびに作成されるファイルシステム操作の一覧。`--dry-run` と通常実行は同じ plan 作成手順に基づく([specification.md 10 章](specification.md#10-安全性要件))。
- dry-run: plan を表示し、ファイルシステムを変更しない実行モード。
- fallback config: 設定ファイルの探索で設定ファイルが見つからない場合に使う、`$HOME` から導出する既定の設定。`init` が生成する設定と同等である([specification.md 3.5 節](specification.md#35-fallback-config))。

## 表記基準

- CLI の名前付きの指定は「オプション」と書き、「フラグ」や flag は使わない。
- コマンドの操作名は「コマンド」と書き、「サブコマンド」は使わない。
- source-relative path と destination-relative path はハイフン付きの形で書く。
- dry-run はハイフン付きの形で書く。
- backup set directory、backup root、backup path を使い分け、まとめて backup directory と曖昧に書かない。
- ファイルシステム上の種別は symlink と書き、symbolic link という表記は使わない。
