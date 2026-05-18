use crate::app::CsgoInventoryEditor;
use eframe::egui;
use egui_i18n::tr;

pub fn draw_inventory_page(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    if state.has_load_errors() {
        let errors = state.get_load_errors();
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading(
                    egui::RichText::new(tr!("load-errors-title"))
                        .size(24.0)
                        .color(egui::Color32::RED),
                );
                ui.add_space(20.0);
                for error in errors {
                    ui.label(
                        egui::RichText::new(error)
                            .size(16.0)
                            .color(egui::Color32::LIGHT_RED),
                    );
                }
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(tr!("load-errors-help"))
                        .size(14.0)
                        .color(egui::Color32::GRAY),
                );
            });
        });
    } else {
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            crate::ui::draw_toolbar(ui, state);
        });

        crate::ui::draw_item_grid(ui, state);
    }
}
