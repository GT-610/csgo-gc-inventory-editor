use crate::inventory::{Inventory, ItemsGame, GameTranslation, InventoryLoader, ItemsGameLoader, LanguageFileParser};
use crate::core::GameDir;
use eframe::egui;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Rarity {
    Default = 0,
    Consumer = 1,
    Industrial = 2,
    MilSpec = 3,
    Restricted = 4,
    Classified = 5,
    Covert = 6,
    Contraband = 7,
}

impl Rarity {
    pub fn from_u32(value: u32) -> Self {
        match value {
            1 => Rarity::Consumer,
            2 => Rarity::Industrial,
            3 => Rarity::MilSpec,
            4 => Rarity::Restricted,
            5 => Rarity::Classified,
            6 => Rarity::Covert,
            7 => Rarity::Contraband,
            _ => Rarity::Default,
        }
    }

    pub fn color(&self) -> Option<egui::Color32> {
        match self {
            Rarity::Default => None,
            Rarity::Consumer => Some(egui::Color32::from_rgb(176, 176, 176)),
            Rarity::Industrial => Some(egui::Color32::from_rgb(94, 152, 217)),
            Rarity::MilSpec => Some(egui::Color32::from_rgb(75, 105, 255)),
            Rarity::Restricted => Some(egui::Color32::from_rgb(136, 71, 255)),
            Rarity::Classified => Some(egui::Color32::from_rgb(211, 44, 230)),
            Rarity::Covert => Some(egui::Color32::from_rgb(235, 75, 75)),
            Rarity::Contraband => Some(egui::Color32::from_rgb(255, 215, 0)),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum InventoryCategory {
    #[default]
    All,
    Equipped,
    StickerAndGraffiti,
    CasesAndMore,
    Collectibles,
}

#[derive(Clone)]
pub struct EditItemState {
    pub level: u32,
    pub custom_name: String,
}

pub struct CsgoInventoryEditor {
    pub inventory: Inventory,
    pub items_game: ItemsGame,
    pub translations: GameTranslation,
    pub selected_category: InventoryCategory,
    pub selected_subcategory: Option<String>,
    pub search_query: String,
    pub open_item_windows: HashSet<u64>,
    pub edit_item_states: HashMap<u64, EditItemState>,
    pub select_window_open: bool,
    pub select_window_items: Vec<(String, String, String)>,
    pub select_window_search: String,
    pub select_window_selected: Option<usize>,
    pub select_window_title: String,
    pub select_window_key_header: String,
    pub select_window_value_header: String,
    pub select_window_seed: u64,
}

impl CsgoInventoryEditor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        
        let font_data = fs::read("assets/fonts/JetBrainsMapleMono-Regular.ttf")
            .expect("Failed to read font file");
        
        fonts.font_data.insert(
            "JetBrainsMapleMono".to_owned(),
            Arc::new(egui::FontData::from_owned(font_data)),
        );
        
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "JetBrainsMapleMono".to_owned());
        
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
            .push("JetBrainsMapleMono".to_owned());
        
        cc.egui_ctx.set_fonts(fonts);
        
        let inventory = match GameDir::new() {
            Ok(game_dir) => {
                match InventoryLoader::load_from_game_dir(&game_dir.path()) {
                    Ok(inv) => inv,
                    Err(e) => {
                        eprintln!("Failed to load inventory: {}", e);
                        Inventory::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to detect game directory: {}", e);
                Inventory::default()
            }
        };

        let mut items_game = ItemsGame::default();
        let mut translations = GameTranslation::default();

        if let Ok(game_dir) = GameDir::new() {
            let items_game_path = game_dir.path().join("csgo").join("scripts").join("items").join("items_game.txt");
            if items_game_path.exists() {
                match ItemsGameLoader::load(&items_game_path) {
                    Ok(ig) => items_game = ig,
                    Err(e) => eprintln!("Failed to load items_game.txt: {}", e),
                }
            }

            let possible_lang_files = [
                game_dir.path().join("csgo").join("resource").join("csgo_english.txt"),
                game_dir.path().join("csgo").join("resource").join("csgo_schinese.txt"),
                game_dir.path().join("csgo").join("resource").join("csgo_tchinese.txt"),
            ];

            for lang_file in &possible_lang_files {
                if lang_file.exists() {
                    match LanguageFileParser::load(lang_file) {
                        Ok(t) => {
                            translations = t;
                            break;
                        }
                        Err(e) => eprintln!("Failed to load language file: {}", e),
                    }
                }
            }
        }
        
        Self {
            inventory,
            items_game,
            translations,
            selected_category: InventoryCategory::All,
            selected_subcategory: None,
            search_query: String::new(),
            open_item_windows: HashSet::new(),
            edit_item_states: HashMap::new(),
            select_window_open: false,
            select_window_items: Vec::new(),
            select_window_search: String::new(),
            select_window_selected: None,
            select_window_title: String::new(),
            select_window_key_header: String::new(),
            select_window_value_header: String::new(),
            select_window_seed: 0,
        }
    }
    
    pub fn get_item_display_name(&self, item: &crate::inventory::Item) -> String {
        self.items_game.get_item_full_name(item, &self.translations)
    }
    
    pub fn get_rarity_name(&self, rarity_id: u32) -> String {
        if let Some(rarity) = self.items_game.rarities.values().find(|r| r.value == rarity_id) {
            self.translations.get(&rarity.loc_key).cloned().unwrap_or_else(|| {
                rarity.loc_key_weapon.as_ref().and_then(|key| self.translations.get(key).cloned())
                    .unwrap_or_else(|| rarity.loc_key.clone())
            })
        } else {
            format!("Unknown ({})", rarity_id)
        }
    }
    
    pub fn open_select_window(&mut self, title: String, key_header: String, value_header: String, items: Vec<(String, String, String)>) {
        self.select_window_title = title;
        self.select_window_key_header = key_header;
        self.select_window_value_header = value_header;
        self.select_window_items = items;
        self.select_window_search = String::new();
        self.select_window_selected = None;
        self.select_window_open = true;
        self.select_window_seed += 1;
    }
}

impl Default for CsgoInventoryEditor {
    fn default() -> Self {
        Self {
            inventory: Inventory::default(),
            items_game: ItemsGame::default(),
            translations: GameTranslation::default(),
            selected_category: InventoryCategory::All,
            selected_subcategory: None,
            search_query: String::new(),
            open_item_windows: HashSet::new(),
            edit_item_states: HashMap::new(),
            select_window_open: false,
            select_window_items: Vec::new(),
            select_window_search: String::new(),
            select_window_selected: None,
            select_window_title: String::new(),
            select_window_key_header: String::new(),
            select_window_value_header: String::new(),
            select_window_seed: 0,
        }
    }
}
