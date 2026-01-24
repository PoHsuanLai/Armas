//! Animated components and visual effects for egui
//!
//! This crate provides animated UI components and decorative backgrounds:
//!
//! ## Text Animations
//! - [`FlipWords`] - Animated word cycling with flip transitions
//! - [`Typewriter`] / [`WordTypewriter`] - Typewriter text effects
//! - [`ScrambleText`] - Text scramble/decode animations
//! - [`ScrollingBanner`] - Infinite scrolling text banner
//! - [`GlowingBorder`] - Animated glowing border effect
//!
//! ## Backgrounds
//! - [`AuroraBackground`] - Animated aurora borealis effect
//! - [`MeteorShower`] - Falling meteor animation
//! - [`Sparkles`] - Animated sparkle particles
//! - [`Spotlight`] - Animated spotlight effect
//! - [`DotPattern`] - Static dot grid pattern
//! - [`GridPattern`] - Static grid pattern
//! - [`RetroGrid`] - Retro-style perspective grid
//!
//! ## Effects
//! - [`GradientText`] - Text with animated gradient colors
//! - [`MovingBorder`] - Button with animated gradient border

// Text animations
pub mod flip_words;
pub mod glowing_border;
pub mod scramble_text;
pub mod scrolling_banner;
pub mod typewriter;

// Backgrounds
pub mod backgrounds;

// Effects
pub mod effects;

// Re-exports - Text animations
pub use flip_words::{FlipStyle, FlipWords};
pub use glowing_border::GlowingBorder;
pub use scramble_text::ScrambleText;
pub use scrolling_banner::{ScrollDirection, ScrollingBanner};
pub use typewriter::{Typewriter, WordTypewriter};

// Re-exports - Backgrounds
pub use backgrounds::{
    AuroraBackground, DotPattern, GridPattern, MeteorShower, RetroGrid, Sparkles, Spotlight,
};

// Re-exports - Effects
pub use effects::{GradientText, MovingBorder};
