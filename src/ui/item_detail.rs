use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::cell::RefCell;
use std::rc::Rc;
use egui_i18n::tr;
use crate::app::{CsgoInventoryEditor, EditItemState};
use crate::inventory::{get_attribute_fluent_key, get_attribute_value_display_name};

pub fn draw_item_detail_windows(
    ctx: &egui::Context,
    state: &mut CsgoInventoryEditor,
    pending_select_window_items: &mut Option<Vec<(String, String, String)>>,
    select_window_open: &mut bool,
) {
    let open_windows = state.open_item_windows.clone();
    let items_game_ref = &state.items_game;
    let translations_ref = &state.translations;
    let mut windows_to_close: Vec<u64> = Vec::new();
    let mut apply_clicked_for: Option<u64> = None;
    let mut pending_open_select_window: Option<Vec<(String, String, String)>> = None;
    
    for inventory_id in open_windows {
        let item_opt = state.inventory.items.iter().find(|i| i.inventory == inventory_id);
        if item_opt.is_none() {
            state.open_item_windows.remove(&inventory_id);
            continue;
        }
        
        let item = item_opt.unwrap();
        let display_name = state.get_item_display_name(item);
        let rarity_name = state.get_rarity_name(item.rarity);
        let mut window_open = true;
        
        let should_open_select_window = Rc::new(RefCell::new(false));
        
        let inventory_id_for_edit = inventory_id;
        
        if !state.edit_item_states.contains_key(&inventory_id) {
            state.edit_item_states.insert(inventory_id, EditItemState {
                level: item.level,
                custom_name: item.custom_name.clone().unwrap_or_default(),
            });
        }
        
        let edit_state = state.edit_item_states.get(&inventory_id).cloned().unwrap_or_else(|| EditItemState {
            level: item.level,
            custom_name: item.custom_name.clone().unwrap_or_default(),
        });
        let mut edit_state = edit_state;
        
        egui::Window::new(format!("{} - {}", tr!("item-detail"), display_name))
            .id(egui::Id::new(format!("item_window_{}", inventory_id)))
            .movable(true)
            .collapsible(true)
            .resizable(false)
            .open(&mut window_open)
            .show(ctx, |ui| {
                let item_base_name = items_game_ref.get_item_display_name(item.def_index, translations_ref);
                let mut save_and_close = false;
                let mut discard_and_close = false;
                
                ui.horizontal(|ui| {
                    if ui.button(tr!("btn-save")).clicked() {
                        apply_clicked_for = Some(inventory_id);
                    }
                    ui.add_space(10.0);
                    if ui.button(tr!("btn-save-close")).clicked() {
                        save_and_close = true;
                    }
                    ui.add_space(10.0);
                    if ui.button(tr!("btn-cancel")).clicked() {
                        discard_and_close = true;
                    }
                });
                
                if save_and_close {
                    apply_clicked_for = Some(inventory_id);
                    windows_to_close.push(inventory_id);
                }
                
                if discard_and_close {
                    windows_to_close.push(inventory_id);
                }
                
                ui.separator();

                let table = TableBuilder::new(ui)
                    .id_salt(inventory_id)
                    .striped(true)
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(100.0))
                    .column(Column::remainder())
                    .min_scrolled_height(0.0);
                
                table
                    .body(|mut body| {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(tr!("item"));
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}", item_base_name));
                                    ui.label(format!("({})", item.def_index));
                                    ui.add_space(10.0);
                                    if ui.button(tr!("btn-select")).clicked() {
                                        let mut items: Vec<(String, String, String)> = items_game_ref.items.iter()
                                            .map(|(def_index, ig_item): (&u32, &crate::inventory::IGItem)| {
                                                let display_name = ig_item.get_display_name(translations_ref);
                                                (def_index.to_string(), display_name, def_index.to_string())
                                            })
                                            .collect();
                                        items.sort_by_key(|(key, _, _): &(String, String, String)| key.parse::<u32>().unwrap_or(0));
                                        *pending_select_window_items = Some(items);
                                        *should_open_select_window.borrow_mut() = true;
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
                                ui.label(format!("{}", item.quality));
                            });
                        });
                        
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(tr!("rarity"));
                            });
                            row.col(|ui| {
                                ui.label(format!("{} ({})", rarity_name, item.rarity));
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
                
                ui.label(tr!("item-properties"));
                
                let attr_vec: Vec<(u32, String)> = item.attributes.iter()
                    .map(|(k, v)| (*k, v.clone()))
                    .collect();
                
                let attr_table = TableBuilder::new(ui)
                    .id_salt(format!("attr_{}", inventory_id))
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::remainder())
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
                    })
                    .body(|mut body| {
                        for (attr_id, attr_value) in &attr_vec {
                            let fluent_key = get_attribute_fluent_key(*attr_id);
                            let attr_name = tr!(&fluent_key);
                            let attr_value_display = get_attribute_value_display_name(
                                *attr_id,
                                attr_value,
                                items_game_ref,
                                translations_ref,
                            );
                            
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(format!("{}", attr_id));
                                });
                                row.col(|ui| {
                                    ui.label(attr_name);
                                });
                                row.col(|ui| {
                                    ui.label(attr_value_display);
                                });
                            });
                        }
                    });
                
                state.edit_item_states.insert(inventory_id_for_edit, edit_state);
            });
        
        if *should_open_select_window.borrow() {
            if pending_select_window_items.is_some() {
                pending_open_select_window = pending_select_window_items.take();
            }
        }
        
        if !window_open {
            state.open_item_windows.remove(&inventory_id);
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
    
    if let Some(apply_id) = apply_clicked_for {
        if let Some(edit_state) = state.edit_item_states.get(&apply_id) {
            if let Some(item) = state.inventory.items.iter_mut().find(|i| i.inventory == apply_id) {
                item.level = edit_state.level;
                item.custom_name = if edit_state.custom_name.is_empty() {
                    None
                } else {
                    Some(edit_state.custom_name.clone())
                };
            }
        }
    }
    
    for window_id in windows_to_close {
        state.open_item_windows.remove(&window_id);
        state.edit_item_states.remove(&window_id);
    }
}
