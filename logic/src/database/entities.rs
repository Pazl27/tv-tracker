use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieToWatch {
    pub id: i32,
    pub title: String,
    pub poster_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvShowToWatch {
    pub id: i32,
    pub name: String,
    pub poster_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedMovie {
    pub id: i32,
    pub title: String,
    pub poster_path: String,
    pub rating: f32,
}
