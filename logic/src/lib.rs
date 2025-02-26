mod config;
mod api;

use crate::config::TmdbConfig;
use crate::api::Tmdb;

pub fn hello_world() -> String {
    "Hello, world from the logic crate!".to_string()
}

pub fn api_key() -> String {
    let config_tmdb = TmdbConfig::default();
    config_tmdb.api_key()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world from the logic crate!");
    }
}
