use eframe::egui::{self, ahash::HashMap}; // use gui library
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default(); // default window settings
    eframe::run_native(
        "USB Table Quiz",
        options,
        Box::new(|_cc| Box::new(UsbApp::default()) as Box<dyn eframe::App>), // expect some box
    )
}

struct UsbRow {
    // a row of the versions table
    version: String, // usb gen 1 idk
    transfer_rate: String,
    market_name: String,
}

struct UsbApp {
    // app data
    drag_drop_options: Vec<String>, // available responses
    table_data: Vec<UsbRow>,        // empty response boxes
    correct_transfer_rates: HashMap<String, String>, // correct rates order
    correct_market_names: HashMap<String, String>,
}

impl Default for UsbApp {
    fn default() -> Self {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();

        let mut drag_drop_options = vec![
            // usb rates
            "5 Gbps".to_string(),
            "20 Gbps".to_string(),
            "1.5 Mbps".to_string(),
            "40 Gbps".to_string(),
            "12 Mbps".to_string(),
            "480 Mbps".to_string(),
            "10 Gbps".to_string(),
            "Low Speed".into(),
            "Full Speed".into(),
            "Hi-Speed".into(),
            "SuperSpeed USB".into(),
            "SuperSpeed USB 10Gbps".into(),
            "SuperSpeed USB 20Gbps".into(),
            "40 Gbps".into(),
        ];
        drag_drop_options.shuffle(&mut rng);

        let mut table_data = vec![
            // initial table
            UsbRow {
                version: "USB 2.0 LowSpeed".into(),
                transfer_rate: "???".into(),
                market_name: "???".into(),
            },
            UsbRow {
                version: "USB 2.0 FullSpeed".into(),
                transfer_rate: "???".into(),
                market_name: "???".into(),
            },
            UsbRow {
                version: "USB 2.0 HiSpeed".into(),
                transfer_rate: "???".into(),
                market_name: "???".into(),
            },
            UsbRow {
                version: "USB 3.2 Gen 1".into(),
                transfer_rate: "???".into(),
                market_name: "???".into(),
            },
            UsbRow {
                version: "USB 3.2 Gen 2".into(),
                transfer_rate: "???".into(),
                market_name: "???".into(),
            },
            UsbRow {
                version: "USB 3.2 Gen 2x2".into(),
                transfer_rate: "???".into(),
                market_name: "???".into(),
            },
            UsbRow {
                version: "USB 4".into(),
                transfer_rate: "???".into(),
                market_name: "???".into(),
            },
        ];
        table_data.shuffle(&mut rng);

        Self {
            drag_drop_options,
            table_data,
            correct_transfer_rates: [
                // correct order for the rates
                ("USB 2.0 LowSpeed".into(), "1.5 Mbps".into()),
                ("USB 2.0 FullSpeed".into(), "12 Mbps".into()),
                ("USB 2.0 HiSpeed".into(), "480 Mbps".into()),
                ("USB 3.2 Gen 1".into(), "5 Gbps".into()),
                ("USB 3.2 Gen 2".into(), "10 Gbps".into()),
                ("USB 3.2 Gen 2x2".into(), "20 Gbps".into()),
                ("USB 4".into(), "40 Gbps".into()),
            ]
            .into_iter()
            .collect(),
            correct_market_names: [
                ("USB 2.0 LowSpeed".into(), "Low Speed".into()),
                ("USB 2.0 FullSpeed".into(), "Full Speed".into()),
                ("USB 2.0 HiSpeed".into(), "Hi-Speed".into()),
                ("USB 3.2 Gen 1".into(), "SuperSpeed USB".into()),
                ("USB 3.2 Gen 2".into(), "SuperSpeed USB 10Gbps".into()),
                ("USB 3.2 Gen 2x2".into(), "SuperSpeed USB 20Gbps".into()),
                ("USB 4".into(), "40 Gbps".into()),
            ]
            .into_iter()
            .collect(),
        }
    }
}

