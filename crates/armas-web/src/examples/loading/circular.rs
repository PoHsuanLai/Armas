// Circular progress indicator
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    let theme = ui.ctx().armas_theme();

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 24.0;

        // Indeterminate spinner
        CircularProgress::new()
            .size(40.0)
            .color(theme.primary())
            .show(ui);

        // Determinate progress bars
        CircularProgressBar::new(0.6)
            .size(48.0)
            .color(theme.success())
            .show(ui);

        CircularProgressBar::new(0.9)
            .size(48.0)
            .color(theme.info())
            .show_percentage(true)
            .show(ui);
    });
}
