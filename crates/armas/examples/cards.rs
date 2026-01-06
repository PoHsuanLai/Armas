//! Card Effects Showcase
//!
//! Demonstrates Phase 3 interactive card effects

use armas::{CardStack, HoverCard, StackCard, Theme, TiltCard};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Armas - Card Effects"),
        ..Default::default()
    };

    eframe::run_native(
        "Card Effects",
        options,
        Box::new(|_cc| Ok(Box::new(CardEffectsApp::new()))),
    )
}

struct CardEffectsApp {
    theme: Theme,
    tilt_card: TiltCard,
    card_stack: CardStack,
    hover_card_1: HoverCard,
    hover_card_2: HoverCard,
}

impl CardEffectsApp {
    fn new() -> Self {
        // Create card stack
        let stack = CardStack::new(300.0, 400.0)
            .add_card(StackCard {
                title: "Feature 1".to_string(),
                description: "Advanced tilt effects with mouse tracking and glare".to_string(),
                color: egui::Color32::from_rgb(60, 40, 100),
            })
            .add_card(StackCard {
                title: "Feature 2".to_string(),
                description: "Auto-rotating card stack with smooth transitions".to_string(),
                color: egui::Color32::from_rgb(40, 80, 100),
            })
            .add_card(StackCard {
                title: "Feature 3".to_string(),
                description: "Hover cards with content reveal animations".to_string(),
                color: egui::Color32::from_rgb(100, 40, 80),
            })
            .with_rotation_interval(4.0)
            .with_transition_duration(0.6);

        let theme = Theme::dark();
        Self {
            theme: theme.clone(),
            tilt_card: TiltCard::new(350.0, 250.0, &theme)
                .with_tilt_strength(0.4)
                .with_glare(true)
                .with_elevation(12.0),
            card_stack: stack,
            hover_card_1: HoverCard::new(280.0, 180.0, &theme)
                .with_scale(1.08)
                .with_elevation(6.0),
            hover_card_2: HoverCard::new(280.0, 180.0, &theme)
                .with_scale(1.08)
                .with_elevation(6.0)
                .with_background(egui::Color32::from_rgb(40, 30, 60))
                .with_hover_background(egui::Color32::from_rgb(55, 40, 80)),
        }
    }
}

impl eframe::App for CardEffectsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading("Phase 3: Interactive Card Effects");
                ui.add_space(10.0);
                ui.label("Hover over cards to see interactive animations");
                ui.add_space(30.0);
            });

            // Layout cards in a grid
            ui.horizontal(|ui| {
                ui.add_space(40.0);

                // Column 1: Tilt Card
                ui.vertical(|ui| {
                    ui.label("3D Tilt Card");
                    ui.add_space(10.0);

                    self.tilt_card.show(ui, &self.theme, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.heading("Tilt Effect");
                            ui.add_space(15.0);
                            ui.label("Move your mouse around");
                            ui.label("to see the 3D tilt effect");
                            ui.label("with dynamic glare!");
                        });
                    });

                    ui.add_space(30.0);

                    ui.label("Hover Cards");
                    ui.add_space(10.0);

                    // Hover card 1
                    self.hover_card_1.show(
                        ui,
                        &self.theme,
                        |ui, opacity| {
                            ui.vertical_centered(|ui| {
                                let alpha = (255.0 * opacity) as u8;
                                ui.visuals_mut().override_text_color = Some(
                                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                                );
                                ui.add_space(20.0);
                                ui.heading("Base Content");
                                ui.add_space(10.0);
                                ui.label("Hover to reveal more");
                            });
                        },
                        |ui, opacity| {
                            ui.vertical_centered(|ui| {
                                let alpha = (255.0 * opacity) as u8;
                                ui.visuals_mut().override_text_color = Some(
                                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                                );
                                ui.add_space(20.0);
                                ui.heading("Hidden Details");
                                ui.add_space(10.0);
                                ui.label("✨ Smooth transitions");
                                ui.label("🎨 Custom styling");
                                ui.label("⚡ Fast animations");
                            });
                        },
                    );
                });

                ui.add_space(50.0);

                // Column 2: Card Stack
                ui.vertical(|ui| {
                    ui.label("Auto-Rotating Card Stack");
                    ui.add_space(10.0);

                    self.card_stack.show(ui, &self.theme);

                    ui.add_space(20.0);
                    ui.label("Cards rotate every 4 seconds");
                });

                ui.add_space(50.0);

                // Column 3: Another hover card + usage
                ui.vertical(|ui| {
                    ui.label("Custom Styled Hover Card");
                    ui.add_space(10.0);

                    self.hover_card_2.show(
                        ui,
                        &self.theme,
                        |ui, opacity| {
                            ui.vertical_centered(|ui| {
                                let alpha = (255.0 * opacity) as u8;
                                ui.visuals_mut().override_text_color = Some(
                                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                                );
                                ui.add_space(20.0);
                                ui.heading("Projects");
                                ui.add_space(10.0);
                                ui.label("Click to see details");
                            });
                        },
                        |ui, opacity| {
                            ui.vertical_centered(|ui| {
                                let alpha = (255.0 * opacity) as u8;
                                ui.visuals_mut().override_text_color = Some(
                                    egui::Color32::from_rgba_unmultiplied(200, 150, 255, alpha),
                                );
                                ui.add_space(20.0);
                                ui.heading("Project Details");
                                ui.add_space(10.0);
                                ui.label("• Rust + egui");
                                ui.label("• 60fps animations");
                                ui.label("• Aceternity styling");
                            });
                        },
                    );

                    ui.add_space(30.0);

                    ui.label("Usage Example:");
                    ui.add_space(5.0);
                    ui.code(
                        "TiltCard::new(350.0, 250.0)
  .with_tilt_strength(0.2)
  .with_glare(true)
  .show(ui, &theme, |ui| {
    // Content here
  });",
                    );
                });
            });

            // Bottom info panel
            ui.add_space(30.0);
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(40.0);
                ui.label("✅ Phase 3 Complete!");
                ui.add_space(20.0);
                ui.label("Components: TiltCard • CardStack • HoverCard");
                ui.add_space(20.0);
                ui.label("Features: Mouse tracking • Auto-rotation • Content reveal");
            });
        });
    }
}
