//! Context extension for Armas theme storage
//!
//! This extension trait allows storing the Armas theme in egui's context,
//! eliminating the need to pass theme to every component.

use crate::Theme;
use egui::Context;

/// Extension trait for storing Armas theme in egui Context
///
/// # Example
///
/// ```rust,no_run
/// use armas_basic::ext::ArmasContextExt;
/// use armas_basic::Theme;
///
/// fn setup(ctx: &egui::Context) {
///     // Set theme once
///     ctx.set_armas_theme(Theme::dark());
/// }
///
/// fn my_ui(ui: &mut egui::Ui) {
///     // Components automatically get theme from context
///     let theme = ui.ctx().armas_theme();
///     // ...
/// }
/// ```
pub trait ArmasContextExt {
    /// Get the current Armas theme from context
    ///
    /// Returns the theme previously set with `set_armas_theme()`.
    /// If no theme was set, returns `Theme::dark()` as default.
    fn armas_theme(&self) -> Theme;

    /// Set the Armas theme in context
    ///
    /// This stores the theme globally in egui's context, making it
    /// available to all components without explicit passing.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use armas_basic::ext::ArmasContextExt;
    /// # use armas_basic::Theme;
    /// # let ctx = &egui::Context::default();
    /// # let user_wants_light_theme = false;
    /// // Set theme once at startup
    /// ctx.set_armas_theme(Theme::dark());
    ///
    /// // Change theme dynamically
    /// if user_wants_light_theme {
    ///     ctx.set_armas_theme(Theme::light());
    /// }
    /// ```
    fn set_armas_theme(&self, theme: Theme);
}

impl ArmasContextExt for Context {
    fn armas_theme(&self) -> Theme {
        self.data(|d| d.get_temp(egui::Id::new("armas_theme")))
            .unwrap_or_else(Theme::dark)
    }

    fn set_armas_theme(&self, theme: Theme) {
        self.data_mut(|d| d.insert_temp(egui::Id::new("armas_theme"), theme));
    }
}
