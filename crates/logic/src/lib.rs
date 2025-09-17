pub mod api;
pub mod config;
pub mod database;

pub use crate::config::TmdbConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let conn = database::Sqlight::get_connection().unwrap();
        let db = conn.lock().unwrap();

        let movies = db.get_all_movies_to_watch().unwrap();
    }
}
