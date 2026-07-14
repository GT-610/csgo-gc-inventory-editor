use eframe::egui;
use egui_i18n::tr;

use crate::app::{CsgoInventoryEditor, ItemTemplate};

pub fn draw_toolbar(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                !state.is_live_rcon(),
                egui::Button::new(tr!("btn-add-item")),
            )
            .clicked()
        {
            state.show_template_modal = true;
            state.selected_template = Some(ItemTemplate::Empty);
        }

        if let Some(message) = &state.status_message {
            ui.add_space(12.0);
            crate::ui::draw_status_message(ui, message);
        }
    });
    ui.add_space(4.0);
}
