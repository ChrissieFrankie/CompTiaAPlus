use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::models::{UsbApp, UsbRow};

impl Default for UsbApp {
    fn default() -> Self {
        let mut rng = thread_rng();

        let mut drag_drop_options = vec![
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
