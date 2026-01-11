//! Text Effects Showcase
//!
//! Demonstrates Phase 4 advanced text effects

use armas::ext::ArmasContextExt;
use armas::{GradientText, ScrambleText, Theme, Typewriter, WordTypewriter};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Armas - Text Effects"),
        ..Default::default()
    };

    eframe::run_native(
        "Text Effects",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(TextEffectsApp::new()))
        }),
    )
}

struct TextEffectsApp {

    // Typewriter effects
    typewriter1: Typewriter,
    typewriter2: Typewriter,
    word_typewriter: WordTypewriter,

    // Gradient texts
    gradient1: GradientText,
    gradient2: GradientText,
    gradient3: GradientText,

    // Scramble effects
    scramble1: ScrambleText,
    scramble2: ScrambleText,
}

impl TextEffectsApp {
    fn new() -> Self {
        Self {

            // Typewriter variants
            typewriter1: Typewriter::new("Welcome to Armas UI Library")
                .with_speed(15.0)
                .with_cursor(true)
                .with_loop(true)
                .with_loop_delay(2.0),

            typewriter2: Typewriter::new("Fast typing with no cursor...")
                .with_speed(30.0)
                .with_cursor(false)
                .with_loop(true)
                .with_loop_delay(1.5),

            word_typewriter: WordTypewriter::new(
                "Build beautiful interfaces with smooth animations",
            )
            .with_speed(3.0)
            .with_loop(true)
            .with_loop_delay(2.0),

            // Gradient variants
            gradient1: GradientText::rainbow("Rainbow Animated Text")
                .with_animation(true)
                .with_animation_speed(0.5)
                .with_per_character(true),

            gradient2: GradientText::two_color(
                "Cyberpunk Gradient",
                egui::Color32::from_rgb(0, 255, 255),
                egui::Color32::from_rgb(255, 0, 255),
            )
            .with_animation(true)
            .with_animation_speed(0.8)
            .with_per_character(true),

            gradient3: GradientText::new(
                "Neon Glow Effect",
                vec![
                    egui::Color32::from_rgb(255, 0, 128),
                    egui::Color32::from_rgb(128, 0, 255),
                    egui::Color32::from_rgb(0, 128, 255),
                ],
            )
            .with_animation(true)
            .with_animation_speed(1.0)
            .with_per_character(false),

            // Scramble variants
            scramble1: ScrambleText::new("TERMINAL HACKING...")
                .with_speed(1.5)
                .with_frame_interval(0.05)
                .with_loop(true)
                .with_loop_delay(1.5),

            scramble2: ScrambleText::new("DECRYPTING DATA...")
                .with_speed(2.0)
                .with_charset("01")
                .with_frame_interval(0.03)
                .with_loop(true)
                .with_loop_delay(1.0),
        }
    }
}

impl eframe::App for TextEffectsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading("Phase 4: Advanced Text Effects");
                ui.add_space(10.0);
                ui.label("Typewriter • Gradient • Scramble effects");
                ui.add_space(30.0);
            });

            ui.horizontal(|ui| {
                ui.add_space(40.0);

                // Column 1: Typewriter Effects
                ui.vertical(|ui| {
                    ui.heading("Typewriter Effects");
                    ui.add_space(15.0);

                    ui.label("With cursor:");
                    self.typewriter1.show_styled(ui, |text| {
                        egui::RichText::new(text)
                            .size(18.0)
                            .color(egui::Color32::from_rgb(100, 200, 255))
                    });

                    ui.add_space(20.0);
                    ui.label("Fast, no cursor:");
                    self.typewriter2.show_styled(ui, |text| {
                        egui::RichText::new(text)
                            .size(16.0)
                            .color(egui::Color32::from_rgb(150, 255, 150))
                    });

                    ui.add_space(20.0);
                    ui.label("Word-by-word:");
                    self.word_typewriter.show_styled(ui, |text| {
                        egui::RichText::new(text)
                            .size(16.0)
                            .color(egui::Color32::from_rgb(255, 200, 100))
                    });

                    ui.add_space(30.0);

                    ui.label("Usage:");
                    ui.code(
                        "Typewriter::new(\"text\")
  .with_speed(15.0)
  .with_cursor(true)
  .with_loop(true)
  .show(ui);",
                    );
                });

                ui.add_space(50.0);

                // Column 2: Gradient & Scramble
                ui.vertical(|ui| {
                    ui.heading("Gradient Text");
                    ui.add_space(15.0);

                    self.gradient1.show(ui);
                    ui.add_space(10.0);
                    self.gradient2.show(ui);
                    ui.add_space(10.0);
                    self.gradient3.show(ui);

                    ui.add_space(30.0);

                    ui.heading("Scramble / Glitch");
                    ui.add_space(15.0);

                    self.scramble1.show_styled(ui, |text| {
                        egui::RichText::new(text)
                            .size(18.0)
                            .color(egui::Color32::from_rgb(0, 255, 100))
                            .monospace()
                    });

                    ui.add_space(10.0);

                    self.scramble2.show_styled(ui, |text| {
                        egui::RichText::new(text)
                            .size(16.0)
                            .color(egui::Color32::from_rgb(100, 200, 255))
                            .monospace()
                    });

                    ui.add_space(30.0);

                    ui.label("Gradient usage:");
                    ui.code(
                        "GradientText::rainbow(\"text\")
  .with_animation(true)
  .with_animation_speed(0.5)
  .show(ui);",
                    );

                    ui.add_space(15.0);

                    ui.label("Scramble usage:");
                    ui.code(
                        "ScrambleText::new(\"text\")
  .with_speed(2.0)
  .with_loop(true)
  .show(ui);",
                    );
                });
            });

            // Bottom info panel
            ui.add_space(30.0);
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(40.0);
                ui.label("✅ Phase 4 Complete!");
                ui.add_space(20.0);
                ui.label("Components: Typewriter • GradientText • ScrambleText");
                ui.add_space(20.0);
                ui.label("Features: Character reveal • Color animation • Glitch effects");
            });
        });
    }
}
