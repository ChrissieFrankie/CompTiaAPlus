use eframe::egui;

use crate::models::UsbApp;
use crate::ui::{drag_ghost, palette, table};

impl eframe::App for UsbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        drag_ghost::show(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drag to the correct USB Version");
            ui.add_space(20.0);

            palette::show(self, ctx, ui);

            ui.add_space(30.0);

            table::show(self, ctx, ui);

            ui.add_space(20.0);
            if ui.button("Reset Table").clicked() {
                for row in &mut self.table_data {
                    row.transfer_rate = "???".into();
                    row.market_name = "???".into();
                }
            }
        });
    }
}
