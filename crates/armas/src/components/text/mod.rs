//! Text animation components
//!
//! Animated text effects including flip words, scramble text,
//! typewriter effects, and gradient text.

pub mod flip_words;
pub mod gradient_text;
pub mod scramble_text;
pub mod typewriter;

pub use flip_words::{FlipStyle, FlipWords};
pub use gradient_text::GradientText;
pub use scramble_text::ScrambleText;
pub use typewriter::{Typewriter, WordTypewriter};
