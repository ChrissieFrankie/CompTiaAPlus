use eframe::egui::{self, ahash::HashMap}; // use gui library
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default(); // default window settings
    eframe::run_native(
        "USB Table Quiz",
        options,
        Box::new(|_cc| Box::new(UsbApp::default()) as Box<dyn eframe::App>), // expect some box
    )
}

struct UsbRow { // a row of the versions table
    version: String, // usb gen 1 idk
    rate: String,
}

struct UsbApp { // app data
    available_rates: Vec<String>, // available responses
    table_data: Vec<UsbRow>,      // empty response boxes
    correct_rates: HashMap<String, String>, // correct rates order
}

impl Default for UsbApp {
    fn default() -> Self {
        Self {
            available_rates: vec![ // usb rates
                "5 Gbps".to_string(),
                "20 Gbps".to_string(),
                "1.5 Mbps".to_string(),
                "40 Gbps".to_string(),
                "12 Mbps".to_string(),
                "480 Mbps".to_string(),
                "10 Gbps".to_string(),
            ],
            table_data: vec![ // initial table
                UsbRow { version: "USB 2.0 LowSpeed".into(),       rate: "???".into() },
                UsbRow { version: "USB 2.0 FullSpeed".into(),       rate: "???".into() },
                UsbRow { version: "USB 2.0 HiSpeed".into(),       rate: "???".into() },
                UsbRow { version: "USB 3.2 Gen 1".into(), rate: "???".into() },
                UsbRow { version: "USB 3.2 Gen 2".into(), rate: "???".into() },
                UsbRow { version: "USB 3.2 Gen 2x2".into(), rate: "???".into() },
                UsbRow { version: "USB 4".into(),         rate: "???".into() },
            ],
            correct_rates: [ // correct order for the rates
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
        }
    }
}

impl eframe::App for UsbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { // basically a game loop
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
            ui.heading("Drag the Transfer Rate to the correct USB Version");
            ui.add_space(20.0);

            ui.group(|ui| {
                ui.label("Available Rates (click and drag):");
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    for rate in &self.available_rates {

                        let (rect, response) = ui.allocate_at_least(
                            egui::vec2(90.0, 32.0),
                            egui::Sense::click_and_drag(),
                        );

                        let visuals = ui.visuals();
                        let bg = if response.hovered() { // set background color for hovered/unhovered
                            egui::Color32::from_rgb(80, 80, 180) 
                        } else {
                            egui::Color32::DARK_GRAY
                        };
                        ui.painter().rect_filled(rect, 6.0, bg); // set border color
                        ui.painter().rect_stroke(                
                            rect, 6.0,
                            egui::Stroke::new(1.0, visuals.widgets.active.bg_stroke.color)
                        );
                        ui.painter().text( // center label                         
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            rate,
                            egui::FontId::proportional(14.0),
                            egui::Color32::WHITE,
                        );

                        if response.drag_started() { // rate persists when dragged
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
                    ui.label(egui::RichText::new("Transfer Rate (drop here)").strong());
                    ui.end_row(); // draw next row

                    for row in &mut self.table_data {
                        ui.label(&row.version);
                        let (rect, response) = ui.allocate_at_least( // hover detect
                            egui::vec2(160.0, 34.0),
                            egui::Sense::hover(),
                        );

                        let is_active_drop_target = response.hovered() // it's being hovered
                            && egui::DragAndDrop::has_any_payload(ctx);

                        if let Some(ref val) = dropped { // perform drop
                            if response.hovered() {
                                if let Some(_i) = self.available_rates.iter().position(|r| r == val.as_str()) {
                                    if self.correct_rates.get(&row.version.to_string()).expect("REASON").to_string() == val.to_string()
                                    {
                                        row.rate = val.clone(); 
                                    }
                                    else {
                                        row.rate = "WRONG".to_owned();
                                    }

                                }
                            }
                        }

                        let color = if is_active_drop_target {
                            egui::Color32::LIGHT_BLUE               // drop here
                        }
                        else if row.rate == "WRONG" {
                            egui::Color32::RED
                        } 
                        else if row.rate != "???" {
                            egui::Color32::from_rgb(100, 180, 100) // answered
                        }
                        else {
                            egui::Color32::from_gray(90)            // empty
                        };

                        ui.painter().rect_filled(rect, 6.0, color); // paint drop

                        ui.painter().rect_stroke(
                            rect, 6.0,
                            egui::Stroke::new(
                                if is_active_drop_target { 2.0 } else { 1.0 }, // thicker border when targeted
                                egui::Color32::WHITE,
                            ),
                        );
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &row.rate,
                            egui::FontId::proportional(15.0),
                            egui::Color32::WHITE,
                        );

                    


                        ui.end_row();
                    }
                });

            ui.add_space(20.0);
            if ui.button("Reset Table").clicked() { // reset table
                for row in &mut self.table_data {
                    row.rate = "???".into();
                }
            }
        });
    }
}