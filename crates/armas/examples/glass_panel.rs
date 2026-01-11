//! GlassPanel component examples
//!
//! Demonstrates glassmorphic panels with different glow intensities
//! and various content types.

use armas::ext::ArmasContextExt;
use armas::{components::GlassPanel, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Glass Panel Examples"),
        ..Default::default()
    };

    eframe::run_native(
        "Glass Panel Examples",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(GlassPanelExample::default()))
        }),
    )
}

struct GlassPanelExample {
    glow_intensity: f32,
}

impl Default for GlassPanelExample {
    fn default() -> Self {
        Self {
            glow_intensity: 0.5,
        }
    }
}

impl eframe::App for GlassPanelExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        // Apply theme colors to egui
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = theme.background();
        style.visuals.panel_fill = theme.background();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Glass Panel Component Examples");
            ui.add_space(20.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Theme selector
                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    if ui.button("Dark").clicked() {
                        ctx.set_armas_theme(Theme::dark());
                    }
                    if ui.button("Light").clicked() {
                        ctx.set_armas_theme(Theme::light());
                    }
                    if ui.button("Nord").clicked() {
                        ctx.set_armas_theme(Theme::nord());
                    }
                    if ui.button("Dracula").clicked() {
                        ctx.set_armas_theme(Theme::dracula());
                    }
                    if ui.button("Studio").clicked() {
                        ctx.set_armas_theme(Theme::studio());
                    }
                });

                ui.add_space(20.0);

                // Glow intensity control
                ui.horizontal(|ui| {
                    ui.label("Global Glow Intensity:");
                    ui.add(egui::Slider::new(&mut self.glow_intensity, 0.0..=1.0));
                });

                ui.add_space(20.0);

                // Example 1: Different glow intensities
                ui.heading("Glow Intensity Levels");
                ui.label("Panels with different glow intensities");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    for intensity in [0.0, 0.3, 0.6, 1.0] {
                        GlassPanel::new()
                            .title(&format!("Glow: {:.1}", intensity))
                            .glow_intensity(intensity)
                            .width(150.0)
                            .show(ui, &theme, |ui| {
                                ui.label(format!("Intensity: {:.1}", intensity));
                                ui.separator();
                                if intensity == 0.0 {
                                    ui.label("No glow");
                                } else if intensity < 0.5 {
                                    ui.label("Subtle glow");
                                } else {
                                    ui.label("Strong glow");
                                }
                            });
                        ui.add_space(10.0);
                    }
                });

                ui.add_space(30.0);

                // Example 2: Overlay panels
                ui.heading("Overlay Panels");
                ui.label("Glass panels work great as overlays");
                ui.add_space(10.0);

                // Background content
                ui.label(
                    egui::RichText::new("Background Content")
                        .size(14.0)
                        .color(theme.on_surface_variant()),
                );
                ui.label("This content sits behind the glass panels above.");

                ui.add_space(10.0);

                // Overlay panels
                ui.horizontal(|ui| {
                    GlassPanel::new()
                        .title("Notification")
                        .glow_intensity(self.glow_intensity)
                        .width(250.0)
                        .show(ui, &theme, |ui| {
                            ui.label(egui::RichText::new("✉ New Message").color(theme.info()));
                            ui.separator();
                            ui.label("You have 3 unread messages");
                            ui.add_space(5.0);
                            if ui.button("View").clicked() {
                                // Handle click
                            }
                        });

                    ui.add_space(10.0);

                    GlassPanel::new()
                        .title("Settings")
                        .glow_intensity(self.glow_intensity)
                        .width(250.0)
                        .show(ui, &theme, |ui| {
                            ui.label("Volume");
                            ui.add(egui::Slider::new(&mut 0.5f32, 0.0..=1.0));
                            ui.separator();
                            ui.label("Brightness");
                            ui.add(egui::Slider::new(&mut 0.7f32, 0.0..=1.0));
                        });
                });

                ui.add_space(30.0);

                // Example 3: Dashboard panels
                ui.heading("Dashboard Panels");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    GlassPanel::new()
                        .title("Server Status")
                        .glow_intensity(self.glow_intensity)
                        .width(200.0)
                        .show(ui, &theme, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("●");
                                ui.label(egui::RichText::new("Online").color(theme.success()));
                            });
                            ui.separator();
                            ui.label("Uptime: 99.9%");
                            ui.label("Response: 45ms");
                        });

                    ui.add_space(10.0);

                    GlassPanel::new()
                        .title("Performance")
                        .glow_intensity(self.glow_intensity)
                        .width(200.0)
                        .show(ui, &theme, |ui| {
                            ui.label("CPU: 23%");
                            ui.label("Memory: 45%");
                            ui.label("Disk: 67%");
                        });

                    ui.add_space(10.0);

                    GlassPanel::new()
                        .title("Alerts")
                        .glow_intensity(self.glow_intensity)
                        .width(200.0)
                        .show(ui, &theme, |ui| {
                            ui.label(
                                egui::RichText::new("⚠ 2 Warnings").color(theme.warning()),
                            );
                            ui.label(egui::RichText::new("✓ 0 Errors").color(theme.success()));
                        });
                });

                ui.add_space(30.0);

                // Example 4: Full-width panel
                ui.heading("Full-Width Panel");
                ui.add_space(10.0);

                GlassPanel::new()
                    .title("Player Controls")
                    .glow_intensity(self.glow_intensity)
                    .show(ui, &theme, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("⏮").clicked() {
                                // Previous
                            }
                            if ui.button("⏯").clicked() {
                                // Play/Pause
                            }
                            if ui.button("⏭").clicked() {
                                // Next
                            }
                            ui.separator();
                            ui.label("Now Playing: Example Track");
                            ui.add_space(20.0);
                            ui.add(egui::Slider::new(&mut 0.5f32, 0.0..=1.0).text("Volume"));
                        });
                    });

                ui.add_space(30.0);

                // Example 5: Nested panels
                ui.heading("Nested Panels");
                ui.label("Glass panels can contain other content");
                ui.add_space(10.0);

                GlassPanel::new()
                    .title("Settings Group")
                    .glow_intensity(self.glow_intensity * 0.7)
                    .show(ui, &theme, |ui| {
                        ui.label("Configure your preferences");
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            GlassPanel::new()
                                .title("Audio")
                                .glow_intensity(self.glow_intensity)
                                .width(200.0)
                                .show(ui, &theme, |ui| {
                                    ui.checkbox(&mut true, "Enable sound");
                                    ui.checkbox(&mut false, "Mute on start");
                                });

                            ui.add_space(10.0);

                            GlassPanel::new()
                                .title("Display")
                                .glow_intensity(self.glow_intensity)
                                .width(200.0)
                                .show(ui, &theme, |ui| {
                                    ui.checkbox(&mut true, "Dark mode");
                                    ui.checkbox(&mut true, "Show tooltips");
                                });
                        });
                    });

                ui.add_space(30.0);

                // Example 6: Panels without titles
                ui.heading("Titleless Panels");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    for icon in ["🎵", "🎨", "⚙️", "📊"] {
                        GlassPanel::new()
                            .glow_intensity(self.glow_intensity)
                            .width(100.0)
                            .show(ui, &theme, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(egui::RichText::new(icon).size(32.0));
                                });
                            });
                        ui.add_space(10.0);
                    }
                });

                ui.add_space(30.0);

                // Example 7: Color showcase
                ui.heading("Themed Panels");
                ui.label("Panels automatically adapt to the selected theme");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    GlassPanel::new()
                        .title("Primary")
                        .glow_intensity(0.8)
                        .width(150.0)
                        .show(ui, &theme, |ui| {
                            ui.label(
                                egui::RichText::new("●")
                                    .size(24.0)
                                    .color(theme.primary()),
                            );
                            ui.label("Primary color");
                        });

                    ui.add_space(10.0);

                    GlassPanel::new()
                        .title("Success")
                        .glow_intensity(0.8)
                        .width(150.0)
                        .show(ui, &theme, |ui| {
                            ui.label(
                                egui::RichText::new("✓")
                                    .size(24.0)
                                    .color(theme.success()),
                            );
                            ui.label("All good!");
                        });

                    ui.add_space(10.0);

                    GlassPanel::new()
                        .title("Warning")
                        .glow_intensity(0.8)
                        .width(150.0)
                        .show(ui, &theme, |ui| {
                            ui.label(
                                egui::RichText::new("⚠")
                                    .size(24.0)
                                    .color(theme.warning()),
                            );
                            ui.label("Caution");
                        });
                });
            });
        });
    }
}
