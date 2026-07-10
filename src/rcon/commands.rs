use crate::inventory::{Item, ItemAttribute};

pub fn build_give_item_command(item: &Item, count: u32) -> Result<String, String> {
    if !(1..=100).contains(&count) {
        return Err("count must be between 1 and 100".to_string());
    }

    let mut parts = vec!["give_item".to_string(), item.def_index.to_string()];
    if count != 1 {
        parts.push(count.to_string());
    }

    parts.push(format!("level={}", item.level));
    parts.push(format!("quality={}", item.quality));
    parts.push(format!("rarity={}", item.rarity));

    if let Some(custom_name) = &item.custom_name
        && !custom_name.is_empty()
    {
        parts.push(format!("name={}", quote_value(custom_name)));
    }

    push_u32_attr(
        &mut parts,
        item,
        ItemAttribute::SkinPaintIndex.id(),
        "paint",
    )?;
    push_u32_attr(&mut parts, item, ItemAttribute::SkinPaintSeed.id(), "seed")?;
    push_float_attr(&mut parts, item, ItemAttribute::SkinPaintWear.id(), "wear")?;
    push_u32_attr(
        &mut parts,
        item,
        ItemAttribute::StatTrakCount.id(),
        "stattrak",
    )?;
    push_u32_attr(&mut parts, item, ItemAttribute::MusicID.id(), "music")?;
    push_u32_attr(
        &mut parts,
        item,
        ItemAttribute::SprayColor.id(),
        "spray_color",
    )?;
    push_u32_attr(
        &mut parts,
        item,
        ItemAttribute::SprayRemain.id(),
        "spray_remaining",
    )?;

    for slot in 0..=5 {
        let base = 113 + slot * 4;
        push_u32_attr(&mut parts, item, base, &format!("sticker{}", slot))?;
        push_float_attr(&mut parts, item, base + 1, &format!("sticker{}_wear", slot))?;
        push_float_attr(
            &mut parts,
            item,
            base + 2,
            &format!("sticker{}_scale", slot),
        )?;
        push_float_attr(
            &mut parts,
            item,
            base + 3,
            &format!("sticker{}_rotation", slot),
        )?;
    }

    Ok(parts.join(" "))
}

pub fn build_remove_item_command(item_id: u64) -> String {
    format!("remove_item {}", item_id)
}

pub fn quote_value(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

fn push_u32_attr(
    parts: &mut Vec<String>,
    item: &Item,
    attr_id: u32,
    key: &str,
) -> Result<(), String> {
    if let Some(value) = item.attributes.get(&attr_id) {
        let parsed = parse_u32_attr(value, key)?;
        parts.push(format!("{}={}", key, parsed));
    }
    Ok(())
}

fn push_float_attr(
    parts: &mut Vec<String>,
    item: &Item,
    attr_id: u32,
    key: &str,
) -> Result<(), String> {
    if let Some(value) = item.attributes.get(&attr_id) {
        let parsed = value
            .parse::<f32>()
            .map_err(|_| format!("invalid parameter {}", key))?;
        parts.push(format!("{}={}", key, parsed));
    }
    Ok(())
}

fn parse_u32_attr(value: &str, key: &str) -> Result<u32, String> {
    if let Ok(parsed) = value.parse::<u32>() {
        return Ok(parsed);
    }

    let parsed_float = value
        .parse::<f32>()
        .map_err(|_| format!("invalid parameter {}", key))?;
    if parsed_float < 0.0 || parsed_float.fract() != 0.0 || parsed_float > u32::MAX as f32 {
        return Err(format!("invalid parameter {}", key));
    }
    Ok(parsed_float as u32)
}
