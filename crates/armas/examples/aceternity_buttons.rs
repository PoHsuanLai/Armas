//! Aceternity Buttons Example
//!
//! Showcases all Aceternity-inspired button styles with Inter font

use armas::ext::ArmasContextExt;
use armas::fonts;
use armas::{
    BrutalButton, FigmaButton, InvertButton, ShimmerButton, SimpleButton, SketchButton,
    SpotifyButton, Theme,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_title("Armas - Aceternity Buttons Collection"),
        ..Default::default()
    };

    eframe::run_native(
        "Aceternity Buttons",
        options,
        Box::new(|cc| {
            // Load Inter fonts
            setup_fonts(&cc.egui_ctx);

            // Set Armas theme
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(AceternityButtonsExample::default()))
        }),
    )
}

/// Load Inter fonts for Aceternity-style buttons
fn setup_fonts(ctx: &egui::Context) {
    let inter_regular = include_bytes!("../fonts/Inter-Regular.otf");
    let inter_medium = include_bytes!("../fonts/Inter-Medium.otf");
    let inter_semibold = include_bytes!("../fonts/Inter-SemiBold.otf");
    let inter_bold = include_bytes!("../fonts/Inter-Bold.otf");

    fonts::load_font_family(
        ctx,
        "Inter",
        inter_regular,
        Some(inter_medium),
        Some(inter_semibold),
        Some(inter_bold),
    );
}

#[derive(Default)]
struct AceternityButtonsExample {}

impl eframe::App for AceternityButtonsExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(20.0);

                ui.heading("🎨 Aceternity Button Collection");
                ui.label("Inspired by Aceternity UI - Premium button styles for your applications");
                ui.add_space(30.0);

                // Row 1: Sketch, Simple, Invert
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Sketch Button");
                        ui.label("Shadow offset on hover");
                        ui.add_space(10.0);
                        if SketchButton::new("Sketch").show(ui).clicked() {
                            println!("Sketch button clicked!");
                        }
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label("Simple Button");
                        ui.label("Elegant with subtle lift");
                        ui.add_space(10.0);
                        if SimpleButton::new("Simple").show(ui).clicked() {
                            println!("Simple button clicked!");
                        }
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label("Invert Button");
                        ui.label("Inverts colors on hover");
                        ui.add_space(10.0);
                        if InvertButton::new("Invert it").show(ui).clicked() {
                            println!("Invert button clicked!");
                        }
                    });
                });

                ui.add_space(40.0);
                ui.separator();
                ui.add_space(40.0);

                // Row 2: Brutal, Shimmer
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Brutal Button");
                        ui.label("Brutalist stacked shadows");
                        ui.add_space(10.0);
                        if BrutalButton::new("Brutal").show(ui).clicked() {
                            println!("Brutal button clicked!");
                        }
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label("Shimmer Button");
                        ui.label("Animated shimmer effect");
                        ui.add_space(10.0);
                        if ShimmerButton::new("Shimmer").show(ui).clicked() {
                            println!("Shimmer button clicked!");
                        }
                    });
                });

                ui.add_space(40.0);
                ui.separator();
                ui.add_space(40.0);

                // Row 3: Spotify, Figma
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Spotify Button");
                        ui.label("Spotify brand style");
                        ui.add_space(10.0);
                        if SpotifyButton::new("Spotify").show(ui).clicked() {
                            println!("Spotify button clicked!");
                        }
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label("Figma Button");
                        ui.label("Simple black with lift");
                        ui.add_space(10.0);
                        if FigmaButton::new("Figma").show(ui).clicked() {
                            println!("Figma button clicked!");
                        }
                    });

                    ui.add_space(30.0);

                    ui.vertical(|ui| {
                        ui.label("Figma Outline");
                        ui.label("Outlined variant");
                        ui.add_space(10.0);
                        if FigmaButton::outlined("Outline").show(ui).clicked() {
                            println!("Figma Outline button clicked!");
                        }
                    });
                });

                ui.add_space(40.0);
                ui.separator();
                ui.add_space(40.0);

                // Info section
                ui.heading("ℹ️ Usage");
                ui.label("All buttons support:");
                ui.label("  • Hover effects");
                ui.label("  • Click detection");
                ui.label("  • Custom sizing with .min_size()");
                ui.label("  • Enable/disable with .enabled()");

                ui.add_space(20.0);

                ui.label("Example code:");
                ui.code("SketchButton::new(\"Click me\").show(ui);");
                ui.code("GradientButton::new(\"Submit\").min_size(Vec2::new(120.0, 40.0)).show(ui);");

                ui.add_space(40.0);
            });
        });
    }
}
