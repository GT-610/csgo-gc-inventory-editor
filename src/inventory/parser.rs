use crate::inventory::models::{DefaultEquip, Inventory, Item};
use crate::inventory::vdf::{VdfParser, VdfValue};
use std::collections::HashMap;

pub trait InventoryParser: Send + Sync {
    fn parse(&self, content: &str) -> Result<Inventory, Box<dyn std::error::Error + Send + Sync>>;
    fn serialize(&self, inventory: &Inventory) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct VdfInventoryParser;

impl InventoryParser for VdfInventoryParser {
    fn parse(&self, content: &str) -> Result<Inventory, Box<dyn std::error::Error + Send + Sync>> {
        let vdf = VdfParser::parse(content)?;
        let items_obj = vdf
            .get("items")
            .and_then(|v| v.as_object())
            .ok_or_else(|| -> Box<dyn std::error::Error + Send + Sync> {
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing 'items' section"))
            })?;

        let default_equips_obj = vdf
            .get("default_equips")
            .and_then(|v| v.as_object());

        let mut items = Vec::new();

        for (_, item_value) in items_obj {
            if let Some(item_obj) = item_value.as_object() {
                items.push(parse_item(item_obj)?);
            }
        }

        let mut default_equips = HashMap::new();

        if let Some(equips_obj) = default_equips_obj {
            for (key, equip_value) in equips_obj {
                if let Some(equip_obj) = equip_value.as_object() {
                    let class_id: u32 = key.parse().map_err(|_| {
                        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid class_id"))
                    })?;
                    default_equips.insert(
                        class_id,
                        DefaultEquip {
                            class_id: get_u32(equip_obj, "class_id")?,
                            slot_id: get_u32(equip_obj, "slot_id")?,
                        },
                    );
                }
            }
        }

        Ok(Inventory {
            items,
            default_equips,
        })
    }

    fn serialize(&self, inventory: &Inventory) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut items_obj = HashMap::new();

        for (idx, item) in inventory.items.iter().enumerate() {
            let key = (idx + 2).to_string();
            items_obj.insert(key, VdfValue::Object(serialize_item(item)));
        }

        let mut default_equips_obj = HashMap::new();

        for (class_id, equip) in &inventory.default_equips {
            let mut equip_obj = HashMap::new();
            equip_obj.insert("class_id".to_string(), VdfValue::String(equip.class_id.to_string()));
            equip_obj.insert("slot_id".to_string(), VdfValue::String(equip.slot_id.to_string()));
            default_equips_obj.insert(class_id.to_string(), VdfValue::Object(equip_obj));
        }

        let mut vdf = HashMap::new();
        vdf.insert("items".to_string(), VdfValue::Object(items_obj));
        if !inventory.default_equips.is_empty() {
            vdf.insert("default_equips".to_string(), VdfValue::Object(default_equips_obj));
        }

        let mut result = VdfParser::to_string(&VdfValue::Object(vdf));
        result = result.replace("\r\n", "\n");
        result = result.replace("\" ", "\"\t\t");

        Ok(result)
    }
}

fn parse_item(obj: &HashMap<String, VdfValue>) -> Result<Item, Box<dyn std::error::Error + Send + Sync>> {
    let mut item = Item::default();

    item.inventory = get_u64(obj, "inventory")?;
    item.def_index = get_u32(obj, "def_index")?;
    item.level = get_u32(obj, "level")?;
    item.quality = get_u32(obj, "quality")?;
    item.flags = get_u32(obj, "flags")?;
    item.origin = get_u32(obj, "origin")?;
    item.in_use = get_u32(obj, "in_use")?;
    item.rarity = get_u32(obj, "rarity")?;

    if let Some(name) = get_string_opt(obj, "custom_name") {
        item.custom_name = Some(name);
    }

    if let Some(attrs_obj) = obj.get("attributes").and_then(|v| v.as_object()) {
        for (key, value) in attrs_obj {
            let id: u32 = key.parse().map_err(|_| {
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid attribute key"))
            })?;
            if let Some(s) = value.as_string() {
                item.attributes.insert(id, s.to_string());
            }
        }
    }

    if let Some(equips_obj) = obj.get("equipped_state").and_then(|v| v.as_object()) {
        for (key, value) in equips_obj {
            let id: u32 = key.parse().map_err(|_| {
                Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid equipped_state key"))
            })?;
            if let Some(s) = value.as_string() {
                item.equipped_state.insert(id, s.to_string());
            }
        }
    }

    Ok(item)
}

fn serialize_item(item: &Item) -> HashMap<String, VdfValue> {
    let mut obj = HashMap::new();

    obj.insert("inventory".to_string(), VdfValue::String(item.inventory.to_string()));
    obj.insert("def_index".to_string(), VdfValue::String(item.def_index.to_string()));
    obj.insert("level".to_string(), VdfValue::String(item.level.to_string()));
    obj.insert("quality".to_string(), VdfValue::String(item.quality.to_string()));
    obj.insert("flags".to_string(), VdfValue::String(item.flags.to_string()));
    obj.insert("origin".to_string(), VdfValue::String(item.origin.to_string()));
    obj.insert("in_use".to_string(), VdfValue::String(item.in_use.to_string()));
    obj.insert("rarity".to_string(), VdfValue::String(item.rarity.to_string()));

    if let Some(name) = &item.custom_name {
        obj.insert("custom_name".to_string(), VdfValue::String(name.clone()));
    }

    if !item.attributes.is_empty() {
        let mut attrs = HashMap::new();
        for (key, value) in &item.attributes {
            attrs.insert(key.to_string(), VdfValue::String(value.clone()));
        }
        obj.insert("attributes".to_string(), VdfValue::Object(attrs));
    }

    if !item.equipped_state.is_empty() {
        let mut equips = HashMap::new();
        for (key, value) in &item.equipped_state {
            equips.insert(key.to_string(), VdfValue::String(value.clone()));
        }
        obj.insert("equipped_state".to_string(), VdfValue::Object(equips));
    }

    obj
}

fn get_u64(obj: &HashMap<String, VdfValue>, key: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let value: &str = obj.get(key)
        .and_then(|v| v.as_string())
        .ok_or_else(|| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Missing '{}'", key)))
        })?;
    value.parse().map_err(|_| -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid value for '{}'", key)))
    })
}

fn get_u32(obj: &HashMap<String, VdfValue>, key: &str) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
    let value: &str = obj.get(key)
        .and_then(|v| v.as_string())
        .ok_or_else(|| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Missing '{}'", key)))
        })?;
    value.parse().map_err(|_| -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid value for '{}'", key)))
    })
}

fn get_string_opt(obj: &HashMap<String, VdfValue>, key: &str) -> Option<String> {
    obj.get(key).and_then(|v| v.as_string().map(|s| s.to_string()))
}
