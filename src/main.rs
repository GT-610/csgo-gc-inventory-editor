use eframe::egui;
use std::fs;
use std::sync::Arc;
use csgo_inventory_editor::inventory::{Inventory, InventoryLoader};
use csgo_inventory_editor::core::GameDir;

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
        
        Self {
            inventory,
            selected_category: InventoryCategory::All,
            selected_subcategory: None,
            search_query: String::new(),
        }
    }
}

impl Default for CsgoInventoryEditor {
    fn default() -> Self {
        Self {
            inventory: Inventory::default(),
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
            let card_width = 80.0;
            let card_height = 100.0;
            let spacing = 8.0;
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(8.0);
                
                egui::Grid::new("item_grid")
                    .num_columns(items_per_row)
                    .spacing([spacing, spacing])
                    .min_col_width(card_width)
                    .min_row_height(card_height)
                    .show(ui, |ui| {
                        for (i, item) in self.inventory.items.iter().enumerate() {
                            let button = egui::Button::new(format!("#{}", item.def_index))
                                .min_size(egui::Vec2::new(card_width, card_height))
                                .wrap();
                            
                            if ui.add(button).clicked() {
                            }
                            
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
