mod config;
pub mod api;

pub use crate::config::TmdbConfig;

pub fn hello_world() -> String {
    "Hello, world from the logic crate!".to_string()
}
