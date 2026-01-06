//! Dot Pattern Background Example
//!
//! Demonstrates dot pattern backgrounds with various configurations

use armas::{DotPattern, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Armas - Dot Pattern Backgrounds"),
        ..Default::default()
    };

    eframe::run_native(
        "Dot Pattern",
        options,
        Box::new(|_cc| Ok(Box::new(DotPatternApp::new()))),
    )
}

struct DotPatternApp {
    theme: Theme,
}

impl DotPatternApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
        }
    }
}

impl eframe::App for DotPatternApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Dot Pattern Backgrounds");
                    ui.add_space(10.0);
                    ui.label("Simple, elegant dot grid patterns for backgrounds and overlays");
                    ui.add_space(30.0);
                });

                ui.horizontal_top(|ui| {
                    ui.add_space(40.0);

                    // Left column
                    ui.vertical(|ui| {
                        ui.set_width(500.0);

                        // Basic dot pattern
                        ui.label(
                            egui::RichText::new("Basic Dot Pattern")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);

                        DotPattern::new(480.0, 300.0, &self.theme).show(ui, &self.theme);

                        ui.add_space(30.0);

                        // Smaller spacing
                        ui.label(
                            egui::RichText::new("Dense Pattern (spacing: 15px)")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);

                        DotPattern::new(480.0, 300.0, &self.theme)
                            .spacing(15.0)
                            .show(ui, &self.theme);

                        ui.add_space(30.0);

                        // Larger dots
                        ui.label(
                            egui::RichText::new("Larger Dots (radius: 2.5px)")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);

                        DotPattern::new(480.0, 300.0, &self.theme)
                            .dot_radius(2.5)
                            .show(ui, &self.theme);
                    });

                    ui.add_space(40.0);

                    // Right column
                    ui.vertical(|ui| {
                        ui.set_width(500.0);

                        // Fade effect
                        ui.label(
                            egui::RichText::new("Fade from Center")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);

                        DotPattern::new(480.0, 300.0, &self.theme)
                            .fade(0.7)
                            .show(ui, &self.theme);

                        ui.add_space(30.0);

                        // Glow effect
                        ui.label(
                            egui::RichText::new("With Glow Effect")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);

                        DotPattern::new(480.0, 300.0, &self.theme)
                            .with_glow(true)
                            .dot_radius(2.0)
                            .show(ui, &self.theme);

                        ui.add_space(30.0);

                        // Custom color
                        ui.label(
                            egui::RichText::new("Custom Color (Blue)")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);

                        DotPattern::new(480.0, 300.0, &self.theme)
                            .color(egui::Color32::from_rgba_unmultiplied(59, 130, 246, 100))
                            .dot_radius(2.0)
                            .with_glow(true)
                            .show(ui, &self.theme);
                    });
                });

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(20.0);

                // Large example with content overlay
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Dot Pattern as Background Layer")
                            .size(16.0)
                            .strong()
                            .color(self.theme.on_surface()),
                    );
                    ui.add_space(20.0);

                    // Create a layered effect
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(1000.0, 400.0), egui::Sense::hover());

                    ui.allocate_ui_at_rect(rect, |ui| {
                        // Background layer
                        DotPattern::new(1000.0, 400.0, &self.theme)
                            .spacing(25.0)
                            .fade(0.8)
                            .with_glow(true)
                            .show(ui, &self.theme);
                    });

                    // Content layer (on top)
                    ui.allocate_ui_at_rect(rect, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(120.0);
                            ui.heading(
                                egui::RichText::new("Beautiful Backgrounds")
                                    .size(48.0)
                                    .color(egui::Color32::WHITE),
                            );
                            ui.add_space(20.0);
                            ui.label(
                                egui::RichText::new(
                                    "Dot patterns create subtle, professional backgrounds",
                                )
                                .size(18.0)
                                .color(egui::Color32::from_gray(180)),
                            );
                            ui.add_space(30.0);

                            ui.horizontal(|ui| {
                                ui.add_space(380.0);
                                armas::Button::new("Get Started")
                                    .variant(armas::ButtonVariant::Filled)
                                    .show(ui, &self.theme);
                                ui.add_space(20.0);
                                armas::Button::new("Learn More")
                                    .variant(armas::ButtonVariant::Outlined)
                                    .show(ui, &self.theme);
                            });
                        });
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
