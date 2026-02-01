//! Color utilities for advanced effects
//!
//! Provides gradient generation, color manipulation, and neon palette presets
//! for creating aceternity-style visual effects.

use egui::{Color32, Mesh, Pos2, Rect, Vec2};
use std::f32::consts::PI;

/// Color stop for gradients (position 0.0-1.0, color)
/// A color stop in a gradient, defining a color at a specific position
#[derive(Clone, Debug)]
pub struct ColorStop {
    /// Position in the gradient (0.0 to 1.0)
    pub position: f32,
    /// Color at this position
    pub color: Color32,
}

impl ColorStop {
    /// Create a new color stop
    #[must_use]
    pub const fn new(position: f32, color: Color32) -> Self {
        Self { position, color }
    }
}

/// Gradient builder for creating various gradient types
pub struct Gradient {
    stops: Vec<ColorStop>,
}

impl Gradient {
    /// Create a new gradient with stops
    #[must_use]
    pub const fn new(stops: Vec<ColorStop>) -> Self {
        Self { stops }
    }

    /// Create a simple two-color gradient
    #[must_use]
    pub fn linear(from: Color32, to: Color32) -> Self {
        Self {
            stops: vec![ColorStop::new(0.0, from), ColorStop::new(1.0, to)],
        }
    }

    /// Sample color at position t (0.0-1.0)
    #[must_use]
    pub fn sample(&self, t: f32) -> Color32 {
        let t = t.clamp(0.0, 1.0);

        if self.stops.is_empty() {
            return Color32::BLACK;
        }

        if self.stops.len() == 1 {
            return self.stops[0].color;
        }

        // Find the two stops to interpolate between
        let mut before = &self.stops[0];
        let mut after = &self.stops[self.stops.len() - 1];

        for i in 0..self.stops.len() - 1 {
            if self.stops[i].position <= t && self.stops[i + 1].position >= t {
                before = &self.stops[i];
                after = &self.stops[i + 1];
                break;
            }
        }

        // Interpolate
        let range = after.position - before.position;
        if range < 0.0001 {
            return before.color;
        }

        let local_t = (t - before.position) / range;
        lerp_color(before.color, after.color, local_t)
    }

    /// Generate a radial gradient mesh
    ///
    /// Creates a circular gradient emanating from a center point
    #[must_use]
    pub fn radial_mesh(&self, center: Pos2, radius: f32, segments: usize) -> Mesh {
        let mut mesh = Mesh::default();

        // Center vertex
        let center_color = self.sample(0.0);
        mesh.colored_vertex(center, center_color);

        // Create rings
        let num_rings = 10;
        for ring in 1..=num_rings {
            let t = ring as f32 / num_rings as f32;
            let ring_radius = radius * t;
            let ring_color = self.sample(t);

            for segment in 0..segments {
                let angle = (segment as f32 / segments as f32) * 2.0 * PI;
                let pos = center + Vec2::new(angle.cos(), angle.sin()) * ring_radius;
                mesh.colored_vertex(pos, ring_color);
            }
        }

        // Generate triangles
        // Center to first ring
        for segment in 0..segments {
            let next = (segment + 1) % segments;
            mesh.add_triangle(0, 1 + segment as u32, 1 + next as u32);
        }

        // Ring to ring
        for ring in 0..num_rings - 1 {
            let base = 1 + ring * segments;
            let next_base = 1 + (ring + 1) * segments;

            for segment in 0..segments {
                let next = (segment + 1) % segments;

                let a = base + segment;
                let b = base + next;
                let c = next_base + segment;
                let d = next_base + next;

                mesh.add_triangle(a as u32, c as u32, b as u32);
                mesh.add_triangle(b as u32, c as u32, d as u32);
            }
        }

        mesh
    }

    /// Generate a conic (angular) gradient mesh
    ///
    /// Creates a gradient that rotates around a center point
    #[must_use]
    pub fn conic_mesh(
        &self,
        center: Pos2,
        radius: f32,
        angle_offset: f32,
        segments: usize,
    ) -> Mesh {
        let mut mesh = Mesh::default();

        // Center vertex (average color)
        let center_color = self.sample(0.5);
        mesh.colored_vertex(center, center_color);

        // Create outer ring with varying colors based on angle
        for segment in 0..segments {
            let angle = angle_offset + (segment as f32 / segments as f32) * 2.0 * PI;
            let t = (angle.rem_euclid(2.0 * PI)) / (2.0 * PI);
            let color = self.sample(t);

            let pos = center + Vec2::new(angle.cos(), angle.sin()) * radius;
            mesh.colored_vertex(pos, color);
        }

        // Generate triangles from center to perimeter
        for segment in 0..segments {
            let next = (segment + 1) % segments;
            mesh.add_triangle(0, 1 + segment as u32, 1 + next as u32);
        }

        mesh
    }

