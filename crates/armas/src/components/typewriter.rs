//! Typewriter Text Effect
//!
//! Character-by-character text reveal with optional cursor

use egui::{Response, RichText, Ui};

/// Typewriter text effect
///
/// Reveals text character by character with configurable speed
/// and optional blinking cursor.
pub struct Typewriter {
    text: String,
    char_index: f32,
    speed: f32, // characters per second
    cursor_enabled: bool,
    cursor_blink_speed: f32,
    cursor_time: f32,
    loop_mode: bool,
    delay_before_loop: f32,
    delay_timer: f32,
}

impl Typewriter {
    /// Create a new typewriter effect
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            char_index: 0.0,
            speed: 20.0,
            cursor_enabled: true,
            cursor_blink_speed: 2.0,
            cursor_time: 0.0,
            loop_mode: false,
            delay_before_loop: 2.0,
            delay_timer: 0.0,
        }
    }

    /// Set typing speed (characters per second)
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Enable/disable cursor
    pub fn with_cursor(mut self, enabled: bool) -> Self {
        self.cursor_enabled = enabled;
        self
    }

    /// Set cursor blink speed (blinks per second)
    pub fn with_cursor_blink_speed(mut self, speed: f32) -> Self {
        self.cursor_blink_speed = speed.max(0.1);
        self
    }

    /// Enable looping
    pub fn with_loop(mut self, enabled: bool) -> Self {
        self.loop_mode = enabled;
        self
    }

    /// Set delay before looping (seconds)
    pub fn with_loop_delay(mut self, delay: f32) -> Self {
        self.delay_before_loop = delay.max(0.0);
        self
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.char_index = 0.0;
        self.delay_timer = 0.0;
        self.cursor_time = 0.0;
    }

    /// Check if typing is complete
    pub fn is_complete(&self) -> bool {
        self.char_index >= self.text.len() as f32
    }

    /// Show the typewriter text
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        self.show_styled(ui, |text| RichText::new(text))
    }

    /// Show the typewriter text with custom styling
    pub fn show_styled<F>(&mut self, ui: &mut Ui, style_fn: F) -> Response
    where
        F: FnOnce(String) -> RichText,
    {
        let dt = ui.input(|i| i.stable_dt);

        // Update animation
        if !self.is_complete() {
            self.char_index += dt * self.speed;
            if self.char_index > self.text.len() as f32 {
                self.char_index = self.text.len() as f32;
                self.delay_timer = 0.0;
            }
        } else if self.loop_mode {
            self.delay_timer += dt;
            if self.delay_timer >= self.delay_before_loop {
                self.reset();
            }
        }

        // Update cursor blink
        if self.cursor_enabled {
            self.cursor_time += dt;
        }

        // Get visible text
        let visible_chars = self.char_index.floor() as usize;
        let visible_text = self.text.chars().take(visible_chars).collect::<String>();

        // Add cursor if enabled
        let cursor_visible = if self.cursor_enabled {
            ((self.cursor_time * self.cursor_blink_speed * 2.0) % 2.0) < 1.0
        } else {
            false
        };

        let display_text = if cursor_visible && !self.is_complete() {
            format!("{}|", visible_text)
        } else if cursor_visible && self.loop_mode {
            format!("{}|", visible_text)
        } else {
            visible_text
        };

        // Request repaint
        if !self.is_complete() || (self.loop_mode && self.is_complete()) || cursor_visible {
            ui.ctx().request_repaint();
        }

        // Show text
        ui.label(style_fn(display_text))
    }
}

/// Typewriter with word-by-word reveal
pub struct WordTypewriter {
    text: String,
    words: Vec<String>,
    word_index: f32,
    speed: f32, // words per second
    loop_mode: bool,
    delay_before_loop: f32,
    delay_timer: f32,
}

impl WordTypewriter {
    /// Create a new word-by-word typewriter
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let words: Vec<String> = text.split_whitespace().map(String::from).collect();

        Self {
            text,
            words,
            word_index: 0.0,
            speed: 4.0,
            loop_mode: false,
            delay_before_loop: 2.0,
            delay_timer: 0.0,
        }
    }

    /// Set typing speed (words per second)
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Enable looping
    pub fn with_loop(mut self, enabled: bool) -> Self {
        self.loop_mode = enabled;
        self
    }

    /// Set delay before looping (seconds)
    pub fn with_loop_delay(mut self, delay: f32) -> Self {
        self.delay_before_loop = delay.max(0.0);
        self
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.word_index = 0.0;
        self.delay_timer = 0.0;
    }

    /// Check if typing is complete
    pub fn is_complete(&self) -> bool {
        self.word_index >= self.words.len() as f32
    }

    /// Show the word typewriter
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        self.show_styled(ui, |text| RichText::new(text))
    }

    /// Show with custom styling
    pub fn show_styled<F>(&mut self, ui: &mut Ui, style_fn: F) -> Response
    where
        F: FnOnce(String) -> RichText,
    {
        let dt = ui.input(|i| i.stable_dt);

        // Update animation
        if !self.is_complete() {
            self.word_index += dt * self.speed;
            if self.word_index > self.words.len() as f32 {
                self.word_index = self.words.len() as f32;
                self.delay_timer = 0.0;
            }
        } else if self.loop_mode {
            self.delay_timer += dt;
            if self.delay_timer >= self.delay_before_loop {
                self.reset();
            }
        }

        // Get visible words
        let visible_count = self.word_index.floor() as usize;
        let visible_text = self
            .words
            .iter()
            .take(visible_count)
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");

        // Request repaint
        if !self.is_complete() || (self.loop_mode && self.is_complete()) {
            ui.ctx().request_repaint();
        }

        // Show text
        ui.label(style_fn(visible_text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typewriter_creation() {
        let tw = Typewriter::new("Hello");
        assert_eq!(tw.char_index, 0.0);
        assert_eq!(tw.text, "Hello");
    }

    #[test]
    fn test_typewriter_config() {
        let tw = Typewriter::new("Hello")
            .with_speed(10.0)
            .with_cursor(false)
            .with_loop(true);

        assert_eq!(tw.speed, 10.0);
        assert!(!tw.cursor_enabled);
        assert!(tw.loop_mode);
    }

    #[test]
    fn test_word_typewriter() {
        let tw = WordTypewriter::new("Hello world test");
        assert_eq!(tw.words.len(), 3);
        assert_eq!(tw.words[0], "Hello");
    }
}
