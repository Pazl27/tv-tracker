use std::fs;
use toml::Value;
use anyhow::{Context, Result};

use crate::config::dir;

pub fn write_api_key_to_config(new_api_key: &str) -> Result<()> {
    match dir::config_dir_exists() {
        Some(_) => (),
        None => dir::create_config_dir()?,
    }

    let content = fs::read_to_string(dir::get_config_path()).context("Failed to read config file")?;
    
    let mut value: Value = content.parse().context("Failed to parse TOML content")?;
    
    if let Some(tmdb) = value.get_mut("tmdb") {
        let tmdb = tmdb.as_table_mut().context("Failed to get mutable TMDB table")?;
        tmdb.insert("api_key".to_owned(), Value::String(new_api_key.to_owned()));
    } else {
        let mut tmdb = toml::map::Map::new();
        tmdb.insert("api_key".to_owned(), Value::String(new_api_key.to_owned()));
        value["tmdb"] = Value::Table(tmdb);
    }

    let new_content = toml::to_string(&value).context("Failed to serialize TOML content")?;
    
    fs::write(dir::get_config_path(), new_content).context("Failed to write config file")?;
    
    Ok(())
}
