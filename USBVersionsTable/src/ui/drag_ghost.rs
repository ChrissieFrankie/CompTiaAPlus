use eframe::egui;

pub fn show(ctx: &egui::Context) {
    if let Some(dragged) = egui::DragAndDrop::payload::<String>(ctx) {
        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
            egui::Area::new(egui::Id::new("drag_ghost"))
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
}
