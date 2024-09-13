# dotfiles manager
## Install
```sh
git clone https://github.com/knrew/dotfiles_manager.git
cd dotfiles_manager
cargo install --path .
```
### without install
インストールせずに使う場合は以下のようにする．

```sh
git clone https://github.com/knrew/dotfiles_manager.git
cd dotfiles_manager
cargo run --release -- install <options>
```

## How to use
dotfileたちをインストールする．

```sh
dotfiles-manager install <dotfiles_dir> <install_dir> <backup_dir>
```

例:
```sh
dotfiles-manager install ~/.dotfiles ~ ~/.backup_dotfiles
```

dotfiles以下のファイルのシンボリックリンクを作成する
dotfiles直下の以下のファイル/ディレクトリは除外される
- /README.md
- /.git
- /.gitignore
- /ex/
    - インストールしないファイルはex以下に置く
