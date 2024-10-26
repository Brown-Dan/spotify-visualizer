use crate::models::Song;
use log::info;
use serde_json::{json, to_string, Value};
use tauri::ipc::IpcResponse;

pub async fn get_currently_playing_song() -> Song {
    let res = get_from_spotify("https://api.spotify.com/v1/me/player/currently-playing")
        .await.expect("Failed to get currently playing song");

    let status = res.status().as_str().to_string();

    let res_body = res.text().await.expect("Failed to get response body for image generation");

    info!("Received response from spotify with status {}", status);

    let data: Value = serde_json::from_str(&*res_body).expect("Failed to parse JSON");

    let song_name = data["item"]["name"].as_str().expect("Failed to get song name");

    let artists = data["item"]["artists"]
        .as_array()
        .expect("Failed to get artists")
        .iter()
        .filter_map(|artist| artist["name"].as_str())
        .collect::<Vec<_>>();

    Song {
        title: song_name.to_string(),
        artist: artists.join(", "),
    }
}

pub async fn pause_play() {
    let res = put_to_spotify("https://api.spotify.com/v1/me/player/pause", json!({}).to_string())
        .await.expect("Failed to pause song");

    let status = res.status().as_str().to_string();

    info!("Received response from spotify pause with status {}", status);

    if status == "403" {
        put_to_spotify("https://api.spotify.com/v1/me/player/play", json!({}).to_string())
            .await.expect("Failed to play song");

        let status = res.status().as_str().to_string();

        info!("Received response from spotify pause with status {}", status);
    }
}

pub async fn next_song() {
    let res = post_to_spotify("https://api.spotify.com/v1/me/player/next")
        .await.expect("Failed to play next song");

    let status = res.status().as_str().to_string();

    info!("Received response from spotify next with status {}", status);
}

pub async fn previous_song() {
    let res = post_to_spotify("https://api.spotify.com/v1/me/player/previous")
        .await.expect("Failed to play previous song");

    let status = res.status().as_str().to_string();

    info!("Received response from spotify previous with status {}", status);
}

async fn get_from_spotify(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.get(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer BQA86F9PZ0kYJRIXoiUzgK_yw4hYdu89CU415PsjnH0dZE9-6GTumufLNfoO3KH5Crzrhxjj5HMrOJ_d7JDNT5KStNJLgnoq4dRK3rJve3NYBO4uKVwLUxMn6fCRFiCbEas-berEbP8_xeEaTzcWc5aTXN9q7AcBV01D5JPPhGcMsPD-18qm8Yh-gL6af7WBv8nYFpFm-M0u7PMtj1pweaOesvqspAgRVNCEZ3qL0lwLT41nk2WHPnduNTzgt-ybsTC6khQEtKhipLvJbxQ8rIovHDXPcaY1lSk_0FfLiQDZAw")
        .send().await
}

async fn put_to_spotify(url: &str, body: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.put(url)
        .body(body)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer BQAblVdQNqpyemR4xif30wndF37aDY5eiqeaPAVylmpkUnIN6OjP1bSNW5zi_BL14Yddc5rHqmZNJKFLWXwjfBbuqsUHc1V5eJV__GGnQaMGbm_v4I-Rq3g7WZbjECYpPbgp52M4FeMS0uCU6r3lQWOkuOGoyK5PGIPqsPYKbgAUmAA92P3Sybzao4l4svpuInBwM28tFw")
        .send().await
}

async fn post_to_spotify(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.post(url)
        .header("Content-Type", "application/json")
        .body(json!({}).to_string())
        .header("Authorization", "Bearer BQAblVdQNqpyemR4xif30wndF37aDY5eiqeaPAVylmpkUnIN6OjP1bSNW5zi_BL14Yddc5rHqmZNJKFLWXwjfBbuqsUHc1V5eJV__GGnQaMGbm_v4I-Rq3g7WZbjECYpPbgp52M4FeMS0uCU6r3lQWOkuOGoyK5PGIPqsPYKbgAUmAA92P3Sybzao4l4svpuInBwM28tFw")
        .send().await
}
