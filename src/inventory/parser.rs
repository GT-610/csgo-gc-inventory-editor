use crate::inventory::models::{DefaultEquip, Inventory, Item};
use crate::inventory::vdf::{VdfParser, VdfValue, get_string_from_obj};
use std::collections::HashMap;

pub trait InventoryParser: Send + Sync {
    fn parse(&self, content: &str) -> Result<Inventory, Box<dyn std::error::Error + Send + Sync>>;
    fn serialize(
        &self,
        inventory: &Inventory,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct VdfInventoryParser;

impl InventoryParser for VdfInventoryParser {
    fn parse(&self, content: &str) -> Result<Inventory, Box<dyn std::error::Error + Send + Sync>> {
        let vdf = VdfParser::parse(content)?;
        let items_obj = match vdf.get("items") {
            Some(VdfValue::Object(items)) => Some(items),
            Some(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "'items' section must be an object",
                )));
            }
            None => None,
        };

        let default_equips_obj = vdf.get("default_equips").and_then(|v| v.as_object());

        let mut items = Vec::new();

        if let Some(items_obj) = items_obj {
            for (key, item_value) in items_obj {
                if let Some(item_obj) = item_value.as_object() {
                    let id: u64 = key.parse().map_err(|_| {
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid item key",
                        ))
                    })?;
                    items.push(parse_item(id, item_obj)?);
                }
            }
        }

        let mut default_equips = HashMap::new();

        if let Some(equips_obj) = default_equips_obj {
            for (key, equip_value) in equips_obj {
                if let Some(equip_obj) = equip_value.as_object() {
                    let class_id: u32 = key.parse().map_err(|_| {
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid class_id",
                        ))
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

    fn serialize(
        &self,
        inventory: &Inventory,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut items_obj = std::collections::BTreeMap::new();

        for item in inventory.items.iter() {
            let key = item.id.to_string();
            items_obj.insert(key, VdfValue::Object(serialize_item(item)));
        }

        let mut default_equips_obj = std::collections::BTreeMap::new();

        for (class_id, equip) in &inventory.default_equips {
            let mut equip_obj = std::collections::BTreeMap::new();
            equip_obj.insert(
                "class_id".to_string(),
                VdfValue::String(equip.class_id.to_string()),
            );
            equip_obj.insert(
                "slot_id".to_string(),
                VdfValue::String(equip.slot_id.to_string()),
            );
            default_equips_obj.insert(
                class_id.to_string(),
                VdfValue::Object(equip_obj.into_iter().collect()),
            );
        }

        let mut vdf = std::collections::BTreeMap::new();
        vdf.insert(
            "items".to_string(),
            VdfValue::Object(items_obj.into_iter().collect()),
        );
        if !inventory.default_equips.is_empty() {
            vdf.insert(
                "default_equips".to_string(),
                VdfValue::Object(default_equips_obj.into_iter().collect()),
            );
        }

        let mut result = VdfParser::to_string(&VdfValue::Object(vdf.into_iter().collect()));
        result = result.replace("\r\n", "\n");

        Ok(result)
    }
}

fn parse_item(
    id: u64,
    obj: &HashMap<String, VdfValue>,
) -> Result<Item, Box<dyn std::error::Error + Send + Sync>> {
    let mut item = Item {
        id,
        inventory: get_u64(obj, "inventory")?,
        def_index: get_u32(obj, "def_index")?,
        level: get_u32(obj, "level")?,
        quality: get_u32(obj, "quality")?,
        flags: get_u32(obj, "flags")?,
        origin: get_u32(obj, "origin")?,
        in_use: get_u32(obj, "in_use")?,
        rarity: get_u32(obj, "rarity")?,
        custom_name: get_string_from_obj(obj, "custom_name"),
        attributes: HashMap::new(),
        equipped_state: HashMap::new(),
    };

    if let Some(attrs_obj) = obj.get("attributes").and_then(|v| v.as_object()) {
        for (key, value) in attrs_obj {
            let id: u32 = key.parse().map_err(|_| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid attribute key",
                ))
            })?;
            if let Some(s) = value.as_string() {
                item.attributes.insert(id, s.to_string());
            }
        }
    }

    if let Some(equips_obj) = obj.get("equipped_state").and_then(|v| v.as_object()) {
        for (key, value) in equips_obj {
            let id: u32 = key.parse().map_err(|_| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid equipped_state key",
                ))
            })?;
            if let Some(s) = value.as_string() {
                item.equipped_state.insert(id, s.to_string());
            }
        }
    }

    Ok(item)
}

