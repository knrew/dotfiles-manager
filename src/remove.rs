use std::{fs, path::Path};

use anyhow::{Context, Result, anyhow};

use crate::{config::Config, utils::*};

pub fn remove(config: Config, path: impl AsRef<Path>) -> Result<()> {
    let Config {
        dotfiles_dir,
        home_dir,
        ..
    } = config;

    let path = path.as_ref();

    let path = path
        .canonicalize()
        .with_context(|| format!("unmanaged path: {}", path.display()))?;

    // dotfiles/home/
    let dotfiles_home = dotfiles_dir
        .join("home")
        .canonicalize()
        .with_context(|| format!("invalid path: {}/home/", dotfiles_dir.display()))?;

    if !path.starts_with(&dotfiles_home) {
        return Err(anyhow!("invalid"));
    }

    let suffix = path.strip_prefix(&dotfiles_home)?;
    let link = home_dir.join(suffix);

    if !is_symlink_pointing_to(&link, &path)? {
        return Ok(());
    }

    fs::remove_file(&link)
        .with_context(|| format!("failed to remove symlink: {}", link.display()))?;

    fs::remove_file(&path).with_context(|| format!("failed to remove file: {}", path.display()))?;

    Ok(())
}
