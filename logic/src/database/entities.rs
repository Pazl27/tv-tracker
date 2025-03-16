pub struct MovieToWatch {
    pub id: i32,
    pub name: String,
    pub poster_url: String,
}

pub struct TvShowToWatch {
    pub id: i32,
    pub name: String,
    pub poster_url: String,
}

pub struct WatchedMovie {
    pub id: i32,
    pub name: String,
    pub poster_url: String,
    pub rating: f32,
}
