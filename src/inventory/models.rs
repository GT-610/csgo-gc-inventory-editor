use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub inventory: u64,
    pub def_index: u32,
    pub level: u32,
    pub quality: u32,
    pub flags: u32,
    pub origin: u32,
    pub in_use: u32,
    pub rarity: u32,
    #[serde(default)]
    pub custom_name: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<u32, String>,
    #[serde(default)]
    pub equipped_state: HashMap<u32, String>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            inventory: 0,
            def_index: 0,
            level: 0,
            quality: 0,
            flags: 0,
            origin: 0,
            in_use: 0,
            rarity: 0,
            custom_name: None,
            attributes: HashMap::new(),
            equipped_state: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefaultEquip {
    pub class_id: u32,
    pub slot_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(default)]
    pub items: Vec<Item>,
    #[serde(default)]
    pub default_equips: HashMap<u32, DefaultEquip>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            default_equips: HashMap::new(),
        }
    }
}
