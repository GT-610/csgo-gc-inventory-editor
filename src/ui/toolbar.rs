use eframe::egui;
use egui_i18n::tr;

use crate::app::{CsgoInventoryEditor, ItemTemplate};

pub fn draw_toolbar(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    ui.horizontal(|ui| {
        if ui.button(tr!("btn-add-item")).clicked() {
            state.show_template_modal = true;
            state.selected_template = Some(ItemTemplate::Empty);
        }

        if let Some(message) = &state.status_message {
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(message)
                    .color(egui::Color32::LIGHT_RED)
                    .size(13.0),
            );
        }
    });
    ui.add_space(4.0);
}
