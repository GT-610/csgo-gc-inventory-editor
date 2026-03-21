use crate::inventory::item_attribute::ItemAttribute;
use crate::inventory::{GameTranslation, Item, ItemsGame};
use crate::online_data::models::OnlineGameData;

pub enum DataProvider {
    Local(Box<ItemsGame>, GameTranslation),
    Online(Box<OnlineGameData>, Box<ItemsGame>, GameTranslation),
}

impl DataProvider {
    pub fn get_item_display_name(&self, def_index: u32) -> String {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_item_display_name(def_index, translations)
            }
            DataProvider::Online(_data, items_game, translations) => {
                // Force use local data for display name (online format incompatible)
                items_game.get_item_display_name(def_index, translations)
            }
        }
    }

    pub fn get_paint_kit_display_name(&self, paint_index: u32) -> Option<String> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_paint_kit_display_name(paint_index, translations)
            }
            DataProvider::Online(_data, items_game, translations) => {
                // Force use local data for display name (online format incompatible)
                items_game.get_paint_kit_display_name(paint_index, translations)
            }
        }
    }

    // Get skin display name from online inventory data (requires weapon_id and paint_index)
    pub fn get_skin_display_name(&self, weapon_id: u32, paint_index: u32) -> Option<String> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_paint_kit_display_name(paint_index, translations)
            }
            DataProvider::Online(data, items_game, translations) => {
                // Try online inventory data first
                if let Some(skin) = data.get_inventory_skin(weapon_id, paint_index) {
                    return Some(skin.name.clone());
                }
                // Fallback to local data
                items_game.get_paint_kit_display_name(paint_index, translations)
            }
        }
    }

    pub fn get_sticker_kit_display_name(&self, sticker_index: u32) -> Option<String> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_sticker_kit_display_name(sticker_index, translations)
            }
            DataProvider::Online(_data, items_game, translations) => {
                // Force use local data for display name (online format incompatible)
                items_game.get_sticker_kit_display_name(sticker_index, translations)
            }
        }
    }

    pub fn get_music_def_display_name(&self, music_index: u32) -> Option<String> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.get_music_def_display_name(music_index, translations)
            }
            DataProvider::Online(data, items_game, translations) => {
                // Try online inventory data first
                if let Some(music_kit) = data.get_inventory_music_kit(music_index) {
                    return Some(music_kit.name.clone());
                }
                // Fallback to local data
                items_game.get_music_def_display_name(music_index, translations)
            }
        }
    }

    pub fn get_paint_kit_rarity(&self, paint_index: u32) -> Option<u32> {
        match self {
            DataProvider::Local(items_game, _translations) => {
                items_game.get_paint_kit_rarity(paint_index)
            }
            DataProvider::Online(_data, items_game, _translations) => {
                items_game.get_paint_kit_rarity(paint_index)
            }
        }
    }

    // Get skin rarity from online inventory data (requires weapon_id and paint_index)
    pub fn get_skin_rarity(&self, weapon_id: u32, paint_index: u32) -> Option<u32> {
        match self {
            DataProvider::Local(items_game, _translations) => {
                items_game.get_paint_kit_rarity(paint_index)
            }
            DataProvider::Online(data, items_game, _translations) => {
                // Try online inventory data first
                if let Some(skin) = data.get_inventory_skin(weapon_id, paint_index)
                    && let Some(ref rarity) = skin.rarity
                {
                    // Convert online rarity id to local rarity value
                    if let Some(value) = items_game.get_rarity_value_by_id(&rarity.id) {
                        return Some(value);
                    }
                }
                // Fallback to local data
                items_game.get_paint_kit_rarity(paint_index)
            }
        }
    }

    pub fn get_item_full_name(&self, item: &Item) -> String {
        let item_name = self.get_item_display_name(item.def_index);

        if let Some(paint_index) = item.attributes.get(&ItemAttribute::SkinPaintIndex.id())
            && let Ok(paint_id_f32) = paint_index.parse::<f32>()
        {
            let paint_id = paint_id_f32 as u32;
            match self {
                DataProvider::Local(_, _) => {
                    // Local mode: combine item name and paint name
                    if let Some(paint_name) = self.get_skin_display_name(item.def_index, paint_id) {
                        return format!("{} | {}", item_name, paint_name);
                    }
                }
                DataProvider::Online(data, _, _) => {
                    // Online mode: skin name is already full name (e.g., "AK-47 | Redline")
                    if let Some(skin) = data.get_inventory_skin(item.def_index, paint_id) {
                        return skin.name.clone();
                    }
                    // Fallback to local format if online data not found
                    if let Some(paint_name) = self.get_skin_display_name(item.def_index, paint_id) {
                        return format!("{} | {}", item_name, paint_name);
                    }
                }
            }
        }

        if let Some(music_index) = item.attributes.get(&ItemAttribute::MusicID.id())
            && let Ok(music_id) = music_index.parse::<u32>()
        {
            match self {
                DataProvider::Local(_, _) => {
                    // Local mode: combine item name and music name
                    if let Some(music_name) = self.get_music_def_display_name(music_id) {
                        return format!("{} | {}", item_name, music_name);
                    }
                }
                DataProvider::Online(data, _, _) => {
                    // Online mode: music kit name is already full name
                    if let Some(music_kit) = data.get_inventory_music_kit(music_id) {
                        return music_kit.name.clone();
                    }
                    // Fallback to local format if online data not found
                    if let Some(music_name) = self.get_music_def_display_name(music_id) {
                        return format!("{} | {}", item_name, music_name);
                    }
                }
            }
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
            DataProvider::Online(_data, items_game, translations) => {
                // Force use local data for display name (online format incompatible)
                items_game.create_item_select_list(translations)
            }
        }
    }

    pub fn create_paint_kit_select_list(&self) -> Vec<(String, String, String)> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.create_paint_kit_select_list(translations)
            }
            DataProvider::Online(_data, items_game, translations) => {
                // Force use local data for display name (online format incompatible)
                items_game.create_paint_kit_select_list(translations)
            }
        }
    }

    // Create skin select list for a specific weapon (online mode only shows skins for that weapon)
    pub fn create_skin_select_list_for_weapon(
        &self,
        weapon_id: u32,
    ) -> Vec<(String, String, String)> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.create_paint_kit_select_list(translations)
            }
            DataProvider::Online(data, items_game, translations) => {
                // Use online inventory data to get skins for this weapon
                if let Some(ref inventory) = data.inventory
                    && let Some(skins) = inventory.skins.get(&weapon_id.to_string())
                {
                    let mut items: Vec<(String, String, String)> = skins
                        .iter()
                        .map(|(paint_index, skin)| {
                            // "null" in online data means no paint (paint_index = 0)
                            let index = if paint_index == "null" {
                                "0".to_string()
                            } else {
                                paint_index.clone()
                            };
                            (index.clone(), skin.name.clone(), index)
                        })
                        .collect();
                    items.sort_by_key(|(key, _, _)| key.parse::<u32>().unwrap_or(0));
                    return items;
                }
                // Fallback to local data
                items_game.create_paint_kit_select_list(translations)
            }
        }
    }

    pub fn create_music_def_select_list(&self) -> Vec<(String, String, String)> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.create_music_def_select_list(translations)
            }
            DataProvider::Online(data, items_game, translations) => {
                // Use online inventory data for music kits
                if let Some(ref inventory) = data.inventory {
                    let mut items: Vec<(String, String, String)> = inventory
                        .music_kits
                        .iter()
                        .map(|(music_index, music_kit)| {
                            (
                                music_index.clone(),
                                music_kit.name.clone(),
                                music_index.clone(),
                            )
                        })
                        .collect();
                    items.sort_by_key(|(key, _, _)| key.parse::<u32>().unwrap_or(0));
                    return items;
                }
                // Fallback to local data
                items_game.create_music_def_select_list(translations)
            }
        }
    }

    pub fn create_sticker_kit_select_list(&self) -> Vec<(String, String, String)> {
        match self {
            DataProvider::Local(items_game, translations) => {
                items_game.create_sticker_kit_select_list(translations)
            }
            DataProvider::Online(_data, items_game, translations) => {
                // Force use local data for display name (online format incompatible)
                items_game.create_sticker_kit_select_list(translations)
            }
        }
    }

    pub fn get_all_rarities_sorted(&self) -> Vec<(u32, String)> {
        match self {
            DataProvider::Local(items_game, _translations) => items_game.get_all_rarities_sorted(),
            DataProvider::Online(_, items_game, _) => items_game.get_all_rarities_sorted(),
        }
    }

    pub fn get_all_qualities_sorted(&self) -> Vec<(u32, String)> {
        match self {
            DataProvider::Local(items_game, _translations) => items_game.get_all_qualities_sorted(),
            DataProvider::Online(_, items_game, _) => items_game.get_all_qualities_sorted(),
        }
    }

    pub fn as_items_game(&self) -> Option<&ItemsGame> {
        match self {
            DataProvider::Local(items_game, _) => Some(items_game),
            DataProvider::Online(_, items_game, _) => Some(items_game),
        }
    }

    pub fn as_items_game_mut(&mut self) -> Option<&mut ItemsGame> {
        match self {
            DataProvider::Local(items_game, _) => Some(items_game),
            DataProvider::Online(_, items_game, _) => Some(items_game),
        }
    }

    pub fn as_translations(&self) -> Option<&GameTranslation> {
        match self {
            DataProvider::Local(_, translations) => Some(translations),
            DataProvider::Online(_, _, translations) => Some(translations),
        }
    }

    pub fn as_translations_mut(&mut self) -> Option<&mut GameTranslation> {
        match self {
            DataProvider::Local(_, translations) => Some(translations),
            DataProvider::Online(_, _, translations) => Some(translations),
        }
    }
}
