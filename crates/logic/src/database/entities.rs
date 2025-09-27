use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieToWatch {
    pub id: i32,
    pub title: String,
    pub poster_path: String,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvShowToWatch {
    pub id: i32,
    pub name: String,
    pub poster_path: String,
    pub first_air_date: String,
    pub vote_average: f32,
    pub overview: String,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedMovie {
    pub id: i32,
    pub title: String,
    pub poster_path: String,
    pub rating: f32,
    pub watched_at: String,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedTvShow {
    pub id: i32,
    pub name: String,
    pub poster_path: String,
    pub first_air_date: String,
    pub vote_average: f32,
    pub overview: String,
    pub rating: f32,
    pub watched_at: String,
    pub notes: String,
}
