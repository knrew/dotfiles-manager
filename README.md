# dotfiles manager
## Install
```sh
cargo install --git https://github.com/knrew/dotfiles_manager.git
```

### without install
インストールせずに使う場合は以下のようにする．

```sh
git clone https://github.com/knrew/dotfiles_manager.git
cd dotfiles_manager
cargo run --release -- install <options>
```

## How to use
### install
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
- `/README.md`
- `/.git`
- `/.gitignore`
- `/ex/`
    - インストールしないファイルはex以下に置く

### backup
特定のディレクトリでは新しいファイルが追加されたか確認してdotfilesに追加する

確認するディレクトリは以下
- `.config/nvim`

```sh
dotfiles-manager backup <dotfiles_dir> <home_dir>
```

`home_dir`はinstallでいう`install_dir`
