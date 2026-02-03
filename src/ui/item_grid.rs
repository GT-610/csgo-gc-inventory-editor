use eframe::egui;
use crate::app::CsgoInventoryEditor;
use crate::app::Rarity;

pub fn draw_item_grid(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    let items_per_row = 8;
    let card_height = 100.0;
    let spacing = 8.0;

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(8.0);

        let available_width = ui.available_width();
        let total_spacing = spacing * (items_per_row as f32 - 1.0);
        let card_width = (available_width - total_spacing) / items_per_row as f32;

        let font_size = (card_width * 0.16).clamp(12.0, 20.0);

        egui::Grid::new("item_grid")
            .num_columns(items_per_row)
            .spacing([spacing, spacing])
            .min_col_width(card_width)
            .min_row_height(card_height)
            .show(ui, |ui| {
                let mut sorted_items: Vec<_> = state.inventory.items.iter().collect();
                sorted_items.sort_by_key(|item| item.inventory);
                
                for (i, item) in sorted_items.iter().enumerate() {
                    let display_name = state.get_item_display_name(item);
                    let rarity = Rarity::from_u32(item.rarity);

                    let (card_rect, card_response) = ui.allocate_exact_size(
                        egui::Vec2::new(card_width, card_height),
                        egui::Sense::click(),
                    );

                    let bg_color = if card_response.clicked() {
                        ui.visuals().widgets.active.bg_fill
                    } else if card_response.hovered() {
                        ui.visuals().widgets.hovered.bg_fill
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    };

                    let stroke = if card_response.hovered() || card_response.clicked() {
                        ui.visuals().widgets.hovered.bg_stroke
                    } else {
                        ui.visuals().widgets.inactive.bg_stroke
                    };

                    let corner_radius = egui::CornerRadius::same(4);
                    ui.painter().rect_filled(card_rect, corner_radius, bg_color);
                    ui.painter().rect_stroke(card_rect, corner_radius, stroke, egui::StrokeKind::Middle);

                    if let Some(color) = rarity.color() {
                        let indicator_width = 4.0;
                        let indicator_margin = 2.0;
                        let indicator_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(
                                card_rect.min.x + indicator_margin,
                                card_rect.min.y + indicator_margin,
                            ),
                            egui::Vec2::new(
                                indicator_width,
                                card_height - 2.0 * indicator_margin,
                            ),
                        );
                        ui.painter().rect_filled(indicator_rect, corner_radius, color);
                    }

                    let text_margin = 8.0;
                    let indicator_space = if rarity.color().is_some() { 10.0 } else { 4.0 };
                    
                    let text_start_x = card_rect.min.x + text_margin + indicator_space;
                    let text_max_width = card_width - 2.0 * text_margin - indicator_space;
                    let text_max_height = card_height - 2.0 * text_margin;
                    let _max_lines_per_text = 1;
                    
                    let id_font_size = (font_size * 0.7).clamp(10.0, 14.0);
                    let id_text = format!("#{}", item.inventory);
                    
                    let padding = 4.0;
                    let actual_wrap_width = text_max_width - padding;
                    let name_max_lines = 2;
                    
                    let id_galley = ui.painter().fonts_mut(|fonts| {
                        fonts.layout(
                            id_text.clone(),
                            egui::FontId::proportional(id_font_size),
                            ui.visuals().text_color(),
                            text_max_width,
                        )
                    });
                    
                    let id_height = id_galley.size().y;
                    
                    let name_available_height = text_max_height - id_height - 4.0;
                    let min_font_size = 10.0;
                    
                    let final_font_size = ui.painter().fonts_mut(|fonts| {
                        let mut current_font_size = font_size;
                        
                        while current_font_size >= min_font_size {
                            let galley = fonts.layout(
                                display_name.clone(),
                                egui::FontId::proportional(current_font_size),
                                ui.visuals().text_color(),
                                actual_wrap_width,
                            );
                            
                            let galley_rows = galley.rows.len();
                            let galley_height = galley.size().y;
                            
                            if galley_rows <= name_max_lines && galley_height <= name_available_height {
                                break;
                            }
                            
                            current_font_size -= 1.0;
                        }
                        
                        current_font_size
                    });

                    let id_text_start_y = card_rect.min.y + text_margin;
                    ui.painter().galley(
                        egui::Pos2::new(text_start_x, id_text_start_y),
                        id_galley,
                        ui.visuals().text_color(),
                    );

                    let name_text_start_y = id_text_start_y + id_height + 4.0;
                    let name_galley = ui.painter().fonts_mut(|fonts| {
                        fonts.layout(
                            display_name.clone(),
                            egui::FontId::proportional(final_font_size),
                            ui.visuals().text_color(),
                            actual_wrap_width,
                        )
                    });
                    
                    ui.painter().galley(
                        egui::Pos2::new(text_start_x, name_text_start_y),
                        name_galley,
                        ui.visuals().text_color(),
                    );

                    if card_response.clicked() {
                        state.open_item_windows.insert(item.inventory);
                    }

                    if (i + 1) % items_per_row == 0 {
                        ui.end_row();
                    }
                }
            });

        ui.add_space(8.0);
    });
}
