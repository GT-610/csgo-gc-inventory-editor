use eframe::egui;

struct CsgoInventoryEditor;

impl eframe::App for CsgoInventoryEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CSGO-GC Inventory Editor");
            
            let system_theme = ctx.input(|i| i.raw.system_theme);
            if let Some(theme) = system_theme {
                ui.label(format!("System theme: {:?}", theme));
            } else {
                ui.label("System theme: unknown");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSGO-GC Inventory Editor",
        options,
        Box::new(|_cc| Ok(Box::new(CsgoInventoryEditor))),
    )
}
