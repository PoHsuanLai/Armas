//! Typewriter Text Effect
//!
//! Character-by-character text reveal with optional cursor

use egui::{Id, Response, RichText, Ui};

/// Typewriter internal state stored in egui memory
#[derive(Clone)]
struct TypewriterState {
    start_time: f32,
    loop_start_time: f32,
}

impl Default for TypewriterState {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            loop_start_time: 0.0,
        }
    }
}

/// Typewriter text effect
///
/// Reveals text character by character with configurable speed
/// and optional blinking cursor.
pub struct Typewriter {
    id: Id,
    text: String,
    speed: f32, // characters per second
    cursor_enabled: bool,
    cursor_blink_speed: f32,
    loop_mode: bool,
    delay_before_loop: f32,
}

impl Typewriter {
    /// Create a new typewriter effect
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Id::new("typewriter"),
            text: text.into(),
            speed: 20.0,
            cursor_enabled: true,
            cursor_blink_speed: 2.0,
            loop_mode: false,
            delay_before_loop: 2.0,
        }
    }

    /// Set unique ID for this typewriter (required for multiple instances)
    #[must_use]
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }

    /// Set typing speed (characters per second)
    #[must_use]
    pub const fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Enable/disable cursor
    #[must_use]
    pub const fn cursor(mut self, enabled: bool) -> Self {
        self.cursor_enabled = enabled;
        self
    }

    /// Set cursor blink speed (blinks per second)
    #[must_use]
    pub const fn cursor_blink_speed(mut self, speed: f32) -> Self {
        self.cursor_blink_speed = speed.max(0.1);
        self
    }

    /// Enable looping
    #[must_use]
    pub const fn loop_mode(mut self, enabled: bool) -> Self {
        self.loop_mode = enabled;
        self
    }

    /// Set delay before looping (seconds)
    #[must_use]
    pub const fn loop_delay(mut self, delay: f32) -> Self {
        self.delay_before_loop = delay.max(0.0);
        self
    }

    /// Show the typewriter text
    pub fn show(&mut self, ui: &mut Ui, _theme: &armas::Theme) -> Response {
        self.show_styled(ui, RichText::new)
    }

    /// Show the typewriter text with custom styling
    pub fn show_styled<F>(&mut self, ui: &mut Ui, style_fn: F) -> Response
    where
        F: FnOnce(String) -> RichText,
    {
        let time = ui.input(|i| i.time) as f32;

        // Get or initialize state
        let mut state = ui.data_mut(|d| d.get_temp::<TypewriterState>(self.id).unwrap_or_default());

        // Initialize start time on first frame
        if state.start_time == 0.0 {
            state.start_time = time;
            state.loop_start_time = time;
        }

        // Calculate elapsed time since start
        let elapsed = time - state.start_time;
        let char_index = (elapsed * self.speed).min(self.text.len() as f32);
        let is_complete = char_index >= self.text.len() as f32;

        // Handle looping
        if is_complete && self.loop_mode {
            let delay_elapsed = time - state.loop_start_time;
            if delay_elapsed >= self.delay_before_loop {
                // Reset for next loop
                state.start_time = time;
                state.loop_start_time = time;
            }
        } else if is_complete && !self.loop_mode {
            // Mark loop start time when complete (for delay)
            if (state.loop_start_time - state.start_time).abs() < f32::EPSILON {
                state.loop_start_time = time;
            }
        }

        // Store state back
        ui.data_mut(|d| d.insert_temp(self.id, state.clone()));

        // Get visible text
        let visible_chars = char_index.floor() as usize;
        let visible_text = self.text.chars().take(visible_chars).collect::<String>();

        // Add cursor if enabled
        let cursor_visible = if self.cursor_enabled {
            ((time * self.cursor_blink_speed * 2.0) % 2.0) < 1.0
        } else {
            false
        };

        let display_text = if cursor_visible && (!is_complete || self.loop_mode) {
            format!("{visible_text}|")
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

/// Word typewriter internal state stored in egui memory
#[derive(Clone)]
struct WordTypewriterState {
    start_time: f32,
    loop_start_time: f32,
}

impl Default for WordTypewriterState {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            loop_start_time: 0.0,
        }
    }
}

/// Typewriter with word-by-word reveal
pub struct WordTypewriter {
    id: Id,
    _text: String,
    words: Vec<String>,
    speed: f32, // words per second
    loop_mode: bool,
    delay_before_loop: f32,
}

impl WordTypewriter {
    /// Create a new word-by-word typewriter
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let words: Vec<String> = text.split_whitespace().map(String::from).collect();

        Self {
            id: Id::new("word_typewriter"),
            _text: text,
            words,
            speed: 4.0,
            loop_mode: false,
            delay_before_loop: 2.0,
        }
    }

    #[must_use]
    /// Set unique ID for this typewriter (required for multiple instances)
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }

    /// Set typing speed (words per second)
    #[must_use]
    pub const fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Enable looping
    #[must_use]
    pub const fn loop_mode(mut self, enabled: bool) -> Self {
        self.loop_mode = enabled;
        self
    }

    /// Set delay before looping (seconds)
    #[must_use]
    pub const fn loop_delay(mut self, delay: f32) -> Self {
        self.delay_before_loop = delay.max(0.0);
        self
    }

    /// Show the word typewriter
    pub fn show(&mut self, ui: &mut Ui, _theme: &armas::Theme) -> Response {
        self.show_styled(ui, RichText::new)
    }

    /// Show with custom styling
    pub fn show_styled<F>(&mut self, ui: &mut Ui, style_fn: F) -> Response
    where
        F: FnOnce(String) -> RichText,
    {
        let time = ui.input(|i| i.time) as f32;

        // Get or initialize state
        let mut state = ui.data_mut(|d| {
            d.get_temp::<WordTypewriterState>(self.id)
                .unwrap_or_default()
        });

        // Initialize start time on first frame
        if state.start_time == 0.0 {
            state.start_time = time;
            state.loop_start_time = time;
        }

        // Calculate elapsed time since start
        let elapsed = time - state.start_time;
        let word_index = (elapsed * self.speed).min(self.words.len() as f32);
        let is_complete = word_index >= self.words.len() as f32;

        // Handle looping
        if is_complete && self.loop_mode {
            let delay_elapsed = time - state.loop_start_time;
            if delay_elapsed >= self.delay_before_loop {
                // Reset for next loop
                state.start_time = time;
                state.loop_start_time = time;
            }
        } else if is_complete && !self.loop_mode {
            // Mark loop start time when complete (for delay)
            if (state.loop_start_time - state.start_time).abs() < f32::EPSILON {
                state.loop_start_time = time;
            }
        }

        // Store state back
        ui.data_mut(|d| d.insert_temp(self.id, state.clone()));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typewriter_creation() {
        let tw = Typewriter::new("Hello");
        assert_eq!(tw.text, "Hello");
        assert_eq!(tw.speed, 20.0);
    }

    #[test]
    fn test_typewriter_config() {
        let tw = Typewriter::new("Hello")
            .speed(10.0)
            .cursor(false)
            .loop_mode(true);

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
