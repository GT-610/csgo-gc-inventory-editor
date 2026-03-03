use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::cell::RefCell;
use std::rc::Rc;
use egui_i18n::tr;
use crate::app::{CsgoInventoryEditor, EditItemState, ItemTemplate};
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
    let mut pending_save_item_id: Option<u64> = None;
    let mut pending_save_and_close: bool = false;
    let mut pending_open_select_window: Option<Vec<(String, String, String)>> = None;
    
    for inventory_id in open_windows {
        let item_opt = state.inventory.items.iter().find(|i| i.inventory == inventory_id);
        if item_opt.is_none() {
            state.open_item_windows.remove(&inventory_id);
            continue;
        }
        
        let item = item_opt.unwrap();
        let display_name = state.get_item_display_name(item);
        let mut window_open = true;
        
        let should_open_select_window = Rc::new(RefCell::new(false));
        let should_open_paint_kit_select = Rc::new(RefCell::new(false));
        
        let inventory_id_for_edit = inventory_id;
        
        if !state.edit_item_states.contains_key(&inventory_id) {
            state.edit_item_states.insert(inventory_id, EditItemState {
                level: item.level,
                custom_name: item.custom_name.clone().unwrap_or_default(),
                rarity: item.rarity,
                quality: item.quality,
                attributes: item.attributes.clone(),
            });
        }
        
        let edit_state = state.edit_item_states.get(&inventory_id).cloned().unwrap_or_else(|| EditItemState {
            level: item.level,
            custom_name: item.custom_name.clone().unwrap_or_default(),
            rarity: item.rarity,
            quality: item.quality,
            attributes: item.attributes.clone(),
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
                
                ui.horizontal(|ui| {
                    if ui.button(tr!("btn-save")).clicked() {
                        pending_save_item_id = Some(inventory_id);
                    }
                    ui.add_space(10.0);
                    if ui.button(tr!("btn-save-close")).clicked() {
                        pending_save_item_id = Some(inventory_id);
                        pending_save_and_close = true;
                    }
                    ui.add_space(10.0);
                    if ui.button(tr!("btn-cancel")).clicked() {
                        windows_to_close.push(inventory_id);
                    }
                    ui.add_space(10.0);
                    if ui.button(tr!("btn-delete")).clicked() {
                        state.delete_confirm_item_id = Some(inventory_id);
                    }
                });
                
                if pending_save_and_close {
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
                                let all_qualities = items_game_ref.get_all_qualities_sorted();
                                let selected_name = all_qualities.iter()
                                    .find(|(v, _)| *v == edit_state.quality)
                                    .and_then(|(_, name)| translations_ref.get(name).cloned())
                                    .unwrap_or_else(|| format!("Unknown ({})", edit_state.quality));
                                
                                egui::ComboBox::from_id_salt(format!("quality_combo_{}", inventory_id))
                                    .selected_text(format!("{} ({})", selected_name, edit_state.quality))
                                    .show_ui(ui, |ui| {
                                        for (value, name) in &all_qualities {
                                            let display_name = translations_ref.get(name).cloned().unwrap_or_else(|| name.clone());
                                            ui.selectable_value(&mut edit_state.quality, *value, format!("{} ({})", display_name, value));
                                        }
                                    });
                            });
                        });
                        
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(tr!("rarity"));
                            });
                            row.col(|ui| {
                                let all_rarities = items_game_ref.get_all_rarities_sorted();
                                let selected_name = all_rarities.iter()
                                    .find(|(v, _)| *v == edit_state.rarity)
                                    .and_then(|(_, k)| translations_ref.get(k).cloned())
                                    .unwrap_or_else(|| format!("Unknown ({})", edit_state.rarity));
                                
                                egui::ComboBox::from_id_salt(format!("rarity_combo_{}", inventory_id))
                                    .selected_text(format!("{} ({})", selected_name, edit_state.rarity))
                                    .show_ui(ui, |ui| {
                                        for (value, loc_key) in &all_rarities {
                                            let display_name = translations_ref.get(loc_key).cloned().unwrap_or_else(|| loc_key.clone());
                                            ui.selectable_value(&mut edit_state.rarity, *value, format!("{} ({})", display_name, value));
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
                
                let mut attr_vec: Vec<(u32, String)> = item.attributes.iter()
                    .map(|(k, v)| (*k, v.clone()))
                    .collect();
                attr_vec.sort_by_key(|(id, _)| *id);
                
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
                        for (attr_id, _attr_value) in &attr_vec {
                            let fluent_key = get_attribute_fluent_key(*attr_id);
                            let attr_name = tr!(&fluent_key);
                            
                            let edit_value = edit_state.attributes.get(attr_id)
                                .cloned()
                                .unwrap_or_else(|| item.attributes.get(attr_id).cloned().unwrap_or_default());
                            
                            let attr_value_display = get_attribute_value_display_name(
                                *attr_id,
                                &edit_value,
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
                                    if *attr_id == 6 {
                                        ui.horizontal(|ui| {
                                            ui.label(attr_value_display);
                                            ui.add_space(10.0);
                                            if ui.button(tr!("btn-select")).clicked() {
                                                state.pending_paint_kit_select = Some(inventory_id_for_edit);
                                                *should_open_paint_kit_select.borrow_mut() = true;
                                            }
                                        });
                                    } else if *attr_id == 113 || *attr_id == 166 {
                                        ui.label(attr_value_display);
                                    } else {
                                        let value_mut = edit_state.attributes.entry(*attr_id)
                                            .or_insert_with(|| edit_value.clone());
                                        ui.text_edit_singleline(value_mut);
                                    }
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
    
    if let Some(item_id) = pending_save_item_id {
        if let Some(edit_state) = state.edit_item_states.get(&item_id) {
            if let Some(item) = state.inventory.items.iter_mut().find(|i| i.inventory == item_id) {
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
        }
        
        let result = state.save_inventory();
        match result {
            Ok(()) => {
                eprintln!("Inventory saved successfully");
            }
            Err(e) => {
                eprintln!("Failed to save inventory: {}", e);
            }
        }
    }
    
    for window_id in windows_to_close {
        state.open_item_windows.remove(&window_id);
        state.edit_item_states.remove(&window_id);
    }
    
    if let Some(item_id) = state.delete_confirm_item_id {
        let item_name = state.inventory.items.iter()
            .find(|i| i.inventory == item_id)
            .map(|i| state.get_item_display_name(i))
            .unwrap_or_default();
        
        let delete_confirmed = Rc::new(RefCell::new(false));
        let delete_confirmed_inner = delete_confirmed.clone();
        
        egui::Modal::new(egui::Id::new("delete_confirm_modal"))
            .show(ctx, |ui| {
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
                            *delete_confirmed_inner.borrow_mut() = true;
                            ui.close();
                        }
                    },
                );
            });
        
        if *delete_confirmed.borrow() {
            if let Some(pos) = state.inventory.items.iter().position(|i| i.inventory == item_id) {
                state.inventory.items.remove(pos);
                state.open_item_windows.remove(&item_id);
                state.edit_item_states.remove(&item_id);
                
                let result = state.save_inventory();
                match result {
                    Ok(()) => {
                        eprintln!("Item deleted successfully");
                    }
                    Err(e) => {
                        eprintln!("Failed to save inventory after delete: {}", e);
                    }
                }
            }
            state.delete_confirm_item_id = None;
        }
    }
    
    if state.show_template_modal {
        let template_confirmed = Rc::new(RefCell::new(false));
        let template_confirmed_inner = template_confirmed.clone();
        
        egui::Modal::new(egui::Id::new("template_modal"))
            .show(ctx, |ui| {
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
                        if ui.selectable_value(&mut state.selected_template, Some(ItemTemplate::Empty), tr!("template-empty")).clicked() {
                            state.selected_template = Some(ItemTemplate::Empty);
                        }
                        if ui.selectable_value(&mut state.selected_template, Some(ItemTemplate::NormalWeapon), tr!("template-normal-weapon")).clicked() {
                            state.selected_template = Some(ItemTemplate::NormalWeapon);
                        }
                        if ui.selectable_value(&mut state.selected_template, Some(ItemTemplate::StatTrakWeapon), tr!("template-stattrack-weapon")).clicked() {
                            state.selected_template = Some(ItemTemplate::StatTrakWeapon);
                        }
                        if ui.selectable_value(&mut state.selected_template, Some(ItemTemplate::NormalMusicKit), tr!("template-normal-musickit")).clicked() {
                            state.selected_template = Some(ItemTemplate::NormalMusicKit);
                        }
                        if ui.selectable_value(&mut state.selected_template, Some(ItemTemplate::StatTrakMusicKit), tr!("template-stattrack-musickit")).clicked() {
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
                            *template_confirmed_inner.borrow_mut() = true;
                            ui.close();
                        }
                    },
                );
            });
        
        if *template_confirmed.borrow() {
            state.show_template_modal = false;
            if state.selected_template.is_some() {
                state.pending_add_item = true;
            }
        }
    }
}
