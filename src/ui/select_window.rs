use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_i18n::tr;

pub fn draw_select_window(
    ctx: &egui::Context,
    open: &mut bool,
    title: &str,
    key_header: &str,
    value_header: &str,
    items: &[(String, String, String)],
    search: &mut String,
    selected: &mut Option<usize>,
) {
    let cache_key = egui::Id::new(format!("select_window_filter_{}", title));

    let filtered_items = ctx.memory_mut(|mem| {
        let cache_data = mem
            .data
            .get_temp_mut_or_insert_with::<(String, Vec<usize>)>(cache_key, || {
                (String::new(), Vec::new())
            });

        if cache_data.0 == *search && !cache_data.1.is_empty() && cache_data.1.len() <= items.len()
        {
            cache_data
                .1
                .iter()
                .map(|&idx| (idx, &items[idx].0, &items[idx].1, &items[idx].2))
                .collect()
        } else {
            let filtered: Vec<(usize, &String, &String, &String)> = items
                .iter()
                .enumerate()
                .filter(|(_, (key, display, _))| {
                    if search.is_empty() {
                        true
                    } else {
                        key.to_lowercase().contains(&search.to_lowercase())
                            || display.to_lowercase().contains(&search.to_lowercase())
                    }
                })
                .map(|(idx, item)| (idx, &item.0, &item.1, &item.2))
                .collect();

            let indices: Vec<usize> = filtered.iter().map(|(idx, _, _, _)| *idx).collect();
            cache_data.0 = search.clone();
            cache_data.1 = indices;

            filtered
        }
    });

    let mut window_open = *open;

    egui::Window::new(title)
        .id(egui::Id::new("select_window"))
        .open(&mut window_open)
        .resizable(true)
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

            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .min_scrolled_height(300.0)
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
                        let (idx, key, display, _): (usize, &String, &String, &String) =
                            filtered_items[row.index()];
                        let is_selected = *selected == Some(idx);
                        row.set_selected(is_selected);

                        row.col(|ui| {
                            ui.label(key.as_str());
                        });
                        row.col(|ui| {
                            ui.label(display.as_str());
                        });

                        if row.response().clicked() {
                            *selected = Some(idx);
                        }
                    });
                });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button(tr!("confirm")).clicked() {
                    *open = false;
                }
                ui.add_space(10.0);
                if ui.button(tr!("cancel")).clicked() {
                    *open = false;
                }
            });
        });

    *open = window_open;
}
