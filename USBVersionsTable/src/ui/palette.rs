use eframe::egui;

use crate::models::UsbApp;

pub fn show(app: &UsbApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.group(|ui| {
        ui.label("Available Rates and Market Names (click and drag):");
        ui.add_space(15.0);
        ui.vertical(|ui| {
            for chunk in app.drag_drop_options.chunks(5) {
                ui.horizontal(|ui| {
                    for rate in chunk {
                        let (rect, response) = ui.allocate_at_least(
                            egui::vec2(175.0, 30.0),
                            egui::Sense::click_and_drag(),
                        );

                        let visuals = ui.visuals();
                        let bg = if response.hovered() {
                            egui::Color32::from_rgb(80, 80, 180)
                        } else {
                            egui::Color32::DARK_GRAY
                        };
                        ui.painter().rect_filled(rect, 6.0, bg);
                        ui.painter().rect_stroke(
                            rect,
                            6.0,
                            egui::Stroke::new(1.0, visuals.widgets.active.bg_stroke.color),
                        );
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            rate,
                            egui::FontId::proportional(14.0),
                            egui::Color32::WHITE,
                        );

                        if response.drag_started() {
                            egui::DragAndDrop::set_payload(ctx, rate.clone());
                        }
                    }
                });
            }
        });
    });
}
