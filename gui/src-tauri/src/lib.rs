use logic::{api, database};

#[tauri::command]
async fn get_trending_movies() -> Result<Vec<api::Movie>, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.trending_movies().await {
        Some(movies) => Ok(movies),
        None => Err("Failed to fetch trending movies".into()),
    }
}

#[tauri::command]
async fn get_trending_tv() -> Result<Vec<api::Tv>, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.trending_tv().await {
        Some(shows) => Ok(shows),
        None => Err("Failed to fetch trending movies".into()),
    }
}

#[tauri::command]
async fn valid_key() -> Result<bool, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    tmdb.valid_key().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_api_key(key: String) -> Result<(), String> {
    logic::config::write_api_key_to_config(&key).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn search_movies(query: String) -> Result<Vec<api::Movie>, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.find_movies(&query).await {
        Some(movies) => Ok(movies),
        None => Err("Failed to fetch movies".into()),
    }
}

#[tauri::command]
async fn search_tv(query: String) -> Result<Vec<api::Tv>, String> {
    let tmdb = api::Tmdb::new(logic::TmdbConfig::default());
    match tmdb.find_tv(&query).await {
        Some(shows) => Ok(shows),
        None => Err("Failed to fetch movies".into()),
    }
}

#[tauri::command]
async fn add_movie_to_watchlist(movie: database::entities::MovieToWatch) -> Result<(), String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    let _ = db.insert_movie_to_watch(&movie).map_err(|e| e.to_string());
    Ok(())
}

#[tauri::command]
async fn get_watchlist_movies() -> Result<Vec<database::entities::MovieToWatch>, String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    db.get_all_movies_to_watch().map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_movie_from_watchlist(movie: database::entities::MovieToWatch) -> Result<(), String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    db.delete_movie_to_watch(movie.id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_show_to_watchlist(show: database::entities::TvShowToWatch) -> Result<(), String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    let _ = db.insert_tv_show_to_watch(&show).map_err(|e| e.to_string());
    Ok(())
}

#[tauri::command]
async fn get_watchlist_shows() -> Result<Vec<database::entities::TvShowToWatch>, String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    db.get_all_tv_shows_to_watch().map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_show_from_watchlist(show: database::entities::TvShowToWatch) -> Result<(), String> {
    let conn = database::Sqlight::get_connection().map_err(|e| e.to_string())?;
    let db = conn.lock().expect("Failed to lock the mutex");
    db.delete_tv_show_to_watch(show.id).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_trending_movies,
            get_trending_tv,
            valid_key,
            add_api_key,
            search_movies,
            search_tv,

            // Database
            add_movie_to_watchlist,
            get_watchlist_movies,
            remove_movie_from_watchlist,
            add_show_to_watchlist,
            get_watchlist_shows,
            remove_show_from_watchlist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
