//! Gradient Showcase Example
//!
//! Demonstrates all gradient types and neon color palettes

use armas::ext::ArmasContextExt;
use armas::{ColorStop, Gradient, NeonPalette, PainterExt, Theme};
use eframe::egui;
use std::f32::consts::PI;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Armas - Gradient Showcase"),
        ..Default::default()
    };

    eframe::run_native(
        "Gradient Showcase",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(GradientShowcase::new()))
        }),
    )
}

struct GradientShowcase {
    rotation: f32,
}

impl GradientShowcase {
    fn new() -> Self {
        Self {
            rotation: 0.0,
        }
    }

    fn show_radial_gradients(&self, ui: &mut egui::Ui) {
        ui.heading("Radial Gradients");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // Simple two-color radial
            ui.vertical(|ui| {
                ui.label("Blue to Purple");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(150.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                let gradient = Gradient::linear(
                    egui::Color32::from_rgb(59, 130, 246),
                    egui::Color32::from_rgb(147, 51, 234),
                );

                let mesh = gradient.radial_mesh(center, 75.0, 32);
                painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
            });

            ui.add_space(20.0);

            // Multi-stop radial
            ui.vertical(|ui| {
                ui.label("Rainbow Radial");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(150.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                let colors = NeonPalette::rainbow();
                let stops: Vec<ColorStop> = colors
                    .iter()
                    .enumerate()
                    .map(|(i, &color)| ColorStop::new(i as f32 / (colors.len() - 1) as f32, color))
                    .collect();

                let gradient = Gradient::new(stops);
                let mesh = gradient.radial_mesh(center, 75.0, 48);
                painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
            });

            ui.add_space(20.0);

            // Cyberpunk radial
            ui.vertical(|ui| {
                ui.label("Cyberpunk");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(150.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                let colors = NeonPalette::cyberpunk();
                let stops: Vec<ColorStop> = colors
                    .iter()
                    .enumerate()
                    .map(|(i, &color)| ColorStop::new(i as f32 / (colors.len() - 1) as f32, color))
                    .collect();

                let gradient = Gradient::new(stops);
                let mesh = gradient.radial_mesh(center, 75.0, 48);
                painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
            });

            ui.add_space(20.0);

            // Aurora radial
            ui.vertical(|ui| {
                ui.label("Aurora");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(150.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                let colors = NeonPalette::aurora();
                let stops: Vec<ColorStop> = colors
                    .iter()
                    .enumerate()
                    .map(|(i, &color)| ColorStop::new(i as f32 / (colors.len() - 1) as f32, color))
                    .collect();

                let gradient = Gradient::new(stops);
                let mesh = gradient.radial_mesh(center, 75.0, 48);
                painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
            });
        });
    }

    fn show_conic_gradients(&self, ui: &mut egui::Ui) {
        ui.heading("Conic (Angular) Gradients");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // Static conic
            ui.vertical(|ui| {
                ui.label("Static Rainbow");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(150.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                let colors = NeonPalette::rainbow();
                let stops: Vec<ColorStop> = colors
                    .iter()
                    .enumerate()
                    .map(|(i, &color)| ColorStop::new(i as f32 / colors.len() as f32, color))
                    .collect();

                let gradient = Gradient::new(stops);
                let mesh = gradient.conic_mesh(center, 75.0, 0.0, 64);
                painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
            });

            ui.add_space(20.0);

            // Rotating conic
            ui.vertical(|ui| {
                ui.label("Rotating Synthwave");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(150.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                let colors = NeonPalette::synthwave();
                let stops: Vec<ColorStop> = colors
                    .iter()
                    .enumerate()
                    .map(|(i, &color)| ColorStop::new(i as f32 / colors.len() as f32, color))
                    .collect();

                let gradient = Gradient::new(stops);
                let mesh = gradient.conic_mesh(center, 75.0, self.rotation, 64);
                painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
            });

            ui.add_space(20.0);

            // Hot/cool split
            ui.vertical(|ui| {
                ui.label("Hot to Cool");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(150.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                let mut colors = NeonPalette::hot();
                colors.extend(NeonPalette::cool());

                let stops: Vec<ColorStop> = colors
                    .iter()
                    .enumerate()
                    .map(|(i, &color)| ColorStop::new(i as f32 / colors.len() as f32, color))
                    .collect();

                let gradient = Gradient::new(stops);
                let mesh = gradient.conic_mesh(center, 75.0, self.rotation * 0.5, 64);
                painter.add(egui::Shape::Mesh(mesh.into()));
            });

            ui.add_space(20.0);

            // Gold conic
            ui.vertical(|ui| {
                ui.label("Premium Gold");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::splat(150.0), egui::Sense::hover());
                let rect = response.rect;
                let center = rect.center();

                let colors = NeonPalette::gold();
                let stops: Vec<ColorStop> = colors
                    .iter()
                    .enumerate()
                    .map(|(i, &color)| ColorStop::new(i as f32 / colors.len() as f32, color))
                    .collect();

                let gradient = Gradient::new(stops);
                let mesh = gradient.conic_mesh(center, 75.0, -self.rotation * 0.7, 64);
                painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
            });
        });
    }

    fn show_linear_gradients(&self, ui: &mut egui::Ui) {
        ui.heading("Linear Gradients");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            for (name, colors) in [
                ("Cyberpunk", NeonPalette::cyberpunk()),
                ("Synthwave", NeonPalette::synthwave()),
                ("Aurora", NeonPalette::aurora()),
                ("Electric", NeonPalette::electric()),
            ] {
                ui.vertical(|ui| {
                    ui.label(name);
                    let (response, painter) =
                        ui.allocate_painter(egui::Vec2::new(150.0, 40.0), egui::Sense::hover());
                    let rect = response.rect;

                    let stops: Vec<ColorStop> = colors
                        .iter()
                        .enumerate()
                        .map(|(i, &color)| {
                            ColorStop::new(i as f32 / (colors.len() - 1) as f32, color)
                        })
                        .collect();

                    let gradient = Gradient::new(stops);
                    let mesh = gradient.rect_mesh(rect, true);
                    painter.add(egui::Shape::Mesh(std::sync::Arc::new(mesh)));
                });

                ui.add_space(10.0);
            }
        });
    }

