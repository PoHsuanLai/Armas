//! Vortex Background Example
//!
//! Demonstrates swirling vortex effects with rotating particles

use armas::ext::ArmasContextExt;
use armas::{Theme, VortexBackground};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Vortex Background"),
        ..Default::default()
    };

    eframe::run_native(
        "Vortex Background",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(VortexApp::new()))
        }),
    )
}

struct VortexApp {
    vortex1: VortexBackground,
    vortex2: VortexBackground,
    vortex3: VortexBackground,
}

impl VortexApp {
    fn new() -> Self {
        Self {
            vortex1: VortexBackground::new(1200.0, 300.0),
            vortex2: VortexBackground::new(1200.0, 300.0)
                .particle_count(25)
                .ring_count(8)
                .rotation_speed(0.5)
                .radius_variation(0.4)
                .particle_size(3.0),
            vortex3: VortexBackground::new(1200.0, 300.0)
                .particle_count(40)
                .ring_count(4)
                .rotation_speed(0.15)
                .radius_variation(0.1)
                .particle_size(1.5)
                .colors(vec![
                    egui::Color32::from_rgba_unmultiplied(34, 197, 94, 100), // Green
                    egui::Color32::from_rgba_unmultiplied(251, 191, 36, 90), // Yellow
                    egui::Color32::from_rgba_unmultiplied(239, 68, 68, 85),  // Red
                ]),
        }
    }
}

impl eframe::App for VortexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Vortex Background");
                    ui.add_space(10.0);
                    ui.label("Swirling particle effects for dynamic backgrounds");
                    ui.add_space(30.0);
                });

                // Example 1: Default vortex
                ui.label(
                    egui::RichText::new("Default Vortex")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.vortex1.show(ui);

                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("Hypnotic Swirl")
                                    .size(42.0)
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new("Rotating particles create a mesmerizing effect")
                                    .size(16.0)
                                    .color(egui::Color32::from_gray(180)),
                            );
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 2: Fast rotation with web lines
                ui.label(
                    egui::RichText::new("Fast Rotation with Web Effect")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.vortex2.show(ui);

                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("Connected Particles")
                                    .size(36.0)
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new("Lines connect particles for a web effect")
                                    .size(16.0)
                                    .color(egui::Color32::from_gray(180)),
                            );
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 3: Smooth slow vortex with custom colors
                ui.label(
                    egui::RichText::new("Smooth Slow Vortex")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.vortex3.show(ui);

                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("Gentle Motion")
                                    .size(36.0)
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new("Slow rotation with warm colors")
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
                        ui.label("• particle_count() sets particles per ring (lower values show web lines)");
                        ui.label("• ring_count() determines number of concentric circles");
                        ui.label("• rotation_speed() controls how fast the vortex spins");
                        ui.label("• radius_variation() adds organic wobble to the rings");
                        ui.label("• particle_size() adjusts the size of each particle");
                        ui.label("• colors() accepts a Vec for multi-color vortex");
                        ui.label("• Perfect for loading screens and hero backgrounds");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
