// Shimmer button example
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    ShimmerButton::new("Shimmer Effect")
        .min_size(egui::vec2(140.0, 48.0))
        .show(ui);
}
