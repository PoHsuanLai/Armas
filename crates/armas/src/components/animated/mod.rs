//! Animated text components
//!
//! Components with text animations and visual effects.

pub mod flip_words;
pub mod glowing_border;
pub mod scramble_text;
pub mod scrolling_banner;
pub mod typewriter;

// Re-exports
pub use flip_words::{FlipStyle, FlipWords};
pub use glowing_border::GlowingBorder;
pub use scramble_text::ScrambleText;
pub use scrolling_banner::{ScrollDirection, ScrollingBanner};
pub use typewriter::{Typewriter, WordTypewriter};
