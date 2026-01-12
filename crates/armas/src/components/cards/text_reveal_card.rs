//! Text Reveal Card
//!
//! Card component that reveals text on mouse hover with clip-path effect

use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Text reveal card component
///
/// Displays a card where moving the mouse reveals hidden text through a clip-path effect
pub struct TextRevealCard {
    width: f32,
    height: f32,
    static_text: String,
    reveal_text: String,
    background_color: Color32,
    border_color: Color32,
    text_color: Color32,
    reveal_color: Color32,
    animation_duration: f32,

    // Animation state
    mouse_x: f32,
    is_hovered: bool,
    unhover_start_time: Option<f32>,
    unhover_start_value: f32,
}

impl TextRevealCard {
    /// Create a new text reveal card
    pub fn new(width: f32, height: f32, static_text: String, reveal_text: String) -> Self {
        Self {
            width,
            height,
            static_text,
            reveal_text,
            background_color: Color32::from_rgb(29, 28, 32), // #1d1c20
            border_color: Color32::from_rgba_unmultiplied(255, 255, 255, 20),
            text_color: Color32::from_gray(200),
            reveal_color: Color32::from_rgb(100, 200, 255), // Cyan gradient
            animation_duration: 0.4,
            mouse_x: 0.0,
            is_hovered: false,
            unhover_start_time: None,
            unhover_start_value: 0.0,
        }
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = color;
        self
    }

    /// Set the border color
    pub fn border_color(mut self, color: Color32) -> Self {
        self.border_color = color;
        self
    }

    /// Set the text color
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = color;
        self
    }

    /// Set the reveal text color
    pub fn reveal_color(mut self, color: Color32) -> Self {
        self.reveal_color = color;
        self
    }

    /// Show the text reveal card
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), Sense::hover());

        let time = ui.input(|i| i.time) as f32;

        // Calculate reveal percentage
        let reveal_percentage = if response.hovered() {
            if let Some(hover_pos) = response.hover_pos() {
                if !self.is_hovered {
                    // Just started hovering
                    self.is_hovered = true;
                    self.unhover_start_time = None;
                }

                self.mouse_x = hover_pos.x - rect.left();
                // Calculate width percentage - instantly follow mouse
                (self.mouse_x / rect.width()).clamp(0.0, 1.0)
            } else {
                0.0
            }
        } else {
            if self.is_hovered {
                // Mouse just left - start unhover animation
                self.is_hovered = false;
                self.unhover_start_time = Some(time);
                self.unhover_start_value = (self.mouse_x / rect.width()).clamp(0.0, 1.0);
            }

            // Animate back to 0
            if let Some(start_time) = self.unhover_start_time {
                let elapsed = time - start_time;
                let t = (elapsed / self.animation_duration).min(1.0);

                // Apply ease-out easing
                let eased_t = 1.0 - (1.0 - t).powi(3);

                let current_value = self.unhover_start_value * (1.0 - eased_t);

                if t >= 1.0 {
                    self.unhover_start_time = None;
                    0.0
                } else {
                    current_value
                }
            } else {
                0.0
            }
        };

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw card background
            painter.rect(
                rect,
                8.0, // rounded corners
                self.background_color,
                egui::Stroke::new(1.0, self.border_color),
                egui::StrokeKind::Outside,
            );

            // Content area with padding
            let content_rect = rect.shrink(20.0);

            // Draw static text
            painter.text(
                content_rect.left_top() + Vec2::new(0.0, 10.0),
                egui::Align2::LEFT_TOP,
                &self.static_text,
                egui::FontId::proportional(24.0),
                self.text_color,
            );

            // Draw revealed text with clip-path effect (bottom of card)
            let reveal_y = content_rect.bottom() - 40.0;
            let reveal_rect = Rect::from_min_max(
                Pos2::new(content_rect.left(), reveal_y),
                Pos2::new(content_rect.right(), content_rect.bottom()),
            );

            if reveal_percentage > 0.0 {
                // Calculate clip width
                let clip_width = reveal_rect.width() * reveal_percentage;
                let clip_rect = Rect::from_min_max(
                    reveal_rect.min,
                    Pos2::new(reveal_rect.left() + clip_width, reveal_rect.max.y),
                );

                // Draw text with manual clipping by intersecting with clip rect
                let current_clip = ui.clip_rect();
                let final_clip = current_clip.intersect(clip_rect);

                // Create a painter with the clipped rect
                let clipped_painter = ui.painter().with_clip_rect(final_clip);

                // Add text shadow effect by drawing slightly offset darker text first
                let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, 128);
                clipped_painter.text(
                    Pos2::new(reveal_rect.left() + 2.0, reveal_y + 12.0),
                    egui::Align2::LEFT_TOP,
                    &self.reveal_text,
                    egui::FontId::proportional(28.0),
                    shadow_color,
                );

                // Draw text with gradient effect (simulate gradient by using lighter color)
                clipped_painter.text(
                    Pos2::new(reveal_rect.left(), reveal_y + 10.0),
                    egui::Align2::LEFT_TOP,
                    &self.reveal_text,
                    egui::FontId::proportional(28.0),
                    self.reveal_color,
                );
            }

            // Draw subtle instruction text
            if !self.is_hovered {
                painter.text(
                    Pos2::new(reveal_rect.left(), reveal_y + 10.0),
                    egui::Align2::LEFT_TOP,
                    "Move mouse to reveal →",
                    egui::FontId::proportional(18.0),
                    Color32::from_gray(100),
                );
            }
        }

        if self.unhover_start_time.is_some() {
            ui.ctx().request_repaint();
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_reveal_card_creation() {
        let card = TextRevealCard::new(
            400.0,
            200.0,
            "Static Text".to_string(),
            "Revealed!".to_string(),
        );
        assert_eq!(card.width, 400.0);
        assert_eq!(card.height, 200.0);
        assert_eq!(card.static_text, "Static Text");
        assert_eq!(card.reveal_text, "Revealed!");
    }

    #[test]
    fn test_text_reveal_card_config() {
        let card = TextRevealCard::new(400.0, 200.0, "Test".to_string(), "Hidden".to_string())
            .reveal_color(Color32::RED)
            .background_color(Color32::BLACK);

        assert_eq!(card.reveal_color, Color32::RED);
        assert_eq!(card.background_color, Color32::BLACK);
    }
}
