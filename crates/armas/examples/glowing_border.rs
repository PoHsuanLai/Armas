//! Glowing Border Example
//!
//! Demonstrates pulsing glow border effects

use armas::ext::ArmasContextExt;
use armas::{GlowingBorder, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Glowing Border"),
        ..Default::default()
    };

    eframe::run_native(
        "Glowing Border",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(GlowingBorderApp::new()))
        }),
    )
}

struct GlowingBorderApp {
}

impl GlowingBorderApp {
    fn new() -> Self {
        Self {
        }
    }
}

impl eframe::App for GlowingBorderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Glowing Border");
                    ui.add_space(10.0);
                    ui.label("Pulsing glow effects for containers");
                    ui.add_space(30.0);
                });

                // Example 1: Default pulsing glow
                ui.label(
                    egui::RichText::new("Default Pulsing Glow")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    let mut glow1 = GlowingBorder::new().width(500.0).height(150.0);
                    glow1.show(ui, &theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(egui::RichText::new("Featured Content").size(24.0).strong());
                            ui.add_space(10.0);
                            ui.label("The border pulses to draw attention");
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 2: Different colors and intensities
                ui.label(
                    egui::RichText::new("Custom Colors & Intensities")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    ui.vertical(|ui| {
                        // Purple glow
                        let mut glow2 = GlowingBorder::new()
                            .width(350.0)
                            .height(120.0)
                            .glow_color(egui::Color32::from_rgb(147, 51, 234))
                            .glow_intensity(1.5);
                        glow2.show(ui, &theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new("Purple Glow").size(20.0).strong());
                                ui.label("Intensity: 1.5");
                            });
                        });

                        ui.add_space(20.0);

                        // Pink glow
                        let mut glow3 = GlowingBorder::new()
                            .width(350.0)
                            .height(120.0)
                            .glow_color(egui::Color32::from_rgb(236, 72, 153))
                            .glow_intensity(0.8)
                            .pulse_speed(2.0);
                        glow3.show(ui, &theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new("Pink Glow").size(20.0).strong());
                                ui.label("Fast pulse (speed: 2.0)");
                            });
                        });
                    });

                    ui.add_space(20.0);

                    // Green glow
                    let mut glow4 = GlowingBorder::new()
                        .width(350.0)
                        .height(260.0)
                        .glow_color(egui::Color32::from_rgb(34, 197, 94))
                        .background(egui::Color32::from_gray(15));
                    glow4.show(ui, &theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(60.0);
                            ui.label(egui::RichText::new("Success").size(28.0).strong());
                            ui.add_space(15.0);
                            ui.label("Green glow for");
                            ui.label("positive feedback");
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 3: Static glow (no pulse)
                ui.label(
                    egui::RichText::new("Static Glow (No Pulse)")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    let mut glow5 = GlowingBorder::new()
                        .width(500.0)
                        .height(150.0)
                        .pulse(false)
                        .glow_color(egui::Color32::from_rgb(251, 191, 36))
                        .glow_intensity(1.2);
                    glow5.show(ui, &theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(30.0);
                            ui.label(egui::RichText::new("Always On").size(24.0).strong());
                            ui.add_space(10.0);
                            ui.label("Constant glow without pulsing");
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
                        ui.label("• Use glow_color() to match your theme");
                        ui.label("• glow_intensity() controls how bright the glow is");
                        ui.label("• pulse_speed() adjusts animation speed");
                        ui.label("• pulse(false) for static glow");
                        ui.label("• Great for highlighting important sections");
                        ui.label("• Combine with cards for premium feel");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
