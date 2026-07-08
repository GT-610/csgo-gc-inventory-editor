use crate::online_data::models::{InventoryData, OnlineGameData};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

const API_BASE: &str =
    "https://raw.githubusercontent.com/ByMykel/CSGO-API/refs/heads/main/public/api";

fn get_cache_base_dir() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    exe_dir.join("csgo_gc").join("editor").join("cache")
}

fn get_cache_dir(language: &str) -> PathBuf {
    let lang_code = if language == "zh-Hans" { "zh-CN" } else { "en" };
    get_cache_base_dir().join(lang_code)
}

fn get_meta_file(language: &str) -> PathBuf {
    get_cache_dir(language).join("meta.json")
}

#[derive(Debug)]
pub enum ApiError {
    Network(String),
    Parse(String),
    Cache(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Network(msg) => write!(f, "Network error: {}", msg),
            ApiError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ApiError::Cache(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CacheMeta {
    timestamp: String,
}

pub fn load_cached_data(language: &str) -> Option<(OnlineGameData, String)> {
    let cache_dir = get_cache_dir(language);
    let meta_file = get_meta_file(language);
    let inventory_file = cache_dir.join("inventory.json");

    println!("[load_cached_data] Cache directory: {:?}", cache_dir);
    println!("[load_cached_data] Inventory file: {:?}", inventory_file);

    if !inventory_file.exists() {
        println!("[load_cached_data] Missing inventory.json");
        return None;
    }

    if !meta_file.exists() {
        println!("[load_cached_data] Missing meta.json");
        return None;
    }

    let meta_content = fs::read_to_string(&meta_file).ok()?;
    let meta: CacheMeta = serde_json::from_str(&meta_content).ok()?;

    let inventory: InventoryData = load_cache_file_single(language, "inventory.json")?;

    let data = OnlineGameData {
        inventory: Some(inventory),
    };

    println!("[load_cached_data] Cache loaded successfully");
    Some((data, meta.timestamp))
}

fn load_cache_file_single<T: serde::de::DeserializeOwned>(
    language: &str,
    filename: &str,
) -> Option<T> {
    let path = get_cache_dir(language).join(filename);
    if !path.exists() {
        return None;
    }
    let content = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn save_cached_data(language: &str, data: &OnlineGameData) -> Result<String, ApiError> {
    println!("[save_cached_data] Starting save...");
    let cache_dir = get_cache_dir(language);
    if !cache_dir.exists() {
        println!(
            "[save_cached_data] Creating cache directory: {:?}",
            cache_dir
        );
        fs::create_dir_all(&cache_dir).map_err(|e| ApiError::Cache(e.to_string()))?;
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if let Some(ref inventory) = data.inventory {
        println!("[save_cached_data] Saving inventory.json");
        save_cache_file_single(language, "inventory.json", inventory)?;
    }

    let meta = CacheMeta {
        timestamp: timestamp.clone(),
    };
    let meta_content =
        serde_json::to_string_pretty(&meta).map_err(|e| ApiError::Cache(e.to_string()))?;
    let meta_file = get_meta_file(language);
    fs::write(&meta_file, meta_content).map_err(|e| ApiError::Cache(e.to_string()))?;

    println!(
        "[save_cached_data] All files saved successfully to {:?}",
        cache_dir
    );
    Ok(timestamp)
}

fn save_cache_file_single<T: serde::Serialize>(
    language: &str,
    filename: &str,
    data: &T,
) -> Result<(), ApiError> {
    let path = get_cache_dir(language).join(filename);
    let content = serde_json::to_string_pretty(data).map_err(|e| ApiError::Cache(e.to_string()))?;
    fs::write(&path, content).map_err(|e| ApiError::Cache(e.to_string()))
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
    let inventory_url = format!("{}/{}/inventory.json", API_BASE, lang_code);

    progress_callback("Downloading inventory.json...");
    let inventory = fetch_json_blocking::<InventoryData>(&inventory_url, mirror_prefix)?;

    let data = OnlineGameData {
        inventory: Some(inventory),
    };

    Ok(data)
}