impl eframe::App for UsbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // basically a game loop
        if let Some(dragged) = egui::DragAndDrop::payload::<String>(ctx) {
            if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                egui::Area::new(egui::Id::new("drag_ghost")) // show a label to indicate dragging
                    .fixed_pos(pos + egui::vec2(12.0, -8.0))
                    .order(egui::Order::Tooltip)
                    .interactable(false)
                    .show(ctx, |ui| {
                        ui.label(
                            egui::RichText::new(dragged.as_str())
                                .background_color(egui::Color32::YELLOW)
                                .color(egui::Color32::BLACK)
                                .size(16.0),
                        );
                    });
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drag to the correct USB Version");
            ui.add_space(20.0);

            ui.group(|ui| {
                ui.label("Available Rates and Market Names (click and drag):");
                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    for rate in &self.drag_drop_options {
                        let (rect, response) = ui.allocate_at_least(
                            egui::vec2(90.0, 60.0),
                            egui::Sense::click_and_drag(),
                        );

                        let visuals = ui.visuals();
                        let bg = if response.hovered() {
                            // set background color for hovered/unhovered
                            egui::Color32::from_rgb(80, 80, 180)
                        } else {
                            egui::Color32::DARK_GRAY
                        };
                        ui.painter().rect_filled(rect, 6.0, bg); // set border color
                        ui.painter().rect_stroke(
                            rect,
                            6.0,
                            egui::Stroke::new(1.0, visuals.widgets.active.bg_stroke.color),
                        );
                        ui.painter().text(
                            // center label
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            rate,
                            egui::FontId::proportional(14.0),
                            egui::Color32::WHITE,
                        );

                        if response.drag_started() {
                            // rate persists when dragged
                            egui::DragAndDrop::set_payload(ctx, rate.clone());
                        }
                    }
                });
            });

            ui.add_space(30.0);

            let dropped: Option<String> = if egui::DragAndDrop::has_any_payload(ctx) // the option has been released
                && ctx.input(|i| i.pointer.any_released())
            {
                egui::DragAndDrop::take_payload::<String>(ctx).map(|arc| (*arc).clone()) // the data has been cloned
            } else {
                None
            };

            egui::Grid::new("usb_grid") // create table
                .striped(true)
                .spacing([40.0, 10.0])
                .show(ui, |ui| {
                    ui.label(egui::RichText::new("USB Version").strong());
                    ui.label(egui::RichText::new("Transfer Rate").strong());
                    ui.label(egui::RichText::new("Market Name").strong());
                    ui.end_row(); // draw next row

                    for row in &mut self.table_data {
                        ui.label(&row.version);

                        // transfer rate drop zone
                        let (rect, response) = ui.allocate_at_least(
                            // hover detect
                            egui::vec2(160.0, 34.0),
                            egui::Sense::hover(),
                        );

                        let is_active_drop_target = response.hovered() // it's being hovered
                            && egui::DragAndDrop::has_any_payload(ctx);

                        if let Some(ref val) = dropped {
                            // perform drop
                            if response.hovered() {
                                if let Some(_i) = self
                                    .drag_drop_options
                                    .iter()
                                    .position(|r| r == val.as_str())
                                {
                                    if self
                                        .correct_transfer_rates
                                        .get(&row.version.to_string())
                                        .expect("REASON")
                                        .to_string()
                                        == val.to_string()
                                    {
                                        row.transfer_rate = val.clone();
                                    } else {
                                        row.transfer_rate = "WRONG".to_owned();
                                    }
                                }
                            }
                        }

                        let color = if is_active_drop_target {
                            egui::Color32::LIGHT_BLUE // drop here
                        } else if row.transfer_rate == "WRONG" {
                            egui::Color32::RED
                        } else if row.transfer_rate != "???" {
                            egui::Color32::from_rgb(100, 180, 100) // answered
                        } else {
                            egui::Color32::from_gray(90) // empty
                        };

                        ui.painter().rect_filled(rect, 6.0, color); // paint drop

                        ui.painter().rect_stroke(
                            rect,
                            6.0,
                            egui::Stroke::new(
                                if is_active_drop_target { 2.0 } else { 1.0 }, // thicker border when targeted
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

                        // market name drop zone
                        let (rect2, response2) =
                            ui.allocate_at_least(egui::vec2(160.0, 34.0), egui::Sense::hover());

                        if let Some(ref val) = dropped {
                            // perform drop
                            if response2.hovered() {
                                if let Some(_i) = self
                                    .drag_drop_options
                                    .iter()
                                    .position(|r| r == val.as_str())
                                {
                                    if self
                                        .correct_market_names
                                        .get(&row.version.to_string())
                                        .expect("REASON")
                                        .to_string()
                                        == val.to_string()
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

            ui.add_space(20.0);
            if ui.button("Reset Table").clicked() {
                // reset table
                for row in &mut self.table_data {
                    row.transfer_rate = "???".into();
                    row.market_name = "???".into();
                }
            }
        });
    }
}
