use logic::{api, database};

// API Commands

#[tauri::command]
pub async fn get_trending_movies() -> Result<Vec<api::Movie>, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.trending_movies().await {
        Some(movies) => Ok(movies),
        None => Err("Failed to fetch trending movies".into()),
    }
}

#[tauri::command]
pub async fn get_trending_tv() -> Result<Vec<api::Tv>, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.trending_tv().await {
        Some(shows) => Ok(shows),
        None => Err("Failed to fetch trending TV shows".into()),
    }
}

#[tauri::command]
pub async fn valid_key() -> Result<bool, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    tmdb.valid_key().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_api_key(key: String) -> Result<(), String> {
    logic::config::write_api_key_to_config(&key).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn search_movies(query: String) -> Result<Vec<api::Movie>, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.find_movies(&query).await {
        Some(movies) => Ok(movies),
        None => Err("Failed to fetch movies".into()),
    }
}

#[tauri::command]
pub async fn search_tv(query: String) -> Result<Vec<api::Tv>, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.find_tv(&query).await {
        Some(shows) => Ok(shows),
        None => Err("Failed to fetch TV shows".into()),
    }
}

#[tauri::command]
pub async fn get_movie_details(id: u32) -> Result<api::MovieDetail, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.get_movie_details(id).await {
        Ok(movie) => Ok(movie),
        Err(_) => Err("Failed to fetch movie details".into()),
    }
}

#[tauri::command]
pub async fn get_tv_show_details(id: u32) -> Result<api::TvDetail, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.get_tv_show_details(id).await {
        Ok(show) => Ok(show),
        Err(_) => Err("Failed to fetch TV show details".into()),
    }
}

// Movie Watchlist Commands

#[tauri::command]
pub async fn add_movie_to_watchlist(movie: serde_json::Value) -> Result<(), String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    // Extract the required fields and convert types
    let movie_to_watch = database::entities::MovieToWatch {
        id: movie["id"].as_i64().unwrap_or(0) as i32,
        title: movie["title"].as_str().unwrap_or("").to_string(),
        poster_path: movie["poster_path"].as_str().unwrap_or("").to_string(),
    };
    
    db.insert_movie_to_watch(&movie_to_watch).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_watchlist_movies() -> Result<Vec<database::entities::MovieToWatch>, String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    db.get_all_movies_to_watch().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_movie_from_watchlist(movie: serde_json::Value) -> Result<(), String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    let movie_id = movie["id"].as_i64().unwrap_or(0) as i32;
    db.delete_movie_to_watch(movie_id)
        .map_err(|e| e.to_string())
}

// TV Show Watchlist Commands

#[tauri::command]
pub async fn add_show_to_watchlist(show: serde_json::Value) -> Result<(), String> {
    println!("Adding TV show to watchlist: {:?}", show);
    
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    // Extract the required fields and convert types
    let show_id = show["id"].as_i64().unwrap_or(0) as i32;
    let show_name = show["name"].as_str().unwrap_or("").to_string();
    let show_poster_path = show["poster_path"].as_str().unwrap_or("").to_string();
    let show_first_air_date = show["first_air_date"].as_str().unwrap_or("").to_string();
    let show_vote_average = show["vote_average"].as_f64().unwrap_or(0.0) as f32;
    let show_overview = show["overview"].as_str().unwrap_or("").to_string();
    
    if show_id == 0 {
        return Err("Invalid show ID".to_string());
    }
    
    if show_name.is_empty() {
        return Err("Show name is required".to_string());
    }
    
    let tv_show = database::entities::TvShowToWatch {
        id: show_id,
        name: show_name,
        poster_path: show_poster_path,
        first_air_date: show_first_air_date,
        vote_average: show_vote_average,
        overview: show_overview,
    };
    
    println!("Inserting TV show: {:?}", tv_show);
    
    db.insert_tv_show_to_watch(&tv_show).map_err(|e| {
        println!("Database error inserting TV show: {}", e);
        e.to_string()
    })?;
    
    println!("Successfully added TV show to database");
    Ok(())
}

#[tauri::command]
pub async fn get_watchlist_shows() -> Result<Vec<database::entities::TvShowToWatch>, String> {
    println!("Getting all TV shows from watchlist");
    
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    let result = db.get_all_tv_shows_to_watch().map_err(|e| {
        println!("Database error getting TV shows: {}", e);
        e.to_string()
    })?;
    
    println!("Retrieved {} TV shows from database", result.len());
    for show in &result {
        println!("TV Show: ID={}, Name={}, Poster={}", show.id, show.name, show.poster_path);
    }
    
    Ok(result)
}

#[tauri::command]
pub async fn remove_show_from_watchlist(show: serde_json::Value) -> Result<(), String> {
    println!("Removing TV show from watchlist: {:?}", show);
    
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    let show_id = show["id"].as_i64().unwrap_or(0) as i32;
    
    if show_id == 0 {
        return Err("Invalid show ID".to_string());
    }
    
    println!("Deleting TV show with ID: {}", show_id);
    
    db.delete_tv_show_to_watch(show_id).map_err(|e| {
        println!("Database error deleting TV show: {}", e);
        e.to_string()
    })?;
    
    println!("Successfully removed TV show from database");
    Ok(())
}

// Rating Commands

#[tauri::command]
pub async fn rate_movie(movie: serde_json::Value, rating: f32) -> Result<(), String> {
    if rating < 0.5 || rating > 5.0 || (rating * 2.0).fract() != 0.0 {
        return Err("Rating must be between 0.5 and 5.0 in 0.5 increments".to_string());
    }

    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    let watched_movie = database::entities::WatchedMovie {
        id: movie["id"].as_i64().unwrap_or(0) as i32,
        title: movie["title"].as_str().unwrap_or("").to_string(),
        poster_path: movie["poster_path"].as_str().unwrap_or("").to_string(),
        rating,
    };
    
    db.rate_movie(&watched_movie).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn rate_tv_show(show: serde_json::Value, rating: f32) -> Result<(), String> {
    if rating < 0.5 || rating > 5.0 || (rating * 2.0).fract() != 0.0 {
        return Err("Rating must be between 0.5 and 5.0 in 0.5 increments".to_string());
    }

    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    let watched_tv_show = database::entities::WatchedTvShow {
        id: show["id"].as_i64().unwrap_or(0) as i32,
        name: show["name"].as_str().unwrap_or("").to_string(),
        poster_path: show["poster_path"].as_str().unwrap_or("").to_string(),
        first_air_date: show["first_air_date"].as_str().unwrap_or("").to_string(),
        vote_average: show["vote_average"].as_f64().unwrap_or(0.0) as f32,
        overview: show["overview"].as_str().unwrap_or("").to_string(),
        rating,
    };
    
    db.rate_tv_show(&watched_tv_show).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_movie_rating(movie_id: i32) -> Result<Option<f32>, String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    db.get_movie_rating(movie_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tv_show_rating(show_id: i32) -> Result<Option<f32>, String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    db.get_tv_show_rating(show_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_movie_rating(movie_id: i32) -> Result<(), String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    db.remove_movie_rating(movie_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_tv_show_rating(show_id: i32) -> Result<(), String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    db.remove_tv_show_rating(show_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_rated_movies() -> Result<Vec<database::entities::WatchedMovie>, String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    db.get_all_rated_movies().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_rated_tv_shows() -> Result<Vec<database::entities::WatchedTvShow>, String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    
    db.get_all_rated_tv_shows().map_err(|e| e.to_string())
}