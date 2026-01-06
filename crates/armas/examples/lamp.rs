//! Lamp Effect Example
//!
//! Demonstrates animated lighting effects with conic gradients

use armas::{LampEffect, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Lamp Effect"),
        ..Default::default()
    };

    eframe::run_native(
        "Lamp Effect",
        options,
        Box::new(|_cc| Ok(Box::new(LampApp::new()))),
    )
}

struct LampApp {
    theme: Theme,
    lamp1: LampEffect,
    lamp2: LampEffect,
    lamp3: LampEffect,
}

impl LampApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            lamp1: LampEffect::new(1150.0, 400.0),
            lamp2: LampEffect::new(1150.0, 400.0)
                .lamp_color(egui::Color32::from_rgb(168, 85, 247)) // Purple
                .animation_duration(1.2),
            lamp3: LampEffect::new(1150.0, 400.0)
                .lamp_color(egui::Color32::from_rgb(34, 197, 94)) // Green
                .background_color(egui::Color32::from_rgb(15, 23, 42)) // slate-900
                .animation_duration(1.5),
        }
    }
}

impl eframe::App for LampApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Lamp Effect");
                    ui.add_space(10.0);
                    ui.label("Animated lighting effects with conic gradients");
                    ui.add_space(30.0);
                });

                // Example 1: Classic cyan lamp
                ui.label(
                    egui::RichText::new("Classic Cyan Lamp")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 400.0));

                        self.lamp1.show_with_content(ui, &self.theme, |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("Build at Light Speed")
                                            .size(48.0)
                                            .strong()
                                            .color(egui::Color32::WHITE),
                                    );
                                    ui.add_space(15.0);
                                    ui.label(
                                        egui::RichText::new("The lamp effect creates dramatic lighting for hero sections")
                                            .size(18.0)
                                            .color(egui::Color32::from_gray(200)),
                                    );
                                })
                            });
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 2: Purple lamp
                ui.label(
                    egui::RichText::new("Purple Glow")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 400.0));

                        self.lamp2.show_with_content(ui, &self.theme, |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("Premium Experience")
                                            .size(48.0)
                                            .strong()
                                            .color(egui::Color32::WHITE),
                                    );
                                    ui.add_space(15.0);
                                    ui.label(
                                        egui::RichText::new("Custom colors and longer animation duration")
                                            .size(18.0)
                                            .color(egui::Color32::from_rgb(216, 180, 254)),
                                    );
                                })
                            });
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 3: Green lamp
                ui.label(
                    egui::RichText::new("Green Energy")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 400.0));

                        self.lamp3.show_with_content(ui, &self.theme, |ui| {
                            ui.centered_and_justified(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("Sustainable Future")
                                            .size(48.0)
                                            .strong()
                                            .color(egui::Color32::WHITE),
                                    );
                                    ui.add_space(15.0);
                                    ui.label(
                                        egui::RichText::new("Different background and slower animation")
                                            .size(18.0)
                                            .color(egui::Color32::from_rgb(134, 239, 172)),
                                    );
                                })
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
                            egui::RichText::new("Tips")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• lamp_color() customizes the lighting color");
                        ui.label("• background_color() sets the dark background");
                        ui.label("• animation_duration() controls the animation speed");
                        ui.label("• Uses Animation<f32> for smooth width and opacity transitions");
                        ui.label("• Conic gradients create the directional lighting effect");
                        ui.label("• Multiple blur layers add depth to the glow");
                        ui.label("• Perfect for hero sections and section headers");
                        ui.label("• Use show_with_content() to overlay text on the lamp");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
