// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod models;
pub mod image_generation;
pub mod spotify_repository;

use log::{info, LevelFilter};
use models::Song;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::{thread, time};
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().level(LevelFilter::Info).build())
        .invoke_handler(tauri::generate_handler![generate_image, do_emit, pause_play, next_song, previous_song])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn generate_image(song: Song) -> String {
    info!("Received Image Generation Request for {:?}", song);
    image_generation::generate_image(song).await
}

#[tauri::command]
async fn do_emit(app: AppHandle) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let song = spotify_repository::get_currently_playing_song().await;
            app.emit("current-song", song).unwrap()
        }
    });
}

#[tauri::command]
async fn pause_play() {
    spotify_repository::pause_play().await;
}

#[tauri::command]
async fn next_song() {
    spotify_repository::next_song().await;
}

#[tauri::command]
async fn previous_song() {
    spotify_repository::previous_song().await;
}


