//! Wavy Background Example
//!
//! Demonstrates animated wave effects for backgrounds and hero sections

use armas::ext::ArmasContextExt;
use armas::{Theme, WavyBackground};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Armas - Wavy Background"),
        ..Default::default()
    };

    eframe::run_native(
        "Wavy Background",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(WavyBackgroundApp::new()))
        }),
    )
}

struct WavyBackgroundApp {
    waves1: WavyBackground,
    waves2: WavyBackground,
    waves3: WavyBackground,
}

impl WavyBackgroundApp {
    fn new() -> Self {
        Self {
            waves1: WavyBackground::new(1200.0, 300.0),
            waves2: WavyBackground::new(1200.0, 300.0)
                .wave_count(8)
                .amplitude(30.0)
                .frequency(0.03)
                .speed(0.3)
                .with_blur(true),
            waves3: WavyBackground::new(1200.0, 300.0)
                .wave_count(3)
                .amplitude(60.0)
                .frequency(0.015)
                .speed(1.5)
                .colors(vec![
                    egui::Color32::from_rgb(34, 197, 94),  // Green
                    egui::Color32::from_rgb(251, 191, 36), // Yellow
                    egui::Color32::from_rgb(239, 68, 68),  // Red
                ]),
        }
    }
}

impl eframe::App for WavyBackgroundApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = ui.ctx().armas_theme();
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Wavy Background");
                    ui.add_space(10.0);
                    ui.label("Animated wave effects for backgrounds and hero sections");
                    ui.add_space(30.0);
                });

                // Example 1: Default waves
                ui.label(
                    egui::RichText::new("Default Waves")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.waves1.show(ui);
                });

                ui.add_space(30.0);

                // Example 2: Blur effect with more waves
                ui.label(
                    egui::RichText::new("With Blur Effect")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.waves2.show(ui);
                });

                ui.add_space(30.0);

                // Example 3: Custom colors and settings
                ui.label(
                    egui::RichText::new("Custom Colors & High Amplitude")
                        .size(18.0)
                        .strong()
                        .color(theme.on_surface()),
                );
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(1150.0, 300.0));
                    self.waves3.show(ui);
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
                        ui.label("• Use wave_count() to add more layered waves");
                        ui.label("• Adjust amplitude() to control wave height");
                        ui.label("• frequency() changes how wavy the pattern is");
                        ui.label("• speed() controls animation speed");
                        ui.label("• with_blur(true) adds a soft glow effect");
                        ui.label("• colors() accepts a Vec of Color32 for custom gradients");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
