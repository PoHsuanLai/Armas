//! Alert Component
//!
//! Inline alert messages with variants and icons
//! Built on top of Card component and layout system for consistency

use crate::ext::ArmasContextExt;
use crate::layout::{HStack, Spacer, VStack};
use crate::{Badge, BadgeColor, Button, ButtonVariant, Card, Theme};
use egui::{vec2, Color32, Ui};

/// Alert variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertVariant {
    /// Informational alert (blue)
    Info,
    /// Success alert (green)
    Success,
    /// Warning alert (orange)
    Warning,
    /// Error alert (red)
    Error,
}

impl AlertVariant {
    fn badge_color(&self) -> BadgeColor {
        match self {
            AlertVariant::Info => BadgeColor::Primary,
            AlertVariant::Success => BadgeColor::Success,
            AlertVariant::Warning => BadgeColor::Warning,
            AlertVariant::Error => BadgeColor::Error,
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            AlertVariant::Info => "ℹ",
            AlertVariant::Success => "✓",
            AlertVariant::Warning => "⚠",
            AlertVariant::Error => "✕",
        }
    }

    fn background_color(&self, theme: &Theme) -> Color32 {
        match self {
            AlertVariant::Info => theme.primary().linear_multiply(0.08),
            AlertVariant::Success => theme.success().linear_multiply(0.08),
            AlertVariant::Warning => theme.warning().linear_multiply(0.08),
            AlertVariant::Error => theme.error().linear_multiply(0.08),
        }
    }

    fn border_color(&self, theme: &Theme) -> Color32 {
        match self {
            AlertVariant::Info => theme.primary(),
            AlertVariant::Success => theme.success(),
            AlertVariant::Warning => theme.warning(),
            AlertVariant::Error => theme.error(),
        }
    }
}

/// Alert component for inline messages
///
/// Built on top of the Card component with custom styling for alerts
pub struct Alert {
    variant: AlertVariant,
    title: Option<String>,
    message: String,
    dismissible: bool,
    width: Option<f32>,
    show_icon: bool,
}

impl Alert {
    /// Create a new alert
    pub fn new(message: impl Into<String>, variant: AlertVariant) -> Self {
        Self {
            variant,
            title: None,
            message: message.into(),
            dismissible: false,
            width: None,
            show_icon: true,
        }
    }

    /// Create an info alert
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message, AlertVariant::Info)
    }

    /// Create a success alert
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message, AlertVariant::Success)
    }

    /// Create a warning alert
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message, AlertVariant::Warning)
    }

    /// Create an error alert
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message, AlertVariant::Error)
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

        // Build the Card with alert-specific styling
        let mut card = Card::new()
            .fill(self.variant.background_color(&theme))
            .stroke(self.variant.border_color(&theme))
            .rounding(8.0)
            .inner_margin(12.0)
            .elevation(0); // No elevation shadow for alerts

        if let Some(width) = self.width {
            card = card.width(width);
        }

        // Show the card with alert content using layout system
        card.show(ui, &theme, |ui| {
            HStack::new(12.0).show(ui, |ui| {
                // Icon badge
                if self.show_icon {
                    Badge::new(self.variant.icon())
                        .color(self.variant.badge_color())
                        .show(ui);
                }

                // Content
                VStack::new(4.0).show(ui, |ui| {
                    if let Some(title) = &self.title {
                        ui.strong(title);
                    }

                    ui.label(&self.message);
                });

                // Spacer pushes close button to the right
                if self.dismissible {
                    Spacer::new().show(ui);

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
pub fn alert(ui: &mut Ui, message: impl Into<String>, variant: AlertVariant) {
    Alert::new(message, variant).show(ui);
}

/// Show an info alert
pub fn alert_info(ui: &mut Ui, message: impl Into<String>) {
    Alert::info(message).show(ui);
}

/// Show a success alert
pub fn alert_success(ui: &mut Ui, message: impl Into<String>) {
    Alert::success(message).show(ui);
}

/// Show a warning alert
pub fn alert_warning(ui: &mut Ui, message: impl Into<String>) {
    Alert::warning(message).show(ui);
}

/// Show an error alert
pub fn alert_error(ui: &mut Ui, message: impl Into<String>) {
    Alert::error(message).show(ui);
}
