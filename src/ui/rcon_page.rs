use crate::app::{CsgoInventoryEditor, SelectWindowPurpose};
use crate::inventory::get_attribute_value_display_name;
use eframe::egui;
use egui_i18n::tr;

pub fn draw_rcon_page(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    let connected = state.is_live_rcon();
    let connecting = state.is_connecting_rcon();
    let can_send = connected && !connecting;
    let mut actions = RconPageActions::default();

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("RCON");
        ui.add_space(8.0);
        draw_status(ui, connected, connecting);
        ui.separator();
        draw_connection(ui, state, connected, connecting, &mut actions);
        ui.separator();
        draw_raw_command(ui, state, can_send, &mut actions);
        ui.separator();
        draw_give_item(ui, state, can_send, &mut actions);
        ui.separator();
        draw_remove_item(ui, state, can_send, &mut actions);
        ui.separator();
        draw_response_and_log(ui, state);
    });

    handle_actions(state, actions);
}

#[derive(Default)]
struct RconPageActions {
    connect: bool,
    disconnect: bool,
    send_raw: bool,
    quick_command: Option<&'static str>,
    send_give: bool,
    remove: bool,
    select_item: bool,
    select_paint: bool,
}

fn draw_status(ui: &mut egui::Ui, connected: bool, connecting: bool) {
    let status = if connecting {
        "Connecting..."
    } else if connected {
        "Connected. Offline files are read-only."
    } else {
        "Disconnected. Offline editing is available."
    };
    ui.label(status);
}

fn draw_connection(
    ui: &mut egui::Ui,
    state: &mut CsgoInventoryEditor,
    connected: bool,
    connecting: bool,
    actions: &mut RconPageActions,
) {
    ui.add_enabled_ui(!connected && !connecting, |ui| {
        egui::Grid::new("rcon_connection_grid")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .show(ui, |ui| {
                ui.label("Address:");
                ui.add(
                    egui::TextEdit::singleline(&mut state.rcon_ui.address)
                        .desired_width(ui.available_width().min(420.0)),
                );
                ui.end_row();

                ui.label("Port:");
                ui.add(egui::DragValue::new(&mut state.rcon_ui.port).range(1..=65535));
                ui.end_row();

                ui.label("Password:");
                ui.add(
                    egui::TextEdit::singleline(&mut state.rcon_ui.password)
                        .password(true)
                        .desired_width(ui.available_width().min(420.0)),
                );
                ui.end_row();
            });
    });

    ui.horizontal_wrapped(|ui| {
        if ui
            .add_enabled(!connected && !connecting, egui::Button::new("Connect"))
            .clicked()
        {
            actions.connect = true;
        }
        if ui
            .add_enabled(connected || connecting, egui::Button::new("Disconnect"))
            .clicked()
        {
            actions.disconnect = true;
        }
        if connecting {
            ui.spinner();
        }
    });
}

fn draw_raw_command(
    ui: &mut egui::Ui,
    state: &mut CsgoInventoryEditor,
    can_send: bool,
    actions: &mut RconPageActions,
) {
    ui.label("Raw command");
    ui.horizontal(|ui| {
        let input_width = (ui.available_width() - 72.0).max(160.0);
        ui.add_enabled(
            can_send,
            egui::TextEdit::singleline(&mut state.rcon_ui.command_input).desired_width(input_width),
        );
        if ui
            .add_enabled(can_send, egui::Button::new("Send"))
            .clicked()
        {
            actions.send_raw = true;
        }
    });

    ui.horizontal_wrapped(|ui| {
        for (label, command) in [
            ("Ping", "ping"),
            ("Status", "status"),
            ("Help", "help"),
            ("Clients", "clients"),
            ("Refresh Inventory", "refresh_inventory"),
        ] {
            if ui.add_enabled(can_send, egui::Button::new(label)).clicked() {
                actions.quick_command = Some(command);
            }
        }
    });
}

