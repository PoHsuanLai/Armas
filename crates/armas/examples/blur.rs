use armas::ext::ArmasContextExt;
use armas::effects::blur::BlurCache;
use armas::Theme;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Blur Effect Demo",
        options,
        Box::new(|cc| Ok(Box::new(BlurDemo::new(cc)))),
    )
}

struct BlurDemo {
    blur_radius: f32,
    original_texture: Option<egui::TextureHandle>,
    blurred_texture: Option<egui::TextureHandle>,
    original_image: Option<egui::ColorImage>,
    blur_cache: BlurCache, // Automatic caching!
}

impl BlurDemo {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            blur_radius: 10.0,
            original_texture: None,
            blurred_texture: None,
            original_image: None,
            blur_cache: BlurCache::new(),
        }
    }

    fn create_sample_image() -> egui::ColorImage {
        let size = [200, 200];
        let mut pixels = vec![egui::Color32::from_rgb(30, 30, 40); size[0] * size[1]];

        // Draw some colorful shapes
        for y in 0..size[1] {
            for x in 0..size[0] {
                let idx = y * size[0] + x;

                // Gradient background
                let t = y as f32 / size[1] as f32;
                let r = (30.0 + t * 60.0) as u8;
                let g = (30.0 + t * 80.0) as u8;
                let b = (40.0 + t * 100.0) as u8;
                pixels[idx] = egui::Color32::from_rgb(r, g, b);

                // Red circle
                let dx = x as f32 - 50.0;
                let dy = y as f32 - 50.0;
                if dx * dx + dy * dy < 900.0 {
                    pixels[idx] = egui::Color32::from_rgb(255, 100, 100);
                }

                // Green circle
                let dx = x as f32 - 150.0;
                let dy = y as f32 - 50.0;
                if dx * dx + dy * dy < 900.0 {
                    pixels[idx] = egui::Color32::from_rgb(100, 255, 100);
                }

                // Blue circle
                let dx = x as f32 - 100.0;
                let dy = y as f32 - 150.0;
                if dx * dx + dy * dy < 900.0 {
                    pixels[idx] = egui::Color32::from_rgb(100, 100, 255);
                }

                // Yellow square
                if x >= 140 && x < 180 && y >= 140 && y < 180 {
                    pixels[idx] = egui::Color32::from_rgb(255, 255, 100);
                }
            }
        }

        egui::ColorImage {
            size,
            pixels,
            source_size: egui::Vec2::new(size[0] as _, size[1] as _),
        }
    }
}

impl eframe::App for BlurDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        // Create textures on first frame
        if self.original_texture.is_none() {
            let image = Self::create_sample_image();
            self.original_texture =
                Some(ctx.load_texture("original", image.clone(), egui::TextureOptions::LINEAR));
            self.original_image = Some(image);

            // Create initial blurred version using cache
            if let Some(ref img) = self.original_image {
                let blurred = self.blur_cache.get_or_blur(img, self.blur_radius);
                self.blurred_texture =
                    Some(ctx.load_texture("blurred", blurred, egui::TextureOptions::LINEAR));
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(theme.background()))
            .show(ctx, |ui| {
                ui.add_space(20.0);

                ui.heading(
                    egui::RichText::new("Blur Effect Demo")
                        .size(24.0)
                        .color(theme.on_background()),
                );

                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new(
                        "CPU-based Gaussian blur (works everywhere, no special setup)",
                    )
                    .color(theme.on_surface_variant()),
                );

                ui.add_space(20.0);

                // Blur radius slider
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Blur Radius:").color(theme.on_background()));
                    let response =
                        ui.add(egui::Slider::new(&mut self.blur_radius, 0.0..=30.0).step_by(1.0));

                    // Update blur when slider changes
                    // The cache makes this super fast - if we've seen this radius before, it's instant!
                    if response.changed() {
                        if let Some(ref img) = self.original_image {
                            let blurred = self.blur_cache.get_or_blur(img, self.blur_radius);
                            self.blurred_texture = Some(ctx.load_texture(
                                "blurred",
                                blurred,
                                egui::TextureOptions::LINEAR,
                            ));
                        }
                    }
                });

                ui.add_space(30.0);

                // Show original and blurred side by side
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Original")
                                .size(16.0)
                                .color(theme.on_background()),
                        );
                        ui.add_space(10.0);
                        if let Some(texture) = &self.original_texture {
                            ui.image(texture);
                        }
                    });

                    ui.add_space(40.0);

                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(format!("Blurred (radius: {})", self.blur_radius))
                                .size(16.0)
                                .color(theme.on_background()),
                        );
                        ui.add_space(10.0);
                        if let Some(texture) = &self.blurred_texture {
                            ui.image(texture);
                        }
                    });
                });

                ui.add_space(30.0);

                // Info box
                egui::Frame::none()
                    .fill(theme.surface())
                    .rounding(8.0)
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("How to use:")
                                .strong()
                                .color(theme.primary()),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(
                                "use armas::effects::blur::{blur_image_cpu, create_blurred_image};

// Blur an image in-place
let mut image = load_image(\"photo.png\");
blur_image_cpu(&mut image, 10.0);

// Or create a blurred copy
let blurred = create_blurred_image(&image, 10.0);
let texture = ctx.load_texture(\"blurred\", blurred, TextureOptions::LINEAR);
ui.image(&texture);",
                            )
                            .code()
                            .color(theme.on_surface()),
                        );
                    });

                ui.add_space(20.0);

                // Use cases
                egui::Frame::none()
                    .fill(theme.surface_variant())
                    .rounding(8.0)
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Use cases:")
                                .strong()
                                .color(theme.primary()),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(
                                "• Blur background images for better text readability",
                            )
                            .color(theme.on_surface()),
                        );
                        ui.label(
                            egui::RichText::new(
                                "• Create frosted glass effects with semi-transparent panels",
                            )
                            .color(theme.on_surface()),
                        );
                        ui.label(
                            egui::RichText::new("• Blur user avatars or profile pictures")
                                .color(theme.on_surface()),
                        );
                        ui.label(
                            egui::RichText::new("• Generate thumbnails with privacy blur")
                                .color(theme.on_surface()),
                        );
                    });
            });
    }
}
