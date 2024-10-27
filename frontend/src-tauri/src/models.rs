use crate::spotify_repository;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static ARTIST_CACHE: RefCell<HashMap<String, Artist>> = RefCell::new(HashMap::new());
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Song {
    pub(crate) title: String,
    pub(crate) artist: Vec<Artist>,
    pub(crate) album_name: String,
    pub(crate) image_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Artist {
    pub(crate) name: String,
    pub(crate) image_url: String,
}

async fn get_or_fetch_artist(json: &Value) -> Artist {
    let artist_name = json["name"].as_str().unwrap().to_string();

    if let Some(artist) = ARTIST_CACHE.with(|cache| {
        cache.borrow().get(&artist_name).cloned()
    }) {
        return artist;
    }

    let artist = Artist::from_json(json).await;

    ARTIST_CACHE.with(|cache| {
        cache.borrow_mut().insert(artist_name.clone(), artist.clone());
    });

    artist
}

impl Artist {
    pub(crate) async fn from_json(json: &Value) -> Self {
        let artist_response = spotify_repository::get(json["href"].as_str().unwrap()).await.unwrap();

        let response_body = artist_response.text().await.unwrap();

        let body_as_json: Value = serde_json::from_str(&*response_body).expect(&format!("Failed to parse JSON {}", response_body));

        let image_urls = body_as_json["images"].as_array()
            .unwrap()
            .iter()
            .filter_map(|image| image["url"].as_str())
            .collect::<Vec<_>>();

        Artist {
            name: json["name"].as_str().unwrap().to_string(),
            image_url: image_urls[0].parse().unwrap(),
        }
    }
}

impl Song {
    pub async fn from_json(json: Value) -> Self {
        let song_name = json["item"]["name"].as_str().unwrap();

        let artist_futures = json["item"]["artists"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|artist| Some(get_or_fetch_artist(artist)))
            .collect::<Vec<_>>();
        let artists = join_all(artist_futures).await;

        let image_urls = json["item"]["album"]["images"].as_array()
            .unwrap()
            .iter()
            .filter_map(|image| image["url"].as_str())
            .collect::<Vec<_>>();

        let image_url = image_urls[0];

        let album_name = json["item"]["album"]["name"].as_str().unwrap();

        Song {
            title: song_name.to_string(),
            artist: artists,
            album_name: album_name.parse().unwrap(),
            image_url: image_url.to_string(),
        }
    }
}

