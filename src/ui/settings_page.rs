use eframe::egui;
use egui_i18n::tr;

pub fn draw_settings_page(ui: &mut egui::Ui, state: &mut crate::app::CsgoInventoryEditor) {
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        
        ui.heading(tr!("settings-title"));
        ui.add_space(32.0);
        
        ui.horizontal(|ui| {
            ui.label(tr!("language-label"));
            let current_lang_display = if state.current_language == "zh-Hans" { "简体中文" } else { "English" };
            egui::ComboBox::from_id_salt("language_combo")
                .selected_text(current_lang_display)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.current_language, "en-US".to_string(), "English");
                    ui.selectable_value(&mut state.current_language, "zh-Hans".to_string(), "简体中文");
                });
            
            let current_lang = state.current_language.clone();
            if ui.button(tr!("btn-switch")).clicked() {
                state.switch_language(&current_lang);
            }
        });
        
        ui.add_space(64.0);
        
        ui.heading(tr!("about-title"));
        ui.add_space(32.0);
        
        ui.vertical_centered(|ui| {
            ui.label("CSGO GC Editor");
            ui.add_space(8.0);
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
        });
    });
}
