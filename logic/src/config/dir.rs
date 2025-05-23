use anyhow::{Context, Result};
use dirs::home_dir;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    let mut path = home_dir().expect("Failed to get home directory");
    path.push(".config/tv/config.toml");
    path
}

pub fn config_dir_exists() -> Option<PathBuf> {
    let path = get_config_path();
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

pub fn create_config_dir() -> Result<()> {
    let path = get_config_path();
    std::fs::create_dir_all(path.parent().context("Failed to get parent directory")?)?;

    //WORKAROUND: write to file this [tmdb]
    let content = "[tmdb]";
    std::fs::write(path, content).context("Failed to write to config file")?;

    Ok(())
}
