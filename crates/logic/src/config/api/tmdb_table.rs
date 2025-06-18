use anyhow::{Context, Result};

use crate::config::toml_parser::{ConfigToml, TmdbTable};

pub struct TmdbConfig {
    api_key: String,
}

impl TmdbConfig {
    fn new(api_key: String) -> Self {
        TmdbConfig { api_key }
    }

    fn initialize() -> Result<Self> {
        let tmdb_table: TmdbTable = ConfigToml::new()
            .context("Failed to load config")?
            .get_tmdb()
            .context("Failed to get TMDB table")?;

        let api_key = tmdb_table.api_key.unwrap_or_else(|| "".to_owned());

        Ok(TmdbConfig::new(api_key))
    }

    pub fn api_key(&self) -> String {
        self.api_key.clone()
    }
}

impl Default for TmdbConfig {
    fn default() -> Self {
        TmdbConfig::initialize().expect("Failed to initialize Tmdb")
    }
}
