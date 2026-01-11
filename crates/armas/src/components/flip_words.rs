//! Flip Words Animation
//!
//! Animated text that cycles through a list of words with flip transitions

use crate::animation::{Animation, EasingFunction};
use egui::{Align2, Color32, FontId, Pos2, Response, Ui, Vec2};

/// Flip transition style
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlipStyle {
    /// Vertical flip (rotate on X axis)
    Vertical,
    /// Horizontal flip (rotate on Y axis)
    Horizontal,
    /// Fade transition
    Fade,
}

/// Animated word-flipping text component
///
/// Cycles through a list of words with smooth flip animations
pub struct FlipWords {
    /// List of words to cycle through
    words: Vec<String>,
    /// Current word index
    current_index: usize,
    /// Time per word (in seconds)
    duration: f32,
    /// Flip transition animation
    flip_animation: Animation<f32>,
    /// Flip style
    style: FlipStyle,
    /// Font size
    font_size: f32,
    /// Text color
    color: Color32,
    /// Highlight color (optional)
    highlight_color: Option<Color32>,
    /// Internal timer
    timer: f32,
}

impl FlipWords {
    /// Create a new flip words component
    pub fn new(words: Vec<impl Into<String>>) -> Self {
        Self {
            words: words.into_iter().map(|w| w.into()).collect(),
            current_index: 0,
            duration: 2.5,
            flip_animation: Animation::new(0.0, 1.0, 0.5).with_easing(EasingFunction::CubicInOut),
            style: FlipStyle::Vertical,
            font_size: 24.0,
            color: Color32::WHITE,
            highlight_color: None,
            timer: 0.0,
        }
    }

    /// Set the duration each word is displayed (in seconds)
    pub fn duration(mut self, seconds: f32) -> Self {
        self.duration = seconds.max(0.5);
        self
    }

