mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // API Commands
            get_trending_movies,
            get_trending_tv,
            valid_key,
            add_api_key,
            search_movies,
            search_tv,
            get_movie_details,
            // Movie Watchlist Commands
            add_movie_to_watchlist,
            get_watchlist_movies,
            remove_movie_from_watchlist,
            // TV Show Watchlist Commands
            add_show_to_watchlist,
            get_watchlist_shows,
            remove_show_from_watchlist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}