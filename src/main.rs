pub mod core;
pub mod inventory;
pub mod ui;
pub mod app;

use eframe::egui;
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
    }
}
