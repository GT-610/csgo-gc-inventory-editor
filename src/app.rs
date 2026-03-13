use crate::config::{Config, ConfigLoader};
use crate::core::GameDir;
use crate::inventory::{
    GameTranslation, Inventory, InventoryLoader, ItemAttribute, ItemsGame, ItemsGameLoader,
    LanguageFileParser,
};
use crate::settings::{Settings, Theme};
use eframe::egui;
use egui_i18n::{load_translations_from_path, set_fallback, set_language};
use std::cell::RefCell;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemTemplate {
    Empty,
    NormalWeapon,
    StatTrakWeapon,
    NormalMusicKit,
    StatTrakMusicKit,
}

impl ItemTemplate {
    pub fn create_item(&self, def_index: u32) -> crate::inventory::Item {
        let mut attributes = HashMap::new();
        let mut quality = 4;

        match self {
            ItemTemplate::Empty => {}
            ItemTemplate::NormalWeapon => {
                attributes.insert(ItemAttribute::SkinPaintIndex.id(), "0".to_string());
                attributes.insert(ItemAttribute::SkinPaintSeed.id(), "0".to_string());
                attributes.insert(ItemAttribute::SkinPaintWear.id(), "0.001".to_string());
            }
            ItemTemplate::StatTrakWeapon => {
                quality = 9;
                attributes.insert(ItemAttribute::SkinPaintIndex.id(), "0".to_string());
                attributes.insert(ItemAttribute::SkinPaintSeed.id(), "0".to_string());
                attributes.insert(ItemAttribute::SkinPaintWear.id(), "0.001".to_string());
                attributes.insert(ItemAttribute::StatTrakCount.id(), "0".to_string());
                attributes.insert(ItemAttribute::StatTrakType.id(), "0".to_string());
            }
            ItemTemplate::NormalMusicKit => {
                attributes.insert(ItemAttribute::MusicID.id(), "0".to_string());
                attributes.insert(ItemAttribute::StatTrakCount.id(), "0".to_string());
                attributes.insert(ItemAttribute::StatTrakType.id(), "1".to_string());
            }
            ItemTemplate::StatTrakMusicKit => {
                quality = 9;
                attributes.insert(ItemAttribute::MusicID.id(), "0".to_string());
                attributes.insert(ItemAttribute::StatTrakCount.id(), "0".to_string());
                attributes.insert(ItemAttribute::StatTrakType.id(), "1".to_string());
            }
        }

        crate::inventory::Item {
            inventory: 0,
            def_index,
            level: 1,
            quality,
            flags: 0,
            origin: 0,
            in_use: 0,
            rarity: 0,
            custom_name: None,
            attributes,
            equipped_state: HashMap::new(),
        }
    }

    pub fn create_music_kit(&self, music_id: u32) -> crate::inventory::Item {
        let mut attributes = HashMap::new();
        let mut quality = 4;

        match self {
            ItemTemplate::NormalMusicKit => {
                attributes.insert(ItemAttribute::MusicID.id(), music_id.to_string());
                attributes.insert(ItemAttribute::StatTrakCount.id(), "0".to_string());
                attributes.insert(ItemAttribute::StatTrakType.id(), "1".to_string());
            }
            ItemTemplate::StatTrakMusicKit => {
                quality = 9;
                attributes.insert(ItemAttribute::MusicID.id(), music_id.to_string());
                attributes.insert(ItemAttribute::StatTrakCount.id(), "0".to_string());
                attributes.insert(ItemAttribute::StatTrakType.id(), "1".to_string());
            }
            _ => {}
        }

        crate::inventory::Item {
            inventory: 0,
            def_index: 1314,
            level: 1,
            quality,
            flags: 0,
            origin: 0,
            in_use: 0,
            rarity: 0,
            custom_name: None,
            attributes,
            equipped_state: HashMap::new(),
        }
    }

