use eframe::egui;
use egui_i18n::tr;

use crate::app::{CsgoInventoryEditor, SettingsPage};

pub fn draw_settings_page(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    let config_title = tr!("config-title");
    let settings_title = tr!("settings-title");
    let about_title = tr!("about-title");

    ui.horizontal(|ui| {
        let pages = [
            (SettingsPage::Config, config_title.as_str()),
            (SettingsPage::Settings, settings_title.as_str()),
            (SettingsPage::About, about_title.as_str()),
        ];

        for (page, label) in pages {
            let is_selected = state.current_settings_page == page;
            if ui.selectable_label(is_selected, label).clicked() {
                state.current_settings_page = page;
            }
        }
    });

    ui.separator();

    if let Some(message) = &state.status_message {
        ui.label(
            egui::RichText::new(message)
                .color(egui::Color32::LIGHT_RED)
                .size(13.0),
        );
        ui.separator();
    }

    match state.current_settings_page {
        SettingsPage::Config => {
            draw_config_page(ui, state);
        }
        SettingsPage::Settings => {
            draw_settings_content(ui, state);
        }
        SettingsPage::About => {
            draw_about_page(ui);
        }
    }
}

fn draw_config_page(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    let read_only = state.is_live_rcon();
    ui.vertical_centered(|ui| {
        if read_only {
            ui.label(
                egui::RichText::new(tr!("readonly-rcon-message")).color(egui::Color32::YELLOW),
            );
            ui.add_space(8.0);
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_enabled_ui(!read_only, |ui| {
                ui.horizontal(|ui| {
                    ui.label(tr!("appid-override"));
                    ui.add(egui::DragValue::new(&mut state.config.appid_override));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("competitive-rank"));
                    ui.add(egui::DragValue::new(&mut state.config.competitive_rank));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("competitive-wins"));
                    ui.add(egui::DragValue::new(&mut state.config.competitive_wins));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("wingman-rank"));
                    ui.add(egui::DragValue::new(&mut state.config.wingman_rank));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("wingman-wins"));
                    ui.add(egui::DragValue::new(&mut state.config.wingman_wins));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("dangerzone-rank"));
                    ui.add(egui::DragValue::new(&mut state.config.dangerzone_rank));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("dangerzone-wins"));
                    ui.add(egui::DragValue::new(&mut state.config.dangerzone_wins));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("vac-banned"));
                    if ui.checkbox(&mut state.config.vac_banned, "").changed() {
                        let result = state.save_config();
                        state.record_result(result, "save config");
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("cmd-friendly"));
                    ui.add(egui::DragValue::new(&mut state.config.cmd_friendly));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("cmd-teaching"));
                    ui.add(egui::DragValue::new(&mut state.config.cmd_teaching));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("cmd-leader"));
                    ui.add(egui::DragValue::new(&mut state.config.cmd_leader));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("player-level"));
                    ui.add(egui::DragValue::new(&mut state.config.player_level));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("player-cur-xp"));
                    ui.add(egui::DragValue::new(&mut state.config.player_cur_xp));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("destroy-used-items"));
                    if ui
                        .checkbox(&mut state.config.destroy_used_items, "")
                        .changed()
                    {
                        let result = state.save_config();
                        state.record_result(result, "save config");
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("show-csgo-gc-servers-only"));
                    if ui
                        .checkbox(&mut state.config.show_csgo_gc_servers_only, "")
                        .changed()
                    {
                        let result = state.save_config();
                        state.record_result(result, "save config");
                    }
                });
                ui.separator();
                ui.label(tr!("config-rcon-title"));
                ui.horizontal(|ui| {
                    ui.label(tr!("config-rcon-enabled"));
                    if ui.checkbox(&mut state.config.rcon_enabled, "").changed() {
                        let _ = state.save_config();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("config-rcon-bind-address"));
                    ui.text_edit_singleline(&mut state.config.rcon_bind_address);
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("config-rcon-port"));
                    ui.add(egui::DragValue::new(&mut state.config.rcon_port).range(1..=65535));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("config-rcon-password"));
                    ui.add(egui::TextEdit::singleline(&mut state.config.rcon_password));
                });
                ui.horizontal(|ui| {
                    ui.label(tr!("config-log-output"));
                    ui.add(egui::DragValue::new(&mut state.config.log_output).range(0..=2));
                });
                ui.label(
                    egui::RichText::new(tr!("config-rcon-restart-note")).color(egui::Color32::GRAY),
                );
            });
        });

        ui.add_space(16.0);

        if ui
            .add_enabled(!read_only, egui::Button::new(tr!("save-config")))
            .clicked()
        {
            let result = state.save_config();
            state.record_result(result, "save config");
        }
    });
}

