pub mod api;
pub mod config;
pub mod database;

pub use crate::config::TmdbConfig;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let conn = database::Sqlight::get_connection().unwrap();
    }
}
