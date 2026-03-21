use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

fn deserialize_u32_from_string_or_int<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Number(n)) => {
            n.as_u64().map(|v| v as u32).ok_or_else(|| {
                D::Error::custom("Invalid number format for def_index")
            }).map(Some)
        }
        Some(serde_json::Value::String(s)) => {
            s.parse::<u32>().map(Some).map_err(|_| {
                D::Error::custom(format!("Invalid string format for def_index: {}", s))
            })
        }
        _ => Err(D::Error::custom("Expected string or number for def_index")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRarity {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiWeapon {
    pub weapon_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiWear {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiItem {
    pub id: String,
    pub name: Option<String>,
    #[serde(default)]
    pub market_hash_name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_u32_from_string_or_int")]
    pub def_index: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_u32_from_string_or_int")]
    pub paint_index: Option<u32>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub rarity: Option<ApiRarity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinItem {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub paint_index: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub weapon: Option<ApiWeapon>,
    #[serde(default)]
    pub min_float: Option<f32>,
    #[serde(default)]
    pub max_float: Option<f32>,
    #[serde(default)]
    pub wear: Option<ApiWear>,
    #[serde(default)]
    pub rarity: Option<ApiRarity>,
    #[serde(default)]
    pub stattrak: Option<bool>,
    #[serde(default)]
    pub souvenir: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerItem {
    pub id: String,
    pub name: String,
    pub sticker_index: String,
    #[serde(default)]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicKitItem {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub market_hash_name: Option<String>,
    #[serde(default)]
    pub def_index: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub rarity: Option<ApiRarity>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub exclusive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectibleItem {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub market_hash_name: Option<String>,
    #[serde(default)]
    pub def_index: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub rarity: Option<ApiRarity>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub premier_season: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct OnlineGameData {
    pub base_weapons: Vec<ApiItem>,
    pub skins: Vec<SkinItem>,
    pub stickers: Vec<StickerItem>,
    pub music_kits: Vec<MusicKitItem>,
    pub collectibles: Vec<CollectibleItem>,
    pub crates: Vec<ApiItem>,
    pub keys: Vec<ApiItem>,

    pub items_by_def_index: HashMap<u32, ApiItem>,
    pub skins_by_paint_index: HashMap<u32, SkinItem>,
    pub stickers_by_index: HashMap<u32, StickerItem>,
    pub music_kits_by_def_index: HashMap<u32, MusicKitItem>,
}

impl OnlineGameData {
    pub fn build_indexes(&mut self) {
        for item in &self.base_weapons {
            if let Some(def_index) = item.def_index {
                self.items_by_def_index.insert(def_index, item.clone());
            }
        }

        for skin in &self.skins {
            if let Some(paint_index_str) = &skin.paint_index
                && let Ok(paint_index) = paint_index_str.parse::<u32>()
            {
                self.skins_by_paint_index.insert(paint_index, skin.clone());
            }
        }

        for sticker in &self.stickers {
            if let Ok(sticker_index) = sticker.sticker_index.parse::<u32>() {
                self.stickers_by_index
                    .insert(sticker_index, sticker.clone());
            }
        }

        for music_kit in &self.music_kits {
            if let Some(def_index_str) = &music_kit.def_index
                && let Ok(def_index) = def_index_str.parse::<u32>()
            {
                self.music_kits_by_def_index
                    .insert(def_index, music_kit.clone());
            }
        }
    }
}
