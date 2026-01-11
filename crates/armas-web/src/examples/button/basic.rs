// Basic button example
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    Button::new("Click me")
        .variant(ButtonVariant::Filled)
        .show(ui);
}
