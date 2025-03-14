use dirs::home_dir;
use rusqlite::{Connection, Result};
use std::fs;
use std::sync::{Arc, Mutex, OnceLock};

pub struct Sqlight {
    conn: Connection,
}

impl Sqlight {
    // Use a static variable to hold the singleton instance
    fn instance() -> &'static OnceLock<Arc<Mutex<Sqlight>>> {
        static INSTANCE: OnceLock<Arc<Mutex<Sqlight>>> = OnceLock::new();
        &INSTANCE
    }

    pub fn get_connection() -> Result<Arc<Mutex<Sqlight>>> {
        let instance = Sqlight::instance();

        let db_path = home_dir()
            .expect("Could not get home directory")
            .join(".local")
            .join("share")
            .join("tv-tracker");

        instance.get_or_init(|| {
            create_directory(&db_path);

            let db_path = db_path.join("tv_tracker.db");

            let conn = Connection::open(db_path).expect("Failed to open the database");

            conn.execute(
                "CREATE TABLE IF NOT EXISTS movies_to_watch (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    poster_url TEXT NOT NULL
                )",
                [],
            )
            .expect("Failed to create movies_to_watch table");

            conn.execute(
                "CREATE TABLE IF NOT EXISTS tv_shows_to_watch (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    poster_url TEXT NOT NULL
                )",
                [],
            )
            .expect("Failed to create tv_shows_to_watch table");

            conn.execute(
                "CREATE TABLE IF NOT EXISTS watched_movies (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    poster_url TEXT NOT NULL,
                    rating REAL CHECK(rating > 0 AND rating < 5) NOT NULL
                )",
                [],
            )
            .expect("Failed to create watched_movies table");

            Arc::new(Mutex::new(Sqlight { conn }))
        });

        Ok(instance.get().unwrap().clone())
    }
}

fn create_directory(dir_path: &std::path::PathBuf) {
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).expect("Failed to create directory");
    }
}
