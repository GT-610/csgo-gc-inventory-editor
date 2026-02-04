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
    let filtered_items: Vec<(usize, &String, &String, &String)> = items
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

    let mut window_open = *open;

    egui::Window::new(title)
        .id(egui::Id::new("select_window"))
        .open(&mut window_open)
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .show(ctx, |ui| {
            ui.label(key_header);

            ui.horizontal(|ui| {
                let search_edit = ui.text_edit_singleline(search);
                if search_edit.changed() {
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
                        let (idx, key, display, _) = filtered_items[row.index()];
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