fn draw_give_item(
    ui: &mut egui::Ui,
    state: &mut CsgoInventoryEditor,
    can_send: bool,
    actions: &mut RconPageActions,
) {
    ui.label("Give Item");
    ui.add_enabled_ui(can_send, |ui| {
        egui::Grid::new("rcon_give_grid")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .show(ui, |ui| {
                ui.label("Item:");
                ui.horizontal_wrapped(|ui| {
                    ui.label(selected_item_label(state));
                    if ui.button(tr!("btn-select")).clicked() {
                        actions.select_item = true;
                    }
                });
                ui.end_row();

                ui.label("Count / Level:");
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::DragValue::new(&mut state.rcon_ui.give_count).range(1..=100));
                    ui.label(tr!("level"));
                    ui.add(egui::DragValue::new(&mut state.rcon_ui.give_level).range(0..=100));
                });
                ui.end_row();

                ui.label(tr!("quality-id"));
                draw_quality_combo(ui, state);
                ui.end_row();

                ui.label(tr!("rarity"));
                draw_rarity_combo(ui, state);
                ui.end_row();

                ui.label(tr!("custom-name"));
                ui.add(
                    egui::TextEdit::singleline(&mut state.rcon_ui.give_custom_name)
                        .desired_width(ui.available_width().min(520.0)),
                );
                ui.end_row();

                ui.label("Paint:");
                ui.horizontal_wrapped(|ui| {
                    ui.label(selected_paint_label(state));
                    if ui.button(tr!("btn-select")).clicked() {
                        actions.select_paint = true;
                    }
                });
                ui.end_row();

                ui.label("Seed / Wear / StatTrak:");
                ui.horizontal_wrapped(|ui| {
                    labeled_text(ui, "Seed", &mut state.rcon_ui.give_seed);
                    labeled_text(ui, "Wear", &mut state.rcon_ui.give_wear);
                    labeled_text(ui, "StatTrak", &mut state.rcon_ui.give_stattrak);
                });
                ui.end_row();
            });

        if ui.button("Give").clicked() {
            actions.send_give = true;
        }
    });
}

fn draw_quality_combo(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    let selected_quality = state
        .get_cached_quality_names()
        .iter()
        .find(|(value, _)| *value == state.rcon_ui.give_quality)
        .map(|(_, name)| name.clone())
        .unwrap_or_else(|| format!("Unknown ({})", state.rcon_ui.give_quality));
    let qualities = state.get_cached_quality_names().to_vec();
    egui::ComboBox::from_id_salt("rcon_quality_combo")
        .width(ui.available_width().min(320.0))
        .selected_text(format!(
            "{} ({})",
            selected_quality, state.rcon_ui.give_quality
        ))
        .show_ui(ui, |ui| {
            for (value, name) in qualities {
                ui.selectable_value(
                    &mut state.rcon_ui.give_quality,
                    value,
                    format!("{} ({})", name, value),
                );
            }
        });
}

fn draw_rarity_combo(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    let selected_rarity = state
        .get_cached_rarity_names()
        .iter()
        .find(|(value, _)| *value == state.rcon_ui.give_rarity)
        .map(|(_, name)| name.clone())
        .unwrap_or_else(|| format!("Unknown ({})", state.rcon_ui.give_rarity));
    let rarities = state.get_cached_rarity_names().to_vec();
    egui::ComboBox::from_id_salt("rcon_rarity_combo")
        .width(ui.available_width().min(320.0))
        .selected_text(format!(
            "{} ({})",
            selected_rarity, state.rcon_ui.give_rarity
        ))
        .show_ui(ui, |ui| {
            for (value, name) in rarities {
                ui.selectable_value(
                    &mut state.rcon_ui.give_rarity,
                    value,
                    format!("{} ({})", name, value),
                );
            }
        });
}

fn labeled_text(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.label(format!("{}:", label));
    ui.add(egui::TextEdit::singleline(value).desired_width(120.0));
}

fn draw_remove_item(
    ui: &mut egui::Ui,
    state: &mut CsgoInventoryEditor,
    can_send: bool,
    actions: &mut RconPageActions,
) {
    ui.label("Remove Item");
    ui.horizontal_wrapped(|ui| {
        ui.add_enabled(
            can_send,
            egui::TextEdit::singleline(&mut state.rcon_ui.remove_item_id)
                .hint_text("Item ID")
                .desired_width(ui.available_width().min(420.0)),
        );
        if ui
            .add_enabled(can_send, egui::Button::new("Remove"))
            .clicked()
        {
            actions.remove = true;
        }
    });
}

fn draw_response_and_log(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    ui.label("Last Response");
    ui.monospace(&state.rcon_ui.last_response);
    ui.separator();

    egui::CollapsingHeader::new("Log")
        .default_open(false)
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .max_height(180.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for line in &state.rcon_ui.log {
                        ui.monospace(line);
                    }
                });
        });
}

