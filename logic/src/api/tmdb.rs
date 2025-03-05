use crate::config::TmdbConfig;
use reqwest::Client;
use serde_json::Value;

pub struct Tmdb {
    api_key: String,
}

impl Tmdb {
    pub fn new(config: TmdbConfig) -> Self {
        Tmdb {
            api_key: config.api_key(),
        }
    }

    pub async fn find_movies(&self, query: &str) {
        let client = Client::new();
        let url = format!(
            "https://api.themoviedb.org/3/search/movie?query={}&include_adult=false&language=en-US&page=1",
            query
        );
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("accept", "application/json")
            .send()
            .await
            .unwrap();

        let response_text = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&response_text).unwrap();

        if let Some(results) = json["results"].as_array() {
            for movie in results {
                let title = movie["original_title"].as_str().unwrap_or("N/A");
                let id = movie["id"].as_i64().unwrap_or(0);
                println!("Title: {}, ID: {}", title, id);
            }
        }
    }

    pub async fn find_tv(&self, query: &str) {
        let client = Client::new();
        let url = format!(
            "https://api.themoviedb.org/3/search/tv?query={}&include_adult=false&language=en-US&page=1",
            query
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("accept", "application/json")
            .send()
            .await
            .unwrap();

        let response_text = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&response_text).unwrap();

        if let Some(results) = json["results"].as_array() {
            for movie in results {
                let title = movie["original_title"].as_str().unwrap_or("N/A");
                let id = movie["id"].as_i64().unwrap_or(0);
                println!("Title: {}, ID: {}", title, id);
            }
        }
    }

    pub async fn find_movie_image_url(&self, id: u32) -> Option<String> {
        let client = Client::new();
        let url = format!(
            "https://api.themoviedb.org/3/movie/{}/images?include_image_language=en",
            id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("accept", "application/json")
            .send()
            .await
            .unwrap();

        let response_text = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&response_text).unwrap();

        if let Some(results) = json["posters"].as_array() {
            let valid_posters: Vec<&Value> = results
                .iter()
                .filter(|&movie| movie["file_path"].as_str().is_some())
                .collect();

            if !valid_posters.is_empty() {
                let best_poster = valid_posters.last().unwrap();
                let file_path = best_poster["file_path"].as_str().unwrap_or("");

                return Some(format!("https://image.tmdb.org/t/p/original{}", file_path));
            }
        }

        None
    }

    pub async fn find_tv_image_url(&self, id: u32) -> Option<String> {
        let client = Client::new();
        let url = format!(
            "https://api.themoviedb.org/3/tv/{}/images?include_image_language=en",
            id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("accept", "application/json")
            .send()
            .await
            .unwrap();

        let response_text = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&response_text).unwrap();

        if let Some(results) = json["posters"].as_array() {
            let valid_posters: Vec<&Value> = results
                .iter()
                .filter(|&movie| movie["file_path"].as_str().is_some())
                .collect();

            if !valid_posters.is_empty() {
                let best_poster = valid_posters.last().unwrap();
                let file_path = best_poster["file_path"].as_str().unwrap_or("");

                return Some(format!("https://image.tmdb.org/t/p/original{}", file_path));
            }
        }

        None
    }
}