    /// Generate a rectangular gradient mesh (corner-to-corner)
    #[must_use]
    pub fn rect_mesh(&self, rect: Rect, horizontal: bool) -> Mesh {
        let mut mesh = Mesh::default();

        let steps = 20;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let color = self.sample(t);

            if horizontal {
                let x = rect.left() + t * rect.width();
                let top = Pos2::new(x, rect.top());
                let bottom = Pos2::new(x, rect.bottom());

                mesh.colored_vertex(top, color);
                mesh.colored_vertex(bottom, color);
            } else {
                let y = rect.top() + t * rect.height();
                let left = Pos2::new(rect.left(), y);
                let right = Pos2::new(rect.right(), y);

                mesh.colored_vertex(left, color);
                mesh.colored_vertex(right, color);
            }
        }

        // Generate triangle strips
        for i in 0..steps {
            let base = i * 2;
            mesh.add_triangle(base, base + 1, base + 2);
            mesh.add_triangle(base + 1, base + 3, base + 2);
        }

        mesh
    }
}

/// Interpolate between two colors
#[must_use]
pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        (f32::from(a.r()) + (f32::from(b.r()) - f32::from(a.r())) * t) as u8,
        (f32::from(a.g()) + (f32::from(b.g()) - f32::from(a.g())) * t) as u8,
        (f32::from(a.b()) + (f32::from(b.b()) - f32::from(a.b())) * t) as u8,
        (f32::from(a.a()) + (f32::from(b.a()) - f32::from(a.a())) * t) as u8,
    )
}

