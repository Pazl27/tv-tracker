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
                    poster_path TEXT NOT NULL
                )",
                [],
            )
            .expect("Failed to create tv_shows_to_watch table");

            conn.execute(
                "CREATE TABLE IF NOT EXISTS watched_movies (
                    id INTEGER PRIMARY KEY,
                    title TEXT NOT NULL,
                    poster_path TEXT NOT NULL,
                    rating REAL CHECK(rating > 0 AND rating < 5) NOT NULL
                )",
                [],
            )
            .expect("Failed to create watched_movies table");

            Arc::new(Mutex::new(Sqlight { conn }))
        });

        Ok(instance.get().unwrap().clone())
    }

    pub fn insert_movie_to_watch(&self, movie: &MovieToWatch) -> Result<()> {
        self.conn.execute(
            "INSERT INTO movies_to_watch (id, title, poster_path) VALUES (?1, ?2, ?3)",
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
            params![movie.id, movie.title, movie.poster_path, movie.rating.to_string()],
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
            "INSERT INTO tv_shows_to_watch (name, poster_path) VALUES (?1, ?2)",
            &[&tv_show.name, &tv_show.poster_path],
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
            .prepare("SELECT id, name, poster_path FROM tv_shows_to_watch")?;
        let tv_show_iter = stmt.query_map([], |row| {
            Ok(TvShowToWatch {
                id: row.get(0)?,
                name: row.get(1)?,
                poster_path: row.get(2)?,
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
