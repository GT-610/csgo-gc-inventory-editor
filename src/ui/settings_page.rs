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
    ui.vertical_centered(|ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
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
                        let _ = state.save_config();
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
                        let _ = state.save_config();
                    }
                });
            });
        });

        ui.add_space(16.0);

        if ui.button(tr!("save-config")).clicked()
            && let Err(e) = state.save_config()
        {
            eprintln!("Failed to save config: {}", e);
        }
    });
}

fn draw_settings_content(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    ui.vertical_centered(|ui| {
        ui.add_space(32.0);

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

        ui.add_space(16.0);

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
                let _ = state.settings.save();
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
            ui.label(format!("Version {}, Rolling release", env!("CARGO_PKG_VERSION")));
            ui.add_space(16.0);
            ui.hyperlink_to(
                tr!("github-repository"),
                "https://github.com/GT-610/csgo-gc-inventory-editor",
            );
        });
    });
}
