mod config;
mod api;

use crate::config::TmdbConfig;
use crate::api::Tmdb;

pub fn hello_world() -> String {
    "Hello, world from the logic crate!".to_string()
}
