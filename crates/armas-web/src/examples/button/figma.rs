// Figma-style button examples
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;

        FigmaButton::new("Default")
            .min_size(egui::vec2(100.0, 36.0))
            .show(ui);

        FigmaButton::outlined("Outlined")
            .min_size(egui::vec2(100.0, 36.0))
            .show(ui);
    });
}
