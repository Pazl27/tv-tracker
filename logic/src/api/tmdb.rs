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
}
