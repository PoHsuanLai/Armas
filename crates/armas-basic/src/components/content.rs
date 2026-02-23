//! Shared content context for components that support closure-based rendering.
//!
//! When components accept custom content via closures (e.g. `Button::content().show_content()`),
//! the closure receives a [`ContentContext`] with state-dependent styling information so that
//! icons and text automatically match the component's visual state.

use egui::Color32;

/// Context passed to custom content closures.
///
/// Provides state-dependent styling information so icons and text
/// match the component's current visual state (hover, pressed, active, disabled).
///
/// The `Ui` passed alongside this context also has `visuals.override_text_color`
/// set to [`color`](Self::color), so plain `ui.label()` calls inside the closure
/// automatically get the right color.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::components::{Button, ContentContext};
///
/// Button::content()
///     .show_content(ui, |ui, ctx| {
///         // ctx.color is the correct text/icon color for the current state
///         ui.label("Save");
///     });
/// # }
/// ```
pub struct ContentContext {
    /// The text/icon color appropriate for the current state.
    /// Changes with hover, pressed, active, and disabled states.
    pub color: Color32,
    /// The font size the component would use for its text label mode.
    pub font_size: f32,
    /// Whether the component is currently in an active/selected/pressed state.
    pub is_active: bool,
}