    fn show_glow_effects(&self, ui: &mut egui::Ui, theme: &Theme) {
        ui.heading("Glow & Shadow Effects");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // Glow rect
            ui.vertical(|ui| {
                ui.label("Neon Glow Rect");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::new(150.0, 100.0), egui::Sense::hover());
                let rect = response.rect.shrink(20.0);

                painter.glow_rect(rect, 5.0.into(), egui::Color32::from_rgb(59, 130, 246), 0.8);
                painter.rect_filled(rect, 5.0, egui::Color32::from_rgb(59, 130, 246));
            });

            ui.add_space(20.0);

            // Shadow
            ui.vertical(|ui| {
                ui.label("Shadow");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::new(150.0, 100.0), egui::Sense::hover());
                let rect = response.rect.shrink(20.0);

                painter.shadow(
                    rect,
                    8.0.into(),
                    egui::Vec2::new(4.0, 4.0),
                    10.0,
                    egui::Color32::from_black_alpha(100),
                );
                painter.rect_filled(rect, 8.0, theme.surface());
            });

            ui.add_space(20.0);

            // Radial glow
            ui.vertical(|ui| {
                ui.label("Radial Glow");
                let (response, painter) =
                    ui.allocate_painter(egui::Vec2::new(150.0, 100.0), egui::Sense::hover());
                let center = response.rect.center();

                painter.radial_glow(center, 40.0, egui::Color32::from_rgb(147, 51, 234), 2.0);
            });
        });
    }

    fn show_color_palettes(&self, ui: &mut egui::Ui) {
        ui.heading("Neon Color Palettes");
        ui.add_space(10.0);

        for (name, colors) in [
            ("Cyberpunk", NeonPalette::cyberpunk()),
            ("Synthwave", NeonPalette::synthwave()),
            ("Aurora", NeonPalette::aurora()),
            ("Rainbow", NeonPalette::rainbow()),
            ("Electric", NeonPalette::electric()),
            ("Hot", NeonPalette::hot()),
            ("Cool", NeonPalette::cool()),
            ("Gold", NeonPalette::gold()),
        ] {
            ui.horizontal(|ui| {
                ui.label(format!("{:12}", name));
                for color in colors {
                    let (response, painter) =
                        ui.allocate_painter(egui::Vec2::splat(30.0), egui::Sense::hover());
                    painter.rect_filled(response.rect, 4.0, color);

                    response.on_hover_text(format!(
                        "RGB({}, {}, {})",
                        color.r(),
                        color.g(),
                        color.b()
                    ));
                }
            });
            ui.add_space(5.0);
        }
    }
}

impl eframe::App for GradientShowcase {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        // Animate rotation
        self.rotation += ctx.input(|i| i.stable_dt) * PI / 2.0;
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("🎨 Armas Gradient Showcase");
                ui.label("Demonstrating all gradient types and neon color palettes");
                ui.add_space(20.0);

                self.show_radial_gradients(ui);
                ui.add_space(30.0);

                self.show_conic_gradients(ui);
                ui.add_space(30.0);

                self.show_linear_gradients(ui);
                ui.add_space(30.0);

                self.show_glow_effects(ui, &theme);
                ui.add_space(30.0);

                self.show_color_palettes(ui);
                ui.add_space(30.0);
            });
        });
    }
}
