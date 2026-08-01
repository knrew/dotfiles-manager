# dotkoke 仕様

この文書は、`dotkoke` の公開挙動、CLI の契約、設定スキーマ、安全性要件の正本である。外部から観測・依存できる契約を定義し、内部モジュール構成、実装方針、利用する Rust crate の選定理由は扱わない。利用手順と操作例は [usage.md](usage.md)、設定の使い方は [configuration.md](configuration.md)、開発・検証手順は [development.md](development.md) を参照する。

この文書で使う主要用語は [glossary.md](glossary.md) で定義する。本文では glossary の定義に従う。

## 1. スコープ

### 1.1 目的

`dotkoke` は、Linux と macOS を対象とする dotfiles 管理 CLI である。dotfiles リポジトリ内の source tree に置いた通常ファイルを destination tree へ反映する。

dotkoke は利用者の既存データを失わせないことを最優先する。安全性要件は [10 章](#10-安全性要件)に定義する。

### 1.2 対応する挙動

- `init`、`install`、`add`、`remove`、`status` の 5 コマンド。
- TOML の設定ファイル。`--config`、環境変数 `DOTKOKE_CONFIG`、`$XDG_CONFIG_HOME`、fallback config による探索。
- placement method `symlink` / `copy` による managed file の配置。
- 既存の destination path の backup set directory への退避。
- `--dry-run` による plan の事前確認。

### 1.3 対象外

- `list` と `clean` は提供しない。この仕様に定義しないコマンドは CLI に存在しない。
- machine-readable な出力は初期仕様では提供しない([9.1 節](#91-出力))。
- destination tree 全体を走査して未管理のファイルを探す機能([2.6 節](#26-status))。
- 実行済みのファイルシステム操作の自動 rollback([10 章](#10-安全性要件))。
- `init` による Git リポジトリの作成([2.2 節](#22-init))。

## 2. CLI

### 2.1 共通形式

```text
dotkoke init [--dry-run]
dotkoke init --print
dotkoke install [--config <PATH>] [--dry-run]
dotkoke add [--config <PATH>] [--dry-run] [--install | --update] <PATH>...
dotkoke remove [--config <PATH>] [--dry-run] <PATH>...
dotkoke status [--config <PATH>]
```

- CLI の help、出力、ログ、エラーメッセージは英語で表示する。
- `--config <PATH>` は設定ファイルを明示する。設定ファイルの探索は [3.1 節](#31-設定ファイルと探索)に定義する。`init` は設定ファイルを読み込まない。
- `--dry-run` は plan を表示し、ファイルシステムを変更しない。dry-run と通常実行は同じ plan に基づく([10 章](#10-安全性要件))。
- exit code の契約は [9.2 節](#92-exit-code)に定義する。

### 2.2 `init`

```text
dotkoke init [--dry-run]
dotkoke init --print
```

`init` は設定ファイルと source root のディレクトリを作成する。`init` は設定ファイルを読み込まない。

設定ファイルの作成先は以下の順で決める。

1. 環境変数 `DOTKOKE_CONFIG` が設定されている場合はそのパス。
2. `$XDG_CONFIG_HOME/dotkoke/config.toml`。
3. `XDG_CONFIG_HOME` が未設定、空文字、または相対パスの場合は `$HOME/.config/dotkoke/config.toml`。

`DOTKOKE_CONFIG` が空文字の場合は未設定として扱う。
`DOTKOKE_CONFIG` が相対パスの場合はカレントディレクトリ基準で解決する。

作成先は、設定ファイルの探索([3.1 節](#31-設定ファイルと探索))が設定ファイルを発見するパスと一致させる。同じ環境変数とカレントディレクトリのもとで `--config` を指定しない直後の他コマンドは、`init` が作成した設定ファイルを発見する。

`HOME` が未設定、空文字、または相対パスの場合、`init` はエラーとする。生成される設定の内容が `$HOME` に依存するためである。

作成先に設定ファイルが既に存在する場合はエラーとし、ファイルシステムを変更しない。
設定ファイルの親ディレクトリは必要なら作成する。

生成される設定は fallback config([3.5 節](#35-fallback-config))と同等である。
設定に書かれるパスは `$HOME` 展開済みの絶対パスである。
設定には環境変数や `~` を書かない。

`init` は `paths.dotfiles` と source root が存在しない場合は作成する。
既存のパスがディレクトリでない場合はエラーとする。
`init` は Git リポジトリを作成しない。

`--dry-run` は作成予定を表示し、ファイルシステムを変更しない。

`--print` は `$HOME` 展開済みの fallback config を stdout に出力するだけで、ファイルシステムを変更しない。
`HOME` のエラー規定は `--print` にも適用する。
`--print` は `--dry-run` と併用できない。

### 2.3 `install`

```text
dotkoke install [--config <PATH>] [--dry-run]
```

`install` は source tree 全体を destination root に反映する。
`source.ignore` に一致するパスは plan に含めず、destination path の作成、コピー、リンク作成、backup path への移動を行わない。
destination path が source root や backup root 自身、またはその配下になる managed file がある場合は、plan 作成時のエラーとし、ファイルシステムを変更しない。
各 managed file の placement method は placement rule と既定の method から決定する([3.4 節](#34-placement))。

`install` は実行前に plan を作成する。
plan 作成時に検出可能なエラーがある場合、ファイルシステムを変更しない。

`--dry-run` は plan を表示し、ファイルシステムを変更しない。

### 2.4 `add`

```text
dotkoke add [--config <PATH>] [--dry-run] [--install | --update] <PATH>...
```

`add` は destination root 配下の通常ファイルを source root 配下へ取り込む。
`<PATH>...` は 1 個以上指定する。
指定されたパスが存在しない場合はエラーとする。

ファイルが指定された場合、そのファイルを対象にする。
ディレクトリが指定された場合、その配下を再帰走査し、通常ファイルを対象にする。
symlink は辿らず、警告を出して対象から除外する。
unknown file type は警告を出して対象から除外する。
読み取れないディレクトリ、エントリの読み取り失敗、file kind の判定不能はエラーとする。

相対パスが指定された場合はカレントディレクトリ基準で解決する。
解決後のパスが destination root 配下でない場合はエラーとする。
解決後のパスが source root 自身またはその配下にある場合はエラーとする。
解決後のパスが backup root 自身またはその配下にある場合もエラーとする。
ディレクトリの再帰走査中に source root または backup root に到達した場合は、その subtree を走査せず警告を出して対象から除外する。

対応する source-relative path が `source.ignore` のパスと同じ、または `source.ignore` のパスをディレクトリとみなした subtree 配下にあるファイルは警告を出して対象から除外し、source root 側のファイルを作成しない。

対象のファイルは destination-relative path 昇順で安定処理する。
同じファイルが複数回対象になった場合は重複を除去する。

通常の `add` は、対応する source root 側のパスが存在しない場合だけコピーする。
コピーは destination path の内容のバイト列と permission bits を複製する。
対応する source root 側のパスに何か存在する場合は上書きせず対象から除外する。
対応する source root 側のパスの親ディレクトリは必要なら作成する。

通常の `add` は destination root 側を変更しない。
placement rule も変更しない。

通常の `add` で source root 側のファイルを新規作成した対象が `copy` 配置の場合、取り込み後の destination path は desired state と一致する。
通常の `add` で source root 側のファイルを新規作成した対象が `symlink` 配置の場合、destination path は通常ファイルのまま残るため、取り込み後は `drifted` になる。

`--update` が指定された場合、対応する source root 側のパスが既存の managed file である対象だけを更新する。
対応する source root 側のパスが managed file でない対象は、`source.ignore` に一致する対象を含め、警告を出して対象から除外する。
`--update` は `copy` 配置の managed file だけを対象にする。
`symlink` 配置の managed file は警告を出して対象から除外する。
destination path が通常ファイルでない場合は警告を出して対象から除外し、symlink は辿らない。
`--update` はマージを行わず、destination path の内容のバイト列と permission bits を managed file に反映する。

`--install` が指定された場合、この `add` で対象になったファイルだけに `install` と同じ placement 処理を適用する。
取り込みと placement は 1 つの plan として作成する。
placement 側に plan 作成時に検出可能なエラーがある場合、取り込みを含めてファイルシステムを変更しない。
`source.ignore` に一致して警告を出して対象から除外されたファイルには、`install` と同じ placement 処理も適用しない。
全 managed file への反映は行わない。

`--install` と `--update` は併用できない。

`--dry-run` は取り込み予定を表示し、ファイルシステムを変更しない。
`--dry-run --install` は取り込み予定と install 予定の両方を表示し、ファイルシステムを変更しない。
`--dry-run --update` は更新予定を表示し、ファイルシステムを変更しない。

### 2.5 `remove`

```text
dotkoke remove [--config <PATH>] [--dry-run] <PATH>...
```

`remove` は source root 配下の managed file を削除する。
`<PATH>...` は 1 個以上指定する。
指定されたパスが存在しない場合はエラーとする。

ファイルが指定された場合、その managed file を対象にする。
ディレクトリが指定された場合、その配下を再帰走査し、managed file を対象にする。
symlink は警告を出して対象から除外する。
unknown file type は警告を出して対象から除外する。
読み取れないディレクトリ、エントリの読み取り失敗、file kind の判定不能はエラーとする。

相対パスが指定された場合はカレントディレクトリ基準で解決する。
対象のパスが source root 配下でない場合はエラーとする。

対象のパスが `source.ignore` のパスと同じ、または `source.ignore` のパスをディレクトリとみなした subtree 配下にある場合は警告を出して対象から除外し、source root と destination root のどちらも変更しない。

対象のファイルは source-relative path 昇順で安定処理する。
同じファイルが複数回対象になった場合は重複を除去する。

`symlink` 配置の managed file を remove する場合、destination path がその managed file を指す symlink なら destination path の symlink を削除する。
destination path の symlink を削除してから source root 側の managed file を削除する。
destination path が broken symlink を含む別の symlink、通常ファイル、ディレクトリ、unknown file type の場合は触らない。
触らなかったことを出力で明示する。

`copy` 配置の managed file を remove する場合、destination path は削除しない。
destination path を残したことを出力で明示する。

source root 側の managed file は削除する。

`--dry-run` は削除予定を表示し、ファイルシステムを変更しない。

### 2.6 `status`

```text
dotkoke status [--config <PATH>]
```

`status` は読み取り専用である。
source tree の managed file を基準に destination path の状態を表示する。
`source.ignore` に一致するパスは表示しない。
destination root 全体を走査して未管理のファイルを探すことはしない。

`status` はテキスト出力のみ提供する。
JSON 出力は提供しない。

状態は以下のいずれかである。

- `ok`: destination path が desired state と一致している。
- `missing`: destination path が存在せず、親パスが作成可能である。
- `drifted`: destination path が存在するが desired state と一致していない。
- `blocked`: destination path 自体またはその親パスに install を妨げる問題がある。
- `unsupported`: source tree 内に存在するが、symlink や unknown file type などのため managed file にならない。

`ok` の定義は placement method によって異なる。
`symlink` では destination path の symlink が managed file の canonical path を指している場合に `ok` とする。
`copy` では destination path の通常ファイルの内容のバイト列と permission bits が managed file と一致する場合に `ok` とする。

`copy` の `drifted` は、destination path の通常ファイルの内容のバイト列または permission bits が managed file と異なる状態を含む。

`drifted` は差分が存在する状態であり、`status` は解決方向を選ばない。
managed file を desired state として destination path へ反映する場合は `install` を使う。
`copy` 配置の destination path の通常ファイルを managed file へ反映する場合は `add --update <PATH>` を使う。
`blocked` は `install` できない状態である。
`unsupported` は source tree の symlink や unknown file type などを表す。
`source.ignore` によって除外されたパスは `unsupported` として表示しない。

表示順は source-relative path 昇順とする。
表示は行ごとに status state と source-relative path を示し、必要に応じて placement method と補足理由を添える。
`drifted` の補足理由は、内容の差分、permission bits の差分、file kind の不一致、symlink target の不一致を区別する。
summary は常に表示し、`ok`、`missing`、`drifted`、`blocked`、`unsupported` の順で件数を示す。
表示文言の詳細はこの仕様では固定しない。

例:

```text
Status:
  unsupported  .config/app/link (source symlink)
  missing      .config/foo/config.toml (copy)
  drifted      .gitconfig (copy, content differs)
  blocked      .local/share/tool/config.toml (symlink, parent is a file: /home/me/.local)
  ok           .zshrc (symlink)

Summary: 1 ok, 1 missing, 1 drifted, 1 blocked, 1 unsupported
```

判定に成功した場合、差分や `drifted` が存在しても exit code は 0 とする。
destination path の検査や一致判定の判定不能は `blocked` として表示し、エラーとして扱わない。
設定エラーまたは source tree scan error がある場合は exit code を非 0 とする。

## 3. 設定

設定ファイルは TOML で記述する。

```toml
[paths]
dotfiles = "/path/to/dotfiles"
destination = "/home/user"
backup = "/home/user/.backup_dotfiles"

[source]
root = "home"
ignore = [
  ".config/some-tool/local.toml",
  ".config/some-tool/cache",
]

[placement]
default_method = "symlink"

[[placement.rules]]
path = ".config/some-tool/config.toml"
method = "copy"
```

### 3.1 設定ファイルと探索

設定ファイルの探索は、設定を読み込むコマンド `install`、`add`、`remove`、`status` に適用される。
`init` は設定ファイルを読み込まない。

設定ファイルは以下の優先順位で探索する。

1. コマンドラインオプション `--config <PATH>`
2. 環境変数 `DOTKOKE_CONFIG`
3. `$XDG_CONFIG_HOME/dotkoke/config.toml`
4. `$HOME/.config/dotkoke/config.toml`
5. fallback config([3.5 節](#35-fallback-config))

`--config` または `DOTKOKE_CONFIG` で指定されたパスが存在しない場合はエラーとする。
指定されたパスがファイルでない場合もエラーとする。
`--config` または `DOTKOKE_CONFIG` に相対パスが指定された場合はカレントディレクトリ基準で解決する。
`DOTKOKE_CONFIG` が空文字の場合は未設定として扱い、次の探索へ進む。
3 と 4 の探索先のパスが存在してファイルでない場合もエラーとし、次の探索へ進まない。

`XDG_CONFIG_HOME` が未設定、空文字、または相対パスの場合は無視し、`$HOME/.config/dotkoke/config.toml` の探索へ進む。

`HOME` が未設定、空文字、または相対パスの場合、`$HOME/.config/dotkoke/config.toml` の探索と fallback config は使えない。
探索がこれらの段階に到達した場合はエラーとする。

### 3.2 `[paths]`

`paths.dotfiles` は dotfiles リポジトリの root ディレクトリを表す。
絶対パスでなければならない。
存在するディレクトリでなければならない。

`paths.destination` は destination root を表す。
絶対パスでなければならない。
存在するディレクトリでなければならない。

`paths.backup` は backup root を表す。
絶対パスでなければならない。
存在しない場合は必要になった時点で作成される。
存在する場合はディレクトリでなければならない。

`paths.destination` と source root が同じディレクトリであってはならない。
`paths.destination` が source root の配下にあってもならない。
source root が `paths.destination` の配下にある構成は許容する。
これらの判定は canonical path で行う。

### 3.3 `[source]`

`source.root` は `paths.dotfiles` から source root のディレクトリへの相対パスである。
空文字、`.`、絶対パス、`..` を含むパスは設定エラーとする。
`{paths.dotfiles}/{source.root}` は存在するディレクトリでなければならない。

`source.ignore` は managed file から除外する source-relative path を表す。
省略時の既定値は空配列である。

`source.ignore` の各要素は source-relative path である。
glob、正規表現、negation は使えない。
一致判定は完全一致で行う。
絶対パス、空文字、`.`、`..` を含むパスは設定エラーとする。

`source.ignore` のパスが通常ファイルとして存在する場合、そのファイルは managed file にしない。
`source.ignore` のパスがディレクトリとして存在する場合、そのディレクトリと配下の subtree は managed file にしない。
`source.ignore` のパスが存在しない場合は設定エラーにしない。
`source.ignore` のパスが symlink や unknown file type に一致する場合も設定エラーにしない。
subtree の一致判定は path component の境界で行う。

`source.ignore` によって除外されたパスは excluded path であり、`install`、`add`、`remove`、`status` のいずれでも managed file として扱わない。

### 3.4 `[placement]`

`placement.default_method` は placement rule に一致しない managed file の placement method を表す。
指定できる値は `symlink` と `copy` である。
省略時の既定値は `symlink` である。

`[[placement.rules]]` は managed file ごとの placement method を指定する。

`placement.rules.path` は source-relative path である。
glob は使えない。
一致判定は完全一致で行う。
絶対パス、空文字、`.`、`..` を含むパスは設定エラーとする。

`placement.rules.method` に指定できる値は `symlink` と `copy` である。

同じ `path` を持つ placement rule が複数存在する場合は設定エラーとする。
placement rule が存在しない managed file は `placement.default_method` を使う。

`placement.rules.path` が `source.ignore` のパスと同じ、または `source.ignore` のパスをディレクトリとみなした subtree 配下にある場合は設定エラーとする。
これ以外では、どの managed file にも一致しない placement rule は設定エラーにしない。
rule のパスが存在しない場合やディレクトリを指す場合も同様である。

### 3.5 fallback config

fallback config は以下と同等である。

```toml
[paths]
dotfiles = "$HOME/.dotfiles"
destination = "$HOME"
backup = "$HOME/.backup_dotfiles"

[source]
root = "home"

[placement]
default_method = "symlink"
```

fallback config のパスは実行時に展開済みの絶対パスとして扱う。

## 4. パス解決

パスの同一性判定と包含判定は canonical path で行う。

`paths.dotfiles`、`paths.destination`、source root は設定の読み込み時に canonicalize する。
`paths.backup` は存在する場合は canonicalize し、存在しない場合は指定された絶対パスをそのまま backup root として使う。

`add` と `remove` の `<PATH>` は、存在と file kind を確認したうえで canonicalize してから包含判定を行う。
`<PATH>` 自体が symlink の場合は canonicalize せず、各コマンドの規定に従って除外する。
`<PATH>` の親パスに含まれる symlink は canonicalize で解決する。

存在しない可能性のあるパスは canonicalize しない。
managed file の destination path は、canonical な destination root に source-relative path を連結したパスである。
backup path は、backup set directory に destination-relative path を連結したパスである。
destination path とその親パスの検査は、symlink を辿らず path component 単位で行う。

パスの文字列比較はバイト単位で行い、ファイルシステムの case-insensitivity は考慮しない。

## 5. managed file

dotkoke は source root 配下を再帰的に走査する。
`source.ignore` に一致するパスは走査対象から除外する。
excluded path がディレクトリの場合、その中は走査しない。

通常ファイルだけを managed file とする。
ディレクトリは走査対象であり、managed file ではない。
symlink は辿らず、managed file にしない。
FIFO、socket、device など、通常ファイル、ディレクトリ、symlink のいずれでもないファイル種別は managed file にしない。

`source.ignore` によって除外されたパスは、警告や `status` の `unsupported` にはしない。
source tree の symlink と unknown file type は、`source.ignore` に一致しない場合、警告または `status` の `unsupported` として扱う。
読み取れないディレクトリ、エントリの読み取り失敗、file kind の判定不能など、走査が不完全になる問題はエラーとする。
source tree scan error がある場合、部分的なファイルシステム変更を残さないためファイルシステムを変更しない。

managed file の処理順は source-relative path 昇順で安定させる。

## 6. placement method

placement method は managed file を destination path に配置する方法であり、`symlink` と `copy` がある。

### 6.1 `symlink`

`symlink` は destination path に managed file への symlink を作成する。
destination path の symlink が managed file の canonical path を指している場合、desired state と一致しているとみなす。

symlink の一致判定では symlink の target を解決して managed file の canonical path と比較する。
target を解決できない symlink は broken symlink であり、一致とみなさない。
managed file と同じ inode を持つ別のパスを指す symlink は一致とみなさない。

### 6.2 `copy`

`copy` は destination path に通常ファイルとして managed file をコピーする。
コピーは managed file の内容のバイト列と permission bits を複製する。
destination path が通常ファイルで、内容のバイト列と permission bits が managed file と一致している場合、desired state と一致しているとみなす。

owner、group、xattr、ACL は複製も比較もしない。
mtime は一致判定に使わない。

## 7. destination path と conflict

destination path が存在せず、親ディレクトリが作成可能な場合、`install` は desired state を作成する。
親パスが作成可能とは、親パスの途中に install を妨げるパスがないことを指す。
書き込み権限は plan では検査せず、権限による作成失敗は plan 実行中の失敗として扱う。

destination path が desired state と一致する場合、`install` は何もしない。

destination path が存在し、desired state と一致しない場合、その状態を destination conflict とする。
destination conflict では、その destination path を対応する backup path へ移動してから desired state を作成する。
destination conflict のあるパスの種類は問わない。
通常ファイル、ディレクトリ、symlink、broken symlink、unknown file type はすべて backup 対象の destination path である。

symlink は target を辿らず、symlink 自体を backup path へ移動する。
相対 symlink の raw target は保持される。

destination path の親パスの途中に通常ファイル、symlink、unknown file type、判定不能のパスがある場合、その managed file は install できない。
destination path 自体の存在、file kind、または一致判定が判定不能の場合も、その managed file は install できない。
`status` では `blocked` として表示する。
`install` ではエラーとし、部分的なファイルシステム変更を残さない。

## 8. backup

backup root は、`install` または `add --install` が既存の destination path を置き換える場合に使う。

backup root は destination root と同じファイルシステム上になければならない。
backup path への移動は rename で行い、コピーによる代替は行わない。
backup を伴う plan では、plan 作成時に backup root が destination root と同じファイルシステム上にあることを検査し、異なる場合はエラーとし、ファイルシステムを変更しない。
backup root が存在しない場合は、存在する最も近い祖先ディレクトリのファイルシステムで判定する。
backup を伴わない実行ではこの検査を行わない。

1 回の実行につき 1 つの backup set directory を使う。
backup set directory の名前はローカルタイムの `YYYYmmdd_HHMMSS` とする。
同名のディレクトリが既に存在する場合は `YYYYmmdd_HHMMSS-1`、`YYYYmmdd_HHMMSS-2` のように suffix を付ける。
backup set directory の名前と suffix は plan 作成時に決定する。

backup path は destination-relative path を backup set directory 配下に維持する。

例:

```text
destination path: /home/me/.config/foo/config.toml
backup root:      /home/me/.backup_dotfiles
backup set dir:   /home/me/.backup_dotfiles/20260702_213000
backup path:      /home/me/.backup_dotfiles/20260702_213000/.config/foo/config.toml
```

backup set directory は backup 対象の destination path がある場合だけ作成する。
backup 対象の destination path がない実行では作成しない。

dry-run では backup set directory を作成しない。
ただし、作成予定の backup path は出力に表示する。

個別の backup path が既に存在する場合は上書きせずエラーとする。

## 9. 出力と exit code

### 9.1 出力

通常の出力は人間が読むテキストとする。
machine-readable format は初期仕様では提供しない。

変更を伴うコマンドは、実行したファイルシステム操作または dry-run の plan を表示する。
警告は stderr に表示する。
進捗表示を行う場合、通常の出力を壊してはならない。
非 TTY への出力では制御文字に依存した表示を行わない。

### 9.2 exit code

エラーなく完了した実行の exit code は 0 とする。
警告があってもエラーがなければ exit code は 0 とする。
設定エラー、plan 作成時に検出したエラー、plan 実行中の失敗、その他のエラーがある場合は exit code を非 0 とする。
exit code の契約は 0 と非 0 の区別だけとし、非 0 の値は細分しない。

## 10. 安全性要件

dotkoke は利用者の既存データを失わせないことを最優先する。

既存の destination path を置き換える場合は、削除ではなく対応する backup path へ移動する。
backup path への上書きは行わない。
dry-run と通常実行は同じ plan に基づく。
plan 作成時に検出可能なエラーがある場合、ファイルシステムを変更しない。

plan 実行中にファイルシステム操作が失敗した場合、失敗した操作で実行を停止し、以降の操作を実行しない。
実行済みの操作は取り消さず、自動 rollback は行わない。
停止までに実行した操作は出力から判別できるようにする。
backup path へ移動済みの destination path がある場合は、その backup path を出力に含める。

source tree と destination tree のパス解決では、symlink 判定、存在確認、権限エラー、broken symlink、親パスがディレクトリでない場合を明示的に扱う。
