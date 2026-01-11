//! Background Beams Example
//!
//! Demonstrates diagonal light beams across backgrounds

use armas::ext::ArmasContextExt;
use armas::{BackgroundBeams, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Background Beams"),
        ..Default::default()
    };

    eframe::run_native(
        "Background Beams",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(BackgroundBeamsApp::new()))
        }),
    )
}

struct BackgroundBeamsApp {
    beams1: BackgroundBeams,
    beams2: BackgroundBeams,
    beams3: BackgroundBeams,
}

impl BackgroundBeamsApp {
    fn new() -> Self {
        Self {
            beams1: BackgroundBeams::new(1200.0, 300.0),
            beams2: BackgroundBeams::new(1200.0, 300.0)
                .beam_count(12)
                .beam_width(80.0)
                .beam_angle(30.0)
                .opacity(0.25),
            beams3: BackgroundBeams::new(1200.0, 300.0)
                .beam_count(6)
                .beam_width(150.0)
                .beam_angle(90.0)
                .blur(false)
                .animate(false)
                .colors(vec![
                    egui::Color32::from_rgba_unmultiplied(34, 197, 94, 50), // Green
                    egui::Color32::from_rgba_unmultiplied(251, 191, 36, 45), // Yellow
                ]),
        }
    }
}

impl eframe::App for BackgroundBeamsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = ui.ctx().armas_theme();
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Background Beams");
                    ui.add_space(10.0);
                    ui.label("Diagonal light beams that fill the background");
                    ui.add_space(30.0);
                });

                // Example 1: Default beams
                ui.label(
                    egui::RichText::new("Default Diagonal Beams")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.beams1.show(ui);

                    // Overlay content
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("Hero Section").size(48.0).strong());
                            ui.label(
                                egui::RichText::new("Beautiful backgrounds for modern UIs")
                                    .size(18.0)
                                    .color(egui::Color32::from_gray(180)),
                            );
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 2: More beams, different angle
                ui.label(
                    egui::RichText::new("More Beams with Custom Angle")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.beams2.show(ui);

                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("30° Angle").size(36.0).strong());
                            ui.label(
                                egui::RichText::new("12 beams with higher opacity")
                                    .size(16.0)
                                    .color(egui::Color32::from_gray(180)),
                            );
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 3: Vertical beams, no blur
                ui.label(
                    egui::RichText::new("Vertical Beams (No Animation)")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.beams3.show(ui);

                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("90° Vertical").size(36.0).strong());
                            ui.label(
                                egui::RichText::new("Solid beams with custom colors")
                                    .size(16.0)
                                    .color(egui::Color32::from_gray(180)),
                            );
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
                        ui.label("• Use beam_angle() to control the direction (0-360 degrees)");
                        ui.label("• beam_count() determines how many beams fill the space");
                        ui.label("• beam_width() sets how wide each beam is");
                        ui.label("• opacity() controls overall transparency");
                        ui.label("• blur(true) adds soft edges for a glow effect");
                        ui.label("• animate(false) for static beams");
                        ui.label("• Great for hero sections and backgrounds");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
