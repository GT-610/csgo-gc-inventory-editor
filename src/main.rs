pub mod core;
pub mod inventory;
pub mod ui;
pub mod app;

use eframe::egui;
use egui_i18n::tr;
use crate::app::CsgoInventoryEditor;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSGO Inventory Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(CsgoInventoryEditor::new(cc)))),
    )
}

impl eframe::App for CsgoInventoryEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui::draw_toolbar(ui, self);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::draw_item_grid(ui, self);
        });
        
        let mut pending_select_window_items: Option<Vec<(String, String, String)>> = None;
        let mut select_window_open = self.select_window_open;
        
        ui::draw_item_detail_windows(ctx, self, &mut pending_select_window_items, &mut select_window_open);
        
        if self.select_window_open {
            ui::draw_select_window(
                ctx,
                &mut self.select_window_open,
                &self.select_window_title,
                &self.select_window_key_header,
                &self.select_window_value_header,
                &self.select_window_items,
                &mut self.select_window_search,
                &mut self.select_window_selected,
            );
        }
        
        if self.pending_add_item {
            self.pending_add_item = false;
            let mut items: Vec<(String, String, String)> = self.items_game.items.iter()
                .map(|(def_index, ig_item): (&u32, &crate::inventory::items_game::IGItem)| {
                    let display_name = ig_item.get_display_name(&self.translations);
                    (def_index.to_string(), display_name, def_index.to_string())
                })
                .collect();
            items.sort_by_key(|(key, _, _): &(String, String, String)| key.parse::<u32>().unwrap_or(0));
            self.open_select_window(
                tr!("select-item-to-add").to_string(),
                tr!("header-item-id").to_string(),
                tr!("header-item-name").to_string(),
                items,
            );
        }
        
        if let Some(selected_idx) = self.select_window_selected {
            if self.select_window_title == tr!("select-item-to-add") {
                if let Some((def_index_str, _, _)) = self.select_window_items.get(selected_idx) {
                    if let Ok(def_index) = def_index_str.parse::<u32>() {
                        let new_inventory_id = self.inventory.items.iter()
                            .map(|i| i.inventory)
                            .max()
                            .unwrap_or(0) + 1;
                        
                        let new_item = crate::inventory::Item {
                            inventory: new_inventory_id,
                            def_index,
                            level: 1,
                            quality: 4,
                            flags: 0,
                            origin: 0,
                            in_use: 0,
                            rarity: self.items_game.rarities.values()
                                .find(|r| r.value == 4)
                                .map(|r| r.value)
                                .unwrap_or(0),
                            custom_name: None,
                            attributes: std::collections::HashMap::new(),
                            equipped_state: std::collections::HashMap::new(),
                        };
                        
                        self.inventory.items.push(new_item);
                        self.open_item_windows.insert(new_inventory_id);
                        let _ = self.save_inventory();
                    }
                }
                self.select_window_open = false;
                self.select_window_selected = None;
            }
        }
    }
}
