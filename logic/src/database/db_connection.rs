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

            let db_path = db_path.join("tv-tracker.db");

            let conn = Connection::open(db_path).expect("Failed to open the database");

            conn.execute(
                "CREATE TABLE IF NOT EXISTS shows (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    season INTEGER NOT NULL,
                    episode INTEGER NOT NULL
                )",
                [],
            )
            .expect("Failed to create table");

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
