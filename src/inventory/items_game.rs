use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::inventory::item_attribute::ItemAttribute;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGItem {
    pub name: String,
    pub item_name: String,
    pub prefab: Option<String>,
    pub item_class: Option<String>,
    pub item_type_name: Option<String>,
    pub inv_container_and_tools: Option<String>,
    pub associated_items: Vec<u32>,
}

impl IGItem {
    pub fn get_display_name(&self, translations: &GameTranslation) -> String {
        translations
            .get(&self.item_name)
            .unwrap_or(&self.item_name)
            .clone()
    }

    pub fn is_weapon_case(&self) -> bool {
        let has_weapon_case_prefab = self.prefab.as_deref() == Some("weapon_case");
        let has_weapon_case_container = self.item_class.as_deref() == Some("supply_crate")
            && self.inv_container_and_tools.as_deref() == Some("weapon_case")
            && self.item_type_name.as_deref() == Some("#CSGO_Type_WeaponCase");

        has_weapon_case_prefab || has_weapon_case_container
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
        translations
            .get(&self.description_tag)
            .unwrap_or(&self.description_string)
            .clone()
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
        translations
            .get(&self.item_name)
            .unwrap_or(&self.item_name)
            .clone()
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
        translations
            .get(&self.loc_name)
            .unwrap_or(&self.loc_name)
            .clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGRarity {
    pub value: u32,
    pub loc_key: String,
    pub loc_key_weapon: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IGQuality {
    pub name: String,
    pub value: u32,
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

#[derive(Default, Clone)]
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

impl ItemsGame {
    pub fn get_all_rarities_sorted(&self) -> Vec<(u32, String)> {
        let mut rarities: Vec<(u32, String)> = self
            .rarities
            .values()
            .map(|r| (r.value, r.loc_key.clone()))
            .collect();
        rarities.sort_by_key(|(value, _)| *value);
        rarities
    }

    pub fn get_all_qualities_sorted(&self) -> Vec<(u32, String)> {
        let mut qualities: Vec<(u32, String)> = self
            .qualities
            .values()
            .map(|q| (q.value, q.name.clone()))
            .collect();
        qualities.sort_by_key(|(value, _)| *value);
        qualities
    }

    pub fn get_item_display_name(&self, def_index: u32, translations: &GameTranslation) -> String {
        if let Some(item) = self.items.get(&def_index) {
            item.get_display_name(translations)
        } else {
            format!("??? {}", def_index)
        }
    }

    pub fn get_paint_kit_display_name(
        &self,
        paint_index: u32,
        translations: &GameTranslation,
    ) -> Option<String> {
        self.paint_kits
            .get(&paint_index)
            .map(|pk| pk.get_display_name(translations))
    }

    pub fn get_paint_kit_rarity(&self, paint_index: u32) -> Option<u32> {
        self.paint_kits.get(&paint_index).and_then(|pk| {
            self.paint_kits_rarity
                .get(&pk.name)
                .and_then(|rarity_name| self.rarities.get(rarity_name).map(|r| r.value))
        })
    }

    // Get rarity value by rarity id (e.g., "rarity_uncommon_weapon")
    pub fn get_rarity_value_by_id(&self, rarity_id: &str) -> Option<u32> {
        // rarity_id format: "rarity_uncommon_weapon" -> need to find "uncommon" rarity
        // Also handles "rarity_contraband" (no suffix) and "rarity_ancient_character"
        let rarity_name = rarity_id
            .strip_prefix("rarity_")
            .map(|s| {
                s.strip_suffix("_weapon")
                    .or_else(|| s.strip_suffix("_character"))
                    .unwrap_or(s)
            })
            .unwrap_or(rarity_id);
        self.rarities.get(rarity_name).map(|r| r.value)
    }

    pub fn get_sticker_kit_display_name(
        &self,
        sticker_index: u32,
        translations: &GameTranslation,
    ) -> Option<String> {
        self.sticker_kits
            .get(&sticker_index)
            .map(|sk| sk.get_display_name(translations))
    }

    pub fn get_music_def_display_name(
        &self,
        music_index: u32,
        translations: &GameTranslation,
    ) -> Option<String> {
        self.music_defs
            .get(&music_index)
            .map(|md| md.get_display_name(translations))
    }

    pub fn get_item_full_name(
        &self,
        item: &crate::inventory::models::Item,
        translations: &GameTranslation,
    ) -> String {
        let item_name = self.get_item_display_name(item.def_index, translations);

        if let Some(paint_index) = item.attributes.get(&ItemAttribute::SkinPaintIndex.id())
            && let Ok(paint_id_f32) = paint_index.parse::<f32>()
        {
            let paint_id = paint_id_f32 as u32;
            if let Some(paint_name) = self.get_paint_kit_display_name(paint_id, translations) {
                return format!("{} | {}", item_name, paint_name);
            }
        }

        if let Some(music_index) = item.attributes.get(&ItemAttribute::MusicID.id())
            && let Ok(music_id) = music_index.parse::<u32>()
            && let Some(music_name) = self.get_music_def_display_name(music_id, translations)
        {
            return format!("{} | {}", item_name, music_name);
        }

        if let Some(sticker_index) = item.attributes.get(&ItemAttribute::Sticker0ID.id())
            && let Ok(sticker_id) = sticker_index.parse::<u32>()
            && let Some(sticker_name) = self.get_sticker_kit_display_name(sticker_id, translations)
        {
            return format!("{} | {}", item_name, sticker_name);
        }

        item_name
    }

    pub fn create_item_select_list(
        &self,
        translations: &GameTranslation,
    ) -> Vec<(String, String, String)> {
        build_select_list(&self.items, translations)
    }

    pub fn create_weapon_case_select_list(
        &self,
        translations: &GameTranslation,
    ) -> Vec<(String, String, String)> {
        let cases: HashMap<u32, &IGItem> = self
            .items
            .iter()
            .filter(|(def_index, item)| **def_index != 0 && item.is_weapon_case())
            .map(|(k, v)| (*k, v))
            .collect();
        build_select_list(&cases, translations)
    }

    pub fn get_associated_item_def_indexes(&self, def_index: u32) -> &[u32] {
        self.items
            .get(&def_index)
            .map(|item| item.associated_items.as_slice())
            .unwrap_or(&[])
    }

    pub fn create_paint_kit_select_list(
        &self,
        translations: &GameTranslation,
    ) -> Vec<(String, String, String)> {
        build_select_list(&self.paint_kits, translations)
    }

    pub fn create_music_def_select_list(
        &self,
        translations: &GameTranslation,
    ) -> Vec<(String, String, String)> {
        build_select_list(&self.music_defs, translations)
    }

    pub fn create_sticker_kit_select_list(
        &self,
        translations: &GameTranslation,
    ) -> Vec<(String, String, String)> {
        build_select_list(&self.sticker_kits, translations)
    }

    pub fn create_graffiti_tint_select_list(
        &self,
    ) -> Vec<(String, String, String, Option<String>)> {
        let mut items: Vec<(String, String, String, Option<String>)> = self
            .graffiti_tints
            .values()
            .map(|tint| {
                (
                    tint.id.to_string(),
                    tint.name.clone(),
                    tint.id.to_string(),
                    Some(tint.hex_color.clone()),
                )
            })
            .collect();
        items.sort_by_key(|(key, _, _, _)| key.parse::<u32>().unwrap_or(0));
        items
    }
}

trait SelectDisplayName {
    fn select_display_name(&self, translations: &GameTranslation) -> String;
}

impl SelectDisplayName for IGItem {
    fn select_display_name(&self, translations: &GameTranslation) -> String {
        self.get_display_name(translations)
    }
}

impl SelectDisplayName for IGPaintKit {
    fn select_display_name(&self, translations: &GameTranslation) -> String {
        self.get_display_name(translations)
    }
}

impl SelectDisplayName for IGStickerKit {
    fn select_display_name(&self, translations: &GameTranslation) -> String {
        self.get_display_name(translations)
    }
}

impl SelectDisplayName for IGMusicDef {
    fn select_display_name(&self, translations: &GameTranslation) -> String {
        self.get_display_name(translations)
    }
}

impl<T: SelectDisplayName + ?Sized> SelectDisplayName for &T {
    fn select_display_name(&self, translations: &GameTranslation) -> String {
        (**self).select_display_name(translations)
    }
}

fn build_select_list<V: SelectDisplayName>(
    map: &HashMap<u32, V>,
    translations: &GameTranslation,
) -> Vec<(String, String, String)> {
    let mut items: Vec<(String, String, String)> = map
        .iter()
        .map(|(key, v)| {
            let display_name = v.select_display_name(translations);
            (key.to_string(), display_name, key.to_string())
        })
        .collect();
    items.sort_by_key(|(key, _, _)| key.parse::<u32>().unwrap_or(0));
    items
}
