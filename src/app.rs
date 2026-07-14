use crate::config::{Config, ConfigLoader};
use crate::core::GameDir;
use crate::inventory::{
    AVAILABLE_ATTRIBUTES, GameTranslation, Inventory, InventoryLoader, ItemAttribute, ItemsGame,
    ItemsGameLoader, LanguageFileParser, get_attribute_fluent_key,
};
use crate::online_data::{
    DataProvider, OnlineGameData, fetch_online_data_with_progress, load_cached_data,
    save_cached_data,
};
use crate::rcon::RconClient;
use crate::settings::{Settings, Theme};
use eframe::egui;
use egui_i18n::tr;
use egui_i18n::{load_translations_from_path, set_fallback, set_language};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

// Type alias for select window items: (id, name, value, optional_color)
pub type SelectWindowItem = (String, String, String, Option<String>);
pub type SelectWindowItems = Vec<SelectWindowItem>;

// Type alias for online data fetch result: (data, timestamp, language)
pub type OnlineDataFetchResult = Result<(OnlineGameData, String, String), String>;
pub type RconConnectResult = Result<RconClient, String>;
pub type RconCommandResult = (RconClient, String, Result<String, String>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectWindowPurpose {
    AddItem,
    AddWeaponCase,
    EditItemDef,
    SelectPaintKit,
    SelectMusicDef,
    SelectStickerKit,
    SelectGraffitiTint,
    AddAttribute,
    RconItemDef,
    RconPaintKit,
}

impl SelectWindowPurpose {
    pub fn cache_salt(self) -> &'static str {
        match self {
            SelectWindowPurpose::AddItem => "add_item",
            SelectWindowPurpose::AddWeaponCase => "add_weapon_case",
            SelectWindowPurpose::EditItemDef => "edit_item_def",
            SelectWindowPurpose::SelectPaintKit => "select_paint_kit",
            SelectWindowPurpose::SelectMusicDef => "select_music_def",
            SelectWindowPurpose::SelectStickerKit => "select_sticker_kit",
            SelectWindowPurpose::SelectGraffitiTint => "select_graffiti_tint",
            SelectWindowPurpose::AddAttribute => "add_attribute",
            SelectWindowPurpose::RconItemDef => "rcon_item_def",
            SelectWindowPurpose::RconPaintKit => "rcon_paint_kit",
        }
    }
}

fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

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
            Rarity::Consumer => Some(egui::Color32::from_rgb(176, 195, 217)),
            Rarity::Industrial => Some(egui::Color32::from_rgb(94, 152, 217)),
            Rarity::MilSpec => Some(egui::Color32::from_rgb(75, 105, 255)),
            Rarity::Restricted => Some(egui::Color32::from_rgb(136, 71, 255)),
            Rarity::Classified => Some(egui::Color32::from_rgb(211, 44, 230)),
            Rarity::Covert => Some(egui::Color32::from_rgb(235, 75, 75)),
            Rarity::Contraband => Some(egui::Color32::from_rgb(228, 174, 57)),
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
    WeaponCase,
}

impl ItemTemplate {
    pub fn create_item(&self, id: u64, def_index: u32) -> crate::inventory::Item {
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
            ItemTemplate::WeaponCase => {}
        }

