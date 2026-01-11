// Loading dots indicator
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    let theme = ui.ctx().armas_theme();

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 16.0;

        LoadingDots::new()
            .dot_size(8.0)
            .spacing(8.0)
            .color(theme.primary())
            .show(ui);

        LoadingDots::new()
            .dot_size(12.0)
            .spacing(12.0)
            .color(theme.secondary())
            .show(ui);
    });
}