fn handle_actions(state: &mut CsgoInventoryEditor, actions: RconPageActions) {
    let language = state.current_language.clone();

    if actions.connect {
        state.connect_rcon();
    }
    if actions.disconnect {
        state.disconnect_rcon();
    }
    if actions.select_item {
        let items = state.create_item_select_list();
        state.open_select_window(
            SelectWindowPurpose::RconItemDef,
            text(&language, "选择物品", "Select Item").to_string(),
            tr!("header-item-id").to_string(),
            tr!("header-item-name").to_string(),
            items,
        );
    }
    if actions.select_paint {
        let items = state.create_skin_select_list_for_weapon(state.rcon_ui.give_def_index);
        state.open_select_window(
            SelectWindowPurpose::RconPaintKit,
            text(&language, "选择涂装", "Select Paint Kit").to_string(),
            tr!("header-paintkit-id").to_string(),
            tr!("header-paintkit-name").to_string(),
            items,
        );
    }
    if actions.send_raw {
        let command = state.rcon_ui.command_input.clone();
        state.send_rcon_command(&command);
    }
    if let Some(command) = actions.quick_command {
        state.send_rcon_command(command);
    }
    if actions.send_give {
        match build_manual_give_command(state) {
            Ok(command) => state.send_rcon_command(&command),
            Err(e) => state.push_rcon_log(format!("ERR {}", e)),
        }
    }
    if actions.remove {
        match state.rcon_ui.remove_item_id.trim().parse::<u64>() {
            Ok(item_id) => {
                let command = crate::rcon::commands::build_remove_item_command(item_id);
                state.send_rcon_command(&command);
            }
            Err(_) => state.push_rcon_log("ERR invalid item id".to_string()),
        }
    }
}

fn selected_item_label(state: &CsgoInventoryEditor) -> String {
    let name = state
        .items_game
        .get_item_display_name(state.rcon_ui.give_def_index, &state.translations);
    format!("{} ({})", name, state.rcon_ui.give_def_index)
}

fn selected_paint_label(state: &CsgoInventoryEditor) -> String {
    if state.rcon_ui.give_paint.trim().is_empty() {
        return "-".to_string();
    }
    get_attribute_value_display_name(
        crate::inventory::ItemAttribute::SkinPaintIndex.id(),
        &state.rcon_ui.give_paint,
        &state.items_game,
        &state.translations,
    )
}

fn build_manual_give_command(state: &CsgoInventoryEditor) -> Result<String, String> {
    let count = state.rcon_ui.give_count;
    if !(1..=100).contains(&count) {
        return Err("count must be between 1 and 100".to_string());
    }

    let mut parts = vec![
        "give_item".to_string(),
        state.rcon_ui.give_def_index.to_string(),
    ];
    if count != 1 {
        parts.push(count.to_string());
    }
    parts.push(format!("level={}", state.rcon_ui.give_level));
    parts.push(format!("quality={}", state.rcon_ui.give_quality));
    parts.push(format!("rarity={}", state.rcon_ui.give_rarity));

    if !state.rcon_ui.give_custom_name.is_empty() {
        parts.push(format!(
            "name={}",
            crate::rcon::commands::quote_value(&state.rcon_ui.give_custom_name)
        ));
    }
    push_optional_u32(&mut parts, "paint", &state.rcon_ui.give_paint)?;
    push_optional_u32(&mut parts, "seed", &state.rcon_ui.give_seed)?;
    push_optional_f32(&mut parts, "wear", &state.rcon_ui.give_wear)?;
    push_optional_u32(&mut parts, "stattrak", &state.rcon_ui.give_stattrak)?;

    Ok(parts.join(" "))
}

fn push_optional_u32(parts: &mut Vec<String>, key: &str, value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    let parsed = trimmed
        .parse::<u32>()
        .map_err(|_| format!("invalid parameter {}", key))?;
    parts.push(format!("{}={}", key, parsed));
    Ok(())
}

fn push_optional_f32(parts: &mut Vec<String>, key: &str, value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    let parsed = trimmed
        .parse::<f32>()
        .map_err(|_| format!("invalid parameter {}", key))?;
    parts.push(format!("{}={}", key, parsed));
    Ok(())
}

fn text<'a>(language: &str, zh: &'a str, en: &'a str) -> &'a str {
    if language.starts_with("zh") { zh } else { en }
}
