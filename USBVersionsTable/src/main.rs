mod app;
mod data;
mod models;
mod ui;

use models::UsbApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "USB Table Quiz",
        options,
        Box::new(|_cc| Box::new(UsbApp::default()) as Box<dyn eframe::App>),
    )
}
