use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Song {
    title: String,
    artist: String,
}

#[derive(Serialize, Debug)]
pub struct ImageRequest {
    model: String,
    prompt: String,
    n: u8,
    size: String,
    style: String,
}

#[derive(Serialize, Debug)]
pub struct PromptRequest {
    model: String,
    messages: Vec<PromptRequestMessage>,
}

#[derive(Serialize, Debug)]
pub struct PromptRequestMessage {
    role: String,
    content: String,
}

impl PromptRequest {
    pub(crate) fn from_song(song: &Song) -> Self {
        PromptRequest {
            model: "gpt-4o".to_string(),
            messages: vec![
                PromptRequestMessage {
                    role: "user".to_string(),
                    content: format!("Give me an image prompt for the song {} by {}. The image should not contain a person as the primary piece of content - should be vintage/artistic", song.title, song.artist)
                },
            ],
        }
    }
}

impl ImageRequest {
    pub(crate) fn from_prompt(prompt: &str) -> Self {
        ImageRequest {
            model: "dall-e-3".to_string(),
            prompt: prompt.to_string(),
            n: 1,
            size: "1024x1024".to_string(),
            style: "natural".to_string(),
        }
    }
}