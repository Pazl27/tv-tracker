use dirs::home_dir;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::sync::{Arc, Mutex, OnceLock};

use crate::database::entities::*;

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
                    title TEXT NOT NULL,
                    poster_path TEXT NOT NULL
                )",
                [],
            )
            .expect("Failed to create movies_to_watch table");

            conn.execute(
                "CREATE TABLE IF NOT EXISTS tv_shows_to_watch (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    poster_path TEXT NOT NULL,
                    first_air_date TEXT NOT NULL DEFAULT '',
                    vote_average REAL NOT NULL DEFAULT 0.0,
                    overview TEXT NOT NULL DEFAULT ''
                )",
                [],
            )
            .expect("Failed to create tv_shows_to_watch table");

            // Add migration for existing databases
            let _ = conn.execute(
                "ALTER TABLE tv_shows_to_watch ADD COLUMN first_air_date TEXT NOT NULL DEFAULT ''",
                [],
            );
            let _ = conn.execute(
                "ALTER TABLE tv_shows_to_watch ADD COLUMN vote_average REAL NOT NULL DEFAULT 0.0",
                [],
            );
            let _ = conn.execute(
                "ALTER TABLE tv_shows_to_watch ADD COLUMN overview TEXT NOT NULL DEFAULT ''",
                [],
            );

            conn.execute(
                "CREATE TABLE IF NOT EXISTS watched_movies (
                    id INTEGER PRIMARY KEY,
                    title TEXT NOT NULL,
                    poster_path TEXT NOT NULL,
                    rating REAL CHECK(rating >= 0.5 AND rating <= 5.0) NOT NULL
                )",
                [],
            )
            .expect("Failed to create watched_movies table");

            conn.execute(
                "CREATE TABLE IF NOT EXISTS watched_tv_shows (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    poster_path TEXT NOT NULL,
                    first_air_date TEXT NOT NULL DEFAULT '',
                    vote_average REAL NOT NULL DEFAULT 0.0,
                    overview TEXT NOT NULL DEFAULT '',
                    rating REAL CHECK(rating >= 0.5 AND rating <= 5.0) NOT NULL
                )",
                [],
            )
            .expect("Failed to create watched_tv_shows table");

            // Migration: Always recreate watched_movies table with correct constraint
            // This ensures any existing wrong constraints are fixed
            let _ = conn.execute("DROP TABLE IF EXISTS watched_movies_temp", []);
            let _ = conn.execute(
                "CREATE TABLE watched_movies_temp AS SELECT * FROM watched_movies WHERE 1=0",
                [],
            );
            
            // Try to copy existing data if table exists
            let _ = conn.execute(
                "INSERT INTO watched_movies_temp SELECT * FROM watched_movies",
                [],
            );
            
            // Drop old table and create new one with correct constraint
            let _ = conn.execute("DROP TABLE IF EXISTS watched_movies", []);
            conn.execute(
                "CREATE TABLE watched_movies (
                    id INTEGER PRIMARY KEY,
                    title TEXT NOT NULL,
                    poster_path TEXT NOT NULL,
                    rating REAL CHECK(rating >= 0.5 AND rating <= 5.0) NOT NULL
                )",
                [],
            ).expect("Failed to create watched_movies table with correct constraint");
            
            // Copy back valid data
            let _ = conn.execute(
                "INSERT INTO watched_movies SELECT * FROM watched_movies_temp WHERE rating >= 0.5 AND rating <= 5.0",
                [],
            );
            let _ = conn.execute("DROP TABLE watched_movies_temp", []);

            Arc::new(Mutex::new(Sqlight { conn }))
        });

        Ok(instance.get().unwrap().clone())
    }

    pub fn insert_movie_to_watch(&self, movie: &MovieToWatch) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO movies_to_watch (id, title, poster_path) VALUES (?1, ?2, ?3)",
            params![movie.id, movie.title, movie.poster_path],
        )?;
        Ok(())
    }

    pub fn delete_movie_to_watch(&self, id: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM movies_to_watch WHERE id = ?1", &[&id])?;
        Ok(())
    }

    pub fn get_all_movies_to_watch(&self) -> Result<Vec<MovieToWatch>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, poster_path FROM movies_to_watch")?;
        let movie_iter = stmt.query_map([], |row| {
            Ok(MovieToWatch {
                id: row.get(0)?,
                title: row.get(1)?,
                poster_path: row.get(2)?,
            })
        })?;

        let mut movies = Vec::new();
        for movie in movie_iter {
            movies.push(movie?);
        }
        Ok(movies)
    }

    pub fn insert_watched_movie(&self, movie: &WatchedMovie) -> Result<()> {
        self.conn.execute(
            "INSERT INTO watched_movies (id, title, poster_path, rating) VALUES (?1, ?2, ?3, ?4)",
            params![
                movie.id,
                movie.title,
                movie.poster_path,
                movie.rating.to_string()
            ],
        )?;
        Ok(())
    }

    pub fn delete_watched_movie(&self, id: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM watched_movies WHERE id = ?1", &[&id])?;
        Ok(())
    }

    pub fn get_watched_movies(&self) -> Result<Vec<WatchedMovie>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, poster_path, rating FROM watched_movies")?;
        let movie_iter = stmt.query_map([], |row| {
            Ok(WatchedMovie {
                id: row.get(0)?,
                title: row.get(1)?,
                poster_path: row.get(2)?,
                rating: row.get(3)?,
            })
        })?;

        let mut movies = Vec::new();
        for movie in movie_iter {
            movies.push(movie?);
        }
        Ok(movies)
    }

    pub fn insert_tv_show_to_watch(&self, tv_show: &TvShowToWatch) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO tv_shows_to_watch (id, name, poster_path, first_air_date, vote_average, overview) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![tv_show.id, tv_show.name, tv_show.poster_path, tv_show.first_air_date, tv_show.vote_average, tv_show.overview],
        )?;
        Ok(())
    }

    pub fn delete_tv_show_to_watch(&self, id: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM tv_shows_to_watch WHERE id = ?1", &[&id])?;
        Ok(())
    }

    pub fn get_all_tv_shows_to_watch(&self) -> Result<Vec<TvShowToWatch>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, poster_path, COALESCE(first_air_date, '') as first_air_date, COALESCE(vote_average, 0.0) as vote_average, COALESCE(overview, '') as overview FROM tv_shows_to_watch")?;
        let tv_show_iter = stmt.query_map([], |row| {
            Ok(TvShowToWatch {
                id: row.get(0)?,
                name: row.get(1)?,
                poster_path: row.get(2)?,
                first_air_date: row.get(3)?,
                vote_average: row.get(4)?,
                overview: row.get(5)?,
            })
        })?;

        let mut tv_shows = Vec::new();
        for tv_show in tv_show_iter {
            tv_shows.push(tv_show?);
        }
        Ok(tv_shows)
    }

    // Rating methods for movies
    pub fn rate_movie(&self, movie: &WatchedMovie) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO watched_movies (id, title, poster_path, rating) VALUES (?1, ?2, ?3, ?4)",
            params![movie.id, movie.title, movie.poster_path, movie.rating],
        )?;
        Ok(())
    }

    pub fn get_movie_rating(&self, movie_id: i32) -> Result<Option<f32>> {
        let mut stmt = self
            .conn
            .prepare("SELECT rating FROM watched_movies WHERE id = ?1")?;
        let result = stmt.query_row([movie_id], |row| Ok(row.get::<_, f32>(0)?));
        
        match result {
            Ok(rating) => Ok(Some(rating)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn remove_movie_rating(&self, movie_id: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM watched_movies WHERE id = ?1", &[&movie_id])?;
        Ok(())
    }

    pub fn get_all_rated_movies(&self) -> Result<Vec<WatchedMovie>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, poster_path, rating FROM watched_movies")?;
        let movie_iter = stmt.query_map([], |row| {
            Ok(WatchedMovie {
                id: row.get(0)?,
                title: row.get(1)?,
                poster_path: row.get(2)?,
                rating: row.get(3)?,
            })
        })?;

        let mut movies = Vec::new();
        for movie in movie_iter {
            movies.push(movie?);
        }
        Ok(movies)
    }

    // Rating methods for TV shows
    pub fn rate_tv_show(&self, tv_show: &WatchedTvShow) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO watched_tv_shows (id, name, poster_path, first_air_date, vote_average, overview, rating) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![tv_show.id, tv_show.name, tv_show.poster_path, tv_show.first_air_date, tv_show.vote_average, tv_show.overview, tv_show.rating],
        )?;
        Ok(())
    }

    pub fn get_tv_show_rating(&self, tv_show_id: i32) -> Result<Option<f32>> {
        let mut stmt = self
            .conn
            .prepare("SELECT rating FROM watched_tv_shows WHERE id = ?1")?;
        let result = stmt.query_row([tv_show_id], |row| Ok(row.get::<_, f32>(0)?));
        
        match result {
            Ok(rating) => Ok(Some(rating)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn remove_tv_show_rating(&self, tv_show_id: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM watched_tv_shows WHERE id = ?1", &[&tv_show_id])?;
        Ok(())
    }

    pub fn get_all_rated_tv_shows(&self) -> Result<Vec<WatchedTvShow>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, poster_path, first_air_date, vote_average, overview, rating FROM watched_tv_shows")?;
        let tv_show_iter = stmt.query_map([], |row| {
            Ok(WatchedTvShow {
                id: row.get(0)?,
                name: row.get(1)?,
                poster_path: row.get(2)?,
                first_air_date: row.get(3)?,
                vote_average: row.get(4)?,
                overview: row.get(5)?,
                rating: row.get(6)?,
            })
        })?;

        let mut tv_shows = Vec::new();
        for tv_show in tv_show_iter {
            tv_shows.push(tv_show?);
        }
        Ok(tv_shows)
    }
}

fn create_directory(dir_path: &std::path::PathBuf) {
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).expect("Failed to create directory");
    }
}
