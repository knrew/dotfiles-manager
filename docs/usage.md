# dotkoke の使い方

この文書は、dotkoke のインストールと日常操作をまとめた利用者向けの基本ガイドです。設定の使い方は [configuration.md](configuration.md)、正確な公開挙動と安全性要件は [specification.md](specification.md) を参照してください。

## インストール

dotkoke の公式配布とインストール手順は未確定です。リリース手順([release.md](release.md))の確定後に、この節へ手順を記述します。

## クイックスタート

dotkoke は、dotfiles リポジトリ内の source tree に置いた通常ファイルを、ホームディレクトリなどの destination tree へ `symlink` または `copy` で反映します。

設定ファイルと dotfiles のディレクトリを作成します。

```sh
dotkoke init
```

`init` は設定ファイル(既定では `$XDG_CONFIG_HOME/dotkoke/config.toml`)と、dotfiles リポジトリの root(`$HOME/.dotfiles`)、source root(`$HOME/.dotfiles/home`)を作成します。作成される設定の内容は `dotkoke init --print` で確認できます。

管理したいファイルを source tree へ取り込みます。

```sh
dotkoke add ~/.zshrc ~/.gitconfig
```

取り込んだファイルを destination(ホームディレクトリ)へ反映します。まず `--dry-run` で plan を確認し、問題なければ実行します。

```sh
dotkoke install --dry-run
dotkoke install
```

既定の placement method は `symlink` なので、`install` は `~/.zshrc` を dotfiles リポジトリ内の実体への symlink に置き換えます。置き換え前のファイルは削除されず、backup root(既定では `$HOME/.backup_dotfiles`)配下へ移動されます。

状態を確認します。

```sh
dotkoke status
```

## コマンド

各コマンドの正確な契約と全オプションは [specification.md 2 章](specification.md#2-cli)を参照してください。変更を伴うコマンドはすべて `--dry-run` を受け付け、plan の表示だけを行います。

### `dotkoke init`

設定ファイルと source root のディレクトリを作成します。作成先に設定ファイルが既に存在する場合はエラーになり、何も変更しません。

```sh
dotkoke init
dotkoke init --print   # fallback config を表示するだけで、何も作成しない
```

### `dotkoke install`

source tree 全体を destination へ反映します。既存のファイルと衝突した場合は、backup へ退避してから配置します。

```sh
dotkoke install --dry-run
dotkoke install
```

### `dotkoke add`

destination(ホームディレクトリなど)にあるファイルを source tree へ取り込みます。ディレクトリを指定すると配下の通常ファイルをまとめて取り込みます。

```sh
dotkoke add ~/.config/nvim         # ディレクトリごと取り込む
dotkoke add --install ~/.zshrc     # 取り込みと placement を 1 つの plan で行う
dotkoke add --update ~/.gitconfig  # copy 配置のファイルの変更を source 側へ反映する
```

- 通常の `add` は source root 側にファイルを作るだけで、destination 側は変更しません。`symlink` 配置のファイルは取り込み後 `drifted` になるため、続けて `dotkoke install` を実行するか、最初から `--install` を使ってください。
- `--update` は `copy` 配置の managed file だけを対象に、destination 側の変更を source 側へ反映します。

### `dotkoke remove`

source root 配下の managed file を削除します。

```sh
dotkoke remove --dry-run ~/.dotfiles/home/.zshrc
dotkoke remove ~/.dotfiles/home/.zshrc
```

`symlink` 配置では、destination 側の symlink がその managed file を指している場合だけ symlink も削除されます。`copy` 配置では destination 側のファイルは残ります。どちらの場合も、destination 側を触らなかったこと・残したことは出力に明示されます。

### `dotkoke status`

managed file ごとに destination の状態(`ok` / `missing` / `drifted` / `blocked` / `unsupported`)を表示する読み取り専用のコマンドです。

```sh
dotkoke status
```

`drifted` は差分がある状態を示すだけで、解決方向は選ばれません。managed file の内容を destination へ反映するなら `dotkoke install`、`copy` 配置の destination 側の変更を取り込むなら `dotkoke add --update` を使います。

## 設定

設定ファイルは TOML で、`[paths]`(基点のディレクトリ)、`[source]`(source root と除外)、`[placement]`(`symlink` / `copy` の割り当て)を指定します。設定ファイルがない場合は `$HOME` から導出される fallback config で動作します。各設定の使い方は [configuration.md](configuration.md) を参照してください。

## 安全な使い方

dotkoke は利用者の既存データを失わせないことを最優先に設計されています([specification.md 10 章](specification.md#10-安全性要件))。

- 変更を伴うコマンドはすべて `--dry-run` で plan を事前確認できます。dry-run と通常実行は同じ plan に基づくため、確認した内容と実行される操作は一致します。
- `install` が既存のファイルを置き換える場合、削除ではなく backup root 配下の backup set directory(実行ごとの `YYYYmmdd_HHMMSS` 形式のディレクトリ)へ移動します。想定外の置き換えが起きた場合も、backup から手動で復元できます。
- plan の作成時に検出できるエラーがある場合、ファイルシステムは一切変更されません。実行中に操作が失敗した場合はその場で停止し、実行済みの操作と backup へ移動したパスを出力から確認できます。自動 rollback は行われません。