        crate::inventory::Item {
            id,
            inventory: 0,
            def_index,
            level: 1,
            quality,
            flags: 0,
            origin: 0,
            in_use: 0,
            rarity: if matches!(self, ItemTemplate::WeaponCase) {
                1
            } else {
                0
            },
            custom_name: None,
            attributes,
            equipped_state: HashMap::new(),
        }
    }

    pub fn create_music_kit(&self, id: u64, music_id: u32) -> crate::inventory::Item {
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
            id,
            inventory: 0,
            def_index: 1314,
            level: 1,
            quality,
            flags: 0,
            origin: 0,
            in_use: 0,
            rarity: 3,
            custom_name: None,
            attributes,
            equipped_state: HashMap::new(),
        }
    }

    pub fn is_music_kit(&self) -> bool {
        matches!(
            self,
            ItemTemplate::NormalMusicKit | ItemTemplate::StatTrakMusicKit
        )
    }

    pub fn is_weapon_case(&self) -> bool {
        matches!(self, ItemTemplate::WeaponCase)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Page {
    #[default]
    Inventory,
    Rcon,
    Settings,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum RuntimeMode {
    #[default]
    OfflineEdit,
    LiveRcon,
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
    pub items_game: Arc<ItemsGame>,
    pub translations: Arc<GameTranslation>,
    pub open_item_windows: HashSet<u64>,
    pub edit_item_states: HashMap<u64, EditItemState>,
    pub select_window_open: bool,
    pub select_window_purpose: Option<SelectWindowPurpose>,
    pub select_window_items: SelectWindowItems,
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
    pub pending_paint_kit_select: Option<(u64, u32)>,
    pub pending_music_def_select: Option<u64>,
    pub pending_sticker_kit_select: Option<(u64, u32)>,
    pub pending_graffiti_tint_select: Option<u64>,
    pub pending_attribute_select: Option<u64>,
    pub settings: Settings,
    pub data_provider: DataProvider,
    pub is_loading_online: bool,
    online_data_receiver: Option<Receiver<OnlineDataFetchResult>>,
    pub current_page: Page,
    pub current_settings_page: SettingsPage,
    pub runtime_mode: RuntimeMode,
    pub rcon_ui: RconUiState,
    pub rcon_client: Option<RconClient>,
    rcon_connect_receiver: Option<Receiver<RconConnectResult>>,
    rcon_command_receiver: Option<Receiver<RconCommandResult>>,
    pub config: Config,
    pub status_message: Option<String>,
    cached_quality_names: Vec<(u32, String)>,
    cached_rarity_names: Vec<(u32, String)>,
    cached_sorted_inventory_indices: Vec<usize>,
    cached_item_indices_by_id: HashMap<u64, usize>,
    cached_items_count: usize,
    cached_item_display_names: RefCell<HashMap<u64, String>>,
    load_errors: Vec<String>,
    last_theme: Option<Theme>,
}

#[derive(Debug, Clone)]
pub struct RconUiState {
    pub address: String,
    pub port: u16,
    pub password: String,
    pub command_input: String,
    pub give_def_index: u32,
    pub give_count: u32,
    pub give_level: u32,
    pub give_quality: u32,
    pub give_rarity: u32,
    pub give_custom_name: String,
    pub give_paint: String,
    pub give_seed: String,
    pub give_wear: String,
    pub give_stattrak: String,
    pub remove_item_id: String,
    pub last_response: String,
    pub log: Vec<String>,
}

impl Default for RconUiState {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 37016,
            password: String::new(),
            command_input: String::new(),
            give_def_index: 7,
            give_count: 1,
            give_level: 1,
            give_quality: 4,
            give_rarity: 0,
            give_custom_name: String::new(),
            give_paint: String::new(),
            give_seed: String::new(),
            give_wear: String::new(),
            give_stattrak: String::new(),
            remove_item_id: String::new(),
            last_response: String::new(),
            log: Vec::new(),
        }
    }
}

fn init_i18n(language: &str) {
    let languages_path = get_exe_dir()
        .join("csgo_gc")
        .join("editor")
        .join("languages");
    if let Err(e) = load_translations_from_path(languages_path.to_string_lossy().as_ref()) {
        eprintln!("Failed to load translations: {}", e);
    }
    set_language(language);
    set_fallback("en-US");
}

