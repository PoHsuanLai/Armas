//! # Armas
//!
//! UI component library for egui following shadcn/ui design patterns.
//!
//! Includes 50+ reusable components with a theme system supporting serializable
//! color palettes and spacing configurations.
//!
//! ## Example
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas::prelude::*;
//!
//! let theme = ui.ctx().armas_theme();
//!
//! if Button::new("Click me")
//!     .variant(ButtonVariant::Default)
//!     .show(ui, &theme)
//!     .clicked()
//! {
//!     // Handle click
//! }
//! # }
//! ```

#![warn(missing_docs)]

/// Animation utilities and easing functions
pub mod animation;
/// Color manipulation utilities
pub mod color;
/// UI components
pub mod components;
/// Extension traits for egui types
pub mod ext;
/// Font utilities
pub mod fonts;
/// Icon rendering
pub mod icon;
/// Layout components
pub mod layout;
/// Theme system
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
