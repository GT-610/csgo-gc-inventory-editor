use crate::app::CsgoInventoryEditor;
use eframe::egui;

pub fn draw_inventory_page(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    egui::TopBottomPanel::top("toolbar").show_inside(ui, |ui| {
        crate::ui::draw_toolbar(ui, state);
    });

    crate::ui::draw_item_grid(ui, state);
}
