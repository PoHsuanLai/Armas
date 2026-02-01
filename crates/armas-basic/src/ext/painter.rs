//! Painter extensions for advanced rendering effects
//!
//! Provides additional rendering capabilities beyond egui's built-in painter,
//! including blur approximation, glow effects, and shadows.

use egui::{Color32, CornerRadius, Painter, Pos2, Rect, Shape, Stroke, Vec2};

/// Extension trait for egui's Painter with advanced effects
pub trait PainterExt {
    /// Draw a blurred rectangle using layered alpha
    ///
    /// Approximates blur by drawing multiple expanding rectangles with decreasing opacity
    fn blur_rect(&self, rect: Rect, blur_radius: f32, color: Color32);

    /// Draw a glow effect around a rectangle
    ///
    /// Creates a soft glow by drawing multiple expanding outlines
    fn glow_rect(&self, rect: Rect, rounding: CornerRadius, color: Color32, intensity: f32);

    /// Draw a shadow with blur
    ///
    /// Renders a blurred shadow offset from the original rectangle
    fn shadow(
        &self,
        rect: Rect,
        rounding: CornerRadius,
        offset: Vec2,
        blur_radius: f32,
        color: Color32,
    );

    /// Draw a dashed line
    ///
    /// Creates a line with alternating dashes and gaps
    fn dashed_line(&self, points: &[Pos2], stroke: Stroke, dash_length: f32, gap_length: f32);

    /// Draw a dotted line
    ///
    /// Creates a line with dots at regular intervals
    fn dotted_line(&self, points: &[Pos2], color: Color32, dot_radius: f32, spacing: f32);

    /// Draw a gradient-filled rectangle
    ///
    /// Uses mesh rendering for smooth gradients
    fn gradient_rect_horizontal(
        &self,
        rect: Rect,
        rounding: CornerRadius,
        from: Color32,
        to: Color32,
    );

    /// Draw a radial glow at a point
    ///
    /// Useful for creating spotlight or highlight effects
    fn radial_glow(&self, center: Pos2, radius: f32, color: Color32, falloff: f32);
}

impl PainterExt for Painter {
    fn blur_rect(&self, rect: Rect, blur_radius: f32, color: Color32) {
        let layers = 5;
        let base_alpha = f32::from(color.a()) / layers as f32;

        for i in 0..layers {
            let expand = (i as f32 / layers as f32) * blur_radius;
            let alpha = (base_alpha * (1.0 - i as f32 / layers as f32)) as u8;
            let layer_color =
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

            let expanded_rect = rect.expand(expand);
            self.rect_filled(expanded_rect, 0.0, layer_color);
        }
    }

