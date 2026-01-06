//! Moving Border Button Example
//!
//! Demonstrates buttons with animated gradient borders

use armas::{MovingBorder, Theme};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Armas - Moving Border Buttons"),
        ..Default::default()
    };

    eframe::run_native(
        "Moving Border",
        options,
        Box::new(|_cc| Ok(Box::new(MovingBorderApp::new()))),
    )
}

struct MovingBorderApp {
    theme: Theme,
    // Different button instances
    basic_button: MovingBorder,
    fast_button: MovingBorder,
    slow_button: MovingBorder,
    thick_border: MovingBorder,
    custom_colors: MovingBorder,
    large_button: MovingBorder,
}

impl MovingBorderApp {
    fn new() -> Self {
        Self {
            theme: Theme::dark(),
            basic_button: MovingBorder::new("Click Me"),
            fast_button: MovingBorder::new("Fast Animation").animation_speed(3.0),
            slow_button: MovingBorder::new("Slow Animation").animation_speed(0.5),
            thick_border: MovingBorder::new("Thick Border")
                .border_width(4.0)
                .animation_speed(1.5),
            custom_colors: MovingBorder::new("Custom Gradient")
                .border_colors(vec![
                    egui::Color32::from_rgb(255, 100, 100), // Red
                    egui::Color32::from_rgb(255, 200, 100), // Orange
                    egui::Color32::from_rgb(100, 255, 100), // Green
                ])
                .width(200.0),
            large_button: MovingBorder::new("Get Started")
                .width(250.0)
                .height(60.0)
                .border_width(3.0)
                .corner_radius(12.0)
                .animation_speed(1.2),
        }
    }
}

impl eframe::App for MovingBorderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Moving Border Buttons");
                    ui.add_space(10.0);
                    ui.label("Buttons with animated gradient borders that travel around the edges");
                    ui.add_space(40.0);
                });

                // Basic examples
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Basic Button")
                            .size(16.0)
                            .strong()
                            .color(self.theme.on_surface()),
                    );
                    ui.add_space(15.0);

                    if self.basic_button.show(ui, &self.theme).clicked() {
                        println!("Basic button clicked!");
                    }

                    ui.add_space(40.0);
                });

                ui.separator();
                ui.add_space(30.0);

                // Animation speed variations
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Animation Speed Variations")
                            .size(16.0)
                            .strong()
                            .color(self.theme.on_surface()),
                    );
                    ui.add_space(20.0);

                    ui.label(
                        egui::RichText::new("Fast (3x speed)")
                            .size(14.0)
                            .color(egui::Color32::from_gray(180)),
                    );
                    ui.add_space(10.0);
                    if self.fast_button.show(ui, &self.theme).clicked() {
                        println!("Fast button clicked!");
                    }

                    ui.add_space(25.0);

                    ui.label(
                        egui::RichText::new("Slow (0.5x speed)")
                            .size(14.0)
                            .color(egui::Color32::from_gray(180)),
                    );
                    ui.add_space(10.0);
                    if self.slow_button.show(ui, &self.theme).clicked() {
                        println!("Slow button clicked!");
                    }

                    ui.add_space(40.0);
                });

                ui.separator();
                ui.add_space(30.0);

                // Style variations
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Style Variations")
                            .size(16.0)
                            .strong()
                            .color(self.theme.on_surface()),
                    );
                    ui.add_space(20.0);

                    ui.label(
                        egui::RichText::new("Thick Border (4px)")
                            .size(14.0)
                            .color(egui::Color32::from_gray(180)),
                    );
                    ui.add_space(10.0);
                    if self.thick_border.show(ui, &self.theme).clicked() {
                        println!("Thick border button clicked!");
                    }

                    ui.add_space(25.0);

                    ui.label(
                        egui::RichText::new("Custom Colors (Red → Orange → Green)")
                            .size(14.0)
                            .color(egui::Color32::from_gray(180)),
                    );
                    ui.add_space(10.0);
                    if self.custom_colors.show(ui, &self.theme).clicked() {
                        println!("Custom colors button clicked!");
                    }

                    ui.add_space(40.0);
                });

                ui.separator();
                ui.add_space(30.0);

                // Hero CTA example
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Hero Call-to-Action")
                            .size(16.0)
                            .strong()
                            .color(self.theme.on_surface()),
                    );
                    ui.add_space(30.0);

                    ui.label(
                        egui::RichText::new("Ready to Build Something Amazing?")
                            .size(28.0)
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("Join thousands of developers using Armas UI")
                            .size(16.0)
                            .color(egui::Color32::from_gray(180)),
                    );
                    ui.add_space(30.0);

                    if self.large_button.show(ui, &self.theme).clicked() {
                        println!("Get Started clicked!");
                    }

                    ui.add_space(40.0);
                });

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
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(10.0);
                        ui.label("• Use for primary CTAs and important actions");
                        ui.label("• Adjust animation speed based on desired attention level");
                        ui.label("• Thicker borders (3-4px) work well for hero sections");
                        ui.label("• Match gradient colors to your brand palette");
                        ui.label("• Don't overuse - reserve for special buttons");
                    });
                });

                ui.add_space(30.0);
            });
        });
    }
}
