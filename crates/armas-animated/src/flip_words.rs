//! Flip Words Animation
//!
//! Animated text that cycles through a list of words with flip transitions

use egui::{Align2, Color32, FontId, Pos2, Response, Ui, Vec2};

/// Flip transition style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Time per word (in seconds)
    duration: f32,
    /// Flip transition duration (in seconds)
    transition_duration: f32,
    /// Flip style
    style: FlipStyle,
    /// Font size
    font_size: f32,
    /// Text color
    color: Color32,
    /// Highlight color (optional)
    highlight_color: Option<Color32>,
}

impl FlipWords {
    /// Create a new flip words component
    #[must_use]
    pub fn new(words: Vec<impl Into<String>>) -> Self {
        // Note: color will be set from theme in show() if not overridden
        Self {
            words: words.into_iter().map(std::convert::Into::into).collect(),
            duration: 2.5,
            transition_duration: 0.5,
            style: FlipStyle::Vertical,
            font_size: 24.0,
            color: Color32::PLACEHOLDER, // Will be replaced with theme.foreground()
            highlight_color: None,
        }
    }

    /// Set the duration each word is displayed (in seconds)
    #[must_use]
    pub const fn duration(mut self, seconds: f32) -> Self {
        self.duration = seconds.max(0.5);
        self
    }

    /// Set the flip style
    #[must_use]
    pub const fn style(mut self, style: FlipStyle) -> Self {
        self.style = style;
        self
    }

    /// Set font size
    #[must_use]
    pub const fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set text color
    #[must_use]
    pub const fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set highlight color for the active word
    #[must_use]
    pub const fn highlight(mut self, color: Color32) -> Self {
        self.highlight_color = Some(color);
        self
    }

    /// Show the flip words animation
    pub fn show(&mut self, ui: &mut Ui, theme: &armas::Theme) -> Response {
        if self.words.is_empty() {
            return ui.label("(no words)");
        }

        // Use theme color if not explicitly set
        let text_color = if self.color == Color32::PLACEHOLDER {
            theme.foreground()
        } else {
            self.color
        };

        let time = ui.input(|i| i.time) as f32;

        // Calculate current word index and transition progress
        let current_index = ((time / self.duration) as usize) % self.words.len();
        let time_in_cycle = time % self.duration;

        // Calculate transition progress (0.0 = stable, 1.0 = fully transitioned)
        let transition_start = self.duration - self.transition_duration;
        let flip_t = if time_in_cycle >= transition_start {
            ((time_in_cycle - transition_start) / self.transition_duration).min(1.0)
        } else {
            0.0
        };

        ui.ctx().request_repaint();

        let current_word = &self.words[current_index];
        let next_index = (current_index + 1) % self.words.len();
        let next_word = &self.words[next_index];

        // Calculate text size for both words to get max width
        let font_id = FontId::proportional(self.font_size);

        // Estimate size based on font
        let current_galley =
            ui.painter()
                .layout_no_wrap(current_word.clone(), font_id.clone(), text_color);
        let next_galley =
            ui.painter()
                .layout_no_wrap(next_word.clone(), font_id.clone(), text_color);
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
                    if flip_t > 0.0 {
                        // First half: current word flips out (0.0 to 0.5)
                        if flip_t < 0.5 {
                            let scale_y = 1.0 - (flip_t * 2.0);
                            let alpha = (255.0 * (1.0 - flip_t * 2.0)) as u8;
                            let display_color = self.highlight_color.unwrap_or(text_color);
                            let faded_color = Color32::from_rgba_unmultiplied(
                                display_color.r(),
                                display_color.g(),
                                display_color.b(),
                                alpha,
                            );

                            let offset_y = height * 0.3 * (1.0 - scale_y);
                            painter.text(
                                Pos2::new(center.x, center.y + offset_y),
                                Align2::CENTER_CENTER,
                                current_word,
                                font_id,
                                faded_color,
                            );
                        }
                        // Second half: next word flips in (0.5 to 1.0)
                        else {
                            let scale_y = (flip_t - 0.5) * 2.0;
                            let alpha = (255.0 * scale_y) as u8;
                            let display_color = self.highlight_color.unwrap_or(text_color);
                            let faded_color = Color32::from_rgba_unmultiplied(
                                display_color.r(),
                                display_color.g(),
                                display_color.b(),
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
                        let display_color = self.highlight_color.unwrap_or(text_color);
                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            current_word,
                            font_id,
                            display_color,
                        );
                    }
                }

                FlipStyle::Horizontal => {
                    // Horizontal flip (scale X to simulate rotation)
                    if flip_t > 0.0 {
                        if flip_t < 0.5 {
                            // Current word scales down horizontally
                            let _scale_x = 1.0 - (flip_t * 2.0);
                            let alpha = (255.0 * (1.0 - flip_t * 2.0)) as u8;
                            let display_color = self.highlight_color.unwrap_or(text_color);
                            let faded_color = Color32::from_rgba_unmultiplied(
                                display_color.r(),
                                display_color.g(),
                                display_color.b(),
                                alpha,
                            );

                            // Simple fade for horizontal (true 3D would need mesh transforms)
                            painter.text(
                                center,
                                Align2::CENTER_CENTER,
                                current_word,
                                font_id,
                                faded_color,
                            );
                        } else {
                            // Next word scales up horizontally
                            let scale_x = (flip_t - 0.5) * 2.0;
                            let alpha = (255.0 * scale_x) as u8;
                            let display_color = self.highlight_color.unwrap_or(text_color);
                            let faded_color = Color32::from_rgba_unmultiplied(
                                display_color.r(),
                                display_color.g(),
                                display_color.b(),
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
                        let display_color = self.highlight_color.unwrap_or(text_color);
                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            current_word,
                            font_id,
                            display_color,
                        );
                    }
                }

                FlipStyle::Fade => {
                    // Simple crossfade
                    if flip_t > 0.0 {
                        // Fade out current word
                        let current_alpha = (255.0 * (1.0 - flip_t)) as u8;
                        let display_color = self.highlight_color.unwrap_or(text_color);
                        let current_color = Color32::from_rgba_unmultiplied(
                            display_color.r(),
                            display_color.g(),
                            display_color.b(),
                            current_alpha,
                        );

                        // Fade in next word
                        let next_alpha = (255.0 * flip_t) as u8;
                        let next_color = Color32::from_rgba_unmultiplied(
                            display_color.r(),
                            display_color.g(),
                            display_color.b(),
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
                        let display_color = self.highlight_color.unwrap_or(text_color);
                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            current_word,
                            font_id,
                            display_color,
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
        assert_eq!(flip.duration, 2.5);
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
