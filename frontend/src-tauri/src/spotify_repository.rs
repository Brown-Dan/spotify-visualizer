use crate::models::Song;
use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use log::{error, info};
use reqwest::Method;
use serde_json::{json, Value};
use std::sync::Arc;
use tauri::http::StatusCode;

lazy_static! {
    static ref CLIENT: Arc<reqwest::Client> = Arc::new(reqwest::Client::new());
    static ref SPOTIFY_TOKEN: Arc<&'static str> = Arc::new(dotenv!("SPOTIFY_TOKEN"));
}

fn get_client() -> Arc<reqwest::Client> {
    Arc::clone(&CLIENT)
}

fn get_spotify_token() -> Arc<&'static str> {
    Arc::clone(&SPOTIFY_TOKEN)
}

pub async fn get_currently_playing_song() -> Song {
    let response = get("https://api.spotify.com/v1/me/player/currently-playing").await.unwrap();

    let response_body = response.text().await.unwrap();

    let body_as_json: Value = serde_json::from_str(&*response_body).expect(&format!("Failed to parse JSON {}", response_body));

    Song::from_json(body_as_json).await
}

pub async fn pause_play() {
    let response = put("https://api.spotify.com/v1/me/player/pause", json!({}).to_string()).await.unwrap();

    if response.status() == StatusCode::FORBIDDEN {
        play().await
    }
}

pub async fn play() {
    put("https://api.spotify.com/v1/me/player/play", json!({}).to_string()).await.unwrap();
}

pub async fn next_song() {
    post("https://api.spotify.com/v1/me/player/next").await.unwrap();
}

pub async fn previous_song() {
    post("https://api.spotify.com/v1/me/player/previous").await.unwrap();
}

pub async fn get(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let response = send_request(Method::GET, url, None).await;
    handle_response(url, response)
}

async fn put(url: &str, body: String) -> Result<reqwest::Response, reqwest::Error> {
    let response = send_request(Method::PUT, url, Some(body)).await;
    handle_response(url, response)
}

async fn post(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let response = send_request(Method::POST, url, Some("{}".to_string())).await;
    handle_response(url, response)
}

fn handle_response(url: &str, response: Result<reqwest::Response, reqwest::Error>) -> Result<reqwest::Response, reqwest::Error> {
    match response {
        Ok(res) => {
            info!("{} - status {}", url, res.status().as_str().to_string());
            Ok(res)
        }
        Err(e) => {
            error!("Failed to send request to {}", url);
            Err(e)
        }
    }
}

async fn send_request(method: Method, url: &str, body: Option<String>) -> Result<reqwest::Response, reqwest::Error> {
    let req = get_client()
        .request(method, url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", get_spotify_token()));

    if body.is_some() {
        req.body(body.unwrap()).send().await
    } else {
        req.send().await
    }
}
