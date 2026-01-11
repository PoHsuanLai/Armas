//! Complete Background Effects Showcase
//!
//! Demonstrates all Phase 2 background effects

use armas::ext::ArmasContextExt;
use armas::{
    AnimatedBeam, AnimatedBeams, AuroraBackground, BeamLoopMode, GridPattern, MeteorShower,
    PathPoint, Theme,
};
use eframe::egui;
use std::f32::consts::PI;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Armas - Complete Backgrounds"),
        ..Default::default()
    };

    eframe::run_native(
        "Complete Backgrounds",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(BackgroundsComplete::new()))
        }),
    )
}

struct BackgroundsComplete {
    selected: usize,

    // Aurora variants
    aurora_cyberpunk: AuroraBackground,
    aurora_borealis: AuroraBackground,

    // Meteor showers
    meteor_fast: MeteorShower,

    // Grid patterns
    grid_basic: GridPattern,
    grid_perspective: GridPattern,
    grid_dots: GridPattern,

    // Animated beams
    beams_demo: AnimatedBeams,
}

impl BackgroundsComplete {
    fn new() -> Self {
        // Create animated beams demo
        let mut beams = AnimatedBeams::new(1200.0, 800.0);

        // Horizontal wave beam
        let wave_path: Vec<PathPoint> = (0..=12)
            .map(|i| {
                let x = 100.0 + i as f32 * 100.0;
                let y = 400.0 + (i as f32 * 0.5).sin() * 100.0;
                PathPoint::new(x, y)
            })
            .collect();

        let theme = Theme::dark();
        beams = beams.add_beam(
            AnimatedBeam::new(wave_path, &theme)
                .with_color(egui::Color32::from_rgb(100, 200, 255))
                .with_speed(0.3)
                .with_glow(1.0)
                .with_loop_mode(BeamLoopMode::Loop),
        );

        // Diagonal beam
        let diag_path = vec![
            PathPoint::new(100.0, 100.0),
            PathPoint::new(600.0, 400.0),
            PathPoint::new(1100.0, 200.0),
        ];

        beams = beams.add_beam(
            AnimatedBeam::new(diag_path, &theme)
                .with_color(egui::Color32::from_rgb(255, 100, 200))
                .with_speed(0.4)
                .with_glow(0.8)
                .with_loop_mode(BeamLoopMode::PingPong),
        );

        // Circular beam
        let circle_path: Vec<PathPoint> = (0..=32)
            .map(|i| {
                let angle = i as f32 / 32.0 * 2.0 * PI;
                let x = 600.0 + angle.cos() * 200.0;
                let y = 400.0 + angle.sin() * 200.0;
                PathPoint::new(x, y)
            })
            .collect();

        beams = beams.add_beam(
            AnimatedBeam::new(circle_path, &theme)
                .with_color(egui::Color32::from_rgb(100, 255, 100))
                .with_speed(0.5)
                .with_glow(0.9)
                .with_loop_mode(BeamLoopMode::Loop),
        );

        let theme = Theme::dark();
        Self {
            theme: theme.clone(),
            selected: 0,

            // Aurora backgrounds
            aurora_cyberpunk: AuroraBackground::cyberpunk(1200.0, 800.0),
            aurora_borealis: AuroraBackground::borealis(1200.0, 800.0),

            // Meteor showers
            meteor_fast: MeteorShower::new(1200.0, 800.0, &theme)
                .with_spawn_rate(3.0)
                .with_speed_range(1.5, 2.5),

            // Grid patterns
            grid_basic: GridPattern::new(1200.0, 800.0, 50.0, &theme)
                .with_color(egui::Color32::from_rgba_unmultiplied(100, 150, 200, 100))
                .with_fade(0.5),

            grid_perspective: GridPattern::new(1200.0, 800.0, 60.0, &theme)
                .with_color(egui::Color32::from_rgba_unmultiplied(100, 200, 255, 120))
                .with_perspective(true)
                .with_fade(0.4),

            grid_dots: GridPattern::new(1200.0, 800.0, 80.0, &theme)
                .with_color(egui::Color32::from_rgba_unmultiplied(150, 100, 200, 80))
                .with_dots(
                    egui::Color32::from_rgba_unmultiplied(200, 150, 255, 150),
                    3.0,
                )
                .with_fade(0.6),

            // Animated beams
            beams_demo: beams,
        }
    }
}

