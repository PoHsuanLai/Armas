//! Sparkles Effect Example
//!
//! Demonstrates twinkling sparkle particles

use armas::ext::ArmasContextExt;
use armas::{Sparkles, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Sparkles Effect"),
        ..Default::default()
    };

    eframe::run_native(
        "Sparkles",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(SparklesApp::new()))
        }),
    )
}

struct SparklesApp {
    sparkles1: Sparkles,
    sparkles2: Sparkles,
    sparkles3: Sparkles,
}

impl SparklesApp {
    fn new() -> Self {
        Self {
            sparkles1: Sparkles::new(1150.0, 250.0),
            sparkles2: Sparkles::new(1150.0, 250.0)
                .particle_count(60)
                .size_range(3.0, 6.0),
            sparkles3: Sparkles::new(1150.0, 250.0)
                .particle_count(20)
                .size_range(4.0, 8.0)
                .colors(vec![
                    egui::Color32::from_rgb(255, 20, 147), // Deep pink
                    egui::Color32::from_rgb(138, 43, 226), // Blue violet
                    egui::Color32::from_rgb(0, 255, 255),  // Cyan
                ]),
        }
    }
}

impl eframe::App for SparklesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = ui.ctx().armas_theme();
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Sparkles Effect");
                    ui.add_space(10.0);
                    ui.label("Twinkling star particles for magical effects");
                    ui.add_space(30.0);
                });

                // Example 1: Default sparkles
                ui.label(
                    egui::RichText::new("Default Sparkles")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 250.0));
                        self.sparkles1.show_with_content(ui, &theme, |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("✨ Magical Content ✨")
                                            .size(36.0)
                                            .strong(),
                                    );
                                    ui.add_space(10.0);
                                    ui.label(
                                        egui::RichText::new("Sparkles twinkle in the background")
                                            .size(16.0)
                                            .color(egui::Color32::from_gray(180)),
                                    );
                                });
                            });
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 2: More sparkles
                ui.label(
                    egui::RichText::new("Dense Sparkle Field")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 250.0));
                        self.sparkles2.show_with_content(ui, &theme, |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(egui::RichText::new("Starfield").size(42.0).strong());
                                    ui.add_space(10.0);
                                    ui.label(
                                        egui::RichText::new("60 particles, larger size range")
                                            .size(16.0)
                                            .color(egui::Color32::from_gray(180)),
                                    );
                                });
                            });
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 3: Custom colors
                ui.label(
                    egui::RichText::new("Custom Colors")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 250.0));
                        self.sparkles3.show_with_content(ui, &theme, |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("Neon Dreams").size(42.0).strong(),
                                    );
                                    ui.add_space(10.0);
                                    ui.label(
                                        egui::RichText::new("Pink, violet, and cyan sparkles")
                                            .size(16.0)
                                            .color(egui::Color32::from_gray(180)),
                                    );
                                });
                            });
                        });
                    });
                });

                ui.add_space(50.0);
                ui.separator();
                ui.add_space(20.0);

                // Tips
                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("💡 Tips")
                                .size(16.0)
                                .strong()
                                .color(theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• Use show_with_content() to overlay sparkles on content");
                        ui.label("• particle_count() controls density of sparkles");
                        ui.label("• size_range() sets min and max sparkle sizes");
                        ui.label("• colors() accepts a Vec for custom sparkle colors");
                        ui.label("• Sparkles twinkle automatically with sine wave animation");
                        ui.label(
                            "• Perfect for hero sections, premium features, or celebration effects",
                        );
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