    fn glow_rect(&self, rect: Rect, rounding: CornerRadius, color: Color32, intensity: f32) {
        let layers = 8;
        let max_expansion = 12.0 * intensity;

        for i in 0..layers {
            let t = i as f32 / layers as f32;
            let expansion = max_expansion * t;
            let alpha = ((1.0 - t) * intensity * 255.0) as u8;

            let glow_color = Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                alpha.min(color.a()),
            );

            let expanded_rect = rect.expand(expansion);
            self.rect_stroke(
                expanded_rect,
                rounding,
                Stroke::new(1.5, glow_color),
                egui::epaint::StrokeKind::Middle,
            );
        }
    }

    fn shadow(
        &self,
        rect: Rect,
        rounding: CornerRadius,
        offset: Vec2,
        blur_radius: f32,
        color: Color32,
    ) {
        let shadow_rect = rect.translate(offset);
        let layers = 8;

        for i in 0..layers {
            let t = i as f32 / layers as f32;
            let expansion = blur_radius * t;
            let alpha = ((1.0 - t) * f32::from(color.a())) as u8;

            let shadow_color =
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

            let expanded_rect = shadow_rect.expand(expansion);
            self.rect_filled(
                expanded_rect,
                rounding.at_most(expansion as u8),
                shadow_color,
            );
        }
    }

    fn dashed_line(&self, points: &[Pos2], stroke: Stroke, dash_length: f32, gap_length: f32) {
        if points.len() < 2 {
            return;
        }

        for i in 0..points.len() - 1 {
            let start = points[i];
            let end = points[i + 1];
            let segment = end - start;
            let length = segment.length();

            if length < 0.001 {
                continue;
            }

            let direction = segment / length;
            let pattern_length = dash_length + gap_length;
            let num_dashes = (length / pattern_length).ceil() as usize;

            for i in 0..num_dashes {
                let current_pos = i as f32 * pattern_length;
                if current_pos >= length {
                    break;
                }
                let dash_start = start + direction * current_pos;
                let dash_end_pos = (current_pos + dash_length).min(length);
                let dash_end = start + direction * dash_end_pos;

                self.line_segment([dash_start, dash_end], stroke);
            }
        }
    }

    fn dotted_line(&self, points: &[Pos2], color: Color32, dot_radius: f32, spacing: f32) {
        if points.len() < 2 {
            return;
        }

        for i in 0..points.len() - 1 {
            let start = points[i];
            let end = points[i + 1];
            let segment = end - start;
            let length = segment.length();

            if length < 0.001 {
                continue;
            }

            let num_dots = (length / spacing).ceil() as usize;

            for j in 0..=num_dots {
                let t = (j as f32 * spacing / length).min(1.0);
                let pos = start + segment * t;
                self.circle_filled(pos, dot_radius, color);
            }
        }
    }

    fn gradient_rect_horizontal(
        &self,
        rect: Rect,
        rounding: CornerRadius,
        from: Color32,
        to: Color32,
    ) {
        let mut mesh = egui::Mesh::default();

        // Simple 4-vertex gradient
        let tl = rect.left_top();
        let tr = rect.right_top();
        let bl = rect.left_bottom();
        let br = rect.right_bottom();

        // For rounding, we need to subdivide, but for now keep it simple
        if rounding == CornerRadius::ZERO {
            mesh.colored_vertex(tl, from);
            mesh.colored_vertex(tr, to);
            mesh.colored_vertex(bl, from);
            mesh.colored_vertex(br, to);

            mesh.add_triangle(0, 1, 2);
            mesh.add_triangle(1, 3, 2);
        } else {
            // Rounded corners - need to subdivide
            let steps = 10;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let x = rect.left() + t * rect.width();
                let color = lerp_color(from, to, t);

                mesh.colored_vertex(Pos2::new(x, rect.top()), color);
                mesh.colored_vertex(Pos2::new(x, rect.bottom()), color);
            }

            for i in 0..steps {
                let base = i * 2;
                mesh.add_triangle(base as u32, (base + 1) as u32, (base + 2) as u32);
                mesh.add_triangle((base + 1) as u32, (base + 3) as u32, (base + 2) as u32);
            }
        }

        self.add(Shape::Mesh(std::sync::Arc::new(mesh)));
    }

    fn radial_glow(&self, center: Pos2, radius: f32, color: Color32, falloff: f32) {
        let layers = 12;

        for i in 0..layers {
            let t = i as f32 / layers as f32;
            let layer_radius = radius * (1.0 - t.powf(falloff));
            let alpha = ((1.0 - t) * f32::from(color.a())) as u8;

            let glow_color =
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

            self.circle_filled(center, layer_radius, glow_color);
        }
    }
}

/// Helper function to interpolate between colors
fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        (f32::from(a.r()) + (f32::from(b.r()) - f32::from(a.r())) * t) as u8,
        (f32::from(a.g()) + (f32::from(b.g()) - f32::from(a.g())) * t) as u8,
        (f32::from(a.b()) + (f32::from(b.b()) - f32::from(a.b())) * t) as u8,
        (f32::from(a.a()) + (f32::from(b.a()) - f32::from(a.a())) * t) as u8,
    )
}

/// Draw a neon line with glow effect
pub fn neon_line(
    painter: &Painter,
    points: &[Pos2],
    color: Color32,
    thickness: f32,
    glow_intensity: f32,
) {
    if points.len() < 2 {
        return;
    }

    // Draw glow layers
    let glow_layers = 5;
    for i in 0..glow_layers {
        let t = i as f32 / glow_layers as f32;
        let layer_thickness = thickness + (glow_intensity * 8.0 * t);
        let alpha = ((1.0 - t) * glow_intensity * 255.0) as u8;

        let glow_color =
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha.min(80));

        for j in 0..points.len() - 1 {
            painter.line_segment(
                [points[j], points[j + 1]],
                Stroke::new(layer_thickness, glow_color),
            );
        }
    }

    // Draw core line
    for j in 0..points.len() - 1 {
        painter.line_segment([points[j], points[j + 1]], Stroke::new(thickness, color));
    }
}

/// Draw a glowing circle
pub fn neon_circle(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    color: Color32,
    glow_intensity: f32,
) {
    // Draw glow
    let glow_layers = 8;
    for i in 0..glow_layers {
        let t = i as f32 / glow_layers as f32;
        let layer_radius = radius + (glow_intensity * 10.0 * t);
        let alpha = ((1.0 - t) * glow_intensity * 255.0) as u8;

        let glow_color =
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha.min(60));

        painter.circle_stroke(center, layer_radius, Stroke::new(1.5, glow_color));
    }

    // Draw core circle
    painter.circle_filled(center, radius, color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_color() {
        let black = Color32::BLACK;
        let white = Color32::WHITE;
        let gray = lerp_color(black, white, 0.5);

        assert_eq!(gray.r(), 127);
        assert_eq!(gray.g(), 127);
        assert_eq!(gray.b(), 127);
    }
}
