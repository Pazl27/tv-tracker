use crate::config::TmdbConfig;

pub struct Tmdb {
    api_key: String
}

impl Tmdb {
    pub fn new(config: TmdbConfig) -> Self {
        Tmdb {
            api_key: config.api_key()
        }
    }
}
