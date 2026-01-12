//! Text Reveal Card
//!
//! Card component that reveals text on mouse hover with clip-path effect

use crate::animation::{Animation, EasingFunction};
use crate::ext::ArmasContextExt;
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

    // Animation state
    reveal_animation: Animation<f32>,
    mouse_x: f32,
    is_hovered: bool,
}

impl TextRevealCard {
    /// Create a new text reveal card
    pub fn new(width: f32, height: f32, static_text: String, reveal_text: String) -> Self {
        Self {
            width,
            height,
            static_text,
            reveal_text,
            background_color: Color32::PLACEHOLDER, // Use theme.surface()
            border_color: Color32::PLACEHOLDER, // Use theme.outline_variant()
            text_color: Color32::PLACEHOLDER, // Use theme.on_surface()
            reveal_color: Color32::PLACEHOLDER, // Use theme.primary()
            reveal_animation: Animation::new(0.0, 0.0, 0.4).easing(EasingFunction::EaseOut),
            mouse_x: 0.0,
            is_hovered: false,
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
        let theme = ui.ctx().armas_theme();

        // Use theme colors if not explicitly set
        let background_color = if self.background_color == Color32::PLACEHOLDER {
            theme.surface()
        } else {
            self.background_color
        };
        let border_color = if self.border_color == Color32::PLACEHOLDER {
            let outline = theme.outline_variant();
            Color32::from_rgba_unmultiplied(outline.r(), outline.g(), outline.b(), 20)
        } else {
            self.border_color
        };
        let text_color = if self.text_color == Color32::PLACEHOLDER {
            theme.on_surface_variant()
        } else {
            self.text_color
        };
        let reveal_color = if self.reveal_color == Color32::PLACEHOLDER {
            theme.primary()
        } else {
            self.reveal_color
        };

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), Sense::hover());

        let dt = ui.input(|i| i.stable_dt);

        // Track mouse position
        if response.hovered() {
            if let Some(hover_pos) = response.hover_pos() {
                self.is_hovered = true;
                self.mouse_x = hover_pos.x - rect.left();

                // Calculate width percentage
                let width_percentage = (self.mouse_x / rect.width()).clamp(0.0, 1.0);

                // Update animation target instantly on hover
                self.reveal_animation.end = width_percentage;
                self.reveal_animation.start = width_percentage;
                self.reveal_animation.elapsed = self.reveal_animation.duration; // instant
            }
        } else if self.is_hovered {
            // Mouse left - animate back to 0
            self.is_hovered = false;
            self.reveal_animation.start = self.reveal_animation.value();
            self.reveal_animation.end = 0.0;
            self.reveal_animation.elapsed = 0.0;
            self.reveal_animation.start();
        }

        // Update animation
        self.reveal_animation.update(dt);
        let reveal_percentage = self.reveal_animation.value();

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw card background
            painter.rect(
                rect,
                theme.spacing.corner_radius,
                background_color,
                egui::Stroke::new(1.0, border_color),
                egui::StrokeKind::Outside,
            );

            // Content area with padding
            let content_rect = rect.shrink(theme.spacing.md);

            // Draw static text
            painter.text(
                content_rect.left_top() + Vec2::new(0.0, theme.spacing.sm),
                egui::Align2::LEFT_TOP,
                &self.static_text,
                egui::FontId::proportional(24.0),
                text_color,
            );

            // Draw revealed text with clip-path effect (bottom of card)
            let reveal_y = content_rect.bottom() - theme.spacing.lg * 2.0;
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
                    Pos2::new(reveal_rect.left(), reveal_y + theme.spacing.sm),
                    egui::Align2::LEFT_TOP,
                    &self.reveal_text,
                    egui::FontId::proportional(28.0),
                    reveal_color,
                );
            }

            // Draw subtle instruction text
            if !self.is_hovered {
                let hint_color = theme.on_surface_variant();
                painter.text(
                    Pos2::new(reveal_rect.left(), reveal_y + theme.spacing.sm),
                    egui::Align2::LEFT_TOP,
                    "Move mouse to reveal →",
                    egui::FontId::proportional(18.0),
                    Color32::from_rgba_unmultiplied(hint_color.r(), hint_color.g(), hint_color.b(), 100),
                );
            }
        }

        if !self.reveal_animation.is_complete() {
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
