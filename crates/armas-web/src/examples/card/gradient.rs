// Gradient card example
use armas::*;
use eframe::egui;

pub fn demo(ui: &mut egui::Ui) {
    let theme = ui.ctx().armas_theme();
    GradientCard::rainbow()
        .width(250.0)
        .height(150.0)
        .show(ui, &theme, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading(egui::RichText::new("Gradient").color(egui::Color32::WHITE));
                ui.label(
                    egui::RichText::new("Rainbow colors").color(egui::Color32::from_gray(230)),
                );
            });
        });
}
