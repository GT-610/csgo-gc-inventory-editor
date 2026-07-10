use crate::app::CsgoInventoryEditor;
use eframe::egui;
use egui_i18n::tr;

pub fn draw_rcon_page(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    let connected = state.is_live_rcon();
    let mut connect_clicked = false;
    let mut disconnect_clicked = false;
    let mut send_raw_clicked = false;
    let mut quick_command: Option<&'static str> = None;
    let mut send_give_clicked = false;
    let mut remove_clicked = false;

    ui.heading(tr!("rcon-title"));
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        let status = if connected {
            tr!("rcon-status-connected")
        } else {
            tr!("rcon-status-disconnected")
        };
        ui.label(status);
    });

    ui.separator();

    ui.add_enabled_ui(!connected, |ui| {
        ui.horizontal(|ui| {
            ui.label(tr!("rcon-address"));
            ui.text_edit_singleline(&mut state.rcon_ui.address);
            ui.label(tr!("rcon-port"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.port).range(1..=65535));
        });
        ui.horizontal(|ui| {
            ui.label(tr!("rcon-password"));
            ui.add(egui::TextEdit::singleline(&mut state.rcon_ui.password).password(true));
        });
    });

    ui.horizontal(|ui| {
        if ui
            .add_enabled(!connected, egui::Button::new(tr!("rcon-connect")))
            .clicked()
        {
            connect_clicked = true;
        }
        if ui
            .add_enabled(connected, egui::Button::new(tr!("rcon-disconnect")))
            .clicked()
        {
            disconnect_clicked = true;
        }
    });

    ui.separator();
    ui.label(tr!("rcon-raw-command"));
    ui.horizontal(|ui| {
        ui.add_enabled(
            connected,
            egui::TextEdit::singleline(&mut state.rcon_ui.command_input)
                .desired_width(f32::INFINITY),
        );
        if ui
            .add_enabled(connected, egui::Button::new(tr!("rcon-send")))
            .clicked()
        {
            send_raw_clicked = true;
        }
    });

    ui.horizontal(|ui| {
        for (label, command) in [
            (tr!("rcon-ping"), "ping"),
            (tr!("rcon-status"), "status"),
            (tr!("rcon-help"), "help"),
            (tr!("rcon-clients"), "clients"),
            (tr!("rcon-refresh-inventory"), "refresh_inventory"),
        ] {
            if ui
                .add_enabled(connected, egui::Button::new(label))
                .clicked()
            {
                quick_command = Some(command);
            }
        }
    });

    ui.separator();
    ui.label(tr!("rcon-give-item"));
    ui.add_enabled_ui(connected, |ui| {
        ui.horizontal(|ui| {
            ui.label(tr!("rcon-defindex"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.give_def_index).range(0..=u32::MAX));
            ui.label(tr!("rcon-count"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.give_count).range(1..=100));
            ui.label(tr!("level"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.give_level).range(0..=100));
        });
        ui.horizontal(|ui| {
            ui.label(tr!("quality-id"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.give_quality).range(0..=u32::MAX));
            ui.label(tr!("rarity"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.give_rarity).range(0..=u32::MAX));
        });
        ui.horizontal(|ui| {
            ui.label(tr!("custom-name"));
            ui.text_edit_singleline(&mut state.rcon_ui.give_custom_name);
        });
        ui.horizontal(|ui| {
            ui.label(tr!("rcon-paint"));
            ui.text_edit_singleline(&mut state.rcon_ui.give_paint);
            ui.label(tr!("rcon-seed"));
            ui.text_edit_singleline(&mut state.rcon_ui.give_seed);
            ui.label(tr!("rcon-wear"));
            ui.text_edit_singleline(&mut state.rcon_ui.give_wear);
            ui.label(tr!("rcon-stattrak"));
            ui.text_edit_singleline(&mut state.rcon_ui.give_stattrak);
        });
        if ui.button(tr!("rcon-give")).clicked() {
            send_give_clicked = true;
        }
    });

    ui.separator();
    ui.label(tr!("rcon-remove-item"));
    ui.horizontal(|ui| {
        ui.add_enabled(
            connected,
            egui::TextEdit::singleline(&mut state.rcon_ui.remove_item_id)
                .hint_text(tr!("rcon-item-id")),
        );
        if ui
            .add_enabled(connected, egui::Button::new(tr!("rcon-remove")))
            .clicked()
        {
            remove_clicked = true;
        }
    });

    ui.separator();
    ui.label(tr!("rcon-last-response"));
    ui.monospace(&state.rcon_ui.last_response);
    ui.add_space(8.0);
    ui.label(tr!("rcon-log"));
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for line in &state.rcon_ui.log {
                ui.monospace(line);
            }
        });

    if connect_clicked {
        state.connect_rcon();
    }
    if disconnect_clicked {
        state.disconnect_rcon();
    }
    if send_raw_clicked {
        let command = state.rcon_ui.command_input.clone();
        state.send_rcon_command(&command);
    }
    if let Some(command) = quick_command {
        state.send_rcon_command(command);
    }
    if send_give_clicked {
        match build_manual_give_command(state) {
            Ok(command) => state.send_rcon_command(&command),
            Err(e) => state.push_rcon_log(format!("ERR {}", e)),
        }
    }
    if remove_clicked {
        match state.rcon_ui.remove_item_id.trim().parse::<u64>() {
            Ok(item_id) => {
                let command = crate::rcon::commands::build_remove_item_command(item_id);
                state.send_rcon_command(&command);
            }
            Err(_) => state.push_rcon_log("ERR invalid item id".to_string()),
        }
    }
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
