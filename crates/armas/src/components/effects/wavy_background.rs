//! Wavy Background
//!
//! Animated wave effects for backgrounds and hero sections inspired by Aceternity UI
//! Uses Perlin-style noise for organic, natural-looking waves

use crate::ext::ArmasContextExt;
use crate::context::ArmasContextExt;
use egui::{Color32, Pos2, Response, Ui, Vec2};

/// Simple 2D Perlin-style noise implementation
/// Based on improved noise by Ken Perlin (2002)
struct NoiseGenerator {
    permutation: [u8; 512],
}

impl NoiseGenerator {
    fn new(seed: u64) -> Self {
        // Generate permutation table with seed
        let mut perm: Vec<u8> = (0..256).map(|i| i as u8).collect();

        // Simple LCG pseudo-random shuffle
        let mut rng_state = seed;
        for i in (1..256).rev() {
            rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            let j = (rng_state % (i as u64 + 1)) as usize;
            perm.swap(i, j);
        }

        // Duplicate to avoid wrapping
        let mut permutation = [0u8; 512];
        for i in 0..512 {
            permutation[i] = perm[i % 256];
        }

        Self { permutation }
    }

    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(t: f32, a: f32, b: f32) -> f32 {
        a + t * (b - a)
    }

    fn grad(hash: u8, x: f32, y: f32) -> f32 {
        let h = hash & 3;
        let u = if h < 2 { x } else { y };
        let v = if h < 2 { y } else { x };
        (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
    }

    /// Generate 3D Perlin noise value at (x, y, z)
    /// Returns value in range approximately [-1, 1]
    fn noise3d(&self, x: f32, y: f32, z: f32) -> f32 {
        let xi = (x.floor() as i32 & 255) as usize;
        let yi = (y.floor() as i32 & 255) as usize;
        let zi = (z.floor() as i32 & 255) as usize;

        let xf = x - x.floor();
        let yf = y - y.floor();
        let zf = z - z.floor();

        let u = Self::fade(xf);
        let v = Self::fade(yf);
        let w = Self::fade(zf);

        // Hash coordinates of the 8 cube corners
        let a = self.permutation[xi] as usize + yi;
        let aa = self.permutation[a] as usize + zi;
        let ab = self.permutation[a + 1] as usize + zi;
        let b = self.permutation[xi + 1] as usize + yi;
        let ba = self.permutation[b] as usize + zi;
        let bb = self.permutation[b + 1] as usize + zi;

        // Blend results from 8 corners
        let x1 = Self::lerp(u,
            Self::grad3d(self.permutation[aa], xf, yf, zf),
            Self::grad3d(self.permutation[ba], xf - 1.0, yf, zf)
        );
        let x2 = Self::lerp(u,
            Self::grad3d(self.permutation[ab], xf, yf - 1.0, zf),
            Self::grad3d(self.permutation[bb], xf - 1.0, yf - 1.0, zf)
        );
        let y1 = Self::lerp(v, x1, x2);

        let x3 = Self::lerp(u,
            Self::grad3d(self.permutation[aa + 1], xf, yf, zf - 1.0),
            Self::grad3d(self.permutation[ba + 1], xf - 1.0, yf, zf - 1.0)
        );
        let x4 = Self::lerp(u,
            Self::grad3d(self.permutation[ab + 1], xf, yf - 1.0, zf - 1.0),
            Self::grad3d(self.permutation[bb + 1], xf - 1.0, yf - 1.0, zf - 1.0)
        );
        let y2 = Self::lerp(v, x3, x4);

        Self::lerp(w, y1, y2)
    }

    fn grad3d(hash: u8, x: f32, y: f32, z: f32) -> f32 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 { y } else if h == 12 || h == 14 { x } else { z };
        (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
    }
}

/// Wavy background component
///
/// Creates animated organic waves using Perlin noise (inspired by Aceternity UI)
pub struct WavyBackground {
    width: f32,
    height: f32,
    wave_count: usize,
    colors: Vec<Color32>,
    wave_width: f32,  // Renamed from frequency for clarity
    background_fill: Color32,
    wave_opacity: f32,
    speed: f32,
    blur: f32,

