use eframe::egui::ahash::HashMap;

pub struct UsbRow {
    pub version: String,
    pub transfer_rate: String,
    pub market_name: String,
}

pub struct UsbApp {
    pub drag_drop_options: Vec<String>,
    pub table_data: Vec<UsbRow>,
    pub correct_transfer_rates: HashMap<String, String>,
    pub correct_market_names: HashMap<String, String>,
}