/// Add alpha to a color
#[must_use]
pub fn with_alpha(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

/// Blend two colors using different modes
#[must_use]
#[allow(clippy::many_single_char_names)]
pub fn blend(a: Color32, b: Color32, t: f32, mode: BlendMode) -> Color32 {
    match mode {
        BlendMode::Normal => lerp_color(a, b, t),
        BlendMode::Multiply => {
            let r = ((f32::from(a.r()) / 255.0) * (f32::from(b.r()) / 255.0) * 255.0) as u8;
            let g = ((f32::from(a.g()) / 255.0) * (f32::from(b.g()) / 255.0) * 255.0) as u8;
            let b_val = ((f32::from(a.b()) / 255.0) * (f32::from(b.b()) / 255.0) * 255.0) as u8;
            Color32::from_rgb(r, g, b_val)
        }
        BlendMode::Screen => {
            let r = (255.0 - (255.0 - f32::from(a.r())) * (255.0 - f32::from(b.r())) / 255.0) as u8;
            let g = (255.0 - (255.0 - f32::from(a.g())) * (255.0 - f32::from(b.g())) / 255.0) as u8;
            let b_val =
                (255.0 - (255.0 - f32::from(a.b())) * (255.0 - f32::from(b.b())) / 255.0) as u8;
            Color32::from_rgb(r, g, b_val)
        }
        BlendMode::Overlay => {
            let overlay_channel = |base: u8, blend: u8| -> u8 {
                let base_f = f32::from(base) / 255.0;
                let blend_f = f32::from(blend) / 255.0;
                let result = if base_f < 0.5 {
                    2.0 * base_f * blend_f
                } else {
                    1.0 - 2.0 * (1.0 - base_f) * (1.0 - blend_f)
                };
                (result * 255.0) as u8
            };
            Color32::from_rgb(
                overlay_channel(a.r(), b.r()),
                overlay_channel(a.g(), b.g()),
                overlay_channel(a.b(), b.b()),
            )
        }
    }
}

/// Blend modes for color composition
#[derive(Debug, Clone, Copy)]
pub enum BlendMode {
    /// Normal blend mode (no blending)
    Normal,
    /// Multiply blend mode (darkens)
    Multiply,
    /// Screen blend mode (lightens)
    Screen,
    /// Overlay blend mode (combines multiply and screen)
    Overlay,
}

/// Saturate/desaturate a color
#[must_use]
pub fn saturate(color: Color32, amount: f32) -> Color32 {
    let r = f32::from(color.r()) / 255.0;
    let g = f32::from(color.g()) / 255.0;
    let b = f32::from(color.b()) / 255.0;

    let gray = 0.299 * r + 0.587 * g + 0.114 * b;

    let r = (gray + (r - gray) * amount).clamp(0.0, 1.0);
    let g = (gray + (g - gray) * amount).clamp(0.0, 1.0);
    let b = (gray + (b - gray) * amount).clamp(0.0, 1.0);

    Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

/// Neon color palette presets for aceternity-style effects
pub struct NeonPalette;

impl NeonPalette {
    /// Cyberpunk neon palette (blues, purples, pinks)
    #[must_use]
    pub fn cyberpunk() -> Vec<Color32> {
        vec![
            Color32::from_rgb(0, 255, 255),  // Cyan
            Color32::from_rgb(255, 0, 255),  // Magenta
            Color32::from_rgb(138, 43, 226), // Blue Violet
            Color32::from_rgb(255, 20, 147), // Deep Pink
            Color32::from_rgb(0, 191, 255),  // Deep Sky Blue
        ]
    }

    /// Synthwave palette (purples, pinks, oranges)
    #[must_use]
    pub fn synthwave() -> Vec<Color32> {
        vec![
            Color32::from_rgb(251, 86, 7),   // Orange
            Color32::from_rgb(255, 0, 110),  // Pink
            Color32::from_rgb(131, 58, 180), // Purple
            Color32::from_rgb(253, 29, 29),  // Red
            Color32::from_rgb(252, 176, 69), // Yellow
        ]
    }

    /// Aurora palette (blues, greens, purples)
    #[must_use]
    pub fn aurora() -> Vec<Color32> {
        vec![
            Color32::from_rgb(0, 255, 127),   // Spring Green
            Color32::from_rgb(0, 191, 255),   // Deep Sky Blue
            Color32::from_rgb(138, 43, 226),  // Blue Violet
            Color32::from_rgb(64, 224, 208),  // Turquoise
            Color32::from_rgb(123, 104, 238), // Medium Slate Blue
        ]
    }

    /// Neon rainbow (full spectrum, saturated)
    #[must_use]
    pub fn rainbow() -> Vec<Color32> {
        vec![
            Color32::from_rgb(255, 0, 0),   // Red
            Color32::from_rgb(255, 127, 0), // Orange
            Color32::from_rgb(255, 255, 0), // Yellow
            Color32::from_rgb(0, 255, 0),   // Green
            Color32::from_rgb(0, 0, 255),   // Blue
            Color32::from_rgb(75, 0, 130),  // Indigo
            Color32::from_rgb(148, 0, 211), // Violet
        ]
    }

    /// Electric blue palette
    #[must_use]
    pub fn electric() -> Vec<Color32> {
        vec![
            Color32::from_rgb(59, 130, 246),  // Blue 500
            Color32::from_rgb(96, 165, 250),  // Blue 400
            Color32::from_rgb(147, 197, 253), // Blue 300
            Color32::from_rgb(191, 219, 254), // Blue 200
        ]
    }

    /// Hot gradient (red to yellow)
    #[must_use]
    pub fn hot() -> Vec<Color32> {
        vec![
            Color32::from_rgb(139, 0, 0),   // Dark Red
            Color32::from_rgb(220, 20, 60), // Crimson
            Color32::from_rgb(255, 69, 0),  // Red Orange
            Color32::from_rgb(255, 140, 0), // Dark Orange
            Color32::from_rgb(255, 215, 0), // Gold
        ]
    }

    /// Cool gradient (cyan to blue to purple)
    #[must_use]
    pub fn cool() -> Vec<Color32> {
        vec![
            Color32::from_rgb(0, 255, 255),  // Cyan
            Color32::from_rgb(0, 191, 255),  // Deep Sky Blue
            Color32::from_rgb(65, 105, 225), // Royal Blue
            Color32::from_rgb(138, 43, 226), // Blue Violet
        ]
    }

    /// Premium gold gradient
    #[must_use]
    pub fn gold() -> Vec<Color32> {
        vec![
            Color32::from_rgb(255, 215, 0),  // Gold
            Color32::from_rgb(255, 223, 0),  // Golden Yellow
            Color32::from_rgb(255, 193, 37), // Amber
            Color32::from_rgb(218, 165, 32), // Goldenrod
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_lerp() {
        let white = Color32::WHITE;
        let black = Color32::BLACK;

        let mid = lerp_color(black, white, 0.5);
        assert_eq!(mid.r(), 127);
        assert_eq!(mid.g(), 127);
        assert_eq!(mid.b(), 127);
    }

    #[test]
    fn test_gradient_sample() {
        let gradient = Gradient::linear(Color32::BLACK, Color32::WHITE);
        let mid = gradient.sample(0.5);
        assert_eq!(mid.r(), 127);
    }

    #[test]
    fn test_with_alpha() {
        let color = Color32::from_rgb(255, 0, 0);
        let transparent = with_alpha(color, 128);
        assert_eq!(transparent.a(), 128);
    }

    #[test]
    fn test_saturate() {
        let gray = Color32::from_rgb(128, 128, 128);
        let saturated = saturate(gray, 2.0);
        // Should remain gray since there's no color to saturate
        assert_eq!(saturated.r(), saturated.g());
    }
}
