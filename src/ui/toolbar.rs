use eframe::egui;

use crate::app::{CsgoInventoryEditor, InventoryCategory};

pub fn draw_toolbar(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("分类筛选:");
        egui::ComboBox::from_id_salt("category_combo")
            .selected_text(format!("{:?}", state.selected_category))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.selected_category, InventoryCategory::All, "全部");
                ui.selectable_value(&mut state.selected_category, InventoryCategory::Equipped, "装备");
                ui.selectable_value(&mut state.selected_category, InventoryCategory::StickerAndGraffiti, "印花与涂鸦");
                ui.selectable_value(&mut state.selected_category, InventoryCategory::CasesAndMore, "武器箱与更多");
                ui.selectable_value(&mut state.selected_category, InventoryCategory::Collectibles, "展示品");
            });
        
        ui.add_space(20.0);
        
        ui.label("搜索物品:");
        ui.text_edit_singleline(&mut state.search_query);
    });
    ui.add_space(8.0);
    
    ui.separator();
    
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        match state.selected_category {
            InventoryCategory::All => {
                let is_selected = state.selected_subcategory.is_none() || state.selected_subcategory.as_ref().is_some_and(|s| s == "全部");
                if ui.selectable_label(is_selected, "全部").clicked() {
                    state.selected_subcategory = None;
                }
            }
            InventoryCategory::Equipped => {
                let subcategories = ["全部", "全套装备", "近战武器", "手枪", "微型冲锋枪", "步枪", "重型武器", "探员", "手套", "音乐盒"];
                for sub in &subcategories {
                    let is_selected = state.selected_subcategory.as_ref().is_some_and(|s| s == *sub);
                    if ui.selectable_label(is_selected, *sub).clicked() {
                        state.selected_subcategory = Some(sub.to_string());
                    }
                }
            }
            InventoryCategory::StickerAndGraffiti => {
                let subcategories = ["全部艺术作品", "布章", "印花", "涂鸦"];
                for sub in &subcategories {
                    let is_selected = state.selected_subcategory.as_ref().is_some_and(|s| s == *sub);
                    if ui.selectable_label(is_selected, *sub).clicked() {
                        state.selected_subcategory = Some(sub.to_string());
                    }
                }
            }
            InventoryCategory::CasesAndMore => {
                let subcategories = ["所有武器箱", "印花胶嚢", "涂鸦箱", "纪念箱", "工具"];
                for sub in &subcategories {
                    let is_selected = state.selected_subcategory.as_ref().is_some_and(|s| s == *sub);
                    if ui.selectable_label(is_selected, *sub).clicked() {
                        state.selected_subcategory = Some(sub.to_string());
                    }
                }
            }
            InventoryCategory::Collectibles => {
                let subcategories = ["所有", "徽章", "音乐盒"];
                for sub in &subcategories {
                    let is_selected = state.selected_subcategory.as_ref().is_some_and(|s| s == *sub);
                    if ui.selectable_label(is_selected, *sub).clicked() {
                        state.selected_subcategory = Some(sub.to_string());
                    }
                }
            }
        }
    });
    ui.add_space(4.0);
}
