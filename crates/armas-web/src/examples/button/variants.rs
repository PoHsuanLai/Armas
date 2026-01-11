// Button variants example
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;

        Button::new("Filled")
            .variant(ButtonVariant::Filled)
            .show(ui);

        Button::new("Outlined")
            .variant(ButtonVariant::Outlined)
            .show(ui);

        Button::new("Text")
            .variant(ButtonVariant::Text)
            .show(ui);
    });
}
