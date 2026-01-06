//! Aurora Background Example
//!
//! Demonstrates the aurora background effect with floating gradient blobs

use armas::{AuroraBackground, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Armas - Aurora Background"),
        ..Default::default()
    };

    eframe::run_native(
        "Aurora Background",
        options,
        Box::new(|_cc| Ok(Box::new(AuroraDemo::new()))),
    )
}

struct AuroraDemo {
    theme: Theme,
    aurora_cyberpunk: AuroraBackground,
    aurora_borealis: AuroraBackground,
    aurora_sunset: AuroraBackground,
    selected: usize,
}

impl AuroraDemo {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            aurora_cyberpunk: AuroraBackground::cyberpunk(1000.0, 700.0),
            aurora_borealis: AuroraBackground::borealis(1000.0, 700.0),
            aurora_sunset: AuroraBackground::sunset(1000.0, 700.0),
            selected: 0,
        }
    }
}

impl eframe::App for AuroraDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Draw the selected aurora as background
            let aurora = match self.selected {
                0 => &mut self.aurora_cyberpunk,
                1 => &mut self.aurora_borealis,
                2 => &mut self.aurora_sunset,
                _ => &mut self.aurora_cyberpunk,
            };

            aurora.show(ui, &self.theme);

            // Overlay controls
            egui::Window::new("Aurora Controls")
                .default_pos([20.0, 20.0])
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.heading("Aurora Background");
                    ui.add_space(10.0);

                    ui.label("Select Preset:");
                    ui.horizontal(|ui| {
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

                    ui.add_space(20.0);

                    ui.heading("Example Usage");
                    ui.code("AuroraBackground::cyberpunk(w, h)");
                    ui.code("  .with_speed(1.5)");
                    ui.code("  .show(ui, &theme);");
                });

            // Info overlay
            egui::Window::new("Info")
                .default_pos([ui.available_width() - 270.0, 20.0])
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.heading("Phase 2: Backgrounds");
                    ui.add_space(10.0);

                    ui.label("Aurora background creates");
                    ui.label("atmospheric effects with");
                    ui.label("smooth, organic blob motion");
                    ui.label("using gradient meshes.");

                    ui.add_space(10.0);

                    ui.label("Perfect for:");
                    ui.label("• Landing pages");
                    ui.label("• Hero sections");
                    ui.label("• Ambient backgrounds");
                    ui.label("• Modal overlays");
                });
        });
    }
}
