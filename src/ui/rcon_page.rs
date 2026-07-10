use crate::app::{CsgoInventoryEditor, SelectWindowPurpose};
use crate::inventory::get_attribute_value_display_name;
use eframe::egui;
use egui_i18n::tr;

pub fn draw_rcon_page(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    let language = state.current_language.clone();
    let connected = state.is_live_rcon();
    let connecting = state.is_connecting_rcon();
    let can_send = connected && !connecting;
    let mut connect_clicked = false;
    let mut disconnect_clicked = false;
    let mut send_raw_clicked = false;
    let mut quick_command: Option<&'static str> = None;
    let mut send_give_clicked = false;
    let mut remove_clicked = false;
    let mut select_item_clicked = false;
    let mut select_paint_clicked = false;

    ui.heading("RCON");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        let status = if connecting {
            text(&language, "正在连接...", "Connecting...")
        } else if connected {
            text(
                &language,
                "已连接，离线文件为只读。",
                "Connected. Offline files are read-only.",
            )
        } else {
            text(
                &language,
                "未连接，可以进行离线编辑。",
                "Disconnected. Offline editing is available.",
            )
        };
        ui.label(status);
    });

    ui.separator();

    ui.add_enabled_ui(!connected && !connecting, |ui| {
        ui.horizontal(|ui| {
            ui.label(text(&language, "地址:", "Address:"));
            ui.text_edit_singleline(&mut state.rcon_ui.address);
            ui.label(text(&language, "端口:", "Port:"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.port).range(1..=65535));
        });
        ui.horizontal(|ui| {
            ui.label(text(&language, "密码:", "Password:"));
            ui.add(egui::TextEdit::singleline(&mut state.rcon_ui.password).password(true));
        });
    });

    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                !connected && !connecting,
                egui::Button::new(text(&language, "连接", "Connect")),
            )
            .clicked()
        {
            connect_clicked = true;
        }
        if ui
            .add_enabled(
                connected || connecting,
                egui::Button::new(text(&language, "断开", "Disconnect")),
            )
            .clicked()
        {
            disconnect_clicked = true;
        }
        if connecting {
            ui.spinner();
        }
    });

    ui.separator();
    ui.label(text(&language, "原始命令", "Raw command"));
    ui.horizontal(|ui| {
        ui.add_enabled(
            can_send,
            egui::TextEdit::singleline(&mut state.rcon_ui.command_input)
                .desired_width(f32::INFINITY),
        );
        if ui
            .add_enabled(can_send, egui::Button::new(text(&language, "发送", "Send")))
            .clicked()
        {
            send_raw_clicked = true;
        }
    });

    ui.horizontal(|ui| {
        for (label, command) in [
            ("Ping", "ping"),
            (text(&language, "状态", "Status"), "status"),
            (text(&language, "帮助", "Help"), "help"),
            (text(&language, "客户端", "Clients"), "clients"),
            (
                text(&language, "刷新库存", "Refresh Inventory"),
                "refresh_inventory",
            ),
        ] {
            if ui.add_enabled(can_send, egui::Button::new(label)).clicked() {
                quick_command = Some(command);
            }
        }
    });

    ui.separator();
    ui.label(text(&language, "发送物品", "Give Item"));
    ui.add_enabled_ui(can_send, |ui| {
        ui.horizontal(|ui| {
            ui.label(text(&language, "物品:", "Item:"));
            ui.label(selected_item_label(state));
            if ui.button(tr!("btn-select")).clicked() {
                select_item_clicked = true;
            }
            ui.label(text(&language, "数量:", "Count:"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.give_count).range(1..=100));
            ui.label(tr!("level"));
            ui.add(egui::DragValue::new(&mut state.rcon_ui.give_level).range(0..=100));
        });

        ui.horizontal(|ui| {
            ui.label(tr!("quality-id"));
            let selected_quality = state
                .get_cached_quality_names()
                .iter()
                .find(|(value, _)| *value == state.rcon_ui.give_quality)
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| format!("Unknown ({})", state.rcon_ui.give_quality));
            let qualities = state.get_cached_quality_names().to_vec();
            egui::ComboBox::from_id_salt("rcon_quality_combo")
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

            ui.label(tr!("rarity"));
            let selected_rarity = state
                .get_cached_rarity_names()
                .iter()
                .find(|(value, _)| *value == state.rcon_ui.give_rarity)
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| format!("Unknown ({})", state.rcon_ui.give_rarity));
            let rarities = state.get_cached_rarity_names().to_vec();
            egui::ComboBox::from_id_salt("rcon_rarity_combo")
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
        });

        ui.horizontal(|ui| {
            ui.label(tr!("custom-name"));
            ui.text_edit_singleline(&mut state.rcon_ui.give_custom_name);
        });

        ui.horizontal(|ui| {
            ui.label(text(&language, "涂装:", "Paint:"));
            ui.label(selected_paint_label(state));
            if ui.button(tr!("btn-select")).clicked() {
                select_paint_clicked = true;
            }
            ui.label(text(&language, "模板:", "Seed:"));
            ui.text_edit_singleline(&mut state.rcon_ui.give_seed);
            ui.label(text(&language, "磨损:", "Wear:"));
            ui.text_edit_singleline(&mut state.rcon_ui.give_wear);
            ui.label("StatTrak:");
            ui.text_edit_singleline(&mut state.rcon_ui.give_stattrak);
        });

        if ui.button(text(&language, "发送物品", "Give")).clicked() {
            send_give_clicked = true;
        }
    });

    ui.separator();
    ui.label(text(&language, "移除物品", "Remove Item"));
    ui.horizontal(|ui| {
        ui.add_enabled(
            can_send,
            egui::TextEdit::singleline(&mut state.rcon_ui.remove_item_id).hint_text(text(
                &language,
                "物品 ID",
                "Item ID",
            )),
        );
        if ui
            .add_enabled(
                can_send,
                egui::Button::new(text(&language, "移除", "Remove")),
            )
            .clicked()
        {
            remove_clicked = true;
        }
    });

    ui.separator();
    ui.label(text(&language, "最后响应", "Last Response"));
    ui.monospace(&state.rcon_ui.last_response);
    ui.add_space(8.0);
    ui.label(text(&language, "日志", "Log"));
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
    if select_item_clicked {
        let items = state.create_item_select_list();
        state.open_select_window(
            SelectWindowPurpose::RconItemDef,
            text(&language, "选择物品", "Select Item").to_string(),
            tr!("header-item-id").to_string(),
            tr!("header-item-name").to_string(),
            items,
        );
    }
    if select_paint_clicked {
        let items = state.create_skin_select_list_for_weapon(state.rcon_ui.give_def_index);
        state.open_select_window(
            SelectWindowPurpose::RconPaintKit,
            text(&language, "选择涂装", "Select Paint Kit").to_string(),
            tr!("header-paintkit-id").to_string(),
            tr!("header-paintkit-name").to_string(),
            items,
        );
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

fn text(language: &str, zh: &'static str, en: &'static str) -> &'static str {
    if language == "zh-Hans" { zh } else { en }
}
