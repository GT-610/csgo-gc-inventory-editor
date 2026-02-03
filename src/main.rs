use eframe::egui;
use std::fs;
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use egui_extras::{Column, TableBuilder};
use csgo_inventory_editor::inventory::{Inventory, InventoryLoader, ItemsGame, GameTranslation, ItemsGameLoader, LanguageFileParser};
use csgo_inventory_editor::core::GameDir;

/// Rarity levels for CS:GO items
/// 0: Default (no indicator)
/// 1: Consumer (gray-white)
/// 2: Industrial (light blue)
/// 3: Mil-Spec (dark blue)
/// 4: Restricted (purple)
/// 5: Classified (pink)
/// 6: Covert (red)
/// 7: Contraband (yellow)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum Rarity {
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
    fn from_u32(value: u32) -> Self {
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

    fn color(&self) -> Option<egui::Color32> {
        match self {
            Rarity::Default => None,
            Rarity::Consumer => Some(egui::Color32::from_rgb(176, 176, 176)), // Gray-white
            Rarity::Industrial => Some(egui::Color32::from_rgb(94, 152, 217)), // Light blue
            Rarity::MilSpec => Some(egui::Color32::from_rgb(75, 105, 255)),   // Dark blue
            Rarity::Restricted => Some(egui::Color32::from_rgb(136, 71, 255)), // Purple
            Rarity::Classified => Some(egui::Color32::from_rgb(211, 44, 230)), // Pink
            Rarity::Covert => Some(egui::Color32::from_rgb(235, 75, 75)),     // Red
            Rarity::Contraband => Some(egui::Color32::from_rgb(255, 215, 0)), // Yellow
        }
    }
}

#[derive(Debug, Default, PartialEq)]
enum InventoryCategory {
    #[default]
    All,
    Equipped,
    StickerAndGraffiti,
    CasesAndMore,
    Collectibles,
}

#[derive(Clone)]
struct EditItemState {
    level: u32,
    custom_name: String,
}

struct CsgoInventoryEditor {
    inventory: Inventory,
    items_game: ItemsGame,
    translations: GameTranslation,
    selected_category: InventoryCategory,
    selected_subcategory: Option<String>,
    search_query: String,
    open_item_windows: std::collections::HashSet<u64>,
    edit_item_states: std::collections::HashMap<u64, EditItemState>,
    select_window_open: bool,
    select_window_items: Vec<(String, String, String)>,
    select_window_search: String,
    select_window_selected: Option<usize>,
    select_window_title: String,
    select_window_key_header: String,
    select_window_value_header: String,
}

impl CsgoInventoryEditor {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            open_item_windows: std::collections::HashSet::new(),
            edit_item_states: std::collections::HashMap::new(),
            select_window_open: false,
            select_window_items: Vec::new(),
            select_window_search: String::new(),
            select_window_selected: None,
            select_window_title: String::new(),
            select_window_key_header: String::new(),
            select_window_value_header: String::new(),
        }
    }

    fn get_item_display_name(&self, item: &csgo_inventory_editor::inventory::Item) -> String {
        self.items_game.get_item_full_name(item, &self.translations)
    }
    
    fn get_rarity_name(&self, rarity_id: u32) -> String {
        if let Some(rarity) = self.items_game.rarities.values().find(|r| r.value == rarity_id) {
            self.translations.get(&rarity.loc_key).cloned().unwrap_or_else(|| {
                rarity.loc_key_weapon.as_ref().and_then(|key| self.translations.get(key).cloned())
                    .unwrap_or_else(|| rarity.loc_key.clone())
            })
        } else {
            format!("Unknown ({})", rarity_id)
        }
    }
    
    fn open_select_window(&mut self, title: String, key_header: String, value_header: String, items: Vec<(String, String, String)>) {
        self.select_window_title = title;
        self.select_window_key_header = key_header;
        self.select_window_value_header = value_header;
        self.select_window_items = items;
        self.select_window_search = String::new();
        self.select_window_selected = None;
        self.select_window_open = true;
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
            open_item_windows: std::collections::HashSet::new(),
            edit_item_states: std::collections::HashMap::new(),
            select_window_open: false,
            select_window_items: Vec::new(),
            select_window_search: String::new(),
            select_window_selected: None,
            select_window_title: String::new(),
            select_window_key_header: String::new(),
            select_window_value_header: String::new(),
        }
    }
}

