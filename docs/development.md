# dotkoke の開発

この文書は、コントリビューター向けの検証手順、テスト規約、ドキュメント執筆規約をまとめます。利用手順は [usage.md](usage.md)、公開挙動は [specification.md](specification.md)、内部実装の方針は [internals.md](internals.md)、メンテナー向けのリリース手順は [release.md](release.md) を参照してください。

## 標準検証

Rust コードを変更した場合は、少なくとも整形、lint、テストを確認します。

```sh
cargo fmt --all --check
cargo clippy --workspace --all-features --all-targets
cargo test
```

対象を絞って確認する場合は、関連するモジュールやテスト名のフィルタを指定します。

```sh
cargo test <module_or_test_name>
```

ドキュメント、TOML、YAML、シェルスクリプトの変更では、CI(`.github/workflows/ci.yaml`)と同じ lint を使います。ローカル検証のツールバージョンは CI に合わせます。

```sh
npx -y markdownlint-cli2 --config .markdownlint.yaml "docs/*.md"
taplo fmt --check --diff
yamllint .
bash .github/scripts/lint-sh-bash.sh # shellcheck + shfmt
```

## テストの検証範囲

仕様上の振る舞いは、一時ディレクトリを使った integration-style のテストで検証します。テストが作る一時ファイル・ディレクトリは `tempfile` で管理し、`std::env::temp_dir()` から直接パスを組み立てません(`.clippy.toml` の disallowed-methods でも強制されます)。

dotkoke のテストの網羅範囲は、少なくとも以下の挙動グループを含めます。公開挙動を変更する場合は、該当するグループのテストも同じ変更で更新してください。

- 設定の探索と検証。
- `symlink` と `copy` の一致判定。
- `drifted` の destination path の backup。
- `remove` と `add --update` の managed file の backup。
- broken symlink と相対 symlink の扱い。
- source tree の symlink、unknown file type、source tree scan error。
- `add` と `remove` の複数パス、ディレクトリ入力、重複除去。
- `add --install` が対象のファイルだけを反映すること。
- `status` の状態分類と exit code。
- `--dry-run` がファイルシステムを変更しないこと。

CLI の help、stdout、stderr の利用者向けの文字列は snapshot 的に検証します。

## ドキュメント構成と執筆規約

この節は、dotkoke のドキュメント構成と責務分担の唯一の定義です。各文書の冒頭にある 1 行の責務宣言と相互リンクは、この節の要約です。

### 文書の責務

| 文書 | 責務 |
| --- | --- |
| README.md(未作成) | 概要と最短導線(初見の利用者向け)。実装後に整備する |
| [usage.md](usage.md) | 利用者向けのインストール、クイックスタート、日常操作の基本ガイド |
| [configuration.md](configuration.md) | 設定ファイルの使い方と挙動説明のガイド |
| [specification.md](specification.md) | 公開挙動、CLI 契約、設定スキーマ、安全性要件の正本(リファレンス兼務) |
| [internals.md](internals.md) | 内部実装の方針の説明(非規範の内部設計ノート) |
| development.md(この文書) | コントリビューター向けの検証手順、テスト規約、ドキュメント執筆規約 |
| [release.md](release.md) | メンテナー向けのリリース手順書(手順確定までスケルトン) |
| [glossary.md](glossary.md) | 用語の定義と表記基準 |

### 情報の置き場所

同じ情報は 1 つの文書だけを正とし、他の文書は要約とリンクに徹します。正と矛盾する記述を他の文書に持ち込まないでください。

- 公開挙動、CLI の契約、設定スキーマと既定値、安全性要件の正は specification.md。
- インストールと日常操作の手順、安全な使い方の説明の正は usage.md。
- 設定(`[paths]` / `[source]` / `[placement]`)の使い方・挙動説明の正は configuration.md。
- 検証コマンド、テストの網羅範囲の要求、ドキュメント執筆規約の正はこの文書。
- 実装方針、モジュール構成、利用 crate など内部実装の説明は internals.md。非規範であることを明記し、挙動の約束は書かない。
- リリース手順の正は release.md。
- 用語と表記基準の正は glossary.md。本文で用語を追加・変更した場合は glossary.md も更新する。

### 執筆規約

- 地の文は日本語で書く。specification.md の本文は「である」体、その他の docs は「です・ます」体にする。CLI の出力例など利用者向けの文字列は英語のままにする。
- 公開挙動、CLI オプション、設定キー、安全性要件を変更した場合は、同じ変更で specification.md と関連する利用者向けドキュメントも更新する。
- 仕様の規範記述は specification.md だけに書く。ガイドに書いてよい仕様記述は、その場面の理解に必要な要約 1〜2 文と specification.md へのリンクまでとする。
- 仕様、docs、実装、テストが矛盾する場合は、暗黙に実装を正とせず、差分の意図を確認してから揃える。
- 実装作業ログ、マイルストーンの履歴、PR 単位の一時的な課題、エージェント用プロンプトは docs/ の文書に置かない。
- ドキュメントの構成や責務分担を変えたら、同じ変更でこの節の責務定義を追随させる。用語や表記の基準を変えたら、必要に応じて glossary.md も追随させる。

### 意図的に許容する重複

次の重複だけを意図的に許容します。「正」を更新したら、同じ変更で複製先も更新してください。この一覧にない重複は執筆時に解消します。

1. 各文書冒頭の 1 行責務宣言と相互リンク: 責務定義の正 = この節。
2. glossary.md の用語定義: 定義は 1〜2 文と specification.md へのリンクまでとし、挙動の詳細を glossary 側に複製しない。

README.md を作成する際は、README に置く要約(クイックスタートの最小形など)をこの一覧へ追加します。
