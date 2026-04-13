use crate::app::{CsgoInventoryEditor, EditItemState, ItemTemplate, SelectWindowItems};
use crate::inventory::{
    AVAILABLE_ATTRIBUTES, ItemAttribute, get_attribute_fluent_key, get_attribute_value_display_name,
};
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_i18n::tr;

pub fn draw_item_detail_windows(
    ctx: &egui::Context,
    state: &mut CsgoInventoryEditor,
    pending_select_window_items: &mut Option<SelectWindowItems>,
    select_window_open: &mut bool,
) {
    state.refresh_inventory_cache();
    let open_windows = state.open_item_windows.clone();
    let mut windows_to_close: Vec<u64> = Vec::new();
    let mut pending_save_item_id: Option<u64> = None;
    let mut pending_open_select_window: Option<SelectWindowItems> = None;

    for item_id in open_windows {
        let Some(item_idx) = state.get_item_index(item_id) else {
            state.open_item_windows.remove(&item_id);
            continue;
        };

        let item = &state.inventory.items[item_idx];
        let item_def_index = item.def_index;
        let item_level = item.level;
        let item_rarity = item.rarity;
        let item_quality = item.quality;
        let item_attributes = item.attributes.clone();
        let base_custom_name = item.custom_name.clone().unwrap_or_default();
        let display_name = state.get_item_display_name(item);
        let item_base_name = state
            .items_game
            .get_item_display_name(item_def_index, &state.translations);
        let mut window_open = true;
        let mut pending_save_and_close = false;

        let mut should_open_select_window = false;

        let item_id_for_edit = item_id;

        state
            .edit_item_states
            .entry(item_id)
            .or_insert_with(|| EditItemState {
                level: item_level,
                custom_name: base_custom_name.clone(),
                rarity: item_rarity,
                quality: item_quality,
                attributes: item_attributes.clone(),
            });

        let mut edit_state = state
            .edit_item_states
            .get(&item_id)
            .cloned()
            .expect("edit state should exist after insertion");

        let has_unsaved_changes = edit_state.level != item_level
            || edit_state.custom_name != base_custom_name
            || edit_state.rarity != item_rarity
            || edit_state.quality != item_quality
            || edit_state.attributes != item_attributes;

        egui::Window::new(format!("{} - {}", tr!("item-detail"), display_name))
            .id(egui::Id::new(format!("item_window_{}", item_id)))
            .movable(true)
            .collapsible(true)
            .resizable(true)
            .default_size(egui::vec2(720.0, 560.0))
            .min_size(egui::vec2(460.0, 320.0))
            .open(&mut window_open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button(tr!("btn-save")).clicked() {
                        pending_save_item_id = Some(item_id);
                    }
                    ui.add_space(10.0);
                    if ui.button(tr!("btn-save-close")).clicked() {
                        pending_save_item_id = Some(item_id);
                        pending_save_and_close = true;
                    }
                    ui.add_space(10.0);
                    if ui.button(tr!("btn-cancel")).clicked() {
                        windows_to_close.push(item_id);
                    }
                    ui.add_space(10.0);
                    if ui.button(tr!("btn-delete")).clicked() {
                        state.delete_confirm_item_id = Some(item_id);
                    }

                    if has_unsaved_changes {
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new(tr!("status-unsaved"))
                                .color(egui::Color32::from_rgb(200, 150, 0))
                                .size(14.0),
                        );
                    }
                });

                if pending_save_and_close {
                    windows_to_close.push(item_id);
                }

                ui.separator();

                let table = TableBuilder::new(ui)
                    .id_salt(item_id)
                    .striped(true)
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(100.0))
                    .column(Column::remainder())
                    .min_scrolled_height(0.0);

                table.body(|mut body| {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(tr!("item"));
                        });
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(item_base_name.to_string());
                                ui.label(format!("({})", item_def_index));
                                ui.add_space(10.0);
                                if ui.button(tr!("btn-select")).clicked() {
                                    let items = state.create_item_select_list();
                                    *pending_select_window_items = Some(items);
                                    should_open_select_window = true;
                                    state.select_window_for_item = Some(item_id);
                                }
                            });
                        });
                    });

                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(tr!("level"));
                        });
                        row.col(|ui| {
                            ui.add(egui::DragValue::new(&mut edit_state.level).range(0..=100));
                        });
                    });

                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(tr!("quality-id"));
                        });
                        row.col(|ui| {
                            let all_qualities = state.get_cached_quality_names();
                            let selected_name = all_qualities
                                .iter()
                                .find(|(v, _)| *v == edit_state.quality)
                                .map(|(_, name)| name.clone())
                                .unwrap_or_else(|| format!("Unknown ({})", edit_state.quality));

                            egui::ComboBox::from_id_salt(format!("quality_combo_{}", item_id))
                                .selected_text(format!(
                                    "{} ({})",
                                    selected_name, edit_state.quality
                                ))
                                .show_ui(ui, |ui| {
                                    for (value, name) in all_qualities {
                                        ui.selectable_value(
                                            &mut edit_state.quality,
                                            *value,
                                            format!("{} ({})", name, value),
                                        );
                                    }
                                });
                        });
                    });

                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(tr!("rarity"));
                        });
                        row.col(|ui| {
                            let rarity_names = state.get_cached_rarity_names();

                            let selected_name = rarity_names
                                .iter()
                                .find(|(v, _)| *v == edit_state.rarity)
                                .map(|(_, n)| n.clone())
                                .unwrap_or_else(|| format!("Unknown ({})", edit_state.rarity));

                            egui::ComboBox::from_id_salt(format!("rarity_combo_{}", item_id))
                                .selected_text(format!("{} ({})", selected_name, edit_state.rarity))
                                .show_ui(ui, |ui| {
                                    for (value, name) in rarity_names {
                                        ui.selectable_value(
                                            &mut edit_state.rarity,
                                            *value,
                                            format!("{} ({})", name, value),
                                        );
                                    }
                                });
                        });
                    });

                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(tr!("custom-name"));
                        });
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut edit_state.custom_name);
                        });
                    });
                });

                ui.separator();

                ui.horizontal(|ui| {
                    let can_add_attribute = AVAILABLE_ATTRIBUTES
                        .iter()
                        .any(|attr_id| !edit_state.attributes.contains_key(attr_id));
                    if ui
                        .add_enabled(
                            can_add_attribute,
                            egui::Button::new(tr!("btn-add-attribute")),
                        )
                        .clicked()
                    {
                        state.pending_attribute_select = Some(item_id_for_edit);
                    }
                });

                ui.add_space(8.0);

                let mut attr_vec: Vec<(u32, String)> = edit_state
                    .attributes
                    .iter()
                    .map(|(k, v)| (*k, v.clone()))
                    .collect();
                attr_vec.sort_by_key(|(id, _)| *id);

                let attr_table = TableBuilder::new(ui)
                    .id_salt(format!("attr_{}", item_id))
                    .striped(true)
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::remainder())
                    .column(Column::auto())
                    .min_scrolled_height(150.0);

                attr_table
                    .header(30.0, |mut header| {
                        header.col(|ui| {
                            ui.strong(tr!("prop-index"));
                        });
                        header.col(|ui| {
                            ui.strong(tr!("prop-description"));
                        });
                        header.col(|ui| {
                            ui.strong(tr!("prop-value"));
                        });
                        header.col(|ui| {
                            ui.strong(tr!("actions"));
                        });
                    })
                    .body(|mut body| {
                        for (attr_id, _attr_value) in &attr_vec {
                            let fluent_key = get_attribute_fluent_key(*attr_id);
                            let attr_name = tr!(&fluent_key);

                            let edit_value = edit_state
                                .attributes
                                .get(attr_id)
                                .cloned()
                                .unwrap_or_else(|| {
                                    item_attributes.get(attr_id).cloned().unwrap_or_default()
                                });

                            let attr_value_display = get_attribute_value_display_name(
                                *attr_id,
                                &edit_value,
                                &state.items_game,
                                &state.translations,
                            );

                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(format!("{}", attr_id));
                                });
                                row.col(|ui| {
                                    ui.label(attr_name);
                                });
                                row.col(|ui| {
                                    if *attr_id == 6 {
                                        ui.horizontal(|ui| {
                                            ui.label(attr_value_display);
                                            ui.add_space(10.0);
                                            if ui.button(tr!("btn-select")).clicked() {
                                                state.pending_paint_kit_select =
                                                    Some((item_id_for_edit, item_def_index));
                                            }
                                        });
                                    } else if *attr_id == 166 {
                                        ui.horizontal(|ui| {
                                            ui.label(attr_value_display);
                                            ui.add_space(10.0);
                                            if ui.button(tr!("btn-select")).clicked() {
                                                state.pending_music_def_select =
                                                    Some(item_id_for_edit);
                                            }
                                        });
                                    } else if *attr_id == 113
                                        || *attr_id == 117
                                        || *attr_id == 121
                                        || *attr_id == 125
                                        || *attr_id == 129
                                        || *attr_id == 133
                                    {
                                        ui.horizontal(|ui| {
                                            ui.label(attr_value_display);
                                            ui.add_space(10.0);
                                            if ui.button(tr!("btn-select")).clicked() {
                                                state.pending_sticker_kit_select =
                                                    Some((item_id_for_edit, *attr_id));
                                            }
                                        });
                                    } else if *attr_id == ItemAttribute::SprayColor.id() {
                                        ui.horizontal(|ui| {
                                            ui.label(attr_value_display);
                                            ui.add_space(10.0);
                                            if ui.button(tr!("btn-select")).clicked() {
                                                state.pending_graffiti_tint_select =
                                                    Some(item_id_for_edit);
                                            }
                                        });
                                    } else {
                                        let value_mut = edit_state
                                            .attributes
                                            .entry(*attr_id)
                                            .or_insert_with(|| edit_value.clone());
                                        ui.text_edit_singleline(value_mut);
                                    }
                                });
                                row.col(|ui| {
                                    if ui.button(tr!("btn-delete-attribute")).clicked() {
                                        edit_state.attributes.remove(attr_id);
                                    }
                                });
                            });
                        }
                    });

                state.edit_item_states.insert(item_id_for_edit, edit_state);
            });

        if should_open_select_window && pending_select_window_items.is_some() {
            pending_open_select_window = pending_select_window_items.take();
        }

        if !window_open {
            state.open_item_windows.remove(&item_id);
        }
    }

    if let Some(items) = pending_open_select_window {
        state.open_select_window(
            tr!("select-item").to_string(),
            tr!("header-item-id").to_string(),
            tr!("header-item-name").to_string(),
            items,
        );
        *select_window_open = true;
    }

    if let Some(item_id) = pending_save_item_id
        && let Some(edit_state) = state.edit_item_states.get(&item_id)
        && let Some(item_idx) = state.get_item_index(item_id)
    {
        let item = &mut state.inventory.items[item_idx];
        item.level = edit_state.level;
        item.rarity = edit_state.rarity;
        item.quality = edit_state.quality;
        item.custom_name = if edit_state.custom_name.is_empty() {
            None
        } else {
            Some(edit_state.custom_name.clone())
        };
        item.attributes = edit_state.attributes.clone();
    }

    if pending_save_item_id.is_some() {
        let result = state.save_inventory();
        if let Err(e) = result {
            eprintln!("Failed to save inventory: {}", e);
        }
    }

    for window_id in windows_to_close {
        state.open_item_windows.remove(&window_id);
        state.edit_item_states.remove(&window_id);
    }

    if let Some(item_id) = state.delete_confirm_item_id {
        let item_name = state
            .inventory
            .items
            .iter()
            .find(|i| i.id == item_id)
            .map(|i| state.get_item_display_name(i))
            .unwrap_or_default();

        let mut delete_confirmed = false;

        egui::Modal::new(egui::Id::new("delete_confirm_modal")).show(ctx, |ui| {
            ui.label(tr!("modal-delete-message").replace("%1", &item_name));

            ui.add_space(16.0);

            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button(tr!("btn-cancel")).clicked() {
                        state.delete_confirm_item_id = None;
                        ui.close();
                    }

                    if ui.button(tr!("btn-confirm")).clicked() {
                        delete_confirmed = true;
                        ui.close();
                    }
                },
            );
        });

        if delete_confirmed {
            if let Some(item_idx) = state.get_item_index(item_id) {
                state.inventory.items.remove(item_idx);
                state.mark_inventory_changed();
                state.open_item_windows.remove(&item_id);
                state.edit_item_states.remove(&item_id);

                let result = state.save_inventory();
                if let Err(e) = result {
                    eprintln!("Failed to save inventory after delete: {}", e);
                }
            }
            state.delete_confirm_item_id = None;
        }
    }

    if state.show_template_modal {
        let mut template_confirmed = false;

        egui::Modal::new(egui::Id::new("template_modal")).show(ctx, |ui| {
            ui.label(tr!("select-template"));

            ui.add_space(16.0);

            let current = state.selected_template.unwrap_or(ItemTemplate::Empty);
            egui::ComboBox::from_id_salt("template_select")
                .selected_text(match current {
                    ItemTemplate::Empty => tr!("template-empty"),
                    ItemTemplate::NormalWeapon => tr!("template-normal-weapon"),
                    ItemTemplate::StatTrakWeapon => tr!("template-stattrack-weapon"),
                    ItemTemplate::NormalMusicKit => tr!("template-normal-musickit"),
                    ItemTemplate::StatTrakMusicKit => tr!("template-stattrack-musickit"),
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(
                            &mut state.selected_template,
                            Some(ItemTemplate::Empty),
                            tr!("template-empty"),
                        )
                        .clicked()
                    {
                        state.selected_template = Some(ItemTemplate::Empty);
                    }
                    if ui
                        .selectable_value(
                            &mut state.selected_template,
                            Some(ItemTemplate::NormalWeapon),
                            tr!("template-normal-weapon"),
                        )
                        .clicked()
                    {
                        state.selected_template = Some(ItemTemplate::NormalWeapon);
                    }
                    if ui
                        .selectable_value(
                            &mut state.selected_template,
                            Some(ItemTemplate::StatTrakWeapon),
                            tr!("template-stattrack-weapon"),
                        )
                        .clicked()
                    {
                        state.selected_template = Some(ItemTemplate::StatTrakWeapon);
                    }
                    if ui
                        .selectable_value(
                            &mut state.selected_template,
                            Some(ItemTemplate::NormalMusicKit),
                            tr!("template-normal-musickit"),
                        )
                        .clicked()
                    {
                        state.selected_template = Some(ItemTemplate::NormalMusicKit);
                    }
                    if ui
                        .selectable_value(
                            &mut state.selected_template,
                            Some(ItemTemplate::StatTrakMusicKit),
                            tr!("template-stattrack-musickit"),
                        )
                        .clicked()
                    {
                        state.selected_template = Some(ItemTemplate::StatTrakMusicKit);
                    }
                });

            ui.add_space(16.0);

            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button(tr!("btn-cancel")).clicked() {
                        state.show_template_modal = false;
                        state.selected_template = None;
                        ui.close();
                    }

                    if ui.button(tr!("btn-confirm")).clicked() {
                        template_confirmed = true;
                        ui.close();
                    }
                },
            );
        });

        if template_confirmed {
            state.show_template_modal = false;
            if state.selected_template.is_some() {
                state.pending_add_item = true;
            }
        }
    }
}
