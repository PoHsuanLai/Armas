use armas::{Spotlight, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Spotlight Effect Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Spotlight Effect Demo",
        options,
        Box::new(|_cc| Ok(Box::new(SpotlightApp::new()))),
    )
}

struct SpotlightApp {
    theme: Theme,
    spotlight_default: Spotlight,
    spotlight_large: Spotlight,
    spotlight_small: Spotlight,
    spotlight_red: Spotlight,
    spotlight_green: Spotlight,
    spotlight_purple: Spotlight,
    spotlight_smooth: Spotlight,
    spotlight_instant: Spotlight,
}

impl SpotlightApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            spotlight_default: Spotlight::new(),
            spotlight_large: Spotlight::new()
                .radius(300.0)
                .color(egui::Color32::from_rgba_unmultiplied(59, 130, 246, 50)),
            spotlight_small: Spotlight::new()
                .radius(100.0)
                .color(egui::Color32::from_rgba_unmultiplied(168, 85, 247, 60)),
            spotlight_red: Spotlight::new()
                .radius(150.0)
                .color(egui::Color32::from_rgba_unmultiplied(239, 68, 68, 45)),
            spotlight_green: Spotlight::new()
                .radius(180.0)
                .color(egui::Color32::from_rgba_unmultiplied(34, 197, 94, 40)),
            spotlight_purple: Spotlight::new()
                .radius(200.0)
                .color(egui::Color32::from_rgba_unmultiplied(168, 85, 247, 50))
                .intensity(0.8),
            spotlight_smooth: Spotlight::new()
                .radius(150.0)
                .smoothing(0.3)
                .color(egui::Color32::from_rgba_unmultiplied(251, 191, 36, 50)),
            spotlight_instant: Spotlight::new()
                .radius(150.0)
                .smoothing(0.0)
                .color(egui::Color32::from_rgba_unmultiplied(236, 72, 153, 50)),
        }
    }
}

impl eframe::App for SpotlightApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("✨ Spotlight Effect Component");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                // Default spotlight
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(300.0);
                        ui.set_min_height(200.0);

                        self.spotlight_default.show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(60.0);
                                ui.heading("Default Spotlight");
                                ui.label("Radius: 200px");
                                ui.label("Blue color");
                                ui.label("Move mouse to see effect");
                            });
                        });
                    });
                });

                ui.add_space(10.0);

                // Large spotlight
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(300.0);
                        ui.set_min_height(200.0);

                        self.spotlight_large.show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(60.0);
                                ui.heading("Large Spotlight");
                                ui.label("Radius: 300px");
                                ui.label("Covers more area");
                            });
                        });
                    });
                });

                ui.add_space(10.0);

                // Small spotlight
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(300.0);
                        ui.set_min_height(200.0);

                        self.spotlight_small.show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(60.0);
                                ui.heading("Small Spotlight");
                                ui.label("Radius: 100px");
                                ui.label("Purple color");
                                ui.label("Focused effect");
                            });
                        });
                    });
                });
            });

            ui.add_space(20.0);

            // Color variations
            ui.heading("Color Variations");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(300.0);
                        ui.set_min_height(150.0);

                        self.spotlight_red.show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(40.0);
                                ui.heading("🔴 Red Spotlight");
                                ui.colored_label(
                                    egui::Color32::from_rgb(239, 68, 68),
                                    "Danger / Error",
                                );
                            });
                        });
                    });
                });

                ui.add_space(10.0);

                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(300.0);
                        ui.set_min_height(150.0);

                        self.spotlight_green.show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(40.0);
                                ui.heading("🟢 Green Spotlight");
                                ui.colored_label(
                                    egui::Color32::from_rgb(34, 197, 94),
                                    "Success / Active",
                                );
                            });
                        });
                    });
                });

                ui.add_space(10.0);

                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(300.0);
                        ui.set_min_height(150.0);

                        self.spotlight_purple.show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(40.0);
                                ui.heading("🟣 Purple Spotlight");
                                ui.colored_label(
                                    egui::Color32::from_rgb(168, 85, 247),
                                    "Premium / Special",
                                );
                            });
                        });
                    });
                });
            });

            ui.add_space(20.0);

            // Smoothing comparison
            ui.heading("Smoothing Comparison");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(480.0);
                        ui.set_min_height(150.0);

                        self.spotlight_smooth.show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(40.0);
                                ui.heading("Smooth Follow (0.3)");
                                ui.label("Smoothing: 0.3");
                                ui.label("Lags behind cursor for smooth motion");
                            });
                        });
                    });
                });

                ui.add_space(10.0);

                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.set_min_width(480.0);
                        ui.set_min_height(150.0);

                        self.spotlight_instant.show(ui, &self.theme, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(40.0);
                                ui.heading("Instant Follow (0.0)");
                                ui.label("Smoothing: 0.0");
                                ui.label("Follows cursor immediately");
                            });
                        });
                    });
                });
            });

            ui.add_space(20.0);

            // Info panel
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("💡 Tips:");
                ui.label("• Move your mouse over any area to see the spotlight effect");
                ui.label("• Different colors and sizes create different moods");
                ui.label("• Adjust smoothing for different tracking behaviors");
            });
        });
    }
}