    pub fn is_music_kit(&self) -> bool {
        matches!(self, ItemTemplate::NormalMusicKit | ItemTemplate::StatTrakMusicKit)
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

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Page {
    #[default]
    Inventory,
    Settings,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SettingsPage {
    #[default]
    Config,
    Settings,
    About,
}

#[derive(Clone)]
pub struct EditItemState {
    pub level: u32,
    pub custom_name: String,
    pub rarity: u32,
    pub quality: u32,
    pub attributes: HashMap<u32, String>,
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
    pub select_window_for_item: Option<u64>,
    pub select_window_for_attr: Option<u32>,
    pub current_language: String,
    pub game_dir: Option<GameDir>,
    pub delete_confirm_item_id: Option<u64>,
    pub pending_add_item: bool,
    pub selected_template: Option<ItemTemplate>,
    pub show_template_modal: bool,
    pub pending_paint_kit_select: Option<u64>,
    pub pending_music_def_select: Option<u64>,
    pub pending_sticker_kit_select: Option<(u64, u32)>,
    pub settings: Settings,
    pub current_page: Page,
    pub current_settings_page: SettingsPage,
    pub config: Config,
    cached_sorted_inventory_ids: Vec<u64>,
    cached_items_count: usize,
    cached_item_display_names: RefCell<HashMap<u64, String>>,
    last_theme: Option<Theme>,
}

fn init_i18n(language: &str) {
    if let Err(e) = load_translations_from_path("csgo_gc/editor/languages") {
        eprintln!("Failed to load translations: {}", e);
    }
    set_language(language);
    set_fallback("en-US");
}

impl CsgoInventoryEditor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = Settings::load().unwrap_or_default();

        let mut fonts = egui::FontDefinitions::default();

        let font_data = fs::read("csgo_gc/editor/fonts/JetBrainsMapleMono-Regular.ttf")
            .expect("Failed to read font file");

        fonts.font_data.insert(
            "JetBrainsMapleMono".to_owned(),
            Arc::new(egui::FontData::from_owned(font_data)),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "JetBrainsMapleMono".to_owned());

        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("JetBrainsMapleMono".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        init_i18n(&settings.language);

        let detected_game_dir = GameDir::new().ok();

        let inventory = if let Some(ref game_dir) = detected_game_dir {
            match InventoryLoader::load_from_game_dir(&game_dir.path()) {
                Ok(inv) => inv,
                Err(e) => {
                    eprintln!("Failed to load inventory: {}", e);
                    Inventory::default()
                }
            }
        } else {
            eprintln!("Failed to detect game directory");
            Inventory::default()
        };

        let mut items_game = ItemsGame::default();
        let mut translations = GameTranslation::default();

        if let Some(ref game_dir) = detected_game_dir {
            let items_game_path = game_dir
                .path()
                .join("csgo")
                .join("scripts")
                .join("items")
                .join("items_game.txt");
            if items_game_path.exists() {
                match ItemsGameLoader::load(&items_game_path) {
                    Ok(ig) => items_game = ig,
                    Err(e) => eprintln!("Failed to load items_game.txt: {}", e),
                }
            }

            let english_path = game_dir
                .path()
                .join("csgo")
                .join("resource")
                .join("csgo_english.txt");
            let chinese_path = game_dir
                .path()
                .join("csgo")
                .join("resource")
                .join("csgo_schinese.txt");
            let tchinese_path = game_dir
                .path()
                .join("csgo")
                .join("resource")
                .join("csgo_tchinese.txt");

            let lang_file = match settings.language.as_str() {
                "zh-Hans" => {
                    if chinese_path.exists() {
                        chinese_path
                    } else if tchinese_path.exists() {
                        tchinese_path
                    } else {
                        eprintln!("Chinese language file not found, falling back to English");
                        english_path
                    }
                }
                _ => {
                    if english_path.exists() {
                        english_path
                    } else {
                        eprintln!("English language file not found");
                        english_path
                    }
                }
            };

            if lang_file.exists() {
                match LanguageFileParser::load(&lang_file) {
                    Ok(t) => translations = t,
                    Err(e) => eprintln!("Failed to load language file: {}", e),
                }
            }
        }

        let config = if let Some(ref game_dir) = detected_game_dir {
            let config_path = game_dir.path().join("csgo_gc").join("config.txt");
            if config_path.exists() {
                ConfigLoader::load(&config_path).unwrap_or_default()
            } else {
                Config::default()
            }
        } else {
            Config::default()
        };

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
            select_window_for_item: None,
            select_window_for_attr: None,
            current_language: settings.language.clone(),
            game_dir: detected_game_dir,
            delete_confirm_item_id: None,
            pending_add_item: false,
            selected_template: None,
            show_template_modal: false,
            pending_paint_kit_select: None,
            pending_music_def_select: None,
            pending_sticker_kit_select: None,
            settings: settings.clone(),
            current_page: Page::default(),
            current_settings_page: SettingsPage::default(),
            config,
            cached_sorted_inventory_ids: Vec::new(),
            cached_items_count: 0,
            cached_item_display_names: RefCell::new(HashMap::new()),
            last_theme: Some(settings.theme.clone()),
        }
    }

    pub fn switch_language(&mut self, language: &str) {
        self.current_language = language.to_string();
        self.settings.set_language(language.to_string());
        let _ = self.settings.save();
        set_language(language);

        if let Some(ref game_dir) = self.game_dir {
            let lang_file = if language == "zh-Hans" {
                game_dir
                    .path()
                    .join("csgo")
                    .join("resource")
                    .join("csgo_schinese.txt")
            } else {
                game_dir
                    .path()
                    .join("csgo")
                    .join("resource")
                    .join("csgo_english.txt")
            };

            if lang_file.exists() {
                match LanguageFileParser::load(&lang_file) {
                    Ok(t) => {
                        self.translations = t;
                        eprintln!("Loaded language file: {:?}", lang_file);
                    }
                    Err(e) => eprintln!("Failed to load language file: {}", e),
                }
            } else {
                eprintln!("Language file not found: {:?}", lang_file);
            }
        }
        self.cached_item_display_names.borrow_mut().clear();
    }

    pub fn apply_theme(&mut self, ctx: &egui::Context) {
        let current_theme = self.settings.theme;

        if self.last_theme == Some(current_theme) {
            return;
        }

        self.last_theme = Some(current_theme);

        match current_theme {
            Theme::Light => {
                ctx.set_theme(egui::Theme::Light);
            }
            Theme::Dark => {
                ctx.set_theme(egui::Theme::Dark);
            }
            Theme::System => {
                ctx.set_theme(egui::Theme::from_dark_mode(ctx.style().visuals.dark_mode));
            }
        }
    }

    pub fn save_inventory(&mut self) -> Result<(), String> {
        if let Some(ref game_dir) = self.game_dir {
            let result = InventoryLoader::save_to_game_dir(&self.inventory, game_dir.path())
                .map_err(|e| e.to_string());
            if result.is_ok() {
                self.cached_item_display_names.borrow_mut().clear();
            }
            result
        } else {
            Err("Game directory not found".to_string())
        }
    }

    pub fn save_config(&mut self) -> Result<(), String> {
        if let Some(ref game_dir) = self.game_dir {
            let config_path = game_dir.path().join("csgo_gc").join("config.txt");
            ConfigLoader::save(&self.config, &config_path).map_err(|e| e.to_string())
        } else {
            Err("Game directory not found".to_string())
        }
    }

    pub fn get_item_display_name(&self, item: &crate::inventory::Item) -> String {
        let inventory_id = item.inventory;
        if let Some(cached) = self.cached_item_display_names.borrow().get(&inventory_id) {
            return cached.clone();
        }
        let display_name = self.items_game.get_item_full_name(item, &self.translations);
        self.cached_item_display_names.borrow_mut().insert(inventory_id, display_name.clone());
        display_name
    }

    pub fn get_rarity_name(&self, rarity_id: u32) -> String {
        if let Some(rarity) = self
            .items_game
            .rarities
            .values()
            .find(|r| r.value == rarity_id)
        {
            self.translations
                .get(&rarity.loc_key)
                .cloned()
                .unwrap_or_else(|| {
                    rarity
                        .loc_key_weapon
                        .as_ref()
                        .and_then(|key| self.translations.get(key).cloned())
                        .unwrap_or_else(|| rarity.loc_key.clone())
                })
        } else {
            format!("Unknown ({})", rarity_id)
        }
    }

    pub fn open_select_window(
        &mut self,
        title: String,
        key_header: String,
        value_header: String,
        items: Vec<(String, String, String)>,
    ) {
        self.select_window_title = title;
        self.select_window_key_header = key_header;
        self.select_window_value_header = value_header;
        self.select_window_items = items;
        self.select_window_search = String::new();
        self.select_window_selected = None;
        self.select_window_open = true;
    }

    pub fn create_item_select_list(&self) -> Vec<(String, String, String)> {
        self.items_game.create_item_select_list(&self.translations)
    }

    pub fn create_paint_kit_select_list(&self) -> Vec<(String, String, String)> {
        self.items_game
            .create_paint_kit_select_list(&self.translations)
    }

    pub fn create_music_def_select_list(&self) -> Vec<(String, String, String)> {
        self.items_game
            .create_music_def_select_list(&self.translations)
    }

    pub fn create_sticker_kit_select_list(&self) -> Vec<(String, String, String)> {
        self.items_game
            .create_sticker_kit_select_list(&self.translations)
    }

    pub fn get_sorted_inventory_ids(&mut self) -> &[u64] {
        if self.cached_items_count != self.inventory.items.len() {
            self.update_sorted_cache();
        }
        &self.cached_sorted_inventory_ids
    }

    fn update_sorted_cache(&mut self) {
        self.cached_sorted_inventory_ids = self
            .inventory
            .items
            .iter()
            .map(|item| item.inventory)
            .collect();
        self.cached_sorted_inventory_ids.sort_by(|a, b| b.cmp(a));
        self.cached_items_count = self.inventory.items.len();
        self.cached_item_display_names.borrow_mut().clear();
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
            select_window_for_item: None,
            select_window_for_attr: None,
            current_language: "en-US".to_string(),
            game_dir: None,
            delete_confirm_item_id: None,
            pending_add_item: false,
            selected_template: None,
            show_template_modal: false,
            pending_paint_kit_select: None,
            pending_music_def_select: None,
            pending_sticker_kit_select: None,
            settings: Settings::default(),
            current_page: Page::default(),
            current_settings_page: SettingsPage::default(),
            config: Config::default(),
            cached_sorted_inventory_ids: Vec::new(),
            cached_items_count: 0,
            cached_item_display_names: RefCell::new(HashMap::new()),
            last_theme: None,
        }
    }
}
