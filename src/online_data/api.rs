use crate::online_data::models::{
    ApiItem, CollectibleItem, MusicKitItem, OnlineGameData, SkinItem, StickerItem,
};
use std::time::Duration;

const API_BASE: &str =
    "https://raw.githubusercontent.com/ByMykel/CSGO-API/refs/heads/main/public/api";
const STICKERS_URL: &str =
    "https://raw.githubusercontent.com/dricotec/CSGO-API-STICKERS-THING/refs/heads/main/stickers.json";

#[derive(Debug)]
pub enum ApiError {
    Network(String),
    Parse(String),
    Timeout,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Network(msg) => write!(f, "Network error: {}", msg),
            ApiError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ApiError::Timeout => write!(f, "Request timeout"),
        }
    }
}

impl std::error::Error for ApiError {}

fn apply_mirror(url: &str, mirror_prefix: &str) -> String {
    if mirror_prefix.is_empty() {
        url.to_string()
    } else {
        format!("{}{}", mirror_prefix, url)
    }
}

fn fetch_json_blocking<T: serde::de::DeserializeOwned>(
    url: &str,
    mirror_prefix: &str,
) -> Result<T, ApiError> {
    let final_url = apply_mirror(url, mirror_prefix);

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("CSGO-GC-Inventory-Editor")
        .build()
        .map_err(|e| ApiError::Network(e.to_string()))?;

    let response = client
        .get(&final_url)
        .send()
        .map_err(|e| ApiError::Network(e.to_string()))?;

    if !response.status().is_success() {
        return Err(ApiError::Network(format!(
            "HTTP status: {}",
            response.status()
        )));
    }

    let text = response
        .text()
        .map_err(|e| ApiError::Network(e.to_string()))?;

    serde_json::from_str::<T>(&text).map_err(|e| ApiError::Parse(e.to_string()))
}

pub fn fetch_online_data_with_progress<F>(
    language: &str,
    mirror_prefix: &str,
    mut progress_callback: F,
) -> Result<OnlineGameData, ApiError>
where
    F: FnMut(&str),
{
    let lang_code = if language == "zh-Hans" { "zh-CN" } else { "en" };

    let base_weapons_url = format!("{}/{}/base_weapons.json", API_BASE, lang_code);
    let skins_url = format!("{}/{}/skins_not_grouped.json", API_BASE, lang_code);
    let music_kits_url = format!("{}/{}/music_kits.json", API_BASE, lang_code);
    let collectibles_url = format!("{}/{}/collectibles.json", API_BASE, lang_code);
    let crates_url = format!("{}/{}/crates.json", API_BASE, lang_code);
    let keys_url = format!("{}/{}/keys.json", API_BASE, lang_code);

    let mut data = OnlineGameData::default();

    progress_callback("Fetching base weapons...");
    data.base_weapons = fetch_json_blocking::<Vec<ApiItem>>(&base_weapons_url, mirror_prefix)?;

    progress_callback("Fetching skins...");
    data.skins = fetch_json_blocking::<Vec<SkinItem>>(&skins_url, mirror_prefix)?;

    progress_callback("Fetching stickers...");
    data.stickers = fetch_json_blocking::<Vec<StickerItem>>(STICKERS_URL, mirror_prefix)?;

    progress_callback("Fetching music kits...");
    data.music_kits = fetch_json_blocking::<Vec<MusicKitItem>>(&music_kits_url, mirror_prefix)?;

    progress_callback("Fetching collectibles...");
    data.collectibles =
        fetch_json_blocking::<Vec<CollectibleItem>>(&collectibles_url, mirror_prefix)?;

    progress_callback("Fetching crates...");
    data.crates = fetch_json_blocking::<Vec<ApiItem>>(&crates_url, mirror_prefix)?;

    progress_callback("Fetching keys...");
    data.keys = fetch_json_blocking::<Vec<ApiItem>>(&keys_url, mirror_prefix)?;

    progress_callback("Building indexes...");
    data.build_indexes();

    Ok(data)
}
