pub mod core;
pub mod inventory;
pub mod ui;
pub mod app;

use eframe::egui;
use egui_i18n::tr;
use crate::app::{CsgoInventoryEditor, ItemTemplate};

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
        
        if let Some(inventory_id) = self.pending_paint_kit_select.take() {
            self.pending_paint_kit_select = None;
            let mut items: Vec<(String, String, String)> = self.items_game.paint_kits.iter()
                .map(|(paint_index, paint_kit): (&u32, &crate::inventory::items_game::IGPaintKit)| {
                    let display_name = paint_kit.get_display_name(&self.translations);
                    (paint_index.to_string(), display_name, paint_index.to_string())
                })
                .collect();
            items.sort_by_key(|(key, _, _): &(String, String, String)| key.parse::<u32>().unwrap_or(0));
            self.open_select_window(
                tr!("select-paintkit").to_string(),
                tr!("header-paintkit-id").to_string(),
                tr!("header-paintkit-name").to_string(),
                items,
            );
            self.select_window_for_item = Some(inventory_id);
        }
        
        if let Some(selected_idx) = self.select_window_selected {
            if self.select_window_title == tr!("select-item-to-add") {
                if let Some((def_index_str, _, _)) = self.select_window_items.get(selected_idx) {
                    if let Ok(def_index) = def_index_str.parse::<u32>() {
                        let new_inventory_id = self.inventory.items.iter()
                            .map(|i| i.inventory)
                            .max()
                            .unwrap_or(0) + 1;
                        
                        let template = self.selected_template.unwrap_or(ItemTemplate::Empty);
                        let mut new_item = template.create_item(def_index);
                        new_item.inventory = new_inventory_id;
                        
                        self.inventory.items.push(new_item);
                        self.open_item_windows.insert(new_inventory_id);
                        let _ = self.save_inventory();
                    }
                }
                self.select_window_open = false;
                self.select_window_selected = None;
                self.selected_template = None;
            }
            
            if self.select_window_title == tr!("select-paintkit") {
                if let Some(for_item_id) = self.select_window_for_item {
                    if let Some((paint_index_str, _, _)) = self.select_window_items.get(selected_idx) {
                        if let Some(item) = self.inventory.items.iter_mut().find(|i| i.inventory == for_item_id) {
                            item.attributes.insert(6, paint_index_str.clone());
                        }
                        if let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id) {
                            edit_state.attributes.insert(6, paint_index_str.clone());
                        }
                        let _ = self.save_inventory();
                    }
                }
                self.select_window_open = false;
                self.select_window_selected = None;
                self.select_window_for_item = None;
            }
        }
    }
}