impl eframe::App for CsgoInventoryEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("分类筛选:");
                egui::ComboBox::from_id_salt("category_combo")
                    .selected_text(format!("{:?}", self.selected_category))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_category, InventoryCategory::All, "全部");
                        ui.selectable_value(&mut self.selected_category, InventoryCategory::Equipped, "装备");
                        ui.selectable_value(&mut self.selected_category, InventoryCategory::StickerAndGraffiti, "印花与涂鸦");
                        ui.selectable_value(&mut self.selected_category, InventoryCategory::CasesAndMore, "武器箱与更多");
                        ui.selectable_value(&mut self.selected_category, InventoryCategory::Collectibles, "展示品");
                    });
                
                ui.add_space(20.0);
                
                ui.label("搜索物品:");
                ui.text_edit_singleline(&mut self.search_query);
            });
            ui.add_space(8.0);
            
            ui.separator();
            
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                match self.selected_category {
                    InventoryCategory::All => {
                        let is_selected = self.selected_subcategory.is_none() || self.selected_subcategory.as_ref().is_some_and(|s| s == "全部");
                        if ui.selectable_label(is_selected, "全部").clicked() {
                            self.selected_subcategory = None;
                        }
                    }
                    InventoryCategory::Equipped => {
                        let subcategories = ["全部", "全套装备", "近战武器", "手枪", "微型冲锋枪", "步枪", "重型武器", "探员", "手套", "音乐盒"];
                        for sub in &subcategories {
                            let is_selected = self.selected_subcategory.as_ref().is_some_and(|s| s == *sub);
                            if ui.selectable_label(is_selected, *sub).clicked() {
                                self.selected_subcategory = Some(sub.to_string());
                            }
                        }
                    }
                    InventoryCategory::StickerAndGraffiti => {
                        let subcategories = ["全部艺术作品", "布章", "印花", "涂鸦"];
                        for sub in &subcategories {
                            let is_selected = self.selected_subcategory.as_ref().is_some_and(|s| s == *sub);
                            if ui.selectable_label(is_selected, *sub).clicked() {
                                self.selected_subcategory = Some(sub.to_string());
                            }
                        }
                    }
                    InventoryCategory::CasesAndMore => {
                        let subcategories = ["所有武器箱", "印花胶嚢", "涂鸦箱", "纪念箱", "工具"];
                        for sub in &subcategories {
                            let is_selected = self.selected_subcategory.as_ref().is_some_and(|s| s == *sub);
                            if ui.selectable_label(is_selected, *sub).clicked() {
                                self.selected_subcategory = Some(sub.to_string());
                            }
                        }
                    }
                    InventoryCategory::Collectibles => {
                        let subcategories = ["所有", "徽章", "音乐盒"];
                        for sub in &subcategories {
                            let is_selected = self.selected_subcategory.as_ref().is_some_and(|s| s == *sub);
                            if ui.selectable_label(is_selected, *sub).clicked() {
                                self.selected_subcategory = Some(sub.to_string());
                            }
                        }
                    }
                }
            });
            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let items_per_row = 8;
            let card_height = 100.0;
            let spacing = 8.0;

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(8.0);

                // Calculate responsive card width based on available space
                // Must be inside ScrollArea to get correct available width
                let available_width = ui.available_width();
                let total_spacing = spacing * (items_per_row as f32 - 1.0);
                let card_width = (available_width - total_spacing) / items_per_row as f32;

                // Calculate font size based on card size - ensure text fits within 2 lines
                let font_size = (card_width * 0.16).clamp(12.0, 20.0);

                egui::Grid::new("item_grid")
                    .num_columns(items_per_row)
                    .spacing([spacing, spacing])
                    .min_col_width(card_width)
                    .min_row_height(card_height)
                    .show(ui, |ui| {
                        let mut sorted_items: Vec<_> = self.inventory.items.iter().collect();
                        sorted_items.sort_by_key(|item| item.inventory);
                        
                        for (i, item) in sorted_items.iter().enumerate() {
                            let display_name = self.get_item_display_name(item);
                            let rarity = Rarity::from_u32(item.rarity);

                            // Allocate space for the card
                            let (card_rect, card_response) = ui.allocate_exact_size(
                                egui::Vec2::new(card_width, card_height),
                                egui::Sense::click(),
                            );

                            // Card background color based on hover/click state
                            let bg_color = if card_response.clicked() {
                                ui.visuals().widgets.active.bg_fill
                            } else if card_response.hovered() {
                                ui.visuals().widgets.hovered.bg_fill
                            } else {
                                ui.visuals().widgets.inactive.bg_fill
                            };

                            // Card stroke based on state
                            let stroke = if card_response.hovered() || card_response.clicked() {
                                ui.visuals().widgets.hovered.bg_stroke
                            } else {
                                ui.visuals().widgets.inactive.bg_stroke
                            };

                            // Draw card background with rounded corners
                            let corner_radius = egui::CornerRadius::same(4);
                            ui.painter().rect_filled(card_rect, corner_radius, bg_color);
                            ui.painter().rect_stroke(card_rect, corner_radius, stroke, egui::StrokeKind::Middle);

                            // Draw rarity indicator bar on the left side (inside the card)
                            if let Some(color) = rarity.color() {
                                let indicator_width = 4.0;
                                let indicator_margin = 2.0;
                                let indicator_rect = egui::Rect::from_min_size(
                                    egui::Pos2::new(
                                        card_rect.min.x + indicator_margin,
                                        card_rect.min.y + indicator_margin,
                                    ),
                                    egui::Vec2::new(
                                        indicator_width,
                                        card_height - 2.0 * indicator_margin,
                                    ),
                                );
                                ui.painter().rect_filled(indicator_rect, corner_radius, color);
                            }

                            // Draw item text: inventory ID at top, item name below
                            let text_margin = 8.0;
                            let indicator_space = if rarity.color().is_some() { 10.0 } else { 4.0 };
                            
                            let text_start_x = card_rect.min.x + text_margin + indicator_space;
                            let text_max_width = card_width - 2.0 * text_margin - indicator_space;
                            let text_max_height = card_height - 2.0 * text_margin;
                            let _max_lines_per_text = 1;

                            // Calculate font size for inventory ID (smaller)
                            let id_font_size = (font_size * 0.7).clamp(10.0, 14.0);
                            let id_text = format!("#{}", item.inventory);
                            
                            // Calculate font size for item name (larger)
                            let padding = 4.0;
                            let actual_wrap_width = text_max_width - padding;
                            let name_max_lines = 2;
                            
                            let id_galley = ui.painter().fonts_mut(|fonts| {
                                fonts.layout(
                                    id_text.clone(),
                                    egui::FontId::proportional(id_font_size),
                                    ui.visuals().text_color(),
                                    text_max_width,
                                )
                            });
                            
                            let id_height = id_galley.size().y;
                            
                            let name_available_height = text_max_height - id_height - 4.0;
                            let min_font_size = 10.0;
                            
                            let final_font_size = ui.painter().fonts_mut(|fonts| {
                                let mut current_font_size = font_size;
                                
                                while current_font_size >= min_font_size {
                                    let galley = fonts.layout(
                                        display_name.clone(),
                                        egui::FontId::proportional(current_font_size),
                                        ui.visuals().text_color(),
                                        actual_wrap_width,
                                    );
                                    
                                    let galley_rows = galley.rows.len();
                                    let galley_height = galley.size().y;
                                    
                                    if galley_rows <= name_max_lines && galley_height <= name_available_height {
                                        break;
                                    }
                                    
                                    current_font_size -= 1.0;
                                }
                                
                                current_font_size
                            });

                            // Draw inventory ID text
                            let id_text_start_y = card_rect.min.y + text_margin;
                            ui.painter().galley(
                                egui::Pos2::new(text_start_x, id_text_start_y),
                                id_galley,
                                ui.visuals().text_color(),
                            );

                            // Draw item name text
                            let name_text_start_y = id_text_start_y + id_height + 4.0;
                            let name_galley = ui.painter().fonts_mut(|fonts| {
                                fonts.layout(
                                    display_name.clone(),
                                    egui::FontId::proportional(final_font_size),
                                    ui.visuals().text_color(),
                                    actual_wrap_width,
                                )
                            });
                            
                            ui.painter().galley(
                                egui::Pos2::new(text_start_x, name_text_start_y),
                                name_galley,
                                ui.visuals().text_color(),
                            );

                            // Handle card click to open item detail window
                            if card_response.clicked() {
                                self.open_item_windows.insert(item.inventory);
                            }

                            if (i + 1) % items_per_row == 0 {
                                ui.end_row();
                            }
                        }
                    });

                ui.add_space(8.0);
            });

            // Show item detail windows for all open windows
            let open_windows = self.open_item_windows.clone();
            
            let mut pending_select_window_items: Option<Vec<(String, String, String)>> = None;
            
            for inventory_id in open_windows {
                if let Some(item) = self.inventory.items.iter().find(|i| i.inventory == inventory_id) {
                    let display_name = self.get_item_display_name(item);
                    let rarity_name = self.get_rarity_name(item.rarity);
                    let inventory_id_for_closure = inventory_id;
                    let mut window_open = true;
                    
                    let should_open_select_window = Rc::new(RefCell::new(false));
                    let select_window_open_clone = should_open_select_window.clone();
                    
                    let inventory_id_for_edit = inventory_id;
                    
                    if !self.edit_item_states.contains_key(&inventory_id) {
                        self.edit_item_states.insert(inventory_id, EditItemState {
                            level: item.level,
                            custom_name: item.custom_name.clone().unwrap_or_default(),
                        });
                    }
                    
                    let edit_state = self.edit_item_states.get(&inventory_id).cloned().unwrap_or_else(|| EditItemState {
                        level: item.level,
                        custom_name: item.custom_name.clone().unwrap_or_default(),
                    });
                    let mut edit_state = edit_state;
                    
                    egui::Window::new(format!("物品详情 - {}", display_name))
                        .id(egui::Id::new(format!("item_window_{}", inventory_id)))
                        .movable(true)
                        .collapsible(true)
                        .resizable(false)
                        .open(&mut window_open)
                        .show(ctx, |ui| {
                            let item_base_name = self.items_game.get_item_display_name(item.def_index, &self.translations);
                            
                            ui.horizontal(|ui| {
                                if ui.button("应用").clicked() {
                                    // TODO: Save changes without closing window
                                }
                                ui.add_space(10.0);
                                if ui.button("确定").clicked() {
                                    // TODO: Save changes and close window
                                }
                                ui.add_space(10.0);
                                if ui.button("取消").clicked() {
                                    // TODO: Discard changes and close window
                                }
                            });
                            
                            ui.separator();
                            
                            let table = TableBuilder::new(ui)
                                .striped(true)
                                .resizable(false)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::initial(100.0))
                                .column(Column::remainder())
                                .min_scrolled_height(0.0);
                            
                            table
                                 .body(|mut body| {
                                     body.row(30.0, |mut row| {
                                         row.col(|ui| {
                                             ui.label("物品");
                                         });
                                         row.col(|ui| {
                                             ui.horizontal(|ui| {
                                                 ui.label(format!("{}", item_base_name));
                                                 ui.label(format!("({})", item.def_index));
                                                 ui.add_space(10.0);
                                                 if ui.button("选择").clicked() {
                                                     let mut items: Vec<(String, String, String)> = self.items_game.items.iter()
                                                         .map(|(def_index, ig_item)| {
                                                             let display_name = ig_item.get_display_name(&self.translations);
                                                             (def_index.to_string(), display_name, def_index.to_string())
                                                         })
                                                         .collect();
                                                     items.sort_by_key(|(key, _, _)| key.parse::<u32>().unwrap_or(0));
                                                     pending_select_window_items = Some(items);
                                                     *select_window_open_clone.borrow_mut() = true;
                                                 }
                                             });
                                         });
                                     });
                                     
                                     body.row(30.0, |mut row| {
                                         row.col(|ui| {
                                             ui.label("等级");
                                         });
                                         row.col(|ui| {
                                             ui.add(egui::DragValue::new(&mut edit_state.level).range(0..=100));
                                         });
                                     });
                                     
                                     body.row(30.0, |mut row| {
                                         row.col(|ui| {
                                             ui.label("性质编号");
                                         });
                                         row.col(|ui| {
                                             ui.label(format!("{}", item.quality));
                                         });
                                     });
                                     
                                     body.row(30.0, |mut row| {
                                         row.col(|ui| {
                                             ui.label("稀有度");
                                         });
                                         row.col(|ui| {
                                             ui.label(format!("{} ({})", rarity_name, item.rarity));
                                         });
                                     });
                                     
                                     body.row(30.0, |mut row| {
                                         row.col(|ui| {
                                             ui.label("命名标签");
                                         });
                                         row.col(|ui| {
                                             ui.text_edit_singleline(&mut edit_state.custom_name);
                                         });
                                     });
                                 });
                            
                            ui.separator();
                            
                            ui.label("物品属性");
                            
                            let attr_table = TableBuilder::new(ui)
                                .striped(true)
                                .resizable(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::auto())
                                .column(Column::auto())
                                .column(Column::remainder())
                                .min_scrolled_height(150.0)
                                .sense(egui::Sense::click());
                            
                            attr_table
                                 .header(30.0, |mut header| {
                                     header.col(|ui| {
                                         ui.strong("属性索引");
                                     });
                                     header.col(|ui| {
                                         ui.strong("描述");
                                     });
                                     header.col(|ui| {
                                         ui.strong("值");
                                     });
                                 })
                                 .body(|body| {
                                     body.rows(30.0, 0, |_row| {});
                                 });
                            
                            self.edit_item_states.insert(inventory_id_for_edit, edit_state);
                        });
                    
                    if *should_open_select_window.borrow() {
                        if let Some(items) = pending_select_window_items.take() {
                            self.open_select_window(
                                "选择物品".to_string(),
                                "物品编号".to_string(),
                                "物品名称".to_string(),
                                items,
                            );
                        }
                    }
                    
                    if !window_open {
                        self.open_item_windows.remove(&inventory_id_for_closure);
                    }
                } else {
                    self.open_item_windows.remove(&inventory_id);
                }
            }

            let mut select_window_internal_open = self.select_window_open;
            egui::Window::new(&self.select_window_title)
                .id(egui::Id::new("select_window"))
                .open(&mut select_window_internal_open)
                .resizable(true)
                .collapsible(true)
                .movable(true)
                .show(ctx, |ui| {
                    
                    ui.horizontal(|ui| {
                        ui.label(&self.select_window_key_header);
                        ui.add_space(10.0);
                        ui.text_edit_singleline(&mut self.select_window_search);
                    });
                    
                    ui.separator();
                    
                    let search_query = self.select_window_search.clone();
                    let filtered_items: Vec<(usize, String, String, String)> = self.select_window_items.iter()
                        .enumerate()
                        .filter(|(_, (key, display, _))| {
                            if search_query.is_empty() {
                                true
                            } else {
                                key.contains(&search_query) || display.contains(&search_query)
                            }
                        })
                        .map(|(idx, (key, display, value))| (idx, key.clone(), display.clone(), value.clone()))
                        .collect();
                    
                    let text_height = egui::TextStyle::Body.resolve(ui.style()).size.max(ui.spacing().interact_size.y);
                    
                    let table = TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::auto())
                        .column(Column::remainder())
                        .min_scrolled_height(300.0)
                        .max_scroll_height(400.0)
                        .sense(egui::Sense::click());
                    
                    table
                        .header(text_height, |mut header| {
                            header.col(|ui| {
                                ui.strong(&self.select_window_key_header);
                            });
                            header.col(|ui| {
                                ui.strong(&self.select_window_value_header);
                            });
                        })
                        .body(|body| {
                            body.rows(text_height, filtered_items.len(), |mut row| {
                                let row_idx = row.index();
                                if let Some((idx, key, display, _)) = filtered_items.get(row_idx) {
                                    row.set_selected(self.select_window_selected == Some(*idx));
                                    
                                    row.col(|ui| {
                                        ui.label(key);
                                    });
                                    row.col(|ui| {
                                        ui.label(display);
                                    });
                                    
                                    if row.response().clicked() {
                                        self.select_window_selected = Some(*idx);
                                    }
                                }
                            });
                        });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("确认").clicked() {
                            if let Some(selected_idx) = self.select_window_selected {
                                if let Some((key, display, value)) = self.select_window_items.get(selected_idx) {
                                    println!("Selected: Key={}, Display={}, Value={}", key, display, value);
                                }
                                self.select_window_open = false;
                            }
                        }
                        ui.add_space(10.0);
                        if ui.button("取消").clicked() {
                            self.select_window_open = false;
                        }
                    });
                });

            self.select_window_open = select_window_internal_open;

            ui.heading("CSGO-GC Inventory Editor");

            let system_theme = ctx.input(|i| i.raw.system_theme);
            if let Some(theme) = system_theme {
                ui.label(format!("System theme: {:?}", theme));
            } else {
                ui.label("System theme: unknown");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSGO-GC Inventory Editor",
        options,
        Box::new(|cc| Ok(Box::new(CsgoInventoryEditor::new(cc)))),
    )
}
