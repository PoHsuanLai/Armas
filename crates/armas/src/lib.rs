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
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas::prelude::*;
//!
//! // Create a button with shadcn/ui styling
//! if Button::new("Click me")
//!     .variant(ButtonVariant::Default)
//!     .show(ui)
//!     .clicked()
//! {
//!     // Handle click
//! }
//! # }
//! ```

pub mod animation;
pub mod color;
pub mod components;
pub mod ext;
pub mod fonts;
pub mod icon;
pub mod layout;
pub mod theme;

// Prelude module for convenient imports
pub mod prelude;

// Re-exports for convenience
pub use animation::{
    Animation, AnimationSequence, AnimationState, EasingFunction, LoopMode, LoopingAnimation,
    SpringAnimation, StaggeredAnimation,
};
pub use color::{
    blend, lerp_color, saturate, with_alpha, BlendMode, ColorStop, Gradient, NeonPalette,
};
pub use components::*;
pub use ext::{
    ArmasContextExt, {neon_circle, neon_line, PainterExt},
};
pub use fonts::{FontFamilyBuilder, FontWeight};
pub use layout::*;
pub use theme::Theme;
