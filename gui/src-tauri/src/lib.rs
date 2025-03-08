use anyhow::Result;
use logic::api;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_trending_movies,
            get_trending_tv,
            valid_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