fn draw_settings_content(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            ui.label(tr!("language-label"));
            let current_lang_display = if state.current_language == "zh-Hans" {
                "简体中文"
            } else {
                "English"
            };
            egui::ComboBox::from_id_salt("language_combo")
                .selected_text(current_lang_display)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut state.current_language,
                        "en-US".to_string(),
                        "English",
                    );
                    ui.selectable_value(
                        &mut state.current_language,
                        "zh-Hans".to_string(),
                        "简体中文",
                    );
                });

            let current_lang = state.current_language.clone();
            if ui.button(tr!("btn-switch")).clicked() {
                state.switch_language(&current_lang);
            }
        });

        ui.horizontal(|ui| {
            ui.label(tr!("settings-theme"));
            let theme_display = match state.settings.theme {
                crate::settings::Theme::Light => tr!("theme-light"),
                crate::settings::Theme::Dark => tr!("theme-dark"),
                crate::settings::Theme::System => tr!("theme-system"),
            };
            egui::ComboBox::from_id_salt("theme_combo")
                .selected_text(theme_display)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut state.settings.theme,
                        crate::settings::Theme::Light,
                        tr!("theme-light"),
                    );
                    ui.selectable_value(
                        &mut state.settings.theme,
                        crate::settings::Theme::Dark,
                        tr!("theme-dark"),
                    );
                    ui.selectable_value(
                        &mut state.settings.theme,
                        crate::settings::Theme::System,
                        tr!("theme-system"),
                    );
                });

            if ui.button(tr!("btn-switch")).clicked() {
                let result = state.settings.save();
                state.record_result(result, "save settings");
            }
        });

        ui.separator();
        // Online data section
        ui.label(tr!("settings-online-data"));

        ui.horizontal(|ui| {
            ui.label(tr!("settings-mirror-site"));
            let mirror_display = state.settings.mirror_site.display_name();
            egui::ComboBox::from_id_salt("mirror_combo")
                .selected_text(mirror_display)
                .show_ui(ui, |ui| {
                    for mirror in crate::settings::MirrorSite::all() {
                        ui.selectable_value(
                            &mut state.settings.mirror_site,
                            *mirror,
                            mirror.display_name(),
                        );
                    }
                });

            if ui.button(tr!("btn-switch")).clicked() {
                let result = state.settings.save();
                state.record_result(result, "save settings");
            }
        });

        ui.horizontal(|ui| {
            ui.label(tr!("settings-last-update"));
            if let Some(ref timestamp) = state.settings.last_online_update {
                ui.label(timestamp);
            } else {
                ui.label(tr!("settings-never-updated"));
            }
        });

        ui.horizontal(|ui| {
            let button = ui.add_enabled(
                !state.is_loading_online,
                egui::Button::new(tr!("settings-update-now")),
            );
            if button.clicked() {
                state.request_manual_update();
            }
            if state.is_fetching_online_data() {
                ui.spinner();
                ui.label(tr!("settings-updating"));
            }
        });
    });
}

fn draw_about_page(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(32.0);

        ui.vertical_centered(|ui| {
            ui.label("CSGO-GC Editor");
            ui.add_space(8.0);
            ui.label(format!(
                "Version {}, Rolling release",
                env!("CARGO_PKG_VERSION")
            ));
            ui.add_space(8.0);
            ui.label(format!("{}GT610", tr!("author")));
            ui.add_space(16.0);
            ui.hyperlink_to(
                tr!("github-repository"),
                "https://github.com/GT-610/csgo-gc-inventory-editor",
            );
        });
    });
}