    // Internal state
    time: f32,
    noise: NoiseGenerator,
}

impl WavyBackground {
    /// Create a new wavy background
    /// Wave colors will be derived from theme when shown
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            wave_count: 5,
            colors: Vec::new(), // Will be populated from theme
            wave_width: 50.0,
            background_fill: Color32::BLACK,
            wave_opacity: 0.5,
            speed: 1.0,  // "fast"
            blur: 10.0,
            time: 0.0,
            noise: NoiseGenerator::new(12345),
        }
    }

    /// Set number of waves
    pub fn wave_count(mut self, count: usize) -> Self {
        self.wave_count = count.max(1);
        self
    }

    /// Set wave colors (Aceternity default: cyan to purple shades)
    pub fn colors(mut self, colors: Vec<Color32>) -> Self {
        if !colors.is_empty() {
            self.colors = colors;
        }
        self
    }

    /// Set wave width (default: 50)
    pub fn wave_width(mut self, width: f32) -> Self {
        self.wave_width = width.max(10.0);
        self
    }

    /// Set background fill color
    pub fn background_fill(mut self, color: Color32) -> Self {
        self.background_fill = color;
        self
    }

    /// Set wave opacity (0.0 to 1.0, default: 0.5)
    pub fn wave_opacity(mut self, opacity: f32) -> Self {
        self.wave_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set animation speed ("slow" = 0.5, "fast" = 1.0)
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.0);
        self
    }

    /// Set blur amount (default: 10)
    pub fn blur(mut self, blur: f32) -> Self {
        self.blur = blur.max(0.0);
        self
    }

    /// Show the wavy background
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();

        // Initialize wave colors from theme if not set
        if self.colors.is_empty() {
            let [c1, c2, c3] = theme.gradient();
            // Create 5 colors by interpolating between the 3 theme colors
            self.colors = vec![
                c1,
                Color32::from_rgb(
                    ((c1.r() as u16 + c2.r() as u16) / 2) as u8,
                    ((c1.g() as u16 + c2.g() as u16) / 2) as u8,
                    ((c1.b() as u16 + c2.b() as u16) / 2) as u8,
                ),
                c2,
                Color32::from_rgb(
                    ((c2.r() as u16 + c3.r() as u16) / 2) as u8,
                    ((c2.g() as u16 + c3.g() as u16) / 2) as u8,
                    ((c2.b() as u16 + c3.b() as u16) / 2) as u8,
                ),
                c3,
            ];
        }

        let dt = ui.input(|i| i.stable_dt);
        self.time += dt * self.speed;
        ui.ctx().request_repaint();

        let (response, painter) =
            ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());

        let rect = response.rect;

        if ui.is_rect_visible(rect) {
            // Draw background fill
            painter.rect_filled(rect, 0.0, self.background_fill);

            // Draw waves using 3D Perlin noise (Aceternity style: noise(x/800, 0.3*i, time))
            let segments = (rect.width() / 2.0) as usize;  // Higher quality

            for wave_idx in 0..self.wave_count {
                let color_index = wave_idx % self.colors.len();
                let base_color = self.colors[color_index];

                // Apply opacity
                let color = Color32::from_rgba_unmultiplied(
                    base_color.r(),
                    base_color.g(),
                    base_color.b(),
                    (255.0 * self.wave_opacity) as u8,
                );

                // Y position: center the waves vertically
                let y_base = rect.center().y;

                // Generate wave path using 3D noise (x, wave_layer, time)
                let mut points = Vec::new();
                for seg in 0..=segments {
                    let x = rect.left() + (seg as f32 / segments as f32) * rect.width();

                    // Aceternity formula: noise(x / 800, 0.3 * wave_idx, time) * 100
                    // Adjust divisor based on wave_width (default 50 -> ~800 divisor)
                    let x_input = x / (self.wave_width * 16.0);  // 50 * 16 = 800
                    let y_input = 0.3 * wave_idx as f32;
                    let noise_value = self.noise.noise3d(x_input, y_input, self.time);

                    let wave_y = y_base + noise_value * 100.0;
                    points.push(Pos2::new(x, wave_y));
                }

                // Draw with blur effect - simulate Gaussian blur by drawing multiple offset lines
                // with decreasing opacity (since egui doesn't support native blur filters)
                if self.blur > 0.0 {
                    let blur_passes = (self.blur / 1.5).ceil() as usize + 3;  // More passes for smoother blur

                    for blur_idx in 0..blur_passes {
                        // Spread in both directions from center
                        let offset = (blur_idx as f32 - blur_passes as f32 / 2.0) * 1.5;

                        // Gaussian-like falloff for opacity
                        let distance_from_center = (offset.abs() / (blur_passes as f32 / 2.0)).min(1.0);
                        let blur_alpha = (color.a() as f32 * (1.0 - distance_from_center.powi(2)) * 0.4) as u8;

                        if blur_alpha > 5 {  // Skip very transparent passes
                            let blur_color = Color32::from_rgba_unmultiplied(
                                color.r(),
                                color.g(),
                                color.b(),
                                blur_alpha,
                            );

                            let offset_points: Vec<Pos2> = points
                                .iter()
                                .map(|p| Pos2::new(p.x, p.y + offset))
                                .collect();

                            painter.add(egui::Shape::line(
                                offset_points,
                                egui::Stroke::new(4.0, blur_color),
                            ));
                        }
                    }

                    // Draw the main line on top with full opacity for definition
                    painter.add(egui::Shape::line(
                        points,
                        egui::Stroke::new(2.5, color),
                    ));
                } else {
                    // Simple draw without blur
                    painter.add(egui::Shape::line(
                        points,
                        egui::Stroke::new(2.5, color),
                    ));
                }
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wavy_background_creation() {
        let waves = WavyBackground::new(800.0, 600.0);
        assert_eq!(waves.width, 800.0);
        assert_eq!(waves.height, 600.0);
        assert_eq!(waves.wave_count, 5);
    }

    #[test]
    fn test_wavy_background_config() {
        let waves = WavyBackground::new(800.0, 600.0)
            .wave_count(3)
            .wave_width(60.0)
            .speed(1.0);

        assert_eq!(waves.wave_count, 3);
        assert_eq!(waves.wave_width, 60.0);
        assert_eq!(waves.speed, 1.0);
    }

    #[test]
    fn test_custom_colors() {
        let colors = vec![Color32::RED, Color32::GREEN];
        let waves = WavyBackground::new(800.0, 600.0).colors(colors.clone());
        assert_eq!(waves.colors.len(), 2);
    }
}
