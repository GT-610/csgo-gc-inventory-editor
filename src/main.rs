use eframe::egui;
use std::fs;
use std::sync::Arc;
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

struct CsgoInventoryEditor {
    inventory: Inventory,
    items_game: ItemsGame,
    translations: GameTranslation,
    selected_category: InventoryCategory,
    selected_subcategory: Option<String>,
    search_query: String,
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
        }
    }

    fn get_item_display_name(&self, item: &csgo_inventory_editor::inventory::Item) -> String {
        self.items_game.get_item_full_name(item, &self.translations)
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
                        for (i, item) in self.inventory.items.iter().enumerate() {
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

                            // Draw item name text with auto-wrap and font adjustment
                            let text_margin = 8.0;
                            let indicator_space = if rarity.color().is_some() { 10.0 } else { 4.0 };
                            
                            let text_start_x = card_rect.min.x + text_margin + indicator_space;
                            let text_start_y = card_rect.min.y + text_margin;
                            let text_max_width = card_width - 2.0 * text_margin - indicator_space;
                            let text_max_height = card_height - 2.0 * text_margin;
                            let max_lines = 2;

                            // Calculate proper font size that fits within 2 lines
                            let padding = 4.0; // Add extra padding to prevent overflow
                            let actual_wrap_width = text_max_width - padding;
                            
                            let final_font_size = ui.painter().fonts_mut(|fonts| {
                                let mut current_font_size = font_size;
                                let min_font_size = 10.0;
                                
                                while current_font_size >= min_font_size {
                                    let galley = fonts.layout(
                                        display_name.clone(),
                                        egui::FontId::proportional(current_font_size),
                                        ui.visuals().text_color(),
                                        actual_wrap_width,
                                    );
                                    
                                    let galley_rows = galley.rows.len();
                                    let galley_height = galley.size().y;
                                    
                                    if galley_rows <= max_lines && galley_height <= text_max_height {
                                        break;
                                    }
                                    
                                    current_font_size -= 1.0;
                                }
                                
                                current_font_size
                            });

                            // Draw wrapped text with calculated font size using galley
                            let galley = ui.painter().fonts_mut(|fonts| {
                                fonts.layout(
                                    display_name.clone(),
                                    egui::FontId::proportional(final_font_size),
                                    ui.visuals().text_color(),
                                    actual_wrap_width,
                                )
                            });
                            
                            ui.painter().galley(
                                egui::Pos2::new(text_start_x, text_start_y),
                                galley,
                                ui.visuals().text_color(),
                            );

                            if (i + 1) % items_per_row == 0 {
                                ui.end_row();
                            }
                        }
                    });

                ui.add_space(8.0);
            });

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
