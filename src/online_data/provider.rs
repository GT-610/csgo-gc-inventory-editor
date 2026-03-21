use crate::inventory::item_attribute::ItemAttribute;
use crate::inventory::{GameTranslation, Item, ItemsGame};
use crate::online_data::models::OnlineGameData;

pub enum DataProvider {
    Local(ItemsGame, GameTranslation),
    Online(OnlineGameData),
}

impl DataProvider {
    pub fn get_item_display_name(&self, def_index: u32) -> String {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_item_display_name(def_index, translations)
            }
            DataProvider::Online(data) => {
                if let Some(item) = data.items_by_def_index.get(&def_index) {
                    item.name
                        .clone()
                        .unwrap_or_else(|| format!("??? {}", def_index))
                } else {
                    format!("??? {}", def_index)
                }
            }
        }
    }

    pub fn get_paint_kit_display_name(&self, paint_index: u32) -> Option<String> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_paint_kit_display_name(paint_index, translations)
            }
            DataProvider::Online(data) => data.skins_by_paint_index.get(&paint_index).map(|skin| {
                let skin_name = skin.name.clone();
                if let Some(wear) = &skin.wear
                    && let Some(wear_name) = &wear.name
                {
                    return format!("{} ({})", skin_name, wear_name);
                }
                skin_name
            }),
        }
    }

    pub fn get_sticker_kit_display_name(&self, sticker_index: u32) -> Option<String> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_sticker_kit_display_name(sticker_index, translations)
            }
            DataProvider::Online(data) => data
                .stickers_by_index
                .get(&sticker_index)
                .map(|s| s.name.clone()),
        }
    }

    pub fn get_music_def_display_name(&self, music_index: u32) -> Option<String> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_music_def_display_name(music_index, translations)
            }
            DataProvider::Online(data) => data
                .music_kits_by_def_index
                .get(&music_index)
                .and_then(|m| m.name.clone()),
        }
    }

    pub fn get_paint_kit_rarity(&self, paint_index: u32) -> Option<u32> {
        match self {
            DataProvider::Local(items_game, _translations) => {
                items_game.get_paint_kit_rarity(paint_index)
            }
            DataProvider::Online(data) => {
                let skin = data.skins_by_paint_index.get(&paint_index)?;
                let rarity_name = skin.rarity.as_ref()?.name.as_ref()?;
                rarity_name_to_id(rarity_name)
            }
        }
    }

    pub fn get_item_full_name(&self, item: &Item) -> String {
        let item_name = self.get_item_display_name(item.def_index);

        if let Some(paint_index) = item.attributes.get(&ItemAttribute::SkinPaintIndex.id())
            && let Ok(paint_id_f32) = paint_index.parse::<f32>()
        {
            let paint_id = paint_id_f32 as u32;
            if let Some(paint_name) = self.get_paint_kit_display_name(paint_id) {
                return format!("{} | {}", item_name, paint_name);
            }
        }

        if let Some(music_index) = item.attributes.get(&ItemAttribute::MusicID.id())
            && let Ok(music_id) = music_index.parse::<u32>()
            && let Some(music_name) = self.get_music_def_display_name(music_id)
        {
            return format!("{} | {}", item_name, music_name);
        }

        if let Some(sticker_index) = item.attributes.get(&ItemAttribute::Sticker0ID.id())
            && let Ok(sticker_id) = sticker_index.parse::<u32>()
            && let Some(sticker_name) = self.get_sticker_kit_display_name(sticker_id)
        {
            return format!("{} | {}", item_name, sticker_name);
        }

        item_name
    }

    pub fn create_item_select_list(&self) -> Vec<(String, String, String)> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.create_item_select_list(translations)
            }
            DataProvider::Online(data) => {
                let mut items: Vec<(String, String, String)> = data
                    .items_by_def_index
                    .iter()
                    .map(|(def_index, item)| {
                        let display_name = item.name.clone().unwrap_or_default();
                        (def_index.to_string(), display_name, def_index.to_string())
                    })
                    .collect();
                items.sort_by_key(|(key, _, _)| key.parse::<u32>().unwrap_or(0));
                items
            }
        }
    }

    pub fn create_paint_kit_select_list(&self) -> Vec<(String, String, String)> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.create_paint_kit_select_list(translations)
            }
            DataProvider::Online(data) => {
                let mut items: Vec<(String, String, String)> = data
                    .skins_by_paint_index
                    .iter()
                    .map(|(paint_index, skin)| {
                        let display_name = skin.name.clone();
                        (
                            paint_index.to_string(),
                            display_name,
                            paint_index.to_string(),
                        )
                    })
                    .collect();
                items.sort_by_key(|(key, _, _)| key.parse::<u32>().unwrap_or(0));
                items
            }
        }
    }

    pub fn create_music_def_select_list(&self) -> Vec<(String, String, String)> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.create_music_def_select_list(translations)
            }
            DataProvider::Online(data) => {
                let mut items: Vec<(String, String, String)> = data
                    .music_kits_by_def_index
                    .iter()
                    .map(|(music_index, music_kit)| {
                        let display_name = music_kit.name.clone().unwrap_or_default();
                        (
                            music_index.to_string(),
                            display_name,
                            music_index.to_string(),
                        )
                    })
                    .collect();
                items.sort_by_key(|(key, _, _)| key.parse::<u32>().unwrap_or(0));
                items
            }
        }
    }

    pub fn create_sticker_kit_select_list(&self) -> Vec<(String, String, String)> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.create_sticker_kit_select_list(translations)
            }
            DataProvider::Online(data) => {
                let mut items: Vec<(String, String, String)> = data
                    .stickers_by_index
                    .iter()
                    .map(|(sticker_index, sticker)| {
                        let display_name = sticker.name.clone();
                        (
                            sticker_index.to_string(),
                            display_name,
                            sticker_index.to_string(),
                        )
                    })
                    .collect();
                items.sort_by_key(|(key, _, _)| key.parse::<u32>().unwrap_or(0));
                items
            }
        }
    }

    pub fn get_all_rarities_sorted(&self) -> Vec<(u32, String)> {
        match self {
            DataProvider::Local(items_game, _translations) => items_game.get_all_rarities_sorted(),
            DataProvider::Online(_) => {
                vec![
                    (0, "Default".to_string()),
                    (1, "Consumer Grade".to_string()),
                    (2, "Industrial Grade".to_string()),
                    (3, "Mil-Spec Grade".to_string()),
                    (4, "Restricted".to_string()),
                    (5, "Classified".to_string()),
                    (6, "Covert".to_string()),
                    (7, "Contraband".to_string()),
                ]
            }
        }
    }

    pub fn get_all_qualities_sorted(&self) -> Vec<(u32, String)> {
        match self {
            DataProvider::Local(items_game, _translations) => items_game.get_all_qualities_sorted(),
            DataProvider::Online(_) => {
                vec![
                    (0, "Normal".to_string()),
                    (1, "Genuine".to_string()),
                    (2, "Vintage".to_string()),
                    (3, "Unusual".to_string()),
                    (4, "Unique".to_string()),
                    (5, "Community".to_string()),
                    (6, "Developer".to_string()),
                    (7, "Selfmade".to_string()),
                    (8, "Customized".to_string()),
                    (9, "Strange".to_string()),
                    (10, "Completed".to_string()),
                    (11, "Haunted".to_string()),
                    (12, "Tournament".to_string()),
                ]
            }
        }
    }

    pub fn as_items_game(&self) -> Option<&ItemsGame> {
        match self {
            DataProvider::Local(items_game, _) => Some(items_game),
            DataProvider::Online(_) => None,
        }
    }

    pub fn as_items_game_mut(&mut self) -> Option<&mut ItemsGame> {
        match self {
            DataProvider::Local(items_game, _) => Some(items_game),
            DataProvider::Online(_) => None,
        }
    }

    pub fn as_translations(&self) -> Option<&GameTranslation> {
        match self {
            DataProvider::Local(_, translations) => Some(translations),
            DataProvider::Online(_) => None,
        }
    }

    pub fn as_translations_mut(&mut self) -> Option<&mut GameTranslation> {
        match self {
            DataProvider::Local(_, translations) => Some(translations),
            DataProvider::Online(_) => None,
        }
    }
}

fn rarity_name_to_id(name: &str) -> Option<u32> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "consumer grade" | "consumer" => Some(1),
        "industrial grade" | "industrial" => Some(2),
        "mil-spec grade" | "mil-spec" => Some(3),
        "restricted" => Some(4),
        "classified" => Some(5),
        "covert" => Some(6),
        "contraband" => Some(7),
        "extraordinary" => Some(6),
        "distinguished" => Some(3),
        "exceptional" => Some(4),
        "superior" => Some(5),
        "master" => Some(6),
        _ => None,
    }
}
