//! # egui-alig
//!
//! **ALIG** - A component library for egui
//!
//! A reusable component library for egui with Material Design inspired theming.
//! Provides a clean, professional design language with serializable themes.
//!
//! ## Features
//!
//! - 🎨 **Material Design Color Palette** - Primary, secondary, surface, outline, semantic colors
//! - 💾 **Serializable Themes** - Save/load themes as JSON
//! - 🧩 **Reusable Components** - Slots, routing buttons, section headers
//! - 🎯 **Builder Pattern API** - Clean, declarative UI code
//! - ✨ **Professional Design** - Glassmorphism-inspired styling
//!
//! ## Example
//!
//! ```rust,no_run
//! use armas::{Theme, components::Slot};
//!
//! fn ui(ui: &mut egui::Ui) {
//!     let theme = Theme::dark();
//!
//!     let slot = Slot::new(60.0, 30.0)
//!         .with_effect("Reverb")
//!         .level(0.7);
//!
//!     let response = slot.show(ui, &theme);
//!     if response.clicked() {
//!         // Handle click
//!     }
//! }
//! ```

pub mod animation;
pub mod color;
pub mod components;
pub mod layout;
pub mod painter_ext;
pub mod theme;

// Re-exports for convenience
pub use animation::{
    Animation, AnimationSequence, AnimationState, EasingFunction, LoopMode, LoopingAnimation,
    SpringAnimation, StaggeredAnimation,
};
pub use color::{
    blend, lerp_color, saturate, with_alpha, BlendMode, ColorStop, Gradient, NeonPalette,
};
pub use components::*;
pub use layout::*;
pub use painter_ext::{neon_circle, neon_line, PainterExt};
pub use theme::Theme;