impl CsgoInventoryEditor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = Settings::load().unwrap_or_default();

        let mut fonts = egui::FontDefinitions::default();

        let font_path = get_exe_dir()
            .join("csgo_gc")
            .join("editor")
            .join("fonts")
            .join("JetBrainsMapleMono-Regular.ttf");
        let font_data = fs::read(&font_path).expect("Failed to read font file");

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

        let mut load_errors = Vec::new();
        let inventory = if let Some(ref game_dir) = detected_game_dir {
            match InventoryLoader::load_from_game_dir(game_dir.path()) {
                Ok(inv) => inv,
                Err(e) => {
                    load_errors.push(format!("Failed to load inventory: {}", e));
                    Inventory::default()
                }
            }
        } else {
            load_errors.push("Failed to detect game directory".to_string());
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
                    Err(e) => load_errors.push(format!("Failed to load items_game.txt: {}", e)),
                }
            } else {
                load_errors.push(format!(
                    "items_game.txt not found: {}",
                    items_game_path.display()
                ));
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
                        load_errors.push(
                            "Chinese language file not found, falling back to English".to_string(),
                        );
                        english_path
                    }
                }
                _ => {
                    if english_path.exists() {
                        english_path
                    } else {
                        load_errors.push("English language file not found".to_string());
                        english_path
                    }
                }
            };

            if lang_file.exists() {
                match LanguageFileParser::load(&lang_file) {
                    Ok(t) => translations = t,
                    Err(e) => load_errors.push(format!("Failed to load language file: {}", e)),
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

        let items_game = Arc::new(items_game);
        let translations = Arc::new(translations);
        let rcon_ui = RconUiState {
            address: settings.rcon.address.clone(),
            port: settings.rcon.port,
            password: settings.rcon.password.clone(),
            ..RconUiState::default()
        };

        let mut app = Self {
            inventory,
            items_game: Arc::clone(&items_game),
            translations: Arc::clone(&translations),
            open_item_windows: HashSet::new(),
            edit_item_states: HashMap::new(),
            select_window_open: false,
            select_window_purpose: None,
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
            pending_graffiti_tint_select: None,
            pending_attribute_select: None,
            settings: settings.clone(),
            data_provider: DataProvider::Local {
                items_game: Arc::clone(&items_game),
                translations: Arc::clone(&translations),
            },
            is_loading_online: false,
            online_data_receiver: None,
            current_page: Page::default(),
            current_settings_page: SettingsPage::default(),
            runtime_mode: RuntimeMode::default(),
            rcon_ui,
            rcon_client: None,
            rcon_connect_receiver: None,
            rcon_command_receiver: None,
            config,
            status_message: None,
            cached_quality_names: Vec::new(),
            cached_rarity_names: Vec::new(),
            cached_sorted_inventory_indices: Vec::new(),
            cached_item_indices_by_id: HashMap::new(),
            cached_items_count: 0,
            cached_item_display_names: RefCell::new(HashMap::new()),
            load_errors,
            last_theme: Some(settings.theme),
        };

        // Try to load online data cache on startup
        if let Some((data, timestamp)) = load_cached_data(&settings.language) {
            app.data_provider = DataProvider::Online {
                data: Arc::new(data.clone()),
                items_game,
                translations,
            };
            app.settings.last_online_update = Some(timestamp);
        }

        app.refresh_display_metadata_cache();
        app
    }

    pub fn switch_language(&mut self, language: &str) {
        self.current_language = language.to_string();
        self.settings.set_language(language.to_string());
        self.record_result(self.settings.save(), "save settings");
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
                        self.translations = Arc::new(t);
                        self.refresh_display_metadata_cache();
                    }
                    Err(e) => eprintln!("Failed to load language file: {}", e),
                }
            } else {
                eprintln!("Language file not found: {:?}", lang_file);
            }
        }
        self.cached_item_display_names.borrow_mut().clear();

        // Reload online data for the new language if currently using online mode
        if matches!(self.data_provider, DataProvider::Online { .. }) {
            if let Some((data, timestamp)) = load_cached_data(language) {
                self.data_provider = DataProvider::Online {
                    data: Arc::new(data.clone()),
                    items_game: Arc::clone(&self.items_game),
                    translations: Arc::clone(&self.translations),
                };
                self.settings.last_online_update = Some(timestamp);
                self.record_result(self.settings.save(), "save settings");
            } else {
                // No cache for new language, fall back to local mode
                self.data_provider = DataProvider::Local {
                    items_game: Arc::clone(&self.items_game),
                    translations: Arc::clone(&self.translations),
                };
            }
        } else {
            self.data_provider = DataProvider::Local {
                items_game: Arc::clone(&self.items_game),
                translations: Arc::clone(&self.translations),
            };
        }

        self.refresh_display_metadata_cache();
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
                ctx.set_theme(egui::Theme::from_dark_mode(
                    ctx.global_style().visuals.dark_mode,
                ));
            }
        }
    }

    pub fn save_inventory(&mut self) -> Result<(), String> {
        if self.is_live_rcon() {
            return Err("RCON is connected; inventory.txt is read-only".to_string());
        }
        if let Some(ref game_dir) = self.game_dir {
            let result = InventoryLoader::save_to_game_dir(&self.inventory, game_dir.path())
                .map_err(|e| e.to_string());
            if result.is_ok() {
                self.update_sorted_cache();
            }
            result
        } else {
            Err("Game directory not found".to_string())
        }
    }

    pub fn save_config(&mut self) -> Result<(), String> {
        if self.is_live_rcon() {
            return Err("RCON is connected; config.txt is read-only".to_string());
        }
        if let Some(ref game_dir) = self.game_dir {
            let config_path = game_dir.path().join("csgo_gc").join("config.txt");
            ConfigLoader::save(&self.config, &config_path).map_err(|e| e.to_string())
        } else {
            Err("Game directory not found".to_string())
        }
    }

    pub fn is_live_rcon(&self) -> bool {
        self.runtime_mode == RuntimeMode::LiveRcon
    }

    pub fn is_connecting_rcon(&self) -> bool {
        self.rcon_connect_receiver.is_some()
    }

    pub fn is_sending_rcon_command(&self) -> bool {
        self.rcon_command_receiver.is_some()
    }

    pub fn connect_rcon(&mut self) {
        if self.is_connecting_rcon() || self.is_live_rcon() {
            return;
        }

        self.settings.rcon.address = self.rcon_ui.address.clone();
        self.settings.rcon.port = self.rcon_ui.port;
        self.settings.rcon.password = self.rcon_ui.password.clone();
        self.record_result(self.settings.save(), "save settings");

        let address = self.rcon_ui.address.clone();
        let port = self.rcon_ui.port;
        let password = self.rcon_ui.password.clone();

        self.push_rcon_log(format!(
            "Connecting to {}:{}...",
            self.rcon_ui.address, self.rcon_ui.port
        ));

        let (tx, rx) = mpsc::channel();
        self.rcon_connect_receiver = Some(rx);
        std::thread::spawn(move || {
            let result = RconClient::connect(&address, port, &password, Duration::from_secs(3));
            let _ = tx.send(result);
        });
    }

    pub fn disconnect_rcon(&mut self) {
        self.rcon_connect_receiver = None;
        self.rcon_command_receiver = None;
        self.rcon_client = None;
        self.runtime_mode = RuntimeMode::OfflineEdit;
        self.push_rcon_log("Disconnected. Offline editing is available.".to_string());
    }

    pub fn check_rcon_connect_result(&mut self) {
        let Some(receiver) = &self.rcon_connect_receiver else {
            return;
        };

        match receiver.try_recv() {
            Ok(Ok(client)) => {
                self.rcon_connect_receiver = None;
                self.rcon_client = Some(client);
                self.runtime_mode = RuntimeMode::LiveRcon;
                self.push_rcon_log("Connected. Offline files are now read-only.".to_string());
            }
            Ok(Err(e)) => {
                self.rcon_connect_receiver = None;
                self.rcon_client = None;
                self.runtime_mode = RuntimeMode::OfflineEdit;
                self.push_rcon_log(format!("Connect failed: {}", e));
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.rcon_connect_receiver = None;
                self.rcon_client = None;
                self.runtime_mode = RuntimeMode::OfflineEdit;
                self.push_rcon_log("Connect failed: worker disconnected".to_string());
            }
        }
    }

    pub fn send_rcon_command(&mut self, command: &str) {
        let command = command.trim();
        if command.is_empty() {
            return;
        }

        if self.is_sending_rcon_command() {
            self.push_rcon_log("ERR RCON command already in progress".to_string());
            return;
        }

        let Some(mut client) = self.rcon_client.take() else {
            self.push_rcon_log("ERR RCON is not connected".to_string());
            return;
        };

        let command = command.to_string();
        self.push_rcon_log(format!("> {}", command));

        let (tx, rx) = mpsc::channel();
        self.rcon_command_receiver = Some(rx);
        std::thread::spawn(move || {
            let response = client.send_command(&command);
            let _ = tx.send((client, command, response));
        });
    }

    pub fn check_rcon_command_result(&mut self) {
        let Some(receiver) = &self.rcon_command_receiver else {
            return;
        };

        match receiver.try_recv() {
            Ok((client, _command, response)) => {
                self.rcon_command_receiver = None;
                match response {
                    Ok(response) => {
                        self.rcon_client = Some(client);
                        self.rcon_ui.last_response = response.clone();
                        self.push_rcon_log(response);
                    }
                    Err(e) => {
                        self.rcon_ui.last_response = format!("ERR {}", e);
                        self.push_rcon_log(format!("ERR {}", e));
                        self.rcon_client = None;
                        self.runtime_mode = RuntimeMode::OfflineEdit;
                    }
                }
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                self.rcon_command_receiver = None;
                self.rcon_client = None;
                self.runtime_mode = RuntimeMode::OfflineEdit;
                self.push_rcon_log("ERR RCON command worker disconnected".to_string());
            }
        }
    }

    pub fn send_item_via_rcon(&mut self, item_id: u64) {
        let Some(item) = self.inventory.items.iter().find(|item| item.id == item_id) else {
            self.push_rcon_log(format!("ERR item {} not found", item_id));
            return;
        };

        match crate::rcon::commands::build_give_item_command(item, 1) {
            Ok(command) => self.send_rcon_command(&command),
            Err(e) => self.push_rcon_log(format!("ERR {}", e)),
        }
    }

    pub fn push_rcon_log(&mut self, message: String) {
        self.rcon_ui.log.push(message);
        if self.rcon_ui.log.len() > 200 {
            let excess = self.rcon_ui.log.len() - 200;
            self.rcon_ui.log.drain(0..excess);
        }
    }

    pub fn get_item_display_name(&self, item: &crate::inventory::Item) -> String {
        let item_id = item.id;
        if let Some(cached) = self.cached_item_display_names.borrow().get(&item_id) {
            return cached.clone();
        }
        let display_name = self.data_provider.get_item_full_name(item);
        self.cached_item_display_names
            .borrow_mut()
            .insert(item_id, display_name.clone());
        display_name
    }

    pub fn has_load_errors(&self) -> bool {
        !self.load_errors.is_empty()
    }

    pub fn get_load_errors(&self) -> &[String] {
        &self.load_errors
    }

    pub fn get_rarity_name(&self, rarity_id: u32) -> String {
        // Find rarity by value and translate its loc_key
        if let Some(rarity) = self
            .items_game
            .rarities
            .values()
            .find(|r| r.value == rarity_id)
        {
            let display_name = self
                .translations
                .get(&rarity.loc_key)
                .cloned()
                .unwrap_or_else(|| rarity.loc_key.clone());

            if let Some(weapon_key) = &rarity.loc_key_weapon {
                let weapon_name = self
                    .translations
                    .get(weapon_key)
                    .cloned()
                    .unwrap_or_else(|| weapon_key.clone());
                format!("{} | {}", display_name, weapon_name)
            } else {
                display_name
            }
        } else {
            format!("Unknown ({})", rarity_id)
        }
    }

    pub fn open_select_window(
        &mut self,
        purpose: SelectWindowPurpose,
        title: String,
        key_header: String,
        value_header: String,
        items: SelectWindowItems,
    ) {
        self.select_window_purpose = Some(purpose);
        self.select_window_title = title;
        self.select_window_key_header = key_header;
        self.select_window_value_header = value_header;
        self.select_window_items = items;
        self.select_window_search = String::new();
        self.select_window_selected = None;
        self.select_window_open = true;
    }

    pub fn close_select_window(&mut self) {
        self.select_window_open = false;
        self.select_window_selected = None;
        self.select_window_for_item = None;
        self.select_window_for_attr = None;
        self.select_window_purpose = None;
    }

    pub fn record_result<T, E: std::fmt::Display>(
        &mut self,
        result: Result<T, E>,
        action: &str,
    ) -> Option<T> {
        match result {
            Ok(value) => {
                self.status_message = None;
                Some(value)
            }
            Err(e) => {
                let message = format!("Failed to {}: {}", action, e);
                eprintln!("{}", message);
                self.status_message = Some(message);
                None
            }
        }
    }

    pub fn create_item_select_list(&self) -> SelectWindowItems {
        self.data_provider
            .create_item_select_list()
            .into_iter()
            .map(|(id, name, value)| (id, name, value, None))
            .collect()
    }

    pub fn create_weapon_case_select_list(&self) -> SelectWindowItems {
        self.data_provider
            .create_weapon_case_select_list()
            .into_iter()
            .map(|(id, name, value)| (id, name, value, None))
            .collect()
    }

    pub fn get_associated_item_def_indexes(&self, def_index: u32) -> &[u32] {
        self.items_game.get_associated_item_def_indexes(def_index)
    }

    pub fn create_music_def_select_list(&self) -> SelectWindowItems {
        self.data_provider.create_music_def_select_list()
    }

    pub fn create_sticker_kit_select_list(&self) -> SelectWindowItems {
        self.data_provider.create_sticker_kit_select_list()
    }

    pub fn create_graffiti_tint_select_list(&self) -> SelectWindowItems {
        self.items_game.create_graffiti_tint_select_list()
    }

    pub fn create_missing_attribute_select_list(&self, item_id: u64) -> SelectWindowItems {
        let current_attributes = self
            .edit_item_states
            .get(&item_id)
            .map(|state| &state.attributes)
            .or_else(|| {
                self.inventory
                    .items
                    .iter()
                    .find(|item| item.id == item_id)
                    .map(|item| &item.attributes)
            });

        let Some(current_attributes) = current_attributes else {
            return Vec::new();
        };

        let mut items: SelectWindowItems = AVAILABLE_ATTRIBUTES
            .iter()
            .filter(|attr_id| !current_attributes.contains_key(attr_id))
            .map(|attr_id| {
                let fluent_key = get_attribute_fluent_key(*attr_id);
                (
                    attr_id.to_string(),
                    tr!(&fluent_key).to_string(),
                    attr_id.to_string(),
                    None,
                )
            })
            .collect();

        items.sort_by_key(|(id, _, _, _)| id.parse::<u32>().unwrap_or(0));
        items
    }

    pub fn create_skin_select_list_for_weapon(&self, weapon_id: u32) -> SelectWindowItems {
        self.data_provider
            .create_skin_select_list_for_weapon(weapon_id)
    }

    pub fn get_skin_rarity(&self, weapon_id: u32, paint_index: u32) -> Option<u32> {
        self.data_provider.get_skin_rarity(weapon_id, paint_index)
    }

    pub fn load_online_data(&mut self) {
        if !self.is_loading_online {
            return;
        }

        // Already fetching, don't start again
        if self.online_data_receiver.is_some() {
            return;
        }

        self.start_fetch_online_data();
    }

    pub fn start_fetch_online_data(&mut self) {
        let mirror_prefix = self.settings.mirror_site.get_prefix().to_string();
        let language = self.current_language.clone();

        let (tx, rx) = mpsc::channel();
        self.online_data_receiver = Some(rx);

        std::thread::spawn(move || {
            let result = fetch_online_data_with_progress(&language, &mirror_prefix, |_msg: &str| {});

            match result {
                Ok(data) => {
                    match save_cached_data(&language, &data) {
                        Ok(timestamp) => {
                            let _ = tx.send(Ok((data, timestamp, language)));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e.to_string()));
                }
            }
        });
    }

    pub fn check_online_data_result(&mut self) {
        if let Some(ref receiver) = self.online_data_receiver {
            match receiver.try_recv() {
                Ok(Ok((data, timestamp, fetched_language))) => {
                    // Discard result if language changed during fetch
                    if fetched_language != self.current_language {
                        self.is_loading_online = false;
                        self.online_data_receiver = None;
                        return;
                    }
                    self.settings.last_online_update = Some(timestamp);
                    self.record_result(self.settings.save(), "save settings");
                    self.data_provider = DataProvider::Online {
                        data: Arc::new(data.clone()),
                        items_game: Arc::clone(&self.items_game),
                        translations: Arc::clone(&self.translations),
                    };
                    self.is_loading_online = false;
                    self.online_data_receiver = None;
                    self.cached_item_display_names.borrow_mut().clear();
                }
                Ok(Err(_)) => {
                    self.is_loading_online = false;
                    self.online_data_receiver = None;
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.is_loading_online = false;
                    self.online_data_receiver = None;
                }
            }
        }
    }

    pub fn is_fetching_online_data(&self) -> bool {
        self.online_data_receiver.is_some()
    }

    pub fn request_manual_update(&mut self) {
        if self.is_fetching_online_data() {
            return;
        }
        self.is_loading_online = true;
        self.load_online_data();
    }

    pub fn refresh_inventory_cache(&mut self) {
        if self.cached_items_count != self.inventory.items.len() {
            self.update_sorted_cache();
        }
    }

    pub fn get_cached_quality_names(&self) -> &[(u32, String)] {
        &self.cached_quality_names
    }

    pub fn get_cached_rarity_names(&self) -> &[(u32, String)] {
        &self.cached_rarity_names
    }

    pub fn get_sorted_inventory_indices(&self) -> &[usize] {
        &self.cached_sorted_inventory_indices
    }

    pub fn get_item_index(&self, item_id: u64) -> Option<usize> {
        self.cached_item_indices_by_id.get(&item_id).copied()
    }

    pub fn mark_inventory_changed(&mut self) {
        self.cached_items_count = usize::MAX;
        self.cached_item_display_names.borrow_mut().clear();
    }

    fn update_sorted_cache(&mut self) {
        self.cached_sorted_inventory_indices = (0..self.inventory.items.len()).collect();
        self.cached_sorted_inventory_indices
            .sort_by_key(|&idx| std::cmp::Reverse(self.inventory.items[idx].id));
        self.cached_item_indices_by_id = self
            .inventory
            .items
            .iter()
            .enumerate()
            .map(|(idx, item)| (item.id, idx))
            .collect();
        self.cached_items_count = self.inventory.items.len();
        self.cached_item_display_names.borrow_mut().clear();
    }

    fn refresh_display_metadata_cache(&mut self) {
        self.cached_quality_names = self
            .items_game
            .get_all_qualities_sorted()
            .into_iter()
            .map(|(value, key)| {
                let display_name = self.translations.get(&key).cloned().unwrap_or(key);
                (value, display_name)
            })
            .collect();

        self.cached_rarity_names = self
            .items_game
            .get_all_rarities_sorted()
            .into_iter()
            .map(|(value, _)| (value, self.get_rarity_name(value)))
            .collect();
    }
}

