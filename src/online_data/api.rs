use crate::online_data::models::{
    ApiItem, CollectibleItem, MusicKitItem, OnlineGameData, SkinItem, StickerItem,
};
use std::fs;
use std::path::Path;
use std::time::Duration;

const API_BASE: &str =
    "https://raw.githubusercontent.com/ByMykel/CSGO-API/refs/heads/main/public/api";
const STICKERS_URL: &str = "https://raw.githubusercontent.com/dricotec/CSGO-API-STICKERS-THING/refs/heads/main/stickers.json";
const CACHE_DIR: &str = "csgo_gc/editor/cache";
const CACHE_FILE: &str = "csgo_gc/editor/cache/online_data.json";

#[derive(Debug)]
pub enum ApiError {
    Network(String),
    Parse(String),
    Timeout,
    Cache(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Network(msg) => write!(f, "Network error: {}", msg),
            ApiError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ApiError::Timeout => write!(f, "Request timeout"),
            ApiError::Cache(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

pub fn load_cached_data() -> Option<(OnlineGameData, String)> {
    if !Path::new(CACHE_FILE).exists() {
        return None;
    }

    let content = fs::read_to_string(CACHE_FILE).ok()?;
    let cached: CachedOnlineData = serde_json::from_str(&content).ok()?;
    Some((cached.data, cached.timestamp))
}

pub fn save_cached_data(data: &OnlineGameData) -> Result<String, ApiError> {
    if !Path::new(CACHE_DIR).exists() {
        fs::create_dir_all(CACHE_DIR).map_err(|e| ApiError::Cache(e.to_string()))?;
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let cached = CachedOnlineData {
        data: data.clone(),
        timestamp: timestamp.clone(),
    };

    let content =
        serde_json::to_string_pretty(&cached).map_err(|e| ApiError::Cache(e.to_string()))?;

    fs::write(CACHE_FILE, content).map_err(|e| ApiError::Cache(e.to_string()))?;

    Ok(timestamp)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CachedOnlineData {
    data: OnlineGameData,
    timestamp: String,
}

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

    progress_callback("Downloading base_weapons.json...");
    data.base_weapons = fetch_json_blocking::<Vec<ApiItem>>(&base_weapons_url, mirror_prefix)?;

    progress_callback("Downloading skins_not_grouped.json...");
    data.skins = fetch_json_blocking::<Vec<SkinItem>>(&skins_url, mirror_prefix)?;

    progress_callback("Downloading stickers.json...");
    data.stickers = fetch_json_blocking::<Vec<StickerItem>>(STICKERS_URL, mirror_prefix)?;

    progress_callback("Downloading music_kits.json...");
    data.music_kits = fetch_json_blocking::<Vec<MusicKitItem>>(&music_kits_url, mirror_prefix)?;

    progress_callback("Downloading collectibles.json...");
    data.collectibles =
        fetch_json_blocking::<Vec<CollectibleItem>>(&collectibles_url, mirror_prefix)?;

    progress_callback("Downloading crates.json...");
    data.crates = fetch_json_blocking::<Vec<ApiItem>>(&crates_url, mirror_prefix)?;

    progress_callback("Downloading keys.json...");
    data.keys = fetch_json_blocking::<Vec<ApiItem>>(&keys_url, mirror_prefix)?;

    progress_callback("Building indexes...");
    data.build_indexes();

    Ok(data)
}
