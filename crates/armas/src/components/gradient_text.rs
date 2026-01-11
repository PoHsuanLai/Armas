//! Gradient Text Effect
//!
//! Text with gradient colors and optional animation

use egui::{Color32, FontId, Response, Sense, Ui, Vec2};

/// Gradient text with color animation
///
/// Renders text with a gradient that can optionally animate.
pub struct GradientText {
    text: String,
    colors: Vec<Color32>,
    animated: bool,
    animation_speed: f32,
    animation_offset: f32,
    per_character: bool,
    font_id: Option<FontId>,
}

impl GradientText {
    /// Create a new gradient text
    pub fn new(text: impl Into<String>, colors: Vec<Color32>) -> Self {
        Self {
            text: text.into(),
            colors,
            animated: false,
            animation_speed: 1.0,
            animation_offset: 0.0,
            per_character: true,
            font_id: None,
        }
    }

    /// Create from two colors
    pub fn two_color(text: impl Into<String>, from: Color32, to: Color32) -> Self {
        Self::new(text, vec![from, to])
    }

    /// Create with rainbow colors
    pub fn rainbow(text: impl Into<String>) -> Self {
        Self::new(
            text,
            vec![
                Color32::from_rgb(255, 0, 0),   // Red
                Color32::from_rgb(255, 127, 0), // Orange
                Color32::from_rgb(255, 255, 0), // Yellow
                Color32::from_rgb(0, 255, 0),   // Green
                Color32::from_rgb(0, 0, 255),   // Blue
                Color32::from_rgb(75, 0, 130),  // Indigo
                Color32::from_rgb(148, 0, 211), // Violet
            ],
        )
    }

    /// Enable gradient animation
    pub fn with_animation(mut self, enabled: bool) -> Self {
        self.animated = enabled;
        self
    }

    /// Set animation speed
    pub fn with_animation_speed(mut self, speed: f32) -> Self {
        self.animation_speed = speed;
        self
    }

    /// Set whether to apply gradient per character or smoothly
    pub fn with_per_character(mut self, enabled: bool) -> Self {
        self.per_character = enabled;
        self
    }

    /// Set custom font
    pub fn with_font(mut self, font_id: FontId) -> Self {
        self.font_id = Some(font_id);
        self
    }

    /// Show the gradient text
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        if self.colors.is_empty() {
            return ui.label(&self.text);
        }

        let dt = ui.input(|i| i.stable_dt);

        // Update animation
        if self.animated {
            self.animation_offset += dt * self.animation_speed;
            if self.animation_offset > 1.0 {
                self.animation_offset -= 1.0;
            }
        }

        // Get font
        let font_id = self
            .font_id
            .clone()
            .unwrap_or_else(|| FontId::proportional(ui.text_style_height(&egui::TextStyle::Body)));

        // Calculate text size
        let text_size = ui
            .painter()
            .layout_no_wrap(self.text.clone(), font_id.clone(), Color32::WHITE)
            .size();

        let (rect, response) = ui.allocate_exact_size(text_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            if self.per_character {
                // Draw each character with its own color
                let char_count = self.text.chars().count();

                for (i, ch) in self.text.chars().enumerate() {
                    // Calculate position for this character
                    let char_text = ch.to_string();
                    let char_galley =
                        ui.painter()
                            .layout_no_wrap(char_text, font_id.clone(), Color32::WHITE);

                    // Calculate offset from previous characters
                    let prefix: String = self.text.chars().take(i).collect();
                    let prefix_galley =
                        ui.painter()
                            .layout_no_wrap(prefix, font_id.clone(), Color32::WHITE);
                    let x_offset = prefix_galley.size().x;

                    // Calculate color for this character
                    let t = (i as f32 / char_count.max(1) as f32 + self.animation_offset) % 1.0;
                    let color = interpolate_gradient(&self.colors, t);

                    // Draw character
                    let char_pos = rect.min + Vec2::new(x_offset, 0.0);
                    ui.painter().galley(char_pos, char_galley, color);
                }
            } else {
                // Smooth gradient across the entire text
                // Simplified: just use average color for now
                // A proper implementation would use mesh with gradient shader
                let t = self.animation_offset % 1.0;
                let color = interpolate_gradient(&self.colors, t);

                let galley =
                    ui.painter()
                        .layout_no_wrap(self.text.clone(), font_id.clone(), Color32::WHITE);
                ui.painter().galley(rect.min, galley, color);
            }
        }

        // Request repaint if animated
        if self.animated {
            ui.ctx().request_repaint();
        }

        response
    }
}

/// Interpolate color from gradient at position t (0.0 to 1.0)
fn interpolate_gradient(colors: &[Color32], t: f32) -> Color32 {
    if colors.is_empty() {
        return Color32::WHITE;
    }

    if colors.len() == 1 {
        return colors[0];
    }

    let t = t.clamp(0.0, 1.0);
    let segment_count = colors.len() - 1;
    let segment_size = 1.0 / segment_count as f32;
    let segment_index = (t / segment_size).floor() as usize;
    let segment_index = segment_index.min(segment_count - 1);

    let local_t = (t - segment_index as f32 * segment_size) / segment_size;

    let from = colors[segment_index];
    let to = colors[segment_index + 1];

    Color32::from_rgba_unmultiplied(
        lerp_u8(from.r(), to.r(), local_t),
        lerp_u8(from.g(), to.g(), local_t),
        lerp_u8(from.b(), to.b(), local_t),
        lerp_u8(from.a(), to.a(), local_t),
    )
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_text_creation() {
        let gt = GradientText::two_color("Hello", Color32::RED, Color32::BLUE);
        assert_eq!(gt.text, "Hello");
        assert_eq!(gt.colors.len(), 2);
    }

    #[test]
    fn test_rainbow() {
        let gt = GradientText::rainbow("Rainbow");
        assert_eq!(gt.colors.len(), 7);
    }

    #[test]
    fn test_interpolate() {
        let colors = vec![Color32::RED, Color32::BLUE];
        let mid = interpolate_gradient(&colors, 0.5);
        // Should be somewhere between red and blue
        assert!(mid.r() < 255);
        assert!(mid.b() > 0);
    }
}
