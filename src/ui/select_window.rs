use eframe::egui;
use egui_extras::{Column, TableBuilder};

pub fn draw_select_window(
    ctx: &egui::Context,
    open: &mut bool,
    title: &str,
    key_header: &str,
    value_header: &str,
    items: &[(String, String, String)],
    search_query: &mut String,
    selected: &mut Option<usize>,
    window_seed: Option<u64>,
) {
    let mut should_close = false;

    let window_id = egui::Id::new(format!("select_window_{}_{}", title.replace(' ', "_"), window_seed.unwrap_or(0)));

    egui::Window::new(title)
        .id(window_id)
        .open(open)
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(key_header);
                ui.add_space(10.0);
                ui.text_edit_singleline(search_query);
            });
            
            ui.separator();
            
            let search_query_clone = search_query.clone();
            let filtered_items: Vec<(usize, String, String)> = items.iter()
                .enumerate()
                .filter(|(_, (key, display, _))| {
                    if search_query_clone.is_empty() {
                        true
                    } else {
                        key.contains(&search_query_clone) || display.contains(&search_query_clone)
                    }
                })
                .map(|(idx, (key, display, _))| {
                    (idx, key.clone(), display.clone())
                })
                .collect();
            
            let text_height = egui::TextStyle::Body.resolve(ui.style()).size.max(ui.spacing().interact_size.y);
            
            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::remainder())
                .min_scrolled_height(300.0)
                .max_scroll_height(400.0)
                .sense(egui::Sense::click());
            
            table
                .header(text_height, |mut header| {
                    header.col(|ui| {
                        ui.strong(key_header);
                    });
                    header.col(|ui| {
                        ui.strong(value_header);
                    });
                })
                .body(|body| {
                    body.rows(text_height, filtered_items.len(), |mut row| {
                        let row_idx = row.index();
                        if let Some((idx, key, display)) = filtered_items.get(row_idx) {
                            row.set_selected(*selected == Some(*idx));
                            
                            row.col(|ui| {
                                ui.label(key.as_str());
                            });
                            row.col(|ui| {
                                ui.label(display.as_str());
                            });
                            
                            if row.response().clicked() {
                                *selected = Some(*idx);
                            }
                        }
                    });
                });
            
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("确认").clicked() {
                    if let Some(selected_idx) = *selected {
                        if let Some((key, display, value)) = items.get(selected_idx) {
                            println!("Selected: Key={}, Display={}, Value={}", key, display, value);
                        }
                        should_close = true;
                    }
                }
                ui.add_space(10.0);
                if ui.button("取消").clicked() {
                    should_close = true;
                }
            });
        });
    
    if should_close {
        *open = false;
    }
}
