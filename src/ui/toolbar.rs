use eframe::egui;
use egui_i18n::tr;

use crate::app::{CsgoInventoryEditor, InventoryCategory};

pub fn draw_toolbar(ui: &mut egui::Ui, state: &mut CsgoInventoryEditor) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        // ui.label(tr!("category-filter"));
        // let category_display = match state.selected_category {
        //     InventoryCategory::All => tr!("category-all"),
        //     InventoryCategory::Equipped => tr!("category-equipped"),
        //     InventoryCategory::StickerAndGraffiti => tr!("category-stickers"),
        //     InventoryCategory::CasesAndMore => tr!("category-cases"),
        //     InventoryCategory::Collectibles => tr!("category-collectibles"),
        // };
        // egui::ComboBox::from_id_salt("category_combo")
        //     .selected_text(category_display)
        //     .show_ui(ui, |ui| {
        //         ui.selectable_value(&mut state.selected_category, InventoryCategory::All, tr!("category-all"));
        //         ui.selectable_value(&mut state.selected_category, InventoryCategory::Equipped, tr!("category-equipped"));
        //         ui.selectable_value(&mut state.selected_category, InventoryCategory::StickerAndGraffiti, tr!("category-stickers"));
        //         ui.selectable_value(&mut state.selected_category, InventoryCategory::CasesAndMore, tr!("category-cases"));
        //         ui.selectable_value(&mut state.selected_category, InventoryCategory::Collectibles, tr!("category-collectibles"));
        //     });
        
        // ui.add_space(20.0);
        
        if ui.button(tr!("btn-add-item")).clicked() {
            state.show_template_modal = true;
        }
    });
    ui.add_space(8.0);
    
    // ui.separator();
    
    // ui.add_space(4.0);
    // ui.horizontal(|ui| {
    //     match state.selected_category {
    //         InventoryCategory::All => {
    //             let subcategory_all = tr!("subcategory-all");
    //             let is_selected = state.selected_subcategory.is_none() || state.selected_subcategory.as_ref().is_some_and(|s| s == &subcategory_all);
    //             if ui.selectable_label(is_selected, &subcategory_all).clicked() {
    //                 state.selected_subcategory = None;
    //             }
    //         }
    //         InventoryCategory::Equipped => {
    //             let subcategories = [
    //                 tr!("subcategory-all"),
    //                 tr!("subcategory-full-set"),
    //                 tr!("subcategory-melee"),
    //                 tr!("subcategory-pistol"),
    //                 tr!("subcategory-smg"),
    //                 tr!("subcategory-rifle"),
    //                 tr!("subcategory-heavy"),
    //                 tr!("subcategory-agent"),
    //                 tr!("subcategory-gloves"),
    //                 tr!("subcategory-music-kit"),
    //             ];
    //             for sub in &subcategories {
    //                 let is_selected = state.selected_subcategory.as_ref().is_some_and(|s| s == sub);
    //                 if ui.selectable_label(is_selected, sub).clicked() {
    //                     state.selected_subcategory = Some(sub.to_string());
    //                 }
    //             }
    //         }
    //         InventoryCategory::StickerAndGraffiti => {
    //             let subcategories = [
    //                 tr!("subcategory-all-artworks"),
    //                 tr!("subcategory-pins"),
    //                 tr!("subcategory-stickers"),
    //                 tr!("subcategory-graffiti"),
    //             ];
    //             for sub in &subcategories {
    //                 let is_selected = state.selected_subcategory.as_ref().is_some_and(|s| s == sub);
    //                 if ui.selectable_label(is_selected, sub).clicked() {
    //                     state.selected_subcategory = Some(sub.to_string());
    //                 }
    //             }
    //         }
    //         InventoryCategory::CasesAndMore => {
    //             let subcategories = [
    //                 tr!("subcategory-all-cases"),
    //                 tr!("subcategory-sticker-capsule"),
    //                 tr!("subcategory-graffiti-box"),
    //                 tr!("subcategory-souvenir"),
    //                 tr!("subcategory-tools"),
    //             ];
    //             for sub in &subcategories {
    //                 let is_selected = state.selected_subcategory.as_ref().is_some_and(|s| s == sub);
    //                 if ui.selectable_label(is_selected, sub).clicked() {
    //                     state.selected_subcategory = Some(sub.to_string());
    //                 }
    //             }
    //         }
    //         InventoryCategory::Collectibles => {
    //             let subcategories = [
    //                 tr!("subcategory-all-items"),
    //                 tr!("subcategory-badges"),
    //                 tr!("subcategory-music-kits"),
    //             ];
    //             for sub in &subcategories {
    //                 let is_selected = state.selected_subcategory.as_ref().is_some_and(|s| s == sub);
    //                 if ui.selectable_label(is_selected, sub).clicked() {
    //                     state.selected_subcategory = Some(sub.to_string());
    //                 }
    //             }
    //         }
    //     }
    // });
    ui.add_space(4.0);
}
