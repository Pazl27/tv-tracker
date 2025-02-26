use std::fs;
use toml;
use serde::{Deserialize, Serialize};
use dirs::home_dir;
use std::path::PathBuf;
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct TmdbTable {
    pub api_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigToml {
    pub tmdb: Option<TmdbTable>,
}

impl ConfigToml {
    pub fn new() -> Result<Self> {
        let mut config_filepaths: Vec<PathBuf> = vec![PathBuf::from("./config.toml")];

        if let Some(home_path) = home_dir() {
            config_filepaths.push(home_path.join(".config/tv/config.toml"));
        }

        let mut content = "".to_owned();

        for filepath in config_filepaths {
            let result = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }
        let config_toml: ConfigToml =
            toml::from_str(&content).context("Failed to parse .config/tv/config.toml")?;
        Ok(config_toml)
    }

    pub fn get_tmdb(&self) -> Option<TmdbTable> {
        self.tmdb.clone()
    }
}
