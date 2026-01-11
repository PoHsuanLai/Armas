// Glass panel card example
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    let theme = ui.ctx().armas_theme();

    GlassPanel::new()
        .blur(10.0)
        .opacity(0.05)
        .corner_radius(12.0)
        .show(ui, &theme, |ui| {
            ui.set_min_size(egui::vec2(250.0, 150.0));
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading("Glass Panel");
                ui.add_space(8.0);
                ui.label("Beautiful glassmorphism effect");
            });
        });
}
