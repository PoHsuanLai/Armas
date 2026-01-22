//! Alert Component
//!
//! Inline alert messages with variants and icons
//! Built on top of Card component and layout system for consistency

use crate::ext::ArmasContextExt;
use crate::{Badge, Button, ButtonVariant, Card, CardVariant, Theme};
use egui::{vec2, Color32, Ui};

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
    fn icon(&self) -> &'static str {
        match self {
            AlertVariant::Info => "ℹ",
            AlertVariant::Destructive => "✕",
        }
    }

    fn color(&self, theme: &Theme) -> Color32 {
        match self {
            AlertVariant::Info => theme.foreground(),
            AlertVariant::Destructive => theme.destructive(),
        }
    }

    fn background_color(&self, theme: &Theme) -> Color32 {
        match self {
            AlertVariant::Info => theme.muted(),
            AlertVariant::Destructive => theme.destructive().linear_multiply(0.08),
        }
    }

    fn border_color(&self, theme: &Theme) -> Color32 {
        match self {
            AlertVariant::Info => theme.border(),
            AlertVariant::Destructive => theme.destructive(),
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
/// use armas::components::{Alert, AlertVariant};
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
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Make this a destructive alert
    pub fn destructive(mut self) -> Self {
        self.variant = AlertVariant::Destructive;
        self
    }

    /// Set custom color (overrides variant color)
    pub fn color(mut self, color: Color32) -> Self {
        self.custom_color = Some(color);
        self
    }

    /// Make the alert dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Set a fixed width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show or hide the icon
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Show the alert using Card component
    ///
    /// Returns `AlertResponse` with information about user interaction
    pub fn show(self, ui: &mut Ui) -> AlertResponse {
        let theme = ui.ctx().armas_theme();
        let mut dismissed = false;

        let accent_color = self.custom_color.unwrap_or_else(|| self.variant.color(&theme));
        let bg_color = if self.custom_color.is_some() {
            Color32::from_rgba_unmultiplied(accent_color.r(), accent_color.g(), accent_color.b(), 20)
        } else {
            self.variant.background_color(&theme)
        };
        let border_color = if self.custom_color.is_some() {
            accent_color
        } else {
            self.variant.border_color(&theme)
        };

        // Build the Card with alert-specific styling
        let mut card = Card::new()
            .variant(CardVariant::Outlined)
            .fill(bg_color)
            .stroke(border_color)
            .corner_radius(8.0)
            .inner_margin(12.0);

        if let Some(width) = self.width {
            card = card.width(width);
        }

        // Show the card with alert content
        card.show(ui, &theme, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;
                // Icon badge
                if self.show_icon {
                    Badge::new(self.variant.icon())
                        .color(accent_color)
                        .show(ui);
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
                    if Button::new("✕")
                        .variant(ButtonVariant::Text)
                        .min_size(vec2(24.0, 24.0))
                        .show(ui)
                        .clicked()
                    {
                        dismissed = true;
                    }
                }
            });
        });

        AlertResponse { dismissed }
    }
}

/// Response from an alert
#[derive(Debug, Clone, Copy)]
pub struct AlertResponse {
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
