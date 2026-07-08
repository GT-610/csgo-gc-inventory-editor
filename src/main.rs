#![windows_subsystem = "windows"]

pub mod app;
pub mod config;
pub mod core;
pub mod inventory;
pub mod online_data;
pub mod settings;
pub mod ui;

use crate::app::{CsgoInventoryEditor, ItemTemplate, Page, SelectWindowPurpose};
use crate::inventory::{ItemAttribute, get_attribute_default_value};
use eframe::egui;
use egui_i18n::tr;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSGO-GC Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(CsgoInventoryEditor::new(cc)))),
    )
}

impl eframe::App for CsgoInventoryEditor {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.apply_theme(&ctx);

        // Only check result when we have an active receiver
        if self.is_fetching_online_data() {
            self.check_online_data_result();
            // Request repaint periodically while loading
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        // Load online data only once when flag is set and not already fetching
        if self.is_loading_online && !self.is_fetching_online_data() {
            self.load_online_data();
        }

        egui::Panel::left("sidebar")
            .exact_size(120.0)
            .show_inside(ui, |ui| {
                ui::draw_sidebar(ui, self);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| match self.current_page {
            Page::Inventory => {
                ui::draw_inventory_page(ui, self);
            }
            Page::Settings => {
                ui::draw_settings_page(ui, self);
            }
        });

        let mut pending_select_window_items: Option<crate::app::SelectWindowItems> = None;
        let mut select_window_open = self.select_window_open;

        ui::draw_item_detail_windows(
            &ctx,
            self,
            &mut pending_select_window_items,
            &mut select_window_open,
        );

        if self.select_window_open {
            ui::draw_select_window(
                &ctx,
                &mut self.select_window_open,
                self.select_window_purpose
                    .map(SelectWindowPurpose::cache_salt)
                    .unwrap_or("unknown"),
                &self.select_window_title,
                &self.select_window_key_header,
                &self.select_window_value_header,
                &self.select_window_items,
                &mut self.select_window_search,
                &mut self.select_window_selected,
            );

            if !self.select_window_open && self.select_window_selected.is_none() {
                self.close_select_window();
            }
        }

        if self.pending_add_item {
            self.pending_add_item = false;
            let template = self.selected_template.unwrap_or(ItemTemplate::Empty);

            if template.is_music_kit() {
                let items = self.create_music_def_select_list();
                self.open_select_window(
                    SelectWindowPurpose::SelectMusicDef,
                    tr!("select-musicdef").to_string(),
                    tr!("header-musicdef-id").to_string(),
                    tr!("header-musicdef-name").to_string(),
                    items,
                );
                self.select_window_for_item = None;
                self.select_window_for_attr = None;
            } else if template.is_weapon_case() {
                let items = self.create_weapon_case_select_list();
                self.open_select_window(
                    SelectWindowPurpose::AddWeaponCase,
                    tr!("select-weapon-case-to-add").to_string(),
                    tr!("header-weapon-case-id").to_string(),
                    tr!("header-weapon-case-name").to_string(),
                    items,
                );
            } else {
                let items = self.create_item_select_list();
                self.open_select_window(
                    SelectWindowPurpose::AddItem,
                    tr!("select-item-to-add").to_string(),
                    tr!("header-item-id").to_string(),
                    tr!("header-item-name").to_string(),
                    items,
                );
            }
        }

        if let Some((item_id, def_index)) = self.pending_paint_kit_select.take() {
            let items = self.create_skin_select_list_for_weapon(def_index);
            self.open_select_window(
                SelectWindowPurpose::SelectPaintKit,
                tr!("select-paintkit").to_string(),
                tr!("header-paintkit-id").to_string(),
                tr!("header-paintkit-name").to_string(),
                items,
            );
            self.select_window_for_item = Some(item_id);
        }

        if let Some(item_id) = self.pending_music_def_select.take() {
            let items = self.create_music_def_select_list();
            self.open_select_window(
                SelectWindowPurpose::SelectMusicDef,
                tr!("select-musicdef").to_string(),
                tr!("header-musicdef-id").to_string(),
                tr!("header-musicdef-name").to_string(),
                items,
            );
            self.select_window_for_item = Some(item_id);
        }

        if let Some((item_id, attr_id)) = self.pending_sticker_kit_select.take() {
            let items = self.create_sticker_kit_select_list();
            self.open_select_window(
                SelectWindowPurpose::SelectStickerKit,
                tr!("select-stickerkit").to_string(),
                tr!("header-stickerkit-id").to_string(),
                tr!("header-stickerkit-name").to_string(),
                items,
            );
            self.select_window_for_item = Some(item_id);
            self.select_window_for_attr = Some(attr_id);
        }

        if let Some(item_id) = self.pending_graffiti_tint_select.take() {
            let items = self.create_graffiti_tint_select_list();
            self.open_select_window(
                SelectWindowPurpose::SelectGraffitiTint,
                tr!("select-graffiti-tint").to_string(),
                tr!("header-graffiti-tint-id").to_string(),
                tr!("header-graffiti-tint-name").to_string(),
                items,
            );
            self.select_window_for_item = Some(item_id);
        }

        if let Some(item_id) = self.pending_attribute_select.take() {
            let items = self.create_missing_attribute_select_list(item_id);
            if !items.is_empty() {
                self.open_select_window(
                    SelectWindowPurpose::AddAttribute,
                    tr!("select-attribute").to_string(),
                    tr!("header-attribute-id").to_string(),
                    tr!("header-attribute-name").to_string(),
                    items,
                );
                self.select_window_for_item = Some(item_id);
            }
        }

        if let Some(selected_idx) = self.select_window_selected {
            match self.select_window_purpose {
                Some(SelectWindowPurpose::AddItem) => {
                    if let Some((def_index_str, _, _, _)) =
                        self.select_window_items.get(selected_idx)
                        && let Ok(def_index) = def_index_str.parse::<u32>()
                    {
                        let new_inventory_id = self
                            .inventory
                            .items
                            .iter()
                            .map(|i| i.inventory)
                            .max()
                            .unwrap_or(0)
                            + 1;

                        let new_item_id =
                            self.inventory.items.iter().map(|i| i.id).max().unwrap_or(1) + 1;

                        let template = self.selected_template.unwrap_or(ItemTemplate::Empty);
                        let mut new_item = template.create_item(new_item_id, def_index);
                        new_item.inventory = new_inventory_id;

                        self.inventory.items.push(new_item);
                        self.mark_inventory_changed();
                        self.open_item_windows.insert(new_item_id);
                        let result = self.save_inventory();
                        self.record_result(result, "save inventory");
                    }
                    self.close_select_window();
                    self.selected_template = None;
                }

                Some(SelectWindowPurpose::AddWeaponCase) => {
                    if let Some((def_index_str, _, _, _)) =
                        self.select_window_items.get(selected_idx)
                        && let Ok(def_index) = def_index_str.parse::<u32>()
                    {
                        let mut next_inventory_id = self
                            .inventory
                            .items
                            .iter()
                            .map(|i| i.inventory)
                            .max()
                            .unwrap_or(0)
                            + 1;

                        let mut next_item_id =
                            self.inventory.items.iter().map(|i| i.id).max().unwrap_or(1) + 1;

                        let template = self.selected_template.unwrap_or(ItemTemplate::WeaponCase);
                        let mut new_item = template.create_item(next_item_id, def_index);
                        new_item.inventory = next_inventory_id;

                        self.inventory.items.push(new_item);
                        self.open_item_windows.insert(next_item_id);

                        if let Some(key_def_index) = self
                            .get_associated_item_def_indexes(def_index)
                            .first()
                            .copied()
                        {
                            next_inventory_id += 1;
                            next_item_id += 1;

                            let mut key_item =
                                ItemTemplate::Empty.create_item(next_item_id, key_def_index);
                            key_item.inventory = next_inventory_id;
                            self.inventory.items.push(key_item);
                        }

                        self.mark_inventory_changed();
                        let result = self.save_inventory();
                        self.record_result(result, "save inventory");
                    }
                    self.close_select_window();
                    self.selected_template = None;
                }

                Some(SelectWindowPurpose::EditItemDef) => {
                    if let Some(for_item_id) = self.select_window_for_item
                        && let Some((def_index_str, _, _, _)) =
                            self.select_window_items.get(selected_idx)
                    {
                        if let Ok(def_index) = def_index_str.parse::<u32>() {
                            if let Some(item) = self
                                .inventory
                                .items
                                .iter_mut()
                                .find(|i| i.id == for_item_id)
                            {
                                item.def_index = def_index;
                                let result = self.save_inventory();
                                self.record_result(result, "save inventory");
                            } else {
                                self.record_result::<(), _>(
                                    Err(format!("Item with id {} not found", for_item_id)),
                                    "update item definition",
                                );
                            }
                        } else {
                            self.record_result::<(), _>(
                                Err(format!("Invalid def_index: {}", def_index_str)),
                                "update item definition",
                            );
                        }
                    }
                    self.close_select_window();
                }

                Some(SelectWindowPurpose::SelectPaintKit) => {
                    if let Some(for_item_id) = self.select_window_for_item
                        && let Some((paint_index_str, _, _, _)) =
                            self.select_window_items.get(selected_idx)
                    {
                        // Get def_index from the item being edited
                        let def_index = self
                            .inventory
                            .items
                            .iter()
                            .find(|i| i.id == for_item_id)
                            .map(|i| i.def_index);

                        if let Ok(paint_index) = paint_index_str.parse::<u32>()
                            && let Some(weapon_id) = def_index
                            && let Some(rarity) = self.get_skin_rarity(weapon_id, paint_index)
                            && let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id)
                        {
                            edit_state.rarity = rarity;
                        }
                        if let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id) {
                            edit_state.attributes.insert(
                                ItemAttribute::SkinPaintIndex.id(),
                                paint_index_str.clone(),
                            );
                        }
                    }
                    self.close_select_window();
                }

                Some(SelectWindowPurpose::SelectMusicDef) => {
                    if let Some(for_item_id) = self.select_window_for_item
                        && let Some((music_index_str, _, _, _)) =
                            self.select_window_items.get(selected_idx)
                        && let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id)
                    {
                        edit_state
                            .attributes
                            .insert(ItemAttribute::MusicID.id(), music_index_str.clone());
                    } else if let Some((music_index_str, _, _, _)) =
                        self.select_window_items.get(selected_idx)
                        && let Ok(music_id) = music_index_str.parse::<u32>()
                    {
                        let new_inventory_id = self
                            .inventory
                            .items
                            .iter()
                            .map(|i| i.inventory)
                            .max()
                            .unwrap_or(0)
                            + 1;

                        let new_item_id =
                            self.inventory.items.iter().map(|i| i.id).max().unwrap_or(1) + 1;

                        let template = self
                            .selected_template
                            .unwrap_or(ItemTemplate::NormalMusicKit);
                        let mut new_item = template.create_music_kit(new_item_id, music_id);
                        new_item.inventory = new_inventory_id;

                        self.inventory.items.push(new_item);
                        self.mark_inventory_changed();
                        self.open_item_windows.insert(new_item_id);
                        let result = self.save_inventory();
                        self.record_result(result, "save inventory");
                    }
                    self.close_select_window();
                    self.selected_template = None;
                }

                Some(SelectWindowPurpose::SelectStickerKit) => {
                    if let Some(for_item_id) = self.select_window_for_item
                        && let Some(for_attr_id) = self.select_window_for_attr
                        && let Some((sticker_index_str, _, _, _)) =
                            self.select_window_items.get(selected_idx)
                        && let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id)
                    {
                        edit_state
                            .attributes
                            .insert(for_attr_id, sticker_index_str.clone());
                    }
                    self.close_select_window();
                }

                Some(SelectWindowPurpose::SelectGraffitiTint) => {
                    if let Some(for_item_id) = self.select_window_for_item
                        && let Some((tint_id_str, _, _, _)) =
                            self.select_window_items.get(selected_idx)
                        && let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id)
                    {
                        edit_state
                            .attributes
                            .insert(ItemAttribute::SprayColor.id(), tint_id_str.clone());
                    }
                    self.close_select_window();
                }

                Some(SelectWindowPurpose::AddAttribute) => {
                    if let Some(for_item_id) = self.select_window_for_item
                        && let Some((attr_id_str, _, _, _)) =
                            self.select_window_items.get(selected_idx)
                        && let Ok(attr_id) = attr_id_str.parse::<u32>()
                        && let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id)
                    {
                        edit_state
                            .attributes
                            .insert(attr_id, get_attribute_default_value(attr_id).to_string());
                    }
                    self.close_select_window();
                }

                None => self.close_select_window(),
            }
        }
    }
}
