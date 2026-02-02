use crate::inventory::items_game::{IGItem, IGPaintKit, IGStickerKit, IGMusicDef, IGRarity, IGQuality, IGGraffitiTint, ItemsGame};
use crate::inventory::vdf::{VdfParser, VdfValue};
use std::collections::HashMap;
use std::path::Path;

pub struct ItemsGameLoader;

impl ItemsGameLoader {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<ItemsGame, ItemsGameLoadError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ItemsGameLoadError::Io(e))?;
        Self::parse_from_str(&content)
    }

    pub fn parse_from_str(content: &str) -> Result<ItemsGame, ItemsGameLoadError> {
        let vdf = VdfParser::parse(content)
            .map_err(|e| ItemsGameLoadError::Parse(e.to_string()))?;

        let mut items_game = ItemsGame::default();

        if let Some(root_obj) = vdf.get("items_game").and_then(|v| v.as_object()) {
            Self::parse_rarities(root_obj, &mut items_game);
            Self::parse_qualities(root_obj, &mut items_game);
            Self::parse_items(root_obj, &mut items_game);
            Self::parse_music_definitions(root_obj, &mut items_game);
            Self::parse_paint_kits(root_obj, &mut items_game);
            Self::parse_sticker_kits(root_obj, &mut items_game);
            Self::parse_paint_kits_rarity(root_obj, &mut items_game);
            Self::parse_graffiti_tints(root_obj, &mut items_game);
        }

        Ok(items_game)
    }

    fn parse_rarities(root_obj: &HashMap<String, VdfValue>, items_game: &mut ItemsGame) {
        if let Some(rarities_obj) = root_obj.get("rarities").and_then(|v| v.as_object()) {
            for (key, value) in rarities_obj {
                if let Some(obj) = value.as_object() {
                    let rarity = IGRarity {
                        value: get_u32_from_obj(obj, "value").unwrap_or(0),
                        loc_key: get_string_from_obj(obj, "loc_key").unwrap_or_default(),
                        loc_key_weapon: get_string_from_obj(obj, "loc_key_weapon"),
                        loc_key_character: get_string_from_obj(obj, "loc_key_character"),
                    };
                    items_game.rarities.insert(key.clone(), rarity);
                }
            }
        }
    }

    fn parse_qualities(root_obj: &HashMap<String, VdfValue>, items_game: &mut ItemsGame) {
        if let Some(qualities_obj) = root_obj.get("qualities").and_then(|v| v.as_object()) {
            for (key, value) in qualities_obj {
                if let Some(obj) = value.as_object() {
                    let quality = IGQuality {
                        name: key.clone(),
                        value: get_u32_from_obj(obj, "value").unwrap_or(0),
                    };
                    items_game.qualities.insert(key.clone(), quality);
                }
            }
        }
    }

    fn parse_items(root_obj: &HashMap<String, VdfValue>, items_game: &mut ItemsGame) {
        let prefabs_obj = root_obj.get("prefabs").and_then(|v| v.as_object());

        if let Some(items_obj) = root_obj.get("items").and_then(|v| v.as_object()) {
            for (key, value) in items_obj {
                if let Some(obj) = value.as_object() {
                    let name = get_string_from_obj(obj, "name")
                        .unwrap_or_else(|| key.clone());

                    let prefab = get_string_from_obj(obj, "prefab");

                    let item_name = if let Some(item_name) = get_string_from_obj(obj, "item_name") {
                        item_name
                    } else if let Some(ref prefab_name) = prefab {
                        if let Some(prefabs) = prefabs_obj {
                            if let Some(prefab_obj) = prefabs.get(prefab_name).and_then(|v| v.as_object()) {
                                if let Some(prefab_item_name) = get_string_from_obj(prefab_obj, "item_name") {
                                    prefab_item_name
                                } else {
                                    name.clone()
                                }
                            } else {
                                name.clone()
                            }
                        } else {
                            name.clone()
                        }
                    } else {
                        name.clone()
                    };

                    let item = IGItem {
                        name,
                        item_name: item_name.strip_prefix('#').unwrap_or(&item_name).to_string(),
                        prefab,
                    };

                    let def_index = if key == "default" { 0 } else {
                        key.parse::<u32>().unwrap_or(0)
                    };

                    items_game.items.insert(def_index, item);
                }
            }
        }
    }

    fn parse_music_definitions(root_obj: &HashMap<String, VdfValue>, items_game: &mut ItemsGame) {
        if let Some(music_obj) = root_obj.get("music_definitions").and_then(|v| v.as_object()) {
            for (key, value) in music_obj {
                if let Some(obj) = value.as_object() {
                    let loc_name = get_string_from_obj(obj, "loc_name")
                        .map(|s| s.strip_prefix('#').unwrap_or(&s).to_string())
                        .unwrap_or_else(|| key.clone());

                    let loc_description = get_string_from_obj(obj, "loc_description")
                        .map(|s| s.strip_prefix('#').unwrap_or(&s).to_string())
                        .unwrap_or_default();

                    let music = IGMusicDef {
                        name: key.clone(),
                        loc_description,
                        loc_name,
                    };

                    if let Ok(index) = key.parse::<u32>() {
                        items_game.music_defs.insert(index, music);
                    }
                }
            }
        }
    }

    fn parse_paint_kits(root_obj: &HashMap<String, VdfValue>, items_game: &mut ItemsGame) {
        if let Some(paint_kits_obj) = root_obj.get("paint_kits").and_then(|v| v.as_object()) {
            for (key, value) in paint_kits_obj {
                if let Some(obj) = value.as_object() {
                    let name = get_string_from_obj(obj, "name").unwrap_or_else(|| key.clone());

                    let description_string = get_string_from_obj(obj, "description_string")
                        .unwrap_or_else(|| name.clone());

                    let description_tag = get_string_from_obj(obj, "description_tag")
                        .unwrap_or_else(|| name.clone());

                    let paint_kit = IGPaintKit {
                        name,
                        description_string: description_string.strip_prefix('#').unwrap_or(&description_string).to_string(),
                        description_tag: description_tag.strip_prefix('#').unwrap_or(&description_tag).to_string(),
                    };

                    if let Ok(index) = key.parse::<u32>() {
                        items_game.paint_kits.insert(index, paint_kit);
                    }
                }
            }
        }
    }

    fn parse_sticker_kits(root_obj: &HashMap<String, VdfValue>, items_game: &mut ItemsGame) {
        if let Some(sticker_kits_obj) = root_obj.get("sticker_kits").and_then(|v| v.as_object()) {
            for (key, value) in sticker_kits_obj {
                if let Some(obj) = value.as_object() {
                    let name = get_string_from_obj(obj, "name").unwrap_or_else(|| key.clone());

                    let description_string = get_string_from_obj(obj, "description_string")
                        .unwrap_or_else(|| name.clone());

                    let item_name = get_string_from_obj(obj, "item_name")
                        .unwrap_or_else(|| name.clone());

                    let sticker_kit = IGStickerKit {
                        index: key.parse::<u32>().unwrap_or(0),
                        name,
                        description_string: description_string.strip_prefix('#').unwrap_or(&description_string).to_string(),
                        item_name: item_name.strip_prefix('#').unwrap_or(&item_name).to_string(),
                    };

                    items_game.sticker_kits.insert(sticker_kit.index, sticker_kit);
                }
            }
        }
    }

    fn parse_paint_kits_rarity(root_obj: &HashMap<String, VdfValue>, items_game: &mut ItemsGame) {
        if let Some(rarity_obj) = root_obj.get("paint_kits_rarity").and_then(|v| v.as_object()) {
            for (key, value) in rarity_obj {
                if let Some(s) = value.as_string() {
                    items_game.paint_kits_rarity.insert(key.clone(), s.to_string());
                }
            }
        }
    }

    fn parse_graffiti_tints(root_obj: &HashMap<String, VdfValue>, items_game: &mut ItemsGame) {
        if let Some(graffiti_obj) = root_obj.get("graffiti_tints").and_then(|v| v.as_object()) {
            for (key, value) in graffiti_obj {
                if let Some(obj) = value.as_object() {
                    let tint = IGGraffitiTint {
                        name: key.clone(),
                        id: get_u32_from_obj(obj, "id").unwrap_or(0),
                        hex_color: get_string_from_obj(obj, "hex_color").unwrap_or_default(),
                    };
                    items_game.graffiti_tints.insert(key.clone(), tint);
                }
            }
        }
    }
}

fn get_string_from_obj(obj: &HashMap<String, VdfValue>, key: &str) -> Option<String> {
    obj.get(key).and_then(|v| v.as_string().map(|s| s.to_string()))
}

fn get_u32_from_obj(obj: &HashMap<String, VdfValue>, key: &str) -> Option<u32> {
    obj.get(key)
        .and_then(|v| v.as_string())
        .and_then(|s| s.parse::<u32>().ok())
}

#[derive(Debug)]
pub enum ItemsGameLoadError {
    Io(std::io::Error),
    Parse(String),
}

impl std::fmt::Display for ItemsGameLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemsGameLoadError::Io(e) => write!(f, "IO Error: {}", e),
            ItemsGameLoadError::Parse(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl std::error::Error for ItemsGameLoadError {}
