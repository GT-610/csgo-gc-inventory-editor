use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Inventory API structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryRarity {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySkinItem {
    pub name: String,
    pub rarity: Option<InventoryRarity>,
    pub marketable: bool,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryData {
    pub skins: HashMap<String, HashMap<String, InventorySkinItem>>,
    #[serde(default)]
    pub crates: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub collectibles: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub stickers: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub graffiti: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub music_kits: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub keychains: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub highlights: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub agents: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub patches: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub keys: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub sticker_slabs: HashMap<String, InventorySkinItem>,
    #[serde(default)]
    pub tools: HashMap<String, InventorySkinItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnlineGameData {
    #[serde(default)]
    pub inventory: Option<InventoryData>,
}

impl OnlineGameData {
    // Get skin info from inventory data by weapon_id and paint_index
    pub fn get_inventory_skin(
        &self,
        weapon_id: u32,
        paint_index: u32,
    ) -> Option<&InventorySkinItem> {
        let skins = self.inventory.as_ref()?.skins.get(&weapon_id.to_string())?;

        // paint_index 0 corresponds to "null" key in online data
        let key = if paint_index == 0 {
            "null"
        } else {
            &paint_index.to_string()
        };
        skins.get(key)
    }

    // Get music kit info from inventory data by music_index
    pub fn get_inventory_music_kit(&self, music_index: u32) -> Option<&InventorySkinItem> {
        self.inventory
            .as_ref()?
            .music_kits
            .get(&music_index.to_string())
    }

    pub fn get_inventory_sticker(&self, sticker_index: u32) -> Option<&InventorySkinItem> {
        self.inventory
            .as_ref()?
            .stickers
            .get(&sticker_index.to_string())
    }

    pub fn get_inventory_graffiti(&self, graffiti_index: u32) -> Option<&InventorySkinItem> {
        self.inventory
            .as_ref()?
            .graffiti
            .get(&graffiti_index.to_string())
    }

    pub fn get_inventory_keychain(&self, keychain_index: u32) -> Option<&InventorySkinItem> {
        self.inventory
            .as_ref()?
            .keychains
            .get(&keychain_index.to_string())
    }
}
