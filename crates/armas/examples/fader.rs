//! Fader component demo
//!
//! Demonstrates both Fader (minimal) and FaderStrip (with housing) components
//!
//! Run with: cargo run --example fader

use armas::{Fader, FaderStrip, Theme};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Fader Demo - egui-alig"),
        ..Default::default()
    };

    eframe::run_native(
        "Fader Demo",
        options,
        Box::new(|_cc| Ok(Box::new(FaderDemoApp::default()))),
    )
}

struct FaderDemoApp {
    current_theme: usize,
    fader_values: [f32; 8],
}

impl Default for FaderDemoApp {
    fn default() -> Self {
        Self {
            current_theme: 4, // Start with Studio theme
            fader_values: [0.0, 0.25, 0.5, 0.75, 1.0, 0.6, 0.3, 0.8],
        }
    }
}

const THEMES: &[(&str, fn() -> Theme)] = &[
    ("Dark (M3)", Theme::dark),
    ("Light (M3)", Theme::light),
    ("Nord", Theme::nord),
    ("Dracula", Theme::dracula),
    ("Studio", Theme::studio),
];

impl eframe::App for FaderDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = THEMES[self.current_theme].1();

        // Set visuals based on theme
        ctx.set_visuals(if self.current_theme == 1 {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui-alig Fader Component Demo");
            ui.add_space(20.0);

            // Theme selector
            ui.horizontal(|ui| {
                ui.label("Theme:");
                for (i, (name, _)) in THEMES.iter().enumerate() {
                    if ui
                        .selectable_label(self.current_theme == i, *name)
                        .clicked()
                    {
                        self.current_theme = i;
                    }
                }
            });
            ui.add_space(20.0);

            ui.separator();
            ui.add_space(20.0);

            // Mixer-style fader strip (using FaderStrip with housing)
            ui.heading("Mixer Channel Strip (FaderStrip with housing)");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 20.0;

                for (i, value) in self.fader_values.iter_mut().enumerate() {
                    ui.vertical(|ui| {
                        ui.label(format!("Ch {}", i + 1));
                        ui.add_space(5.0);

                        let (response, new_value) =
                            FaderStrip::new(*value).size(39.0, 254.0).show(ui, &theme);

                        if response.changed() {
                            *value = new_value;
                        }

                        ui.add_space(5.0);
                        ui.label(format!("{:.0}%", *value * 100.0));

                        // Reset button
                        if ui.small_button("Reset").clicked() {
                            *value = 0.75;
                        }
                    });
                }
            });

            ui.add_space(40.0);
            ui.separator();
            ui.add_space(20.0);

            // Minimal faders in custom containers
            ui.heading("Minimal Faders (in custom containers)");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 20.0;

                // Example 1: Fader in a Frame (custom card)
                ui.vertical(|ui| {
                    ui.label("In Custom Frame");
                    ui.add_space(5.0);

                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(30, 30, 30))
                        .corner_radius(4.0)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            let (response, new_value) = Fader::new(self.fader_values[0])
                                .size(30.0, 240.0)
                                .show(ui, &theme);

                            if response.changed() {
                                self.fader_values[0] = new_value;
                            }
                        });

                    ui.add_space(5.0);
                    ui.label(format!("{:.0}%", self.fader_values[0] * 100.0));
                });

                // Example 2: Fader without any container
                ui.vertical(|ui| {
                    ui.label("No Container");
                    ui.add_space(5.0);

                    let (response, new_value) = Fader::new(self.fader_values[1])
                        .size(30.0, 240.0)
                        .show(ui, &theme);

                    if response.changed() {
                        self.fader_values[1] = new_value;
                    }

                    ui.add_space(5.0);
                    ui.label(format!("{:.0}%", self.fader_values[1] * 100.0));
                });
            });

            ui.add_space(40.0);
            ui.separator();
            ui.add_space(20.0);

            // Individual fader with controls
            ui.heading("FaderStrip with Controls");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Master Fader");
                    ui.add_space(5.0);

                    let (response, new_value) = FaderStrip::new(self.fader_values[0])
                        .size(39.0, 254.0)
                        .show(ui, &theme);

                    if response.changed() {
                        self.fader_values[0] = new_value;
                    }
                });

                ui.add_space(40.0);

                ui.vertical(|ui| {
                    ui.label("Fader Controls");
                    ui.add_space(10.0);

                    ui.label(format!("Value: {:.2}", self.fader_values[0]));
                    ui.label(format!("Percentage: {:.0}%", self.fader_values[0] * 100.0));
                    ui.label(format!("dB: {:.1}", value_to_db(self.fader_values[0])));

                    ui.add_space(10.0);

                    if ui.button("Set to 0 dB (75%)").clicked() {
                        self.fader_values[0] = 0.75;
                    }
                    if ui.button("Set to -∞ dB (0%)").clicked() {
                        self.fader_values[0] = 0.0;
                    }
                    if ui.button("Set to +6 dB (100%)").clicked() {
                        self.fader_values[0] = 1.0;
                    }

                    ui.add_space(10.0);

                    ui.label("Quick Presets:");
                    ui.horizontal(|ui| {
                        if ui.button("25%").clicked() {
                            self.fader_values[0] = 0.25;
                        }
                        if ui.button("50%").clicked() {
                            self.fader_values[0] = 0.5;
                        }
                        if ui.button("75%").clicked() {
                            self.fader_values[0] = 0.75;
                        }
                    });
                });
            });

            ui.add_space(20.0);
        });
    }
}

/// Convert linear fader value (0.0-1.0) to approximate dB
fn value_to_db(value: f32) -> f32 {
    if value <= 0.0 {
        -std::f32::INFINITY
    } else {
        // Simple approximation: 0.75 = 0dB, 1.0 = +6dB, 0.0 = -∞dB
        20.0 * (value / 0.75).log10()
    }
}
