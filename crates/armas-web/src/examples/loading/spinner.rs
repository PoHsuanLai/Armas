// Spinner loading indicator
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    let theme = ui.ctx().armas_theme();

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 24.0;

        Spinner::new()
            .size(32.0)
            .color(theme.primary())
            .show(ui);

        Spinner::new()
            .size(48.0)
            .color(theme.secondary())
            .show(ui);

        Spinner::new()
            .size(64.0)
            .color(theme.info())
            .show(ui);
    });
}
