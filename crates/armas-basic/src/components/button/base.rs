//! Button component with shadcn/ui styling
//!
//! Provides variants following shadcn/ui conventions:
//! - Default: Primary background, high emphasis
//! - Secondary: Secondary background, medium emphasis
//! - Outline: Border with transparent background
//! - Ghost: No background, hover shows accent
//! - Link: Text style with underline on hover

use egui::{Color32, Response, Sense, Ui, Vec2};

// shadcn Button constants
const CORNER_RADIUS: f32 = 6.0; // rounded-md

/// Button style variant following shadcn/ui
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Default button - primary background, highest emphasis
    #[default]
    Default,
    /// Secondary button - secondary background, medium emphasis
    Secondary,
    /// Outline button - border with transparent background
    Outline,
    /// Ghost button - no background, hover shows accent
    Ghost,
    /// Link button - text style with underline on hover
    Link,
}

/// Button size following shadcn/ui
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    /// Extra small - h-5.5 (22px), px-1.5
    Xs,
    /// Small - h-8 (32px), px-3
    Small,
    /// Default - h-9 (36px), px-4
    #[default]
    Default,
    /// Large - h-10 (40px), px-6
    Large,
}

impl ButtonSize {
    const fn height(self) -> f32 {
        match self {
            Self::Xs => 22.0,
            Self::Small => 32.0,
            Self::Default => 36.0,
            Self::Large => 40.0,
        }
    }

    const fn padding_x(self) -> f32 {
        match self {
            Self::Xs => 6.0,
            Self::Small => 12.0,   // px-3
            Self::Default => 16.0, // px-4
            Self::Large => 24.0,   // px-6
        }
    }

    const fn font_size(self) -> f32 {
        match self {
            Self::Xs => 11.0,
            _ => 14.0, // text-sm
        }
    }
}

/// Button component styled like shadcn/ui
pub struct Button {
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
    enabled: bool,
    full_width: bool,
    min_width: Option<f32>,
    custom_height: Option<f32>,
}

impl Button {
    /// Create a new button with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            enabled: true,
            full_width: false,
            min_width: None,
            custom_height: None,
        }
    }

    /// Set the button variant
    #[must_use]
    pub const fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button size
    #[must_use]
    pub const fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set enabled state
    #[must_use]
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Make button take full width of container
    #[must_use]
    pub const fn full_width(mut self, full: bool) -> Self {
        self.full_width = full;
        self
    }

    /// Set minimum width for the button
    #[must_use]
    pub const fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set explicit height (overrides size-based height)
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.custom_height = Some(height);
        self
    }

    /// Show the button
    pub fn show(self, ui: &mut Ui, theme: &crate::Theme) -> Response {
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        // Calculate size
        let height = self.custom_height.unwrap_or_else(|| self.size.height());
        let padding_x = self.size.padding_x();

        // Measure text
        let font_id = egui::FontId::proportional(self.size.font_size());
        let text_galley =
            ui.painter()
                .layout_no_wrap(self.text.clone(), font_id, Color32::PLACEHOLDER);
        let galley_size = text_galley.rect.size();
        let text_width = galley_size.x;

        let content_width = text_width + padding_x * 2.0;
        let button_width = if self.full_width {
            ui.available_width()
        } else if let Some(min_w) = self.min_width {
            content_width.max(min_w)
        } else {
            content_width
        };

        let button_size = Vec2::new(button_width, height);
        let (rect, mut response) = ui.allocate_exact_size(button_size, sense);

        if self.enabled && response.hovered() {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered() && self.enabled;

            // Get colors based on variant and state
            let (bg_color, text_color, border_color) = if self.enabled {
                match self.variant {
                    ButtonVariant::Default => {
                        let bg = if hovered {
                            theme.primary().gamma_multiply(0.9) // hover:bg-primary/90
                        } else {
                            theme.primary()
                        };
                        (bg, theme.primary_foreground(), Color32::TRANSPARENT)
                    }
                    ButtonVariant::Secondary => {
                        let bg = if hovered {
                            theme.secondary().gamma_multiply(0.8) // hover:bg-secondary/80
                        } else {
                            theme.secondary()
                        };
                        (bg, theme.secondary_foreground(), Color32::TRANSPARENT)
                    }
                    ButtonVariant::Outline => {
                        let bg = if hovered {
                            theme.accent()
                        } else {
                            Color32::TRANSPARENT
                        };
                        let text = if hovered {
                            theme.accent_foreground()
                        } else {
                            theme.foreground()
                        };
                        (bg, text, theme.border())
                    }
                    ButtonVariant::Ghost => {
                        let bg = if hovered {
                            theme.accent()
                        } else {
                            Color32::TRANSPARENT
                        };
                        let text = if hovered {
                            theme.accent_foreground()
                        } else {
                            theme.foreground()
                        };
                        (bg, text, Color32::TRANSPARENT)
                    }
                    ButtonVariant::Link => {
                        (Color32::TRANSPARENT, theme.primary(), Color32::TRANSPARENT)
                    }
                }
            } else {
                // Disabled: opacity-50
                (
                    theme.primary().gamma_multiply(0.5),
                    theme.primary_foreground().gamma_multiply(0.5),
                    Color32::TRANSPARENT,
                )
            };

            // Draw background
            if bg_color != Color32::TRANSPARENT {
                ui.painter().rect_filled(rect, CORNER_RADIUS, bg_color);
            }

            // Draw border for outline variant
            if border_color != Color32::TRANSPARENT {
                ui.painter().rect_stroke(
                    rect,
                    CORNER_RADIUS,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Inside,
                );
            }

            // Draw text
            let text_pos = rect.center() - galley_size / 2.0;
            ui.painter()
                .galley(egui::pos2(text_pos.x, text_pos.y), text_galley, text_color);

            // Draw underline for Link variant on hover
            if self.variant == ButtonVariant::Link && hovered {
                let underline_y = text_pos.y + galley_size.y + 1.0;
                ui.painter().line_segment(
                    [
                        egui::pos2(text_pos.x, underline_y),
                        egui::pos2(text_pos.x + galley_size.x, underline_y),
                    ],
                    egui::Stroke::new(1.0, text_color),
                );
            }
        }

        response
    }
}

// Keep old variant name as alias for backwards compatibility during migration
pub use ButtonVariant as Variant;

// Aliases for old variant names (deprecated, will remove later)
#[allow(non_upper_case_globals)]
impl ButtonVariant {
    /// Alias for Default (was Filled)
    pub const Filled: Self = Self::Default;
    /// Alias for Outline (was Outlined)
    pub const Outlined: Self = Self::Outline;
    /// Alias for Ghost (was Text)
    pub const Text: Self = Self::Ghost;
    /// Alias for Secondary (was `FilledTonal`)
    pub const FilledTonal: Self = Self::Secondary;
    /// Elevated is now Secondary
    pub const Elevated: Self = Self::Secondary;
}