fn serialize_item(item: &Item) -> HashMap<String, VdfValue> {
    let mut obj = std::collections::BTreeMap::new();

    obj.insert(
        "inventory".to_string(),
        VdfValue::String(item.inventory.to_string()),
    );
    obj.insert(
        "def_index".to_string(),
        VdfValue::String(item.def_index.to_string()),
    );
    obj.insert(
        "level".to_string(),
        VdfValue::String(item.level.to_string()),
    );
    obj.insert(
        "quality".to_string(),
        VdfValue::String(item.quality.to_string()),
    );
    obj.insert(
        "flags".to_string(),
        VdfValue::String(item.flags.to_string()),
    );
    obj.insert(
        "origin".to_string(),
        VdfValue::String(item.origin.to_string()),
    );

    if let Some(name) = &item.custom_name {
        obj.insert("custom_name".to_string(), VdfValue::String(name.clone()));
    }

    obj.insert(
        "in_use".to_string(),
        VdfValue::String(item.in_use.to_string()),
    );
    obj.insert(
        "rarity".to_string(),
        VdfValue::String(item.rarity.to_string()),
    );

    if !item.attributes.is_empty() {
        let mut attrs = std::collections::BTreeMap::new();
        for (key, value) in &item.attributes {
            attrs.insert(key.to_string(), VdfValue::String(value.clone()));
        }
        obj.insert(
            "attributes".to_string(),
            VdfValue::Object(attrs.into_iter().collect()),
        );
    }

    if !item.equipped_state.is_empty() {
        let mut equips = std::collections::BTreeMap::new();
        for (key, value) in &item.equipped_state {
            equips.insert(key.to_string(), VdfValue::String(value.clone()));
        }
        obj.insert(
            "equipped_state".to_string(),
            VdfValue::Object(equips.into_iter().collect()),
        );
    }

    obj.into_iter().collect()
}

fn get_u64(
    obj: &HashMap<String, VdfValue>,
    key: &str,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let value: &str = obj.get(key).and_then(|v| v.as_string()).ok_or_else(
        || -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Missing '{}'", key),
            ))
        },
    )?;
    value
        .parse()
        .map_err(|_| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid value for '{}'", key),
            ))
        })
}

fn get_u32(
    obj: &HashMap<String, VdfValue>,
    key: &str,
) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
    let value: &str = obj.get(key).and_then(|v| v.as_string()).ok_or_else(
        || -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Missing '{}'", key),
            ))
        },
    )?;
    value
        .parse()
        .map_err(|_| -> Box<dyn std::error::Error + Send + Sync> {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid value for '{}'", key),
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::{InventoryParser, VdfInventoryParser};

    #[test]
    fn parses_empty_file_as_empty_inventory() {
        let inventory = VdfInventoryParser.parse("").unwrap();

        assert!(inventory.items.is_empty());
        assert!(inventory.default_equips.is_empty());
    }

    #[test]
    fn parses_default_equips_without_items_section() {
        let content = r#"
            "default_equips"
            {
                "61"
                {
                    "class_id" "3"
                    "slot_id" "2"
                }
            }
        "#;

        let inventory = VdfInventoryParser.parse(content).unwrap();

        assert!(inventory.items.is_empty());
        assert_eq!(inventory.default_equips.len(), 1);
        let equip = inventory.default_equips.get(&61).unwrap();
        assert_eq!(equip.class_id, 3);
        assert_eq!(equip.slot_id, 2);
    }

    #[test]
    fn parses_inventory_with_utf8_bom() {
        let content = "\u{feff}\"items\"\n{\n}\n";

        let inventory = VdfInventoryParser.parse(content).unwrap();

        assert!(inventory.items.is_empty());
        assert!(inventory.default_equips.is_empty());
    }

    #[test]
    fn parses_existing_items_section() {
        let content = r#"
            "items"
            {
                "42"
                {
                    "inventory" "7"
                    "def_index" "507"
                    "level" "1"
                    "quality" "3"
                    "flags" "0"
                    "origin" "24"
                    "in_use" "0"
                    "rarity" "6"
                }
            }
        "#;

        let inventory = VdfInventoryParser.parse(content).unwrap();

        assert_eq!(inventory.items.len(), 1);
        assert_eq!(inventory.items[0].id, 42);
        assert_eq!(inventory.items[0].inventory, 7);
    }

    #[test]
    fn rejects_non_object_items_section() {
        let error = VdfInventoryParser
            .parse(r#""items" "invalid""#)
            .unwrap_err();

        assert_eq!(error.to_string(), "'items' section must be an object");
    }
}