impl eframe::App for BackgroundsComplete {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render selected background
            match self.selected {
                0 => { self.aurora_cyberpunk.show(ui); }
                1 => { self.aurora_borealis.show(ui); }
                2 => { self.meteor_fast.show(ui); }
                3 => { self.grid_basic.show(ui); }
                4 => { self.grid_perspective.show(ui); }
                5 => { self.grid_dots.show(ui); }
                6 => { self.beams_demo.show(ui); }
                _ => {}
            }

            // Control panel
            egui::Window::new("Phase 2 Complete!")
                .default_pos([20.0, 20.0])
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui.heading("Background Effects");
                    ui.add_space(10.0);

                    ui.label("Aurora Backgrounds:");
                    ui.horizontal_wrapped(|ui| {
                        if ui.selectable_label(self.selected == 0, "Cyberpunk").clicked() {
                            self.selected = 0;
                        }
                        if ui.selectable_label(self.selected == 1, "Borealis").clicked() {
                            self.selected = 1;
                        }
                    });

                    ui.add_space(8.0);
                    ui.label("Meteor Shower:");
                    if ui.selectable_label(self.selected == 2, "Fast Meteors").clicked() {
                        self.selected = 2;
                    }

                    ui.add_space(8.0);
                    ui.label("Grid Patterns:");
                    ui.horizontal_wrapped(|ui| {
                        if ui.selectable_label(self.selected == 3, "Basic").clicked() {
                            self.selected = 3;
                        }
                        if ui.selectable_label(self.selected == 4, "Perspective").clicked() {
                            self.selected = 4;
                        }
                        if ui.selectable_label(self.selected == 5, "With Dots").clicked() {
                            self.selected = 5;
                        }
                    });

                    ui.add_space(8.0);
                    ui.label("Animated Beams:");
                    if ui.selectable_label(self.selected == 6, "Beam Demo").clicked() {
                        self.selected = 6;
                    }

                    ui.add_space(20.0);

                    ui.heading("✅ Phase 2 Complete!");
                    ui.label("All background effects:");
                    ui.label("• Aurora (3 presets)");
                    ui.label("• Meteor Shower");
                    ui.label("• Grid Pattern (3 variants)");
                    ui.label("• Animated Beams");
                });

            // Usage panel
            egui::Window::new("Usage Examples")
                .default_pos([ui.available_width() - 320.0, 20.0])
                .default_width(300.0)
                .show(ctx, |ui| {
                    let code = match self.selected {
                        0 | 1 => "AuroraBackground::cyberpunk(w, h)\n  .with_speed(1.5)\n  .show(ui);",
                        2 => "MeteorShower::new(w, h)\n  .with_spawn_rate(3.0)\n  .with_angle(PI / 4.0)\n  .show(ui);",
                        3 | 4 | 5 => "GridPattern::new(w, h, 50.0)\n  .with_perspective(true)\n  .with_dots(color, 3.0)\n  .show(ui);",
                        6 => "let beam = AnimatedBeam::new(path)\n  .with_color(color)\n  .with_glow(1.0);\nbeams.add_beam(beam)\n  .show(ui);",
                        _ => "",
                    };

                    ui.code(code);

                    ui.add_space(15.0);
                    ui.label("Perfect for:");
                    ui.label("• Landing pages");
                    ui.label("• Hero sections");
                    ui.label("• Modal backgrounds");
                    ui.label("• Dashboard backdrops");
                });
        });
    }
}
