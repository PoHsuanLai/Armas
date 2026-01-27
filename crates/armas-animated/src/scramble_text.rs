//! Text Scramble / Glitch Effect
//!
//! Random character morphing that gradually reveals target text

use egui::{Id, Response, RichText, Ui};

/// Scramble text internal state stored in egui memory
#[derive(Clone)]
struct ScrambleTextState {
    start_time: f32,
    loop_start_time: f32,
    last_frame_time: f32,
    current: String,
}

impl Default for ScrambleTextState {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            loop_start_time: 0.0,
            last_frame_time: 0.0,
            current: String::new(),
        }
    }
}

/// Text scramble effect
///
/// Displays random characters that gradually morph into the target text.
pub struct ScrambleText {
    id: Id,
    target: String,
    speed: f32,
    charset: Vec<char>,
    frame_interval: f32,
    loop_mode: bool,
    delay_before_loop: f32,
}

impl ScrambleText {
    /// Create a new scramble text effect
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            id: Id::new("scramble_text"),
            target: target.into(),
            speed: 2.0,
            charset: Self::default_charset(),
            frame_interval: 0.05,
            loop_mode: false,
            delay_before_loop: 1.0,
        }
    }

    /// Default character set for scrambling
    fn default_charset() -> Vec<char> {
        "!@#$%^&*()_+-=[]{}|;:',.<>?/~`ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
            .chars()
            .collect()
    }

    /// Set unique ID for this scramble text (required for multiple instances)
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }

    /// Set scramble speed (progress per second, 0.0 to 1.0)
    #[must_use] 
    pub const fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    /// Set custom character set for scrambling
    pub fn charset(mut self, charset: impl Into<String>) -> Self {
        self.charset = charset.into().chars().collect();
        if self.charset.is_empty() {
            self.charset = Self::default_charset();
        }
        self
    }

    /// Set frame interval (seconds between character changes)
    #[must_use] 
    pub const fn frame_interval(mut self, interval: f32) -> Self {
        self.frame_interval = interval.max(0.01);
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

    /// Update and show the scramble text
    pub fn show(&mut self, ui: &mut Ui, _theme: &armas::Theme) -> Response {
        self.show_styled(ui, RichText::new)
    }

    /// Update and show with custom styling
    pub fn show_styled<F>(&mut self, ui: &mut Ui, style_fn: F) -> Response
    where
        F: FnOnce(String) -> RichText,
    {
        let time = ui.input(|i| i.time) as f32;

        // Get or initialize state
        let mut state =
            ui.data_mut(|d| d.get_temp::<ScrambleTextState>(self.id).unwrap_or_default());

        // Initialize start time on first frame
        if state.start_time == 0.0 {
            state.start_time = time;
            state.loop_start_time = time;
            state.last_frame_time = time;
            state.current = (0..self.target.len()).map(|_| self.charset[0]).collect();
        }

        // Calculate elapsed time and progress
        let elapsed = time - state.start_time;
        let progress = (elapsed * self.speed).min(1.0);
        let is_complete = progress >= 1.0;

        // Handle looping
        if is_complete && self.loop_mode {
            let delay_elapsed = time - state.loop_start_time;
            if delay_elapsed >= self.delay_before_loop {
                // Reset for next loop
                state.start_time = time;
                state.loop_start_time = time;
                state.last_frame_time = time;
                state.current = (0..self.target.len()).map(|_| self.charset[0]).collect();
            }
        } else if is_complete && !self.loop_mode {
            // Mark loop start time when complete (for delay)
            if state.loop_start_time == state.start_time {
                state.loop_start_time = time;
            }
        }

        // Update frame for character changes
        if time - state.last_frame_time >= self.frame_interval {
            state.last_frame_time = time;
            self.update_characters(&mut state.current, progress, time);
        }

        // Store state back
        ui.data_mut(|d| d.insert_temp(self.id, state.clone()));

        // Request repaint
        if !is_complete || self.loop_mode {
            ui.ctx().request_repaint();
        }

        ui.label(style_fn(state.current))
    }

    /// Update the scrambled characters
    fn update_characters(&self, current: &mut String, progress: f32, time: f32) {
        let target_chars: Vec<char> = self.target.chars().collect();
        let mut current_chars: Vec<char> = current.chars().collect();

        // Ensure current has same length as target
        while current_chars.len() < target_chars.len() {
            current_chars.push(self.random_char(time, progress));
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

            if progress >= reveal_end {
                // Fully revealed
                current_chars[i] = *target_char;
            } else if progress >= reveal_start {
                // In scramble zone - randomize
                if current_chars[i] != *target_char {
                    current_chars[i] = self.random_char(time + i as f32, progress);
                }
            } else {
                // Not yet started
                current_chars[i] = self.random_char(time + i as f32, progress);
            }
        }

        *current = current_chars.into_iter().collect();
    }

    /// Get a random character from the charset
    fn random_char(&self, time: f32, progress: f32) -> char {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let state = RandomState::new();
        let mut hasher = state.build_hasher();

        // Use time and progress as seed for pseudo-randomness
        ((time * 1000.0) as u64).hash(&mut hasher);
        ((progress * 1000.0) as u64).hash(&mut hasher);

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
        assert_eq!(st.speed, 2.0);
    }

    #[test]
    fn test_scramble_config() {
        let st = ScrambleText::new("Test").speed(5.0).loop_mode(true);

        assert_eq!(st.speed, 5.0);
        assert!(st.loop_mode);
    }
}