impl Default for CsgoInventoryEditor {
    fn default() -> Self {
        Self {
            inventory: Inventory::default(),
            items_game: Arc::new(ItemsGame::default()),
            translations: Arc::new(GameTranslation::default()),
            open_item_windows: HashSet::new(),
            edit_item_states: HashMap::new(),
            select_window_open: false,
            select_window_purpose: None,
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
            pending_graffiti_tint_select: None,
            pending_attribute_select: None,
            settings: Settings::default(),
            data_provider: DataProvider::Local {
                items_game: Arc::new(ItemsGame::default()),
                translations: Arc::new(GameTranslation::default()),
            },
            is_loading_online: false,
            online_data_receiver: None,
            current_page: Page::default(),
            current_settings_page: SettingsPage::default(),
            runtime_mode: RuntimeMode::default(),
            rcon_ui: RconUiState::default(),
            rcon_client: None,
            rcon_connect_receiver: None,
            rcon_command_receiver: None,
            config: Config::default(),
            status_message: None,
            cached_quality_names: Vec::new(),
            cached_rarity_names: Vec::new(),
            cached_sorted_inventory_indices: Vec::new(),
            cached_item_indices_by_id: HashMap::new(),
            cached_items_count: 0,
            cached_item_display_names: RefCell::new(HashMap::new()),
            load_errors: Vec::new(),
            last_theme: None,
        }
    }
}
