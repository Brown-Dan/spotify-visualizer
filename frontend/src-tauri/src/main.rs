// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod models;
pub mod spotify_repository;

use log::LevelFilter;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().level(LevelFilter::Info).build())
        .invoke_handler(tauri::generate_handler![do_emit, pause_play, next_song, previous_song])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn do_emit(app: AppHandle) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(600)).await;
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