    /// Set the flip style
    pub fn style(mut self, style: FlipStyle) -> Self {
        self.style = style;
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set text color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set highlight color for the active word
    pub fn highlight(mut self, color: Color32) -> Self {
        self.highlight_color = Some(color);
        self
    }

    /// Show the flip words animation
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        if self.words.is_empty() {
            return ui.label("(no words)");
        }

        let dt = ui.input(|i| i.stable_dt);

        // Update timer and animation
        self.timer += dt;

        // Check if we should transition to next word
        let transition_start = self.duration - 0.5; // Start transition 0.5s before word change

        if self.timer >= self.duration {
            // Move to next word
            self.current_index = (self.current_index + 1) % self.words.len();
            self.timer = 0.0;
            self.flip_animation.reset();
        } else if self.timer >= transition_start {
            // In transition - start/update animation
            if !self.flip_animation.is_running() && !self.flip_animation.is_complete() {
                self.flip_animation.start();
            }
            self.flip_animation.update(dt);
        } else {
            // Word is stable
            self.flip_animation.reset();
        }

        // Request repaint for animation
        ui.ctx().request_repaint();

        let current_word = &self.words[self.current_index];
        let next_index = (self.current_index + 1) % self.words.len();
        let next_word = &self.words[next_index];

        // Calculate text size for both words to get max width
        let font_id = FontId::proportional(self.font_size);

        // Estimate size based on font
        let current_galley =
            ui.painter()
                .layout_no_wrap(current_word.clone(), font_id.clone(), self.color);
        let next_galley =
            ui.painter()
                .layout_no_wrap(next_word.clone(), font_id.clone(), self.color);
        let current_size = current_galley.size();
        let next_size = next_galley.size();

        let max_width = current_size.x.max(next_size.x);
        let height = current_size.y;

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(max_width, height * 1.5), // Extra height for flip animation
            egui::Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();

            match self.style {
                FlipStyle::Vertical => {
                    // Vertical flip effect (top half disappears, bottom half appears)
                    let t = self.flip_animation.value();
                    if t > 0.0 {
                        // First half: current word flips out (0.0 to 0.5)
                        if t < 0.5 {
                            let scale_y = 1.0 - (t * 2.0);
                            let alpha = (255.0 * (1.0 - t * 2.0)) as u8;
                            let text_color = self.highlight_color.unwrap_or(self.color);
                            let faded_color = Color32::from_rgba_unmultiplied(
                                text_color.r(),
                                text_color.g(),
                                text_color.b(),
                                alpha,
                            );

                            let offset_y = height * 0.3 * (1.0 - scale_y);
                            painter.text(
                                Pos2::new(center.x, center.y + offset_y),
                                Align2::CENTER_CENTER,
                                current_word,
                                font_id.clone(),
                                faded_color,
                            );
                        }
                        // Second half: next word flips in (0.5 to 1.0)
                        else {
                            let scale_y = (t - 0.5) * 2.0;
                            let alpha = (255.0 * scale_y) as u8;
                            let text_color = self.highlight_color.unwrap_or(self.color);
                            let faded_color = Color32::from_rgba_unmultiplied(
                                text_color.r(),
                                text_color.g(),
                                text_color.b(),
                                alpha,
                            );

                            let offset_y = height * 0.3 * (1.0 - scale_y);
                            painter.text(
                                Pos2::new(center.x, center.y - offset_y),
                                Align2::CENTER_CENTER,
                                next_word,
                                font_id,
                                faded_color,
                            );
                        }
                    } else {
                        // No animation, show current word
                        let text_color = self.highlight_color.unwrap_or(self.color);
                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            current_word,
                            font_id,
                            text_color,
                        );
                    }
                }

                FlipStyle::Horizontal => {
                    // Horizontal flip (scale X to simulate rotation)
                    let t = self.flip_animation.value();
                    if t > 0.0 {
                        if t < 0.5 {
                            // Current word scales down horizontally
                            let _scale_x = 1.0 - (t * 2.0);
                            let alpha = (255.0 * (1.0 - t * 2.0)) as u8;
                            let text_color = self.highlight_color.unwrap_or(self.color);
                            let faded_color = Color32::from_rgba_unmultiplied(
                                text_color.r(),
                                text_color.g(),
                                text_color.b(),
                                alpha,
                            );

                            // Simple fade for horizontal (true 3D would need mesh transforms)
                            painter.text(
                                center,
                                Align2::CENTER_CENTER,
                                current_word,
                                font_id.clone(),
                                faded_color,
                            );
                        } else {
                            // Next word scales up horizontally
                            let scale_x = (t - 0.5) * 2.0;
                            let alpha = (255.0 * scale_x) as u8;
                            let text_color = self.highlight_color.unwrap_or(self.color);
                            let faded_color = Color32::from_rgba_unmultiplied(
                                text_color.r(),
                                text_color.g(),
                                text_color.b(),
                                alpha,
                            );

                            painter.text(
                                center,
                                Align2::CENTER_CENTER,
                                next_word,
                                font_id,
                                faded_color,
                            );
                        }
                    } else {
                        let text_color = self.highlight_color.unwrap_or(self.color);
                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            current_word,
                            font_id,
                            text_color,
                        );
                    }
                }

                FlipStyle::Fade => {
                    // Simple crossfade
                    let t = self.flip_animation.value();
                    if t > 0.0 {
                        // Fade out current word
                        let current_alpha = (255.0 * (1.0 - t)) as u8;
                        let text_color = self.highlight_color.unwrap_or(self.color);
                        let current_color = Color32::from_rgba_unmultiplied(
                            text_color.r(),
                            text_color.g(),
                            text_color.b(),
                            current_alpha,
                        );

                        // Fade in next word
                        let next_alpha = (255.0 * t) as u8;
                        let next_color = Color32::from_rgba_unmultiplied(
                            text_color.r(),
                            text_color.g(),
                            text_color.b(),
                            next_alpha,
                        );

                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            current_word,
                            font_id.clone(),
                            current_color,
                        );

                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            next_word,
                            font_id,
                            next_color,
                        );
                    } else {
                        let text_color = self.highlight_color.unwrap_or(self.color);
                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            current_word,
                            font_id,
                            text_color,
                        );
                    }
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
    fn test_flip_words_creation() {
        let flip = FlipWords::new(vec!["Hello", "World", "Test"]);
        assert_eq!(flip.words.len(), 3);
        assert_eq!(flip.current_index, 0);
    }

    #[test]
    fn test_flip_words_config() {
        let flip = FlipWords::new(vec!["A", "B"])
            .duration(3.0)
            .font_size(32.0)
            .style(FlipStyle::Horizontal);

        assert_eq!(flip.duration, 3.0);
        assert_eq!(flip.font_size, 32.0);
        assert_eq!(flip.style, FlipStyle::Horizontal);
    }

    #[test]
    fn test_empty_words() {
        let flip = FlipWords::new(Vec::<String>::new());
        assert_eq!(flip.words.len(), 0);
    }
}
