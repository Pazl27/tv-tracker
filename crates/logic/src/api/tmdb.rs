use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieDetail {
    pub id: u32,
    pub title: String,
    pub poster_path: String,
    pub runtime: u32,
    pub overview: String,
    pub genres: Vec<String>,
    pub vote_average: f32,
    pub release_date: String,
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

    pub async fn find_movies(&self, query: &str) -> Option<Vec<Movie>> {
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

        let mut movies = Vec::new();

        if let Some(results) = json["results"].as_array() {
            for movie in results {
                let title = movie["original_title"].as_str().unwrap_or("N/A");
                let id = movie["id"].as_i64().unwrap_or(0);
                let poster_path = movie["poster_path"].as_str().unwrap_or("");
                movies.push(Movie {
                    id: id as u32,
                    title: title.to_string(),
                    poster_path: poster_path.to_string(),
                });
            }
        }
        if movies.is_empty() {
            return None;
        } else {
            return Some(movies);
        }
    }

    pub async fn find_tv(&self, query: &str) -> Option<Vec<Tv>> {
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

        let mut shows = Vec::new();

        if let Some(results) = json["results"].as_array() {
            for show in results {
                let title = show["name"].as_str().unwrap_or("N/A");
                let id = show["id"].as_i64().unwrap_or(0);
                let poster_path = show["poster_path"].as_str().unwrap_or("");
                shows.push(Tv {
                    id: id as u32,
                    name: title.to_string(),
                    poster_path: poster_path.to_string(),
                });
            }
        }
        if shows.is_empty() {
            return None;
        } else {
            return Some(shows);
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
        let mut all_movies = Vec::new();
        let mut seen_ids = HashSet::new();

        for page in 1..=3 {
            let url = format!(
                "https://api.themoviedb.org/3/trending/movie/day?language=en-US&page={}",
                page
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
                let movies: Vec<Movie> = results
                    .iter()
                    .map(|movie| Movie {
                        id: movie["id"].as_u64().unwrap() as u32,
                        title: movie["title"].as_str().unwrap().to_string(),
                        poster_path: movie["poster_path"].as_str().unwrap().to_string(),
                    })
                    .collect();

                for movie in movies {
                    if seen_ids.insert(movie.id) {
                        all_movies.push(movie);
                    }
                }
            }
        }

        if all_movies.is_empty() {
            None
        } else {
            Some(all_movies)
        }
    }

    pub async fn trending_tv(&self) -> Option<Vec<Tv>> {
        let client = Client::new();
        let mut all_shows = Vec::new();
        let mut seen_ids = HashSet::new();

        for page in 1..=3 {
            let url = format!(
                "https://api.themoviedb.org/3/trending/tv/day?language=en-US&page={}",
                page
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
                let shows: Vec<Tv> = results
                    .iter()
                    .map(|show| Tv {
                        id: show["id"].as_u64().unwrap() as u32,
                        name: show["name"].as_str().unwrap().to_string(),
                        poster_path: show["poster_path"].as_str().unwrap().to_string(),
                    })
                    .collect();

                for show in shows {
                    if seen_ids.insert(show.id) {
                        all_shows.push(show);
                    }
                }
            }
        }

        if all_shows.is_empty() {
            None
        } else {
            Some(all_shows)
        }
    }

    pub async fn get_movie_details(&self, id: u32) -> Result<MovieDetail> {
        let client = Client::new();
        let url = format!("https://api.themoviedb.org/3/movie/{}?language=en-US", id);

        let response = client
            .get(&url)
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

        let genres = json["genres"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|genre| genre["name"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(MovieDetail {
            id: json["id"].as_u64().unwrap_or(0) as u32,
            title: json["title"].as_str().unwrap_or("").to_string(),
            poster_path: json["poster_path"].as_str().unwrap_or("").to_string(),
            runtime: json["runtime"].as_u64().unwrap_or(0) as u32,
            overview: json["overview"].as_str().unwrap_or("").to_string(),
            genres,
            vote_average: json["vote_average"].as_f64().unwrap_or(0.0) as f32,
            release_date: json["release_date"].as_str().unwrap_or("").to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() {
        let tmdb = Tmdb::new(TmdbConfig::default());

        let movies = tmdb.get_movie_details(123).await.unwrap();

        println!("{:?}", movies);
    }
}
