//! Text Scramble / Glitch Effect
//!
//! Random character morphing that gradually reveals target text

use egui::{Response, RichText, Ui};

/// Text scramble effect
///
/// Displays random characters that gradually morph into the target text.
pub struct ScrambleText {
    target: String,
    current: String,
    progress: f32,
    speed: f32,
    charset: Vec<char>,
    frame_time: f32,
    frame_interval: f32,
    loop_mode: bool,
    delay_before_loop: f32,
    delay_timer: f32,
}

impl ScrambleText {
    /// Create a new scramble text effect
    pub fn new(target: impl Into<String>) -> Self {
        let target = target.into();
        let current = (0..target.len())
            .map(|_| Self::default_charset()[0])
            .collect();

        Self {
            target,
            current,
            progress: 0.0,
            speed: 2.0,
            charset: Self::default_charset(),
            frame_time: 0.0,
            frame_interval: 0.05,
            loop_mode: false,
            delay_before_loop: 1.0,
            delay_timer: 0.0,
        }
    }

    /// Default character set for scrambling
    fn default_charset() -> Vec<char> {
        "!@#$%^&*()_+-=[]{}|;:',.<>?/~`ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
            .chars()
            .collect()
    }

    /// Set scramble speed (progress per second, 0.0 to 1.0)
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Set custom character set for scrambling
    pub fn with_charset(mut self, charset: impl Into<String>) -> Self {
        self.charset = charset.into().chars().collect();
        if self.charset.is_empty() {
            self.charset = Self::default_charset();
        }
        self
    }

    /// Set frame interval (seconds between character changes)
    pub fn with_frame_interval(mut self, interval: f32) -> Self {
        self.frame_interval = interval.max(0.01);
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
        self.progress = 0.0;
        self.delay_timer = 0.0;
        self.frame_time = 0.0;
        self.current = (0..self.target.len()).map(|_| self.charset[0]).collect();
    }

    /// Set a new target text
    pub fn set_target(&mut self, target: impl Into<String>) {
        self.target = target.into();
        self.reset();
    }

    /// Check if scrambling is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    /// Update and show the scramble text
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        self.show_styled(ui, RichText::new)
    }

    /// Update and show with custom styling
    pub fn show_styled<F>(&mut self, ui: &mut Ui, style_fn: F) -> Response
    where
        F: FnOnce(String) -> RichText,
    {
        let dt = ui.input(|i| i.stable_dt);

        // Update progress
        if !self.is_complete() {
            self.progress += dt * self.speed;
            if self.progress > 1.0 {
                self.progress = 1.0;
                self.delay_timer = 0.0;
            }
        } else if self.loop_mode {
            self.delay_timer += dt;
            if self.delay_timer >= self.delay_before_loop {
                self.reset();
            }
        }

        // Update frame for character changes
        self.frame_time += dt;
        if self.frame_time >= self.frame_interval {
            self.frame_time = 0.0;
            self.update_characters();
        }

        // Request repaint
        if !self.is_complete() || (self.loop_mode && self.is_complete()) {
            ui.ctx().request_repaint();
        }

        ui.label(style_fn(self.current.clone()))
    }

    /// Update the scrambled characters
    fn update_characters(&mut self) {
        let target_chars: Vec<char> = self.target.chars().collect();
        let mut current_chars: Vec<char> = self.current.chars().collect();

        // Ensure current has same length as target
        while current_chars.len() < target_chars.len() {
            current_chars.push(self.random_char());
        }
        while current_chars.len() > target_chars.len() {
            current_chars.pop();
        }

        // Update each character based on progress
        for (i, target_char) in target_chars.iter().enumerate() {
            // Calculate when this character should be revealed
            // Characters reveal sequentially from left to right
            let char_progress = (i as f32 / target_chars.len() as f32).max(0.0);
            let reveal_start = char_progress * 0.7; // Start revealing at 70% through the position
            let reveal_end = char_progress * 0.7 + 0.3; // Finish by adding 30%

            if self.progress >= reveal_end {
                // Fully revealed
                current_chars[i] = *target_char;
            } else if self.progress >= reveal_start {
                // In scramble zone - randomize
                if current_chars[i] != *target_char {
                    current_chars[i] = self.random_char();
                }
            } else {
                // Not yet started
                current_chars[i] = self.random_char();
            }
        }

        self.current = current_chars.into_iter().collect();
    }

    /// Get a random character from the charset
    fn random_char(&self) -> char {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let state = RandomState::new();
        let mut hasher = state.build_hasher();

        // Use frame time and progress as seed for pseudo-randomness
        ((self.frame_time * 1000.0) as u64).hash(&mut hasher);
        ((self.progress * 1000.0) as u64).hash(&mut hasher);

        let hash = hasher.finish();
        let index = (hash as usize) % self.charset.len();
        self.charset[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scramble_creation() {
        let st = ScrambleText::new("Hello");
        assert_eq!(st.target, "Hello");
        assert_eq!(st.current.len(), 5);
    }

    #[test]
    fn test_scramble_config() {
        let st = ScrambleText::new("Test").with_speed(5.0).with_loop(true);

        assert_eq!(st.speed, 5.0);
        assert!(st.loop_mode);
    }

    #[test]
    fn test_set_target() {
        let mut st = ScrambleText::new("Hello");
        st.set_target("World");
        assert_eq!(st.target, "World");
        assert_eq!(st.progress, 0.0);
    }
}
