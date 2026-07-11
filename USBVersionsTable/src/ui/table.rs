use eframe::egui;

use crate::models::UsbApp;

pub fn show(app: &mut UsbApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    let dropped: Option<String> = if egui::DragAndDrop::has_any_payload(ctx)
        && ctx.input(|i| i.pointer.any_released())
    {
        egui::DragAndDrop::take_payload::<String>(ctx).map(|arc| (*arc).clone())
    } else {
        None
    };

    egui::Grid::new("usb_grid")
        .striped(true)
        .spacing([40.0, 10.0])
        .show(ui, |ui| {
            ui.label(egui::RichText::new("USB Version").strong());
            ui.label(egui::RichText::new("Transfer Rate").strong());
            ui.label(egui::RichText::new("Market Name").strong());
            ui.end_row();

            for row in &mut app.table_data {
                ui.label(&row.version);

                let (rect, response) =
                    ui.allocate_at_least(egui::vec2(175.0, 34.0), egui::Sense::hover());

                let is_active_drop_target =
                    response.hovered() && egui::DragAndDrop::has_any_payload(ctx);

                if let Some(ref val) = dropped {
                    if response.hovered() {
                        if let Some(_i) = app
                            .drag_drop_options
                            .iter()
                            .position(|r| r == val.as_str())
                        {
                            if app
                                .correct_transfer_rates
                                .get(&row.version)
                                .expect("REASON")
                                == val
                            {
                                row.transfer_rate = val.clone();
                            } else {
                                row.transfer_rate = "WRONG".to_owned();
                            }
                        }
                    }
                }

                let color = if is_active_drop_target {
                    egui::Color32::LIGHT_BLUE
                } else if row.transfer_rate == "WRONG" {
                    egui::Color32::RED
                } else if row.transfer_rate != "???" {
                    egui::Color32::from_rgb(100, 180, 100)
                } else {
                    egui::Color32::from_gray(90)
                };

                ui.painter().rect_filled(rect, 6.0, color);
                ui.painter().rect_stroke(
                    rect,
                    6.0,
                    egui::Stroke::new(
                        if is_active_drop_target { 2.0 } else { 1.0 },
                        egui::Color32::WHITE,
                    ),
                );
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &row.transfer_rate,
                    egui::FontId::proportional(15.0),
                    egui::Color32::WHITE,
                );

                let (rect2, response2) =
                    ui.allocate_at_least(egui::vec2(175.0, 34.0), egui::Sense::hover());

                if let Some(ref val) = dropped {
                    if response2.hovered() {
                        if let Some(_i) = app
                            .drag_drop_options
                            .iter()
                            .position(|r| r == val.as_str())
                        {
                            if app
                                .correct_market_names
                                .get(&row.version)
                                .expect("REASON")
                                == val
                            {
                                row.market_name = val.clone();
                            } else {
                                row.market_name = "WRONG".to_owned();
                            }
                        }
                    }
                }

                let is_active_drop_target2 =
                    response2.hovered() && egui::DragAndDrop::has_any_payload(ctx);

                let color2 = if is_active_drop_target2 {
                    egui::Color32::LIGHT_BLUE
                } else if row.market_name == "WRONG" {
                    egui::Color32::RED
                } else if row.market_name != "???" {
                    egui::Color32::from_rgb(100, 180, 100)
                } else {
                    egui::Color32::from_gray(90)
                };

                ui.painter().rect_filled(rect2, 6.0, color2);
                ui.painter().rect_stroke(
                    rect2,
                    6.0,
                    egui::Stroke::new(
                        if is_active_drop_target2 { 2.0 } else { 1.0 },
                        egui::Color32::WHITE,
                    ),
                );
                ui.painter().text(
                    rect2.center(),
                    egui::Align2::CENTER_CENTER,
                    &row.market_name,
                    egui::FontId::proportional(15.0),
                    egui::Color32::WHITE,
                );

                ui.end_row();
            }
        });
}
