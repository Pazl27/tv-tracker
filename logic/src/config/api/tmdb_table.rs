use anyhow::{Result, Context, anyhow};

use crate::config::toml_parser::{ConfigToml, TmdbTable};

pub struct Tmdb {
    api_key: String,
}

impl Tmdb {
    fn new(api_key: String) -> Self {
        Tmdb { api_key }
    }

    fn initialize() -> Result<Self> {
        let tmdb_table: TmdbTable = ConfigToml::new()
            .context("Failed to load config")?
            .get_tmdb()
            .context("Failed to get TMDB table")?;

        let api_key = tmdb_table.api_key.ok_or_else(|| anyhow!("API key is missing"))?;
        api_key.is_empty().then(|| Err::<(), _>(anyhow!("API key is empty"))).transpose()?;

        Ok(Tmdb::new(api_key))
    }
}

impl Default for Tmdb {
    fn default() -> Self {
        Tmdb::initialize().expect("Failed to initialize Tmdb")
    }
}
