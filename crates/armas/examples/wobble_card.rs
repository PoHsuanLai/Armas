//! Wobble Card Example
//!
//! Demonstrates interactive wobble effects on cards

use armas::{Theme, WobbleCard};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Wobble Card"),
        ..Default::default()
    };

    eframe::run_native(
        "Wobble Card",
        options,
        Box::new(|_cc| Ok(Box::new(WobbleCardApp::new()))),
    )
}

struct WobbleCardApp {
    theme: Theme,
}

impl WobbleCardApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
        }
    }
}

impl eframe::App for WobbleCardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Wobble Card");
                    ui.add_space(10.0);
                    ui.label("Cards that jiggle and wobble on hover");
                    ui.add_space(30.0);
                });

                ui.label(
                    egui::RichText::new("Hover over the cards to see them wobble!")
                        .size(16.0)
                        .color(self.theme.on_surface()),
                );
                ui.add_space(20.0);

                // Example 1: Default wobble cards
                ui.label(
                    egui::RichText::new("Default Wobble")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(40.0);

                    let mut card1 = WobbleCard::new(300.0, 200.0);
                    card1.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new("🎨").size(48.0));
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("Design").size(22.0).strong());
                            ui.label(
                                egui::RichText::new("Beautiful UI")
                                    .size(14.0)
                                    .color(egui::Color32::from_gray(160)),
                            );
                        });
                    });

                    ui.add_space(20.0);

                    let mut card2 = WobbleCard::new(300.0, 200.0);
                    card2.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new("⚡").size(48.0));
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("Performance").size(22.0).strong());
                            ui.label(
                                egui::RichText::new("Lightning fast")
                                    .size(14.0)
                                    .color(egui::Color32::from_gray(160)),
                            );
                        });
                    });

                    ui.add_space(20.0);

                    let mut card3 = WobbleCard::new(300.0, 200.0);
                    card3.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(40.0);
                            ui.label(egui::RichText::new("🔒").size(48.0));
                            ui.add_space(15.0);
                            ui.label(egui::RichText::new("Security").size(22.0).strong());
                            ui.label(
                                egui::RichText::new("Rock solid")
                                    .size(14.0)
                                    .color(egui::Color32::from_gray(160)),
                            );
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 2: Different intensities
                ui.label(
                    egui::RichText::new("Different Wobble Intensities")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(40.0);

                    // Subtle wobble
                    let mut card4 = WobbleCard::new(280.0, 180.0)
                        .wobble_intensity(0.5)
                        .background(egui::Color32::from_rgba_unmultiplied(59, 130, 246, 30));
                    card4.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(egui::RichText::new("Subtle").size(20.0).strong());
                            ui.add_space(10.0);
                            ui.label("Intensity: 0.5");
                        });
                    });

                    ui.add_space(20.0);

                    // Normal wobble
                    let mut card5 = WobbleCard::new(280.0, 180.0)
                        .wobble_intensity(1.0)
                        .background(egui::Color32::from_rgba_unmultiplied(147, 51, 234, 30));
                    card5.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(egui::RichText::new("Normal").size(20.0).strong());
                            ui.add_space(10.0);
                            ui.label("Intensity: 1.0");
                        });
                    });

                    ui.add_space(20.0);

                    // Intense wobble
                    let mut card6 = WobbleCard::new(280.0, 180.0)
                        .wobble_intensity(1.8)
                        .background(egui::Color32::from_rgba_unmultiplied(236, 72, 153, 30));
                    card6.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(egui::RichText::new("Intense").size(20.0).strong());
                            ui.add_space(10.0);
                            ui.label("Intensity: 1.8");
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 3: Different speeds
                ui.label(
                    egui::RichText::new("Different Wobble Speeds")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(40.0);

                    let mut card7 = WobbleCard::new(350.0, 180.0)
                        .wobble_speed(4.0)
                        .background(egui::Color32::from_rgba_unmultiplied(34, 197, 94, 30))
                        .border(Some(egui::Color32::from_rgb(34, 197, 94)));
                    card7.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(egui::RichText::new("Slow").size(20.0).strong());
                            ui.add_space(10.0);
                            ui.label("Speed: 4.0 (slower)");
                        });
                    });

                    ui.add_space(20.0);

                    let mut card8 = WobbleCard::new(350.0, 180.0)
                        .wobble_speed(12.0)
                        .background(egui::Color32::from_rgba_unmultiplied(251, 191, 36, 30))
                        .border(Some(egui::Color32::from_rgb(251, 191, 36)));
                    card8.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(egui::RichText::new("Fast").size(20.0).strong());
                            ui.add_space(10.0);
                            ui.label("Speed: 12.0 (faster)");
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
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• Wobble effect activates on hover");
                        ui.label("• wobble_intensity() controls how much the card moves");
                        ui.label("• wobble_speed() adjusts animation speed");
                        ui.label("• Effect automatically dampens over time");
                        ui.label("• Great for product cards and interactive elements");
                        ui.label("• Combines rotation, translation, and subtle shadow");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
