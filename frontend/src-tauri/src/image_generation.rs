use crate::models::{ImageRequest, PromptRequest, Song};
use log::info;
use serde_json::{to_string, Value};
use tauri::image::Image;

pub async fn generate_image(song: Song) -> String {
    let prompt = generate_prompt(song).await;
    generate_image_from_prompt(prompt).await
}

async fn generate_prompt(song: Song) -> String {
    let req_body = to_string(&PromptRequest::from_song(&song)).unwrap();

    info!("Sending request to OpenAI AI for Prompt");
    let res = get_from_open_api("https://api.openai.com/v1/chat/completions", req_body).await.expect(&*format!("Failed to generate prompt for song - {:?}", song));
    let status = res.status().as_str().to_string();

    let res_body = res.text().await.unwrap();

    info!("Received response from OpenAI Prompt generation with status {}", status);

    info!("{:?}", res_body);

    let res_body_json: Value = serde_json::from_str(&res_body).unwrap();
    let mut prompt = res_body_json["choices"][0]["message"]["content"].as_str().unwrap().to_string();
    prompt.truncate(1000);
    prompt
}

async fn generate_image_from_prompt(prompt: String) -> String {
    let req_body = to_string(&ImageRequest::from_prompt(&prompt)).unwrap();

    info!("Sending request to OpenAI AI for Image");
    let res = get_from_open_api("https://api.openai.com/v1/images/generations", req_body)
        .await.expect(&*format!("Failed to generate image for prompt - {:?}", prompt));
    let status = res.status().as_str().to_string();

    let res_body = res.text().await.expect("Failed to get response body for image generation");

    info!("Received response from OpenAI Image generation with status {}", status);

    let res_body_json: Value = serde_json::from_str(&res_body).expect("Failed to parse response body for image generation");
    res_body_json["data"][0]["url"].to_string()
}

async fn get_from_open_api(url: &str, body: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.post(url)
        .body(body)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer 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")
        .send().await
}