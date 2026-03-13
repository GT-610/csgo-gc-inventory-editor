use crate::app::CsgoInventoryEditor;
use eframe::egui;

pub fn draw_inventory_page(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    if state.has_inventory_error() {
        if let Some(error) = state.get_inventory_error() {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading(egui::RichText::new("读取库存错误").size(24.0).color(egui::Color32::RED));
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new(error).size(16.0).color(egui::Color32::LIGHT_RED));
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new("请检查游戏目录设置或文件权限").size(14.0).color(egui::Color32::GRAY));
                });
            });
        }
    } else {
        egui::TopBottomPanel::top("toolbar").show_inside(ui, |ui| {
            crate::ui::draw_toolbar(ui, state);
        });

        crate::ui::draw_item_grid(ui, state);
    }
}
