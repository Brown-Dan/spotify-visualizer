// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod models;
pub mod image_generation;

use log::{info, LevelFilter};
use models::Song;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().level(LevelFilter::Info).build())
        .invoke_handler(tauri::generate_handler![generate_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn generate_image(song: Song) -> String {
    let url = image_generation::generate_image(song).await;
    info!("{}", url);
    url
}


