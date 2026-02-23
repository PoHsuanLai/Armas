//! Alert Component
//!
//! Inline alert messages styled like shadcn/ui Alert.
//! Supports info (default) and destructive variants.
//! Built on top of Card component for consistency.

use crate::ext::ArmasContextExt;
use crate::icon;
use crate::{Card, CardVariant, Theme};
use egui::{vec2, Color32, Sense, Ui};

// shadcn Alert constants
const CORNER_RADIUS: f32 = 8.0; // rounded-lg
const PADDING: f32 = 16.0; // p-4

/// Alert variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertVariant {
    /// Informational alert (default)
    #[default]
    Info,
    /// Destructive/error alert (red)
    Destructive,
}

impl AlertVariant {
    const fn color(self, theme: &Theme) -> Color32 {
        match self {
            Self::Info => theme.foreground(),
            Self::Destructive => theme.destructive(),
        }
    }

    fn background_color(self, theme: &Theme) -> Color32 {
        match self {
            Self::Info => theme.muted(),
            Self::Destructive => theme.destructive().linear_multiply(0.08),
        }
    }

    const fn border_color(self, theme: &Theme) -> Color32 {
        match self {
            Self::Info => theme.border(),
            Self::Destructive => theme.destructive(),
        }
    }
}

/// Alert component for inline messages
///
/// Built on top of the Card component with custom styling for alerts
///
/// # Example
///
/// ```rust,no_run
/// use armas_basic::components::{Alert, AlertVariant};
///
/// fn ui(ui: &mut egui::Ui) {
///     // Default info alert
///     Alert::new("Operation completed").show(ui);
///
///     // Destructive alert
///     Alert::new("Something went wrong")
///         .variant(AlertVariant::Destructive)
///         .show(ui);
///
///     // Custom color alert
///     Alert::new("Custom alert")
///         .color(egui::Color32::from_rgb(100, 200, 150))
///         .show(ui);
/// }
/// ```
pub struct Alert {
    variant: AlertVariant,
    title: Option<String>,
    message: String,
    dismissible: bool,
    width: Option<f32>,
    show_icon: bool,
    custom_color: Option<Color32>,
}

impl Alert {
    /// Create a new alert
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            variant: AlertVariant::default(),
            title: None,
            message: message.into(),
            dismissible: false,
            width: None,
            show_icon: true,
            custom_color: None,
        }
    }

    /// Set the alert title
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the variant
    #[must_use]
    pub const fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Make this a destructive alert
    #[must_use]
    pub const fn destructive(mut self) -> Self {
        self.variant = AlertVariant::Destructive;
        self
    }

    /// Set custom color (overrides variant color)
    #[must_use]
    pub const fn color(mut self, color: Color32) -> Self {
        self.custom_color = Some(color);
        self
    }

    /// Make the alert dismissible
    #[must_use]
    pub const fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Set a fixed width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show or hide the icon
    #[must_use]
    pub const fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Show the alert using Card component
    ///
    /// Returns `AlertResponse` with information about user interaction
    pub fn show(self, ui: &mut Ui) -> AlertResponse {
        let theme = ui.ctx().armas_theme();
        let mut dismissed = false;
        let alert_id = ui.make_persistent_id("alert");

        let accent_color = self
            .custom_color
            .unwrap_or_else(|| self.variant.color(&theme));
        let bg_color = if self.custom_color.is_some() {
            Color32::from_rgba_unmultiplied(
                accent_color.r(),
                accent_color.g(),
                accent_color.b(),
                20,
            )
        } else {
            self.variant.background_color(&theme)
        };
        let border_color = if self.custom_color.is_some() {
            accent_color
        } else {
            self.variant.border_color(&theme)
        };

        // Build the Card with alert-specific styling (shadcn style)
        let mut card = Card::new()
            .variant(CardVariant::Outlined)
            .fill(bg_color)
            .stroke(border_color)
            .corner_radius(CORNER_RADIUS)
            .inner_margin(PADDING);

        if let Some(width) = self.width {
            card = card.width(width);
        }

        // Show the card with alert content
        card.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;

                // Icon
                if self.show_icon {
                    let icon_size = 16.0;
                    let (rect, _) =
                        ui.allocate_exact_size(vec2(icon_size, icon_size), Sense::hover());
                    match self.variant {
                        AlertVariant::Info => icon::draw_info(ui.painter(), rect, accent_color),
                        AlertVariant::Destructive => {
                            icon::draw_error(ui.painter(), rect, accent_color);
                        }
                    }
                }

                // Content
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 4.0;
                    if let Some(title) = &self.title {
                        ui.strong(title);
                    }

                    ui.label(&self.message);
                });

                // Spacer pushes close button to the right
                if self.dismissible {
                    ui.allocate_space(ui.available_size());

                    // Close button
                    let btn_size = 20.0;
                    let (close_rect, close_response) =
                        ui.allocate_exact_size(vec2(btn_size, btn_size), Sense::click());
                    if ui.is_rect_visible(close_rect) {
                        if close_response.hovered() {
                            ui.painter().rect_filled(close_rect, 4.0, theme.accent());
                        }
                        let icon_color = if close_response.hovered() {
                            theme.foreground()
                        } else {
                            theme.muted_foreground()
                        };
                        let icon_rect =
                            egui::Rect::from_center_size(close_rect.center(), vec2(12.0, 12.0));
                        icon::draw_close(ui.painter(), icon_rect, icon_color);
                    }

                    if close_response.clicked() {
                        dismissed = true;
                    }
                }
            });
        });

        let response = ui.interact(ui.min_rect(), alert_id.with("response"), Sense::hover());

        AlertResponse {
            response,
            dismissed,
        }
    }
}

/// Response from an alert
pub struct AlertResponse {
    /// The UI response
    pub response: egui::Response,
    /// Whether the alert was dismissed
    pub dismissed: bool,
}

/// Simple helper to show an alert with just a message
pub fn alert(ui: &mut Ui, message: impl Into<String>) {
    Alert::new(message).show(ui);
}

/// Show a destructive alert
pub fn alert_destructive(ui: &mut Ui, message: impl Into<String>) {
    Alert::new(message).destructive().show(ui);
}
