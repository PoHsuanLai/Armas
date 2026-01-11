//! Background Effects Showcase
//!
//! Demonstrates aurora and meteor shower effects

use armas::ext::ArmasContextExt;
use armas::{AuroraBackground, MeteorShower, Theme};
use eframe::egui;
use std::f32::consts::PI;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Armas - Background Effects"),
        ..Default::default()
    };

    eframe::run_native(
        "Background Effects",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(BackgroundsDemo::new()))
        }),
    )
}

struct BackgroundsDemo {
    selected: usize,

    // Aurora variants
    aurora_cyberpunk: AuroraBackground,
    aurora_borealis: AuroraBackground,
    aurora_sunset: AuroraBackground,

    // Meteor showers
    meteor_default: MeteorShower,
    meteor_fast: MeteorShower,
    meteor_blue: MeteorShower,
}

impl BackgroundsDemo {
    fn new() -> Self {
        let theme = Theme::dark();
        Self {
            theme: theme.clone(),
            selected: 0,

            // Aurora backgrounds
            aurora_cyberpunk: AuroraBackground::cyberpunk(1200.0, 800.0),
            aurora_borealis: AuroraBackground::borealis(1200.0, 800.0),
            aurora_sunset: AuroraBackground::sunset(1200.0, 800.0),

            // Meteor showers
            meteor_default: MeteorShower::new(1200.0, 800.0, &theme).with_spawn_rate(1.0),
            meteor_fast: MeteorShower::new(1200.0, 800.0, &theme)
                .with_spawn_rate(3.0)
                .with_speed_range(1.5, 2.5),
            meteor_blue: MeteorShower::new(1200.0, 800.0, &theme)
                .with_spawn_rate(2.0)
                .with_color(egui::Color32::from_rgb(100, 200, 255))
                .with_angle(PI / 3.0),
        }
    }
}

impl eframe::App for BackgroundsDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render selected background
            match self.selected {
                0 => {
                    self.aurora_cyberpunk.show(ui);
                }
                1 => {
                    self.aurora_borealis.show(ui);
                }
                2 => {
                    self.aurora_sunset.show(ui);
                }
                3 => {
                    self.meteor_default.show(ui);
                }
                4 => {
                    self.meteor_fast.show(ui);
                }
                5 => {
                    self.meteor_blue.show(ui);
                }
                _ => {}
            }

            // Control panel
            egui::Window::new("Background Effects")
                .default_pos([20.0, 20.0])
                .default_width(280.0)
                .show(ctx, |ui| {
                    ui.heading("Select Effect");
                    ui.add_space(10.0);

                    ui.label("Aurora Backgrounds:");
                    ui.horizontal_wrapped(|ui| {
                        if ui
                            .selectable_label(self.selected == 0, "Cyberpunk")
                            .clicked()
                        {
                            self.selected = 0;
                        }
                        if ui
                            .selectable_label(self.selected == 1, "Borealis")
                            .clicked()
                        {
                            self.selected = 1;
                        }
                        if ui.selectable_label(self.selected == 2, "Sunset").clicked() {
                            self.selected = 2;
                        }
                    });

                    ui.add_space(10.0);
                    ui.label("Meteor Showers:");
                    ui.horizontal_wrapped(|ui| {
                        if ui.selectable_label(self.selected == 3, "Default").clicked() {
                            self.selected = 3;
                        }
                        if ui.selectable_label(self.selected == 4, "Fast").clicked() {
                            self.selected = 4;
                        }
                        if ui.selectable_label(self.selected == 5, "Blue").clicked() {
                            self.selected = 5;
                        }
                    });

                    ui.add_space(20.0);

                    let description = match self.selected {
                        0 => "Cyberpunk: Cyan, magenta, and\nblue floating blobs",
                        1 => "Borealis: Green, turquoise, and\npurple aurora colors",
                        2 => "Sunset: Orange, pink, and\nyellow warm tones",
                        3 => "Default: White meteors at\nmoderate speed",
                        4 => "Fast: Rapid meteor shower\nwith high spawn rate",
                        5 => "Blue: Cyan meteors at\n60-degree angle",
                        _ => "",
                    };

                    ui.label(description);

                    ui.add_space(20.0);
                    ui.heading("Phase 2 Complete!");
                    ui.label("✅ Aurora Background");
                    ui.label("✅ Meteor Shower");
                });

            // Info panel
            egui::Window::new("Usage")
                .default_pos([ui.available_width() - 300.0, 20.0])
                .default_width(280.0)
                .show(ctx, |ui| {
                    ui.heading("Example Code");
                    ui.add_space(10.0);

                    if self.selected < 3 {
                        ui.label("Aurora:");
                        ui.code("AuroraBackground::cyberpunk(w, h)");
                        ui.code("  .with_speed(1.5)");
                        ui.code("  .show(ui);");
                    } else {
                        ui.label("Meteor Shower:");
                        ui.code("MeteorShower::new(w, h)");
                        ui.code("  .with_spawn_rate(2.0)");
                        ui.code("  .with_angle(PI / 4.0)");
                        ui.code("  .with_color(color)");
                        ui.code("  .show(ui);");
                    }

                    ui.add_space(15.0);
                    ui.label("Perfect for:");
                    ui.label("• Hero sections");
                    ui.label("• Landing pages");
                    ui.label("• Modal backgrounds");
                    ui.label("• Ambient effects");
                });
        });
    }
}
