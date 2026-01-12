//! Typewriter Text Effect
//!
//! Character-by-character text reveal with optional cursor

use crate::traits::{ArmasModifiers, ArmasViewMut};
use egui::{Response, RichText, Ui};

/// Typewriter text effect
///
/// Reveals text character by character with configurable speed
/// and optional blinking cursor.
pub struct Typewriter {
    text: String,
    speed: f32, // characters per second
    cursor_enabled: bool,
    cursor_blink_speed: f32,
    loop_mode: bool,
    delay_before_loop: f32,
    start_time: Option<f32>,
}

impl Typewriter {
    /// Create a new typewriter effect
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            speed: 20.0,
            cursor_enabled: true,
            cursor_blink_speed: 2.0,
            loop_mode: false,
            delay_before_loop: 2.0,
            start_time: None,
        }
    }

    /// Set typing speed (characters per second)
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Enable/disable cursor
    pub fn cursor(mut self, enabled: bool) -> Self {
        self.cursor_enabled = enabled;
        self
    }

    /// Set cursor blink speed (blinks per second)
    pub fn cursor_blink_speed(mut self, speed: f32) -> Self {
        self.cursor_blink_speed = speed.max(0.1);
        self
    }

    /// Enable looping
    pub fn looping(mut self, enabled: bool) -> Self {
        self.loop_mode = enabled;
        self
    }

    /// Set delay before looping (seconds)
    pub fn loop_delay(mut self, delay: f32) -> Self {
        self.delay_before_loop = delay.max(0.0);
        self
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.start_time = None;
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
        let time = ui.input(|i| i.time) as f32;

        // Initialize start time on first show
        if self.start_time.is_none() {
            self.start_time = Some(time);
        }

        let elapsed = time - self.start_time.unwrap();
        let typing_duration = self.text.len() as f32 / self.speed;

        // Calculate char index
        let (char_index, is_complete) = if elapsed < typing_duration {
            (elapsed * self.speed, false)
        } else if self.loop_mode {
            // Check if in delay period
            let delay_end = typing_duration + self.delay_before_loop;
            if elapsed < delay_end {
                (self.text.len() as f32, true)
            } else {
                // Reset for next loop
                self.start_time = Some(time);
                (0.0, false)
            }
        } else {
            (self.text.len() as f32, true)
        };

        // Get visible text
        let visible_chars = char_index.floor() as usize;
        let visible_text = self.text.chars().take(visible_chars).collect::<String>();

        // Add cursor if enabled
        let cursor_visible = if self.cursor_enabled {
            ((time * self.cursor_blink_speed * 2.0) % 2.0) < 1.0
        } else {
            false
        };

        let display_text = if cursor_visible && !is_complete {
            format!("{}|", visible_text)
        } else if cursor_visible && self.loop_mode {
            format!("{}|", visible_text)
        } else {
            visible_text
        };

        // Request repaint
        if !is_complete || self.loop_mode || cursor_visible {
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
    speed: f32, // words per second
    loop_mode: bool,
    delay_before_loop: f32,
    start_time: Option<f32>,
}

impl WordTypewriter {
    /// Create a new word-by-word typewriter
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let words: Vec<String> = text.split_whitespace().map(String::from).collect();

        Self {
            text,
            words,
            speed: 4.0,
            loop_mode: false,
            delay_before_loop: 2.0,
            start_time: None,
        }
    }

    /// Set typing speed (words per second)
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Enable looping
    pub fn looping(mut self, enabled: bool) -> Self {
        self.loop_mode = enabled;
        self
    }

    /// Set delay before looping (seconds)
    pub fn loop_delay(mut self, delay: f32) -> Self {
        self.delay_before_loop = delay.max(0.0);
        self
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.start_time = None;
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
        let time = ui.input(|i| i.time) as f32;

        // Initialize start time on first show
        if self.start_time.is_none() {
            self.start_time = Some(time);
        }

        let elapsed = time - self.start_time.unwrap();
        let typing_duration = self.words.len() as f32 / self.speed;

        // Calculate word index
        let (word_index, is_complete) = if elapsed < typing_duration {
            (elapsed * self.speed, false)
        } else if self.loop_mode {
            // Check if in delay period
            let delay_end = typing_duration + self.delay_before_loop;
            if elapsed < delay_end {
                (self.words.len() as f32, true)
            } else {
                // Reset for next loop
                self.start_time = Some(time);
                (0.0, false)
            }
        } else {
            (self.words.len() as f32, true)
        };

        // Get visible words
        let visible_count = word_index.floor() as usize;
        let visible_text = self
            .words
            .iter()
            .take(visible_count)
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");

        // Request repaint
        if !is_complete || self.loop_mode {
            ui.ctx().request_repaint();
        }

        // Show text
        ui.label(style_fn(visible_text))
    }
}

impl ArmasViewMut for Typewriter {
    type Output = Response;

    fn ui(&mut self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

impl ArmasModifiers for Typewriter {}

impl ArmasViewMut for WordTypewriter {
    type Output = Response;

    fn ui(&mut self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

impl ArmasModifiers for WordTypewriter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typewriter_creation() {
        let tw = Typewriter::new("Hello");
        assert_eq!(tw.start_time, None);
        assert_eq!(tw.text, "Hello");
    }

    #[test]
    fn test_typewriter_config() {
        let tw = Typewriter::new("Hello")
            .speed(10.0)
            .cursor(false)
            .looping(true);

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
