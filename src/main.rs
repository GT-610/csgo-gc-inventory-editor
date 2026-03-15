// #![windows_subsystem = "windows"]

pub mod app;
pub mod config;
pub mod core;
pub mod inventory;
pub mod settings;
pub mod ui;

use crate::app::{CsgoInventoryEditor, ItemTemplate, Page};
use crate::inventory::ItemAttribute;
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);

        egui::SidePanel::left("sidebar")
            .exact_width(120.0)
            .show(ctx, |ui| {
                ui::draw_sidebar(ui, self);
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.current_page {
            Page::Inventory => {
                ui::draw_inventory_page(ui, self);
            }
            Page::Settings => {
                ui::draw_settings_page(ui, self);
            }
        });

        let mut pending_select_window_items: Option<Vec<(String, String, String)>> = None;
        let mut select_window_open = self.select_window_open;

        ui::draw_item_detail_windows(
            ctx,
            self,
            &mut pending_select_window_items,
            &mut select_window_open,
        );

        if self.select_window_open {
            ui::draw_select_window(
                ctx,
                &mut self.select_window_open,
                &self.select_window_title,
                &self.select_window_key_header,
                &self.select_window_value_header,
                &self.select_window_items,
                &mut self.select_window_search,
                &mut self.select_window_selected,
            );
        }

        if self.pending_add_item {
            self.pending_add_item = false;
            let template = self.selected_template.unwrap_or(ItemTemplate::Empty);

            if template.is_music_kit() {
                let items = self.create_music_def_select_list();
                self.open_select_window(
                    tr!("select-musicdef").to_string(),
                    tr!("header-musicdef-id").to_string(),
                    tr!("header-musicdef-name").to_string(),
                    items,
                );
                self.select_window_for_item = None;
                self.select_window_for_attr = None;
            } else {
                let items = self.create_item_select_list();
                self.open_select_window(
                    tr!("select-item-to-add").to_string(),
                    tr!("header-item-id").to_string(),
                    tr!("header-item-name").to_string(),
                    items,
                );
            }
        }

        if let Some(inventory_id) = self.pending_paint_kit_select.take() {
            self.pending_paint_kit_select = None;
            let items = self.create_paint_kit_select_list();
            self.open_select_window(
                tr!("select-paintkit").to_string(),
                tr!("header-paintkit-id").to_string(),
                tr!("header-paintkit-name").to_string(),
                items,
            );
            self.select_window_for_item = Some(inventory_id);
        }

        if let Some(inventory_id) = self.pending_music_def_select.take() {
            self.pending_music_def_select = None;
            let items = self.create_music_def_select_list();
            self.open_select_window(
                tr!("select-musicdef").to_string(),
                tr!("header-musicdef-id").to_string(),
                tr!("header-musicdef-name").to_string(),
                items,
            );
            self.select_window_for_item = Some(inventory_id);
        }

        if let Some((inventory_id, attr_id)) = self.pending_sticker_kit_select.take() {
            self.pending_sticker_kit_select = None;
            let items = self.create_sticker_kit_select_list();
            self.open_select_window(
                tr!("select-stickerkit").to_string(),
                tr!("header-stickerkit-id").to_string(),
                tr!("header-stickerkit-name").to_string(),
                items,
            );
            self.select_window_for_item = Some(inventory_id);
            self.select_window_for_attr = Some(attr_id);
        }

        if let Some(selected_idx) = self.select_window_selected {
            if self.select_window_title == tr!("select-item-to-add") {
                if let Some((def_index_str, _, _)) = self.select_window_items.get(selected_idx)
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
                    self.open_item_windows.insert(new_inventory_id);
                    let _ = self.save_inventory();
                }
                self.select_window_open = false;
                self.select_window_selected = None;
                self.selected_template = None;
            }

            if self.select_window_title == tr!("select-paintkit") {
                if let Some(for_item_id) = self.select_window_for_item
                    && let Some((paint_index_str, _, _)) =
                        self.select_window_items.get(selected_idx)
                {
                    if let Ok(paint_index) = paint_index_str.parse::<u32>()
                        && let Some(rarity) = self.items_game.get_paint_kit_rarity(paint_index)
                    {
                        let rarity = rarity + 1;
                        if let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id) {
                            edit_state.rarity = rarity;
                        }
                    }
                    if let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id) {
                        edit_state
                            .attributes
                            .insert(ItemAttribute::SkinPaintIndex.id(), paint_index_str.clone());
                    }
                }
                self.select_window_open = false;
                self.select_window_selected = None;
                self.select_window_for_item = None;
            }

            if self.select_window_title == tr!("select-musicdef") {
                if let Some(for_item_id) = self.select_window_for_item
                    && let Some((music_index_str, _, _)) =
                        self.select_window_items.get(selected_idx)
                    && let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id)
                {
                    edit_state
                        .attributes
                        .insert(ItemAttribute::MusicID.id(), music_index_str.clone());
                } else if let Some((music_index_str, _, _)) =
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
                    self.open_item_windows.insert(new_inventory_id);
                    let _ = self.save_inventory();
                }
                self.select_window_open = false;
                self.select_window_selected = None;
                self.select_window_for_item = None;
                self.selected_template = None;
            }

            if self.select_window_title == tr!("select-stickerkit") {
                if let Some(for_item_id) = self.select_window_for_item
                    && let Some(for_attr_id) = self.select_window_for_attr
                    && let Some((sticker_index_str, _, _)) =
                        self.select_window_items.get(selected_idx)
                    && let Some(edit_state) = self.edit_item_states.get_mut(&for_item_id)
                {
                    edit_state
                        .attributes
                        .insert(for_attr_id, sticker_index_str.clone());
                }
                self.select_window_open = false;
                self.select_window_selected = None;
                self.select_window_for_item = None;
                self.select_window_for_attr = None;
            }
        }
    }
}
