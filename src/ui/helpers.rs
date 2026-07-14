use eframe::egui;

pub(crate) fn is_chinese(language: &str) -> bool {
    language == "zh-Hans"
}

pub(crate) fn rcon_readonly_message(language: &str) -> &'static str {
    if is_chinese(language) {
        "RCON 已连接。断开前 inventory.txt 和 config.txt 为只读。"
    } else {
        "RCON is connected. inventory.txt and config.txt are read-only until you disconnect."
    }
}

pub(crate) fn draw_named_combo(
    ui: &mut egui::Ui,
    id_salt: impl Into<String>,
    cache: &[(u32, String)],
    selected: &mut u32,
    read_only: bool,
    width: Option<f32>,
) {
    let selected_name = cache
        .iter()
        .find(|(v, _)| *v == *selected)
        .map(|(_, name)| name.clone())
        .unwrap_or_else(|| format!("Unknown ({})", *selected));

    let mut combo = egui::ComboBox::from_id_salt(id_salt.into())
        .selected_text(format!("{} ({})", selected_name, *selected));
    if let Some(w) = width {
        combo = combo.width(w);
    }

    ui.add_enabled_ui(!read_only, |ui| {
        combo.show_ui(ui, |ui| {
            for (value, name) in cache {
                ui.selectable_value(selected, *value, format!("{} ({})", name, value));
            }
        });
    });
}

pub(crate) fn draw_status_message(ui: &mut egui::Ui, message: &str) {
    let status_label = egui::Label::new(
        egui::RichText::new(message)
            .color(egui::Color32::LIGHT_RED)
            .size(13.0),
    )
    .truncate();
    ui.add_sized(
        egui::vec2(
            ui.available_width(),
            ui.text_style_height(&egui::TextStyle::Body),
        ),
        status_label,
    );
}
