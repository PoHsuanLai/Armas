//! Retro Grid Example
//!
//! Demonstrates cyberpunk-style perspective grid backgrounds

use armas::{RetroGrid, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Retro Grid"),
        ..Default::default()
    };

    eframe::run_native(
        "Retro Grid",
        options,
        Box::new(|_cc| Ok(Box::new(RetroGridApp::new()))),
    )
}

struct RetroGridApp {
    theme: Theme,
    grid1: RetroGrid,
    grid2: RetroGrid,
    grid3: RetroGrid,
}

impl RetroGridApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            grid1: RetroGrid::new(1150.0, 350.0),
            grid2: RetroGrid::new(1150.0, 350.0)
                .grid_color(egui::Color32::from_rgba_unmultiplied(255, 0, 255, 100))
                .horizon_color(egui::Color32::from_rgba_unmultiplied(0, 255, 255, 120))
                .perspective_depth(0.8)
                .cell_size(60.0),
            grid3: RetroGrid::new(1150.0, 350.0)
                .grid_color(egui::Color32::from_rgba_unmultiplied(0, 255, 0, 90))
                .horizon_color(egui::Color32::from_rgba_unmultiplied(255, 255, 0, 110))
                .perspective_depth(0.4)
                .animate(false),
        }
    }
}

impl eframe::App for RetroGridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Retro Grid Background");
                    ui.add_space(10.0);
                    ui.label("Cyberpunk-style perspective grids with animated motion");
                    ui.add_space(30.0);
                });

                // Example 1: Classic cyan grid
                ui.label(
                    egui::RichText::new("Classic Cyberpunk")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 350.0));

                        // Draw grid first
                        self.grid1.show(ui, &self.theme);

                        // Overlay content
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new("CYBER GRID")
                                        .size(52.0)
                                        .strong()
                                        .color(egui::Color32::from_rgb(0, 255, 255)),
                                );
                                ui.add_space(15.0);
                                ui.label(
                                    egui::RichText::new("Neon Dreams • Retro Future")
                                        .size(20.0)
                                        .color(egui::Color32::from_rgb(255, 0, 255)),
                                );
                            });
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 2: Purple/cyan inverted
                ui.label(
                    egui::RichText::new("Inverted Colors")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 350.0));

                        self.grid2.show(ui, &self.theme);

                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new("SYNTHWAVE")
                                        .size(52.0)
                                        .strong()
                                        .color(egui::Color32::from_rgb(255, 0, 255)),
                                );
                                ui.add_space(15.0);
                                ui.label(
                                    egui::RichText::new("Deeper perspective • Larger cells")
                                        .size(18.0)
                                        .color(egui::Color32::from_rgb(0, 255, 255)),
                                );
                            });
                        });
                    });
                });

                ui.add_space(30.0);

                // Example 3: Green/yellow matrix style
                ui.label(
                    egui::RichText::new("Matrix Style (Static)")
                        .size(18.0)
                        .strong()
                        .color(self.theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    ui.add_space(25.0);
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(1150.0, 350.0));

                        self.grid3.show(ui, &self.theme);

                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new("THE MATRIX")
                                        .size(52.0)
                                        .strong()
                                        .color(egui::Color32::from_rgb(0, 255, 0)),
                                );
                                ui.add_space(15.0);
                                ui.label(
                                    egui::RichText::new("Static grid • Shallow perspective")
                                        .size(18.0)
                                        .color(egui::Color32::from_rgb(255, 255, 0)),
                                );
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
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• grid_color() sets the color of grid lines");
                        ui.label("• horizon_color() sets the glow at the vanishing point");
                        ui.label("• cell_size() controls how dense the grid appears");
                        ui.label(
                            "• perspective_depth() adjusts how steep the perspective is (0.0-1.0)",
                        );
                        ui.label("• animate(false) for static grids");
                        ui.label("• animation_speed() controls how fast the grid moves");
                        ui.label("• Perfect for cyberpunk, synthwave, and retro-futuristic themes");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
