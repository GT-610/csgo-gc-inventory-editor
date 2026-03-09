use eframe::egui;
use egui_i18n::tr;

pub fn draw_sidebar(ui: &mut egui::Ui, state: &mut crate::app::CsgoInventoryEditor) {
    ui.vertical(|ui| {
        ui.add_space(8.0);

        if ui.button(tr!("sidebar-inventory")).clicked() {
            state.current_page = crate::app::Page::Inventory;
        }

        ui.add_space(8.0);

        if ui.button(tr!("sidebar-settings")).clicked() {
            state.current_page = crate::app::Page::Settings;
        }

        ui.add_space(8.0);
    });
}
