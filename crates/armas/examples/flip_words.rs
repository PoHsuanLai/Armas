//! Flip Words Animation Example
//!
//! Demonstrates animated word-flipping text effects

use armas::{FlipStyle, FlipWords, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Armas - Flip Words Animation"),
        ..Default::default()
    };

    eframe::run_native(
        "Flip Words",
        options,
        Box::new(|_cc| Ok(Box::new(FlipWordsApp::new()))),
    )
}

struct FlipWordsApp {
    theme: Theme,
    // Different flip word instances
    vertical_flip: FlipWords,
    horizontal_flip: FlipWords,
    fade_flip: FlipWords,
    fast_flip: FlipWords,
    large_flip: FlipWords,
    highlighted_flip: FlipWords,
}

impl FlipWordsApp {
    fn new() -> Self {
        let theme = Theme::dark();

        Self {
            theme: theme.clone(),
            vertical_flip: FlipWords::new(vec!["Innovative", "Modern", "Powerful", "Beautiful"])
                .duration(2.5)
                .font_size(32.0)
                .style(FlipStyle::Vertical),
            horizontal_flip: FlipWords::new(vec!["Fast", "Reliable", "Scalable", "Secure"])
                .duration(2.5)
                .font_size(32.0)
                .style(FlipStyle::Horizontal),
            fade_flip: FlipWords::new(vec!["Simple", "Elegant", "Clean", "Smooth"])
                .duration(2.5)
                .font_size(32.0)
                .style(FlipStyle::Fade),
            fast_flip: FlipWords::new(vec!["Quick", "Rapid", "Swift", "Speedy"])
                .duration(1.5)
                .font_size(24.0)
                .style(FlipStyle::Vertical),
            large_flip: FlipWords::new(vec!["BOLD", "EPIC", "HUGE", "MEGA"])
                .duration(3.0)
                .font_size(64.0)
                .style(FlipStyle::Vertical)
                .highlight(egui::Color32::from_rgb(59, 130, 246)),
            highlighted_flip: FlipWords::new(vec!["Design", "Create", "Build", "Launch"])
                .duration(2.0)
                .font_size(40.0)
                .style(FlipStyle::Fade)
                .highlight(egui::Color32::from_rgb(236, 72, 153)),
        }
    }
}

impl eframe::App for FlipWordsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Flip Words Animation");
                    ui.add_space(10.0);
                    ui.label("Animated text that cycles through words with smooth transitions");
                    ui.add_space(40.0);
                });

                // Comparison of flip styles
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Animation Styles")
                            .size(18.0)
                            .strong()
                            .color(self.theme.on_surface()),
                    );
                    ui.add_space(20.0);

                    // Vertical Flip
                    ui.horizontal(|ui| {
                        ui.add_space(250.0);
                        ui.label(
                            egui::RichText::new("Vertical Flip:")
                                .size(16.0)
                                .color(egui::Color32::from_gray(180)),
                        );
                        ui.add_space(20.0);
                        self.vertical_flip.show(ui, &self.theme);
                    });
                    ui.add_space(25.0);

                    // Horizontal Flip
                    ui.horizontal(|ui| {
                        ui.add_space(230.0);
                        ui.label(
                            egui::RichText::new("Horizontal Flip:")
                                .size(16.0)
                                .color(egui::Color32::from_gray(180)),
                        );
                        ui.add_space(20.0);
                        self.horizontal_flip.show(ui, &self.theme);
                    });
                    ui.add_space(25.0);

                    // Fade
                    ui.horizontal(|ui| {
                        ui.add_space(280.0);
                        ui.label(
                            egui::RichText::new("Fade:")
                                .size(16.0)
                                .color(egui::Color32::from_gray(180)),
                        );
                        ui.add_space(20.0);
                        self.fade_flip.show(ui, &self.theme);
                    });
                    ui.add_space(50.0);
                });

                ui.separator();
                ui.add_space(40.0);

                // Speed variations
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Speed Variations")
                            .size(18.0)
                            .strong()
                            .color(self.theme.on_surface()),
                    );
                    ui.add_space(20.0);

                    ui.label(
                        egui::RichText::new("Fast (1.5s per word)")
                            .size(14.0)
                            .color(egui::Color32::from_gray(150)),
                    );
                    ui.add_space(10.0);
                    self.fast_flip.show(ui, &self.theme);
                    ui.add_space(50.0);
                });

                ui.separator();
                ui.add_space(40.0);

                // Hero examples
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Hero Section Examples")
                            .size(18.0)
                            .strong()
                            .color(self.theme.on_surface()),
                    );
                    ui.add_space(30.0);

                    // Large bold text
                    ui.label(
                        egui::RichText::new("Make Your Ideas")
                            .size(48.0)
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(10.0);
                    self.large_flip.show(ui, &self.theme);
                    ui.add_space(40.0);

                    // With context
                    ui.horizontal(|ui| {
                        ui.add_space(320.0);
                        ui.label(
                            egui::RichText::new("We help you")
                                .size(36.0)
                                .color(egui::Color32::from_gray(200)),
                        );
                        ui.add_space(10.0);
                        self.highlighted_flip.show(ui, &self.theme);
                    });
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("amazing products")
                            .size(36.0)
                            .color(egui::Color32::from_gray(200)),
                    );
                    ui.add_space(50.0);
                });

                ui.separator();
                ui.add_space(20.0);

                // Usage tips
                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("💡 Usage Tips")
                                .size(16.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• Use vertical flip for emphasis and impact");
                        ui.label("• Fade is best for subtle, professional transitions");
                        ui.label("• Keep word lists short (3-5 words) for best effect");
                        ui.label("• Adjust duration based on word length and context");
                        ui.label("• Use highlight colors to match your brand");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
