use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::TmdbConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub poster_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tv {
    pub id: u32,
    pub name: String,
    pub poster_path: String,
}

pub struct Tmdb {
    api_key: String,
}

impl Tmdb {
    pub fn new(config: TmdbConfig) -> Self {
        Tmdb {
            api_key: config.api_key(),
        }
    }

    pub async fn valid_key(&self) -> Result<bool> {
        let client = Client::new();
        let url = "https://api.themoviedb.org/3/authentication";
        let response = client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("accept", "application/json")
            .send()
            .await
            .context("Failed to send request")?;

        let response_text = response
            .text()
            .await
            .context("Failed to read response text")?;
        let json: Value = serde_json::from_str(&response_text).context("Failed to parse JSON")?;

        Ok(json["success"].as_bool().unwrap_or(false))
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

    pub async fn trending_movies(&self) -> Option<Vec<Movie>> {
        let client = Client::new();
        let url = format!("https://api.themoviedb.org/3/trending/movie/day?language=en-US");

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
            let movies: Vec<Movie> = results
                .iter()
                .map(|movie| Movie {
                    id: movie["id"].as_u64().unwrap() as u32,
                    title: movie["title"].as_str().unwrap().to_string(),
                    poster_path: movie["poster_path"].as_str().unwrap().to_string(),
                })
                .collect();
            return Some(movies);
        }
        None
    }

    pub async fn trending_tv(&self) -> Option<Vec<Tv>> {
        let client = Client::new();
        let url = format!("https://api.themoviedb.org/3/trending/tv/day?language=en-US");

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
            let shows: Vec<Tv> = results
                .iter()
                .map(|movie| Tv {
                    id: movie["id"].as_u64().unwrap() as u32,
                    name: movie["name"].as_str().unwrap().to_string(),
                    poster_path: movie["poster_path"].as_str().unwrap().to_string(),
                })
                .collect();
            return Some(shows);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_trending() {
        let tmdb = Tmdb::new(TmdbConfig::default());
        let shows = tmdb.trending_tv().await.unwrap();

        println!("{:?}", shows);
    }
}
