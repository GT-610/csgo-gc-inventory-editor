use crate::app::SelectWindowItems;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_i18n::tr;

fn parse_hex_color(hex: &str) -> Option<egui::Color32> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(egui::Color32::from_rgb(r, g, b))
}

#[allow(clippy::too_many_arguments)]
pub fn draw_select_window(
    ctx: &egui::Context,
    open: &mut bool,
    cache_salt: &str,
    title: &str,
    key_header: &str,
    value_header: &str,
    items: &SelectWindowItems,
    search: &mut String,
    selected: &mut Option<usize>,
) {
    let cache_key = egui::Id::new(format!("select_window_filter_{}", cache_salt));

    let filtered_items = ctx.memory_mut(|mem| {
        let cache_data = mem
            .data
            .get_temp_mut_or_insert_with::<(String, usize, Vec<usize>)>(cache_key, || {
                (String::new(), 0, Vec::new())
            });

        // Cache is valid if: search matches, items count matches, and indices are valid
        if cache_data.0 == *search
            && cache_data.1 == items.len()
            && !cache_data.2.is_empty()
            && cache_data.2.len() <= items.len()
        {
            cache_data
                .2
                .iter()
                .map(|&idx| {
                    let item = &items[idx];
                    (idx, &item.0, &item.1, &item.2)
                })
                .collect()
        } else {
            let search_lower = search.to_lowercase();
            let filtered: Vec<(usize, &String, &String, &Option<String>)> = items
                .iter()
                .enumerate()
                .filter(|(_, (key, display, _))| {
                    if search.is_empty() {
                        true
                    } else {
                        key.to_lowercase().contains(&search_lower)
                            || display.to_lowercase().contains(&search_lower)
                    }
                })
                .map(|(idx, item)| (idx, &item.0, &item.1, &item.2))
                .collect();

            let indices: Vec<usize> = filtered.iter().map(|(idx, _, _, _)| *idx).collect();
            cache_data.0 = search.clone();
            cache_data.1 = items.len();
            cache_data.2 = indices;

            filtered
        }
    });

    let mut window_open = *open;
    let mut row_clicked = false;

    egui::Window::new(title)
        .id(egui::Id::new("select_window"))
        .open(&mut window_open)
        .resizable(true)
        .default_size(egui::vec2(720.0, 520.0))
        .min_size(egui::vec2(420.0, 260.0))
        .collapsible(true)
        .movable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut search_edit = egui::TextEdit::singleline(search);
                search_edit = search_edit.hint_text(tr!("search"));
                let response = ui.add(search_edit);
                if response.changed() {
                    *selected = None;
                }
            });

            ui.add_space(8.0);

            let text_height = ui.text_style_height(&egui::TextStyle::Body);

            let available_height = ui.available_height() - 80.0;
            let max_table_height = available_height.max(100.0);

            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::remainder())
                .max_scroll_height(max_table_height)
                .sense(egui::Sense::click());

            table
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.strong(key_header);
                    });
                    header.col(|ui| {
                        ui.strong(value_header);
                    });
                })
                .body(|body| {
                    body.rows(text_height, filtered_items.len(), |mut row| {
                        let (idx, key, display, color): (usize, &String, &String, &Option<String>) =
                            filtered_items[row.index()];
                        let is_selected = *selected == Some(idx);
                        row.set_selected(is_selected);

                        row.col(|ui| {
                            ui.label(key.as_str());
                        });
                        row.col(|ui| {
                            // Apply color if available
                            if let Some(hex_color) = color
                                && let Some(color32) = parse_hex_color(hex_color)
                            {
                                ui.colored_label(color32, display.as_str());
                            } else {
                                ui.label(display.as_str());
                            }
                        });

                        if row.response().clicked() {
                            *selected = Some(idx);
                            row_clicked = true;
                        }
                    });
                });
        });

    if row_clicked {
        window_open = false;
    }

    *open = window_open;
}
