#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemAttribute {
    SkinPaintIndex = 6,
    SkinPaintSeed = 7,
    SkinPaintWear = 8,
    StatTrakCount = 80,
    StatTrakType = 81,
    Sticker0ID = 113,
    Sticker0Wear = 114,
    Sticker0Scale = 115,
    Sticker0Rotation = 116,
    Sticker1ID = 117,
    Sticker1Wear = 118,
    Sticker1Scale = 119,
    Sticker1Rotation = 120,
    Sticker2ID = 121,
    Sticker2Wear = 122,
    Sticker2Scale = 123,
    Sticker2Rotation = 124,
    Sticker3ID = 125,
    Sticker3Wear = 126,
    Sticker3Scale = 127,
    Sticker3Rotation = 128,
    Sticker4ID = 129,
    Sticker4Wear = 130,
    Sticker4Scale = 131,
    Sticker4Rotation = 132,
    Sticker5ID = 133,
    Sticker5Wear = 134,
    Sticker5Scale = 135,
    Sticker5Rotation = 136,
    MusicID = 166,
    SprayRemain = 232,
    SprayColor = 233,
}

impl ItemAttribute {
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            6 => Some(Self::SkinPaintIndex),
            7 => Some(Self::SkinPaintSeed),
            8 => Some(Self::SkinPaintWear),
            80 => Some(Self::StatTrakCount),
            81 => Some(Self::StatTrakType),
            113 => Some(Self::Sticker0ID),
            114 => Some(Self::Sticker0Wear),
            115 => Some(Self::Sticker0Scale),
            116 => Some(Self::Sticker0Rotation),
            117 => Some(Self::Sticker1ID),
            118 => Some(Self::Sticker1Wear),
            119 => Some(Self::Sticker1Scale),
            120 => Some(Self::Sticker1Rotation),
            121 => Some(Self::Sticker2ID),
            122 => Some(Self::Sticker2Wear),
            123 => Some(Self::Sticker2Scale),
            124 => Some(Self::Sticker2Rotation),
            125 => Some(Self::Sticker3ID),
            126 => Some(Self::Sticker3Wear),
            127 => Some(Self::Sticker3Scale),
            128 => Some(Self::Sticker3Rotation),
            129 => Some(Self::Sticker4ID),
            130 => Some(Self::Sticker4Wear),
            131 => Some(Self::Sticker4Scale),
            132 => Some(Self::Sticker4Rotation),
            133 => Some(Self::Sticker5ID),
            134 => Some(Self::Sticker5Wear),
            135 => Some(Self::Sticker5Scale),
            136 => Some(Self::Sticker5Rotation),
            166 => Some(Self::MusicID),
            232 => Some(Self::SprayRemain),
            233 => Some(Self::SprayColor),
            _ => None,
        }
    }

    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::SkinPaintIndex => "SkinPaintIndex",
            Self::SkinPaintSeed => "SkinPaintSeed",
            Self::SkinPaintWear => "SkinPaintWear",
            Self::StatTrakCount => "StatTrakCount",
            Self::StatTrakType => "StatTrakType",
            Self::Sticker0ID => "Sticker0ID",
            Self::Sticker0Wear => "Sticker0Wear",
            Self::Sticker0Scale => "Sticker0Scale",
            Self::Sticker0Rotation => "Sticker0Rotation",
            Self::Sticker1ID => "Sticker1ID",
            Self::Sticker1Wear => "Sticker1Wear",
            Self::Sticker1Scale => "Sticker1Scale",
            Self::Sticker1Rotation => "Sticker1Rotation",
            Self::Sticker2ID => "Sticker2ID",
            Self::Sticker2Wear => "Sticker2Wear",
            Self::Sticker2Scale => "Sticker2Scale",
            Self::Sticker2Rotation => "Sticker2Rotation",
            Self::Sticker3ID => "Sticker3ID",
            Self::Sticker3Wear => "Sticker3Wear",
            Self::Sticker3Scale => "Sticker3Scale",
            Self::Sticker3Rotation => "Sticker3Rotation",
            Self::Sticker4ID => "Sticker4ID",
            Self::Sticker4Wear => "Sticker4Wear",
            Self::Sticker4Scale => "Sticker4Scale",
            Self::Sticker4Rotation => "Sticker4Rotation",
            Self::Sticker5ID => "Sticker5ID",
            Self::Sticker5Wear => "Sticker5Wear",
            Self::Sticker5Scale => "Sticker5Scale",
            Self::Sticker5Rotation => "Sticker5Rotation",
            Self::MusicID => "MusicID",
            Self::SprayRemain => "SprayRemain",
            Self::SprayColor => "SprayColor",
        }
    }
}

pub static AVAILABLE_ATTRIBUTES: [u32; 26] = [
    6, 7, 8, 80, 81,
    113, 114, 115, 116,
    117, 118, 119, 120,
    121, 122, 123, 124,
    125, 126, 127, 128,
    129, 130, 131, 132,
    166,
];

pub fn get_attribute_fluent_key(attr_id: u32) -> String {
    format!("attr-{}", attr_id)
}

pub fn get_attribute_display_name(attr_id: u32, translations: &crate::inventory::GameTranslation) -> String {
    let key = format!("attr-{}", attr_id);
    if let Some(translated) = translations.get(&key) {
        return translated.clone();
    }
    
    if let Some(attr) = ItemAttribute::from_id(attr_id) {
        attr.name().to_string()
    } else {
        format!("Unknown ({})", attr_id)
    }
}

pub fn get_attribute_value_display_name(
    attr_id: u32,
    value: &str,
    items_game: &crate::inventory::ItemsGame,
    translations: &crate::inventory::GameTranslation,
) -> String {
    match attr_id {
        6 => {
            if let Ok(paint_id_f32) = value.parse::<f32>() {
                let paint_id = paint_id_f32 as u32;
                if let Some(paint_name) = items_game.get_paint_kit_display_name(paint_id, translations) {
                    return format!("{} ({})", paint_name, paint_id);
                }
            }
            value.to_string()
        }
        113 | 117 | 121 | 125 | 129 | 133 => {
            if let Ok(sticker_id) = value.parse::<u32>() {
                if let Some(sticker_name) = items_game.get_sticker_kit_display_name(sticker_id, translations) {
                    return sticker_name;
                }
            }
            value.to_string()
        }
        166 => {
            if let Ok(music_id) = value.parse::<u32>() {
                if let Some(music_name) = items_game.get_music_def_display_name(music_id, translations) {
                    return music_name;
                }
            }
            value.to_string()
        }
        233 => {
            if let Ok(tint_id) = value.parse::<u32>() {
                for (_, tint) in &items_game.graffiti_tints {
                    if tint.id == tint_id {
                        return tint.name.clone();
                    }
                }
            }
            value.to_string()
        }
        _ => value.to_string(),
    }
}
