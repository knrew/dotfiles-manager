# dotkoke config ガイド

この文書は、dotkoke の設定ファイルの使い方と挙動の説明をまとめた利用者向けガイドです。スキーマ、既定値、探索順序の正は [specification.md 3 章](specification.md#3-設定)です。日常操作は [usage.md](usage.md) を参照してください。

## 設定ファイルの場所と探索

設定ファイルは TOML で記述し、以下の優先順位で探索されます。

1. コマンドラインオプション `--config <PATH>`
2. 環境変数 `DOTKOKE_CONFIG`
3. `$XDG_CONFIG_HOME/dotkoke/config.toml`
4. `$HOME/.config/dotkoke/config.toml`
5. fallback config

`--config` と `DOTKOKE_CONFIG` で指定したパスが存在しない場合はエラーになります。設定ファイルがどこにもない場合は、`$HOME` から導出する fallback config が使われます。`dotkoke init` は、この探索が発見するパスに設定ファイルを作成します。探索の正確な規定は [specification.md 3.1 節](specification.md#31-設定ファイルと探索)を参照してください。

最小の設定例:

```toml
[paths]
dotfiles = "/home/user/.dotfiles"
destination = "/home/user"
backup = "/home/user/.backup_dotfiles"

[source]
root = "home"

[placement]
default_method = "symlink"
```

## `[paths]`

管理の基点になる 3 つのディレクトリを絶対パスで指定します。

- `dotfiles`: dotfiles リポジトリの root ディレクトリです。存在している必要があります。
- `destination`: managed file の配置先(destination root)です。通常はホームディレクトリです。
- `backup`: 既存ファイルの退避先(backup root)です。存在しない場合は必要になった時点で作成されます。

`destination` と source root を同じディレクトリにする構成、および `destination` を source root の配下に置く構成はエラーになります。source root が `destination` の配下にある構成(ホームディレクトリ内に dotfiles リポジトリを置く一般的な構成)は使えます。制約の正は [specification.md 3.2 節](specification.md#32-paths)を参照してください。

## `[source]`

- `root`: `paths.dotfiles` から source root への相対パスです。`{paths.dotfiles}/{source.root}` 配下が管理対象の source tree になります。
- `ignore`: managed file から除外する source-relative path の配列です。

```toml
[source]
root = "home"
ignore = [
  ".config/some-tool/local.toml",
  ".config/some-tool/cache",
]
```

`ignore` の一致判定は完全一致で、glob や正規表現は使えません。ディレクトリに一致した場合は配下の subtree がまとめて除外されます。除外されたパスは `install`、`add`、`remove`、`status` のいずれでも managed file として扱われません。スキーマの正は [specification.md 3.3 節](specification.md#33-source)を参照してください。

## `[placement]`

managed file を destination path へ配置する方法(placement method)を指定します。

- `default_method`: 既定の placement method です。`symlink`(既定)または `copy` を指定します。
- `[[placement.rules]]`: 特定の managed file にだけ別の placement method を割り当てます。

```toml
[placement]
default_method = "symlink"

[[placement.rules]]
path = ".config/some-tool/config.toml"
method = "copy"
```

- `symlink` は destination path に managed file への symlink を作成します。source tree 側での編集がそのまま destination からも見えます。
- `copy` は destination path に通常ファイルとしてコピーします。symlink を扱えないツールの設定ファイルなどに使います。destination 側で加えた変更は `dotkoke add --update` で source tree へ取り込めます。

`rules.path` は source-relative path の完全一致で、glob は使えません。同じ `path` の rule を複数書くとエラーになります。2 つの placement method の一致判定の正は [specification.md 6 章](specification.md#6-placement-method)を参照してください。

## fallback config

設定ファイルがどこにもない場合、dotkoke は fallback config で動作します。fallback config は `$HOME/.dotfiles` を dotfiles リポジトリ、`$HOME` を destination root とする既定の設定で、内容は `dotkoke init --print` で確認できます。

```sh
dotkoke init --print
```

fallback config の内容の正は [specification.md 3.5 節](specification.md#35-fallback-config)を参照してください。
