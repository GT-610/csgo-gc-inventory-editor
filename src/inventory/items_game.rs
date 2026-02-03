use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGItem {
    pub name: String,
    pub item_name: String,
    pub prefab: Option<String>,
}

impl IGItem {
    pub fn get_display_name(&self, translations: &GameTranslation) -> String {
        translations.get(&self.item_name).unwrap_or(&self.item_name).clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGPaintKit {
    pub name: String,
    pub description_string: String,
    pub description_tag: String,
}

impl IGPaintKit {
    pub fn get_display_name(&self, translations: &GameTranslation) -> String {
        translations.get(&self.description_tag).unwrap_or(&self.description_string).clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGStickerKit {
    pub index: u32,
    pub name: String,
    pub description_string: String,
    pub item_name: String,
}

impl IGStickerKit {
    pub fn get_display_name(&self, translations: &GameTranslation) -> String {
        translations.get(&self.item_name).unwrap_or(&self.item_name).clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGMusicDef {
    pub name: String,
    pub loc_description: String,
    pub loc_name: String,
}

impl IGMusicDef {
    pub fn get_display_name(&self, translations: &GameTranslation) -> String {
        translations.get(&self.loc_name).unwrap_or(&self.loc_name).clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGRarity {
    pub value: u32,
    pub loc_key: String,
    pub loc_key_weapon: Option<String>,
    pub loc_key_character: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGQuality {
    pub name: String,
    pub value: u32,
}

impl IGQuality {
    pub fn get_display_name(&self, translations: &GameTranslation) -> String {
        translations.get(&self.name).unwrap_or(&self.name).clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGGraffitiTint {
    pub name: String,
    pub id: u32,
    pub hex_color: String,
}

#[derive(Debug, Clone, Default)]
pub struct GameTranslation {
    pub map: HashMap<String, String>,
}

impl GameTranslation {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}

pub struct ItemsGame {
    pub items: HashMap<u32, IGItem>,
    pub paint_kits: HashMap<u32, IGPaintKit>,
    pub sticker_kits: HashMap<u32, IGStickerKit>,
    pub music_defs: HashMap<u32, IGMusicDef>,
    pub rarities: HashMap<String, IGRarity>,
    pub qualities: HashMap<String, IGQuality>,
    pub graffiti_tints: HashMap<String, IGGraffitiTint>,
    pub paint_kits_rarity: HashMap<String, String>,
}

impl Default for ItemsGame {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            paint_kits: HashMap::new(),
            sticker_kits: HashMap::new(),
            music_defs: HashMap::new(),
            rarities: HashMap::new(),
            qualities: HashMap::new(),
            graffiti_tints: HashMap::new(),
            paint_kits_rarity: HashMap::new(),
        }
    }
}

impl ItemsGame {
    pub fn get_item_display_name(&self, def_index: u32, translations: &GameTranslation) -> String {
        if let Some(item) = self.items.get(&def_index) {
            item.get_display_name(translations)
        } else {
            format!("??? {}", def_index)
        }
    }

    pub fn get_paint_kit_display_name(&self, paint_index: u32, translations: &GameTranslation) -> Option<String> {
        self.paint_kits.get(&paint_index).map(|pk| pk.get_display_name(translations))
    }

    pub fn get_sticker_kit_display_name(&self, sticker_index: u32, translations: &GameTranslation) -> Option<String> {
        self.sticker_kits.get(&sticker_index).map(|sk| sk.get_display_name(translations))
    }

    pub fn get_music_def_display_name(&self, music_index: u32, translations: &GameTranslation) -> Option<String> {
        self.music_defs.get(&music_index).map(|md| md.get_display_name(translations))
    }

    pub fn get_item_full_name(&self, item: &crate::inventory::models::Item, translations: &GameTranslation) -> String {
        let item_name = self.get_item_display_name(item.def_index, translations);

        if let Some(paint_index) = item.attributes.get(&6) {
            // Parse as f32 first (e.g., "1149.000000"), then convert to u32
            if let Ok(paint_id_f32) = paint_index.parse::<f32>() {
                let paint_id = paint_id_f32 as u32;
                if let Some(paint_name) = self.get_paint_kit_display_name(paint_id, translations) {
                    return format!("{} | {}", item_name, paint_name);
                }
            }
        }
        
        if let Some(music_index) = item.attributes.get(&166) {
            if let Ok(music_id) = music_index.parse::<u32>() {
                if let Some(music_name) = self.get_music_def_display_name(music_id, translations) {
                    return format!("{} | {}", item_name, music_name);
                }
            }
        }
        
        if let Some(sticker_index) = item.attributes.get(&113) {
            if let Ok(sticker_id) = sticker_index.parse::<u32>() {
                if let Some(sticker_name) = self.get_sticker_kit_display_name(sticker_id, translations) {
                    return format!("{} | {}", item_name, sticker_name);
                }
            }
        }

        item_name
    }
}
