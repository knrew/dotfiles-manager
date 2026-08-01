# dotkoke 内部設計ノート

この文書は、`dotkoke` の内部実装の方針をコントリビューター向けに記録する参考情報です。公開挙動の正ではありません。挙動の契約は [specification.md](specification.md) にあり、この文書と実装が specification.md と矛盾する場合は specification.md と実装を正として、この文書を追随させます。検証手順と執筆規約は [development.md](development.md) を参照してください。

## 1. crate 構成

Cargo package は単一 crate の `dotkoke` で、バイナリターゲット `dotkoke` とライブラリターゲットを持ちます。主要な依存と用途は次のとおりです。

- `clap`(derive): CLI と引数のパース。
- `serde` + `toml`: TOML 設定のパース。
- `anyhow`: エラー型とエラーコンテキスト。
- `console` / `indicatif`: 端末出力と進捗表示(4 節)。
- `tempfile`(dev-dependency): テストの一時ディレクトリ管理。

## 2. モジュール構成と責務分離

ファイルシステムを変更するコマンドは、plan の作成と plan の実行を分離します。`--dry-run` は通常実行と同じ手順で plan を作成し、plan を表示するだけでファイルシステムを変更しません。

実装では以下の責務を分離します。

- 設定の探索、パース、検証。
- source tree と destination tree のパス解決。
- ファイルシステムの検査。
- コマンドごとの plan の作成。
- plan の実行。
- 端末出力。

source tree の走査と destination path の検査は、通常ファイル、ディレクトリ、symlink、broken symlink、unknown file type、判定不能の状態を区別します。symlink を扱う処理では、目的に応じて `symlink_metadata`、`metadata`、`read_link` を使い分けます。

## 3. パス解決の実装方針

存在が保証されたパスは canonicalize してから比較します。設定の各 root は読み込み時に、`add` と `remove` の入力パスは存在確認後に canonicalize します。未作成の backup root は、存在する最も近い祖先を canonicalize してから残りの component を連結して扱います。未作成の destination path、backup path、broken symlink は、存在する前提で canonicalize しません。

source tree の走査は symlink を辿らないため、canonical な source root に source-relative path を連結して得た managed file のパスは canonical になります。

## 4. terminal output

CLI 出力の整形には `console` を使う方針とします。`console` は端末アクセス、スタイル、ANSI の扱い、Unicode 幅の扱いを提供します。TTY と非 TTY の違いを考慮し、パイプやログファイルに出力しても読みやすいテキスト出力を維持します。

長い走査や install 実行の進捗表示には `indicatif` を使う方針とします。`indicatif` は progress bar と spinner を提供し、progress bar は通常 stderr に描画されます。通常の stdout の出力と進捗の描画が混ざらないように扱います。

進捗表示は仕様上の必須の出力ではありません。非 TTY では進捗の描画を抑制するか、単純なテキスト出力にします。

参考:

- [`console`](https://docs.rs/console/latest/console/)
- [`indicatif`](https://docs.rs/indicatif/latest/indicatif/)

## 5. エラー処理

通常実行経路では panic しません。失敗は `Result` で返し、対象のパスと失敗した操作が分かるエラーコンテキストを付けます。

plan 作成中に検出できるエラーは、実行開始前にまとめて検出します。部分的なファイルシステム変更を残さないため、source tree scan error や destination path の `blocked` 状態がある場合はファイルシステムを変更しません。

plan 実行では plan 作成時の検査結果を再検証せず、rename や作成には既存のパスを上書きしない操作を使い、前提の変化を操作の失敗として検出します。plan 実行中に失敗した場合は仕様に従って実行を停止し、実行済みのファイルシステム操作が分かるコンテキストを付けます。自動 rollback は行いません。
