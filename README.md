# dotfiles manager
自分用の[dotfiles](https://github.com/knrew/dotfiles)を管理するためのツール．

## (dotfiles-managerの)install
```sh
cargo install --git https://github.com/knrew/dotfiles-manager.git 
```

### uninstall
```sh
cargo uninstall dotfiles-manager 
```

### without install
インストールせずに使う場合，たとえば以下のようにする．

```sh
git clone https://github.com/knrew/dotfiles_manager.git
cd dotfiles_manager
cargo run --release -- <command> <options>
```

## 使い方
### (dotfilesの)install
dotfileたちをホームディレクトリに配置する．

```sh
dotfiles-manager install <dotfiles_dir> <install_dir> <backup_dir>
```
`install_dir`は基本的にホームディレクトリ(`~/`)

例:
```sh
dotfiles-manager install ~/.dotfiles ~/ ~/.backup_dotfiles
```

dotfiles以下のファイルのシンボリックリンクを作成する．

例: `~/.dotfiles/.config/i3/config` -> `~/.config/i3/config`

以下のファイルは除外される.
- `dotfiles/.git`
- `dotfiles/.gitignore`
- `dotfiles/README.md`
- `dotfiles/ex/`以下に置かれたファイル

### backup
頻繁に変更される特定のディレクトリでは新しいファイルが追加されたか確認してdotfilesに追加する

```sh
dotfiles-manager backup <dotfiles_dir> <home_dir>
```

例:
```sh
dotfiles-manager backup ~/.dotfiles/ ~/
```

確認するディレクトリは以下(適宜変更)
- `.config/nvim`


