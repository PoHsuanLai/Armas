//! Button component with shadcn/ui styling
//!
//! Provides variants following shadcn/ui conventions:
//! - Default: Primary background, high emphasis
//! - Secondary: Secondary background, medium emphasis
//! - Outline: Border with transparent background
//! - Ghost: No background, hover shows accent
//! - Link: Text style with underline on hover

use super::content::ContentContext;
use crate::ext::ArmasContextExt;
use crate::Theme;
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

    const fn font_size(self, typo: &crate::theme::Typography) -> f32 {
        match self {
            Self::Xs => typo.xs,
            _ => typo.base,
        }
    }
}

/// Get colors for a button variant and state.
///
/// Returns `(bg_color, text_color, border_color)`.
fn get_colors(
    theme: &Theme,
    variant: ButtonVariant,
    enabled: bool,
    hovered: bool,
) -> (Color32, Color32, Color32) {
    if enabled {
        match variant {
            ButtonVariant::Default => {
                let bg = if hovered {
                    theme.primary().gamma_multiply(0.9)
                } else {
                    theme.primary()
                };
                (bg, theme.primary_foreground(), Color32::TRANSPARENT)
            }
            ButtonVariant::Secondary => {
                let bg = if hovered {
                    theme.secondary().gamma_multiply(0.8)
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
            ButtonVariant::Link => (Color32::TRANSPARENT, theme.primary(), Color32::TRANSPARENT),
        }
    } else {
        (
            theme.primary().gamma_multiply(0.5),
            theme.primary_foreground().gamma_multiply(0.5),
            Color32::TRANSPARENT,
        )
    }
}

/// Draw button background and border. Returns whether the button is hovered.
fn draw_button_frame(
    ui: &mut Ui,
    rect: egui::Rect,
    theme: &Theme,
    variant: ButtonVariant,
    enabled: bool,
    hovered: bool,
) -> (Color32, Color32) {
    let (bg_color, text_color, border_color) = get_colors(theme, variant, enabled, hovered);

    if bg_color != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, CORNER_RADIUS, bg_color);
    }
    if border_color != Color32::TRANSPARENT {
        ui.painter().rect_stroke(
            rect,
            CORNER_RADIUS,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );
    }

    (bg_color, text_color)
}

/// Button component styled like shadcn/ui
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::components::{Button, ButtonVariant, ButtonSize};
///
/// if Button::new("Save")
///     .variant(ButtonVariant::Default)
///     .size(ButtonSize::Default)
///     .show(ui)
///     .clicked()
/// {
///     // handle click
/// }
/// # }
/// ```
pub struct Button {
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
    enabled: bool,
    full_width: bool,
    min_width: Option<f32>,
    custom_height: Option<f32>,
    content_width: Option<f32>,
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
            content_width: None,
        }
    }

    /// Create a button for use with [`show_content`](Self::show_content).
    ///
    /// The button has no text label; content is provided in the show call.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use egui::Ui;
    /// # fn example(ui: &mut Ui) {
    /// use armas_basic::components::{Button, ButtonVariant};
    ///
    /// Button::content()
    ///     .variant(ButtonVariant::Ghost)
    ///     .show_content(ui, |ui, ctx| {
    ///         ui.label("Custom content");
    ///     });
    /// # }
    /// ```
    #[must_use]
    pub const fn content() -> Self {
        Self {
            text: String::new(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            enabled: true,
            full_width: false,
            min_width: None,
            custom_height: None,
            content_width: None,
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

    /// Set explicit content area width for custom content buttons.
    ///
    /// When using [`show_content`](Self::show_content), this controls the inner width
    /// available for the closure. If not set, defaults to a square (height-based) layout.
    #[must_use]
    pub const fn content_width(mut self, width: f32) -> Self {
        self.content_width = Some(width);
        self
    }

    /// Show the button with a text label.
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let height = self.custom_height.unwrap_or_else(|| self.size.height());
        let padding_x = self.size.padding_x();

        // Measure text
        let font_id = egui::FontId::proportional(self.size.font_size(&theme.typography));
        let text_galley =
            ui.painter()
                .layout_no_wrap(self.text.clone(), font_id, Color32::PLACEHOLDER);
        let galley_size = text_galley.rect.size();
        let text_width = galley_size.x;

        let total_content_width = text_width + padding_x * 2.0;
        let button_width = if self.full_width {
            ui.available_width()
        } else if let Some(min_w) = self.min_width {
            total_content_width.max(min_w)
        } else {
            total_content_width
        };

        let button_size = Vec2::new(button_width, height);
        let (rect, mut response) = ui.allocate_exact_size(button_size, sense);

        if self.enabled && response.hovered() {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered() && self.enabled;
            let (_, text_color) =
                draw_button_frame(ui, rect, &theme, self.variant, self.enabled, hovered);

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

    /// Show the button with custom content instead of a text label.
    ///
    /// The closure receives a `&mut Ui` (with override text color set) and a
    /// [`ContentContext`] with the state-dependent color, font size, and active state.
    ///
    /// Use [`content_width`](Self::content_width) to control the inner width.
    /// If not set, the button defaults to a square layout (width = height).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use egui::Ui;
    /// # fn example(ui: &mut Ui) {
    /// use armas_basic::components::{Button, ButtonVariant};
    ///
    /// // Icon-only button (square)
    /// Button::content()
    ///     .variant(ButtonVariant::Ghost)
    ///     .show_content(ui, |ui, ctx| {
    ///         ui.label("X");
    ///     });
    ///
    /// // Icon + text button
    /// Button::content()
    ///     .content_width(80.0)
    ///     .show_content(ui, |ui, ctx| {
    ///         // render_icon(ui.painter(), my_icon, ctx.color);
    ///         ui.label("Save");
    ///     });
    /// # }
    /// ```
    pub fn show_content(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui, &ContentContext),
    ) -> Response {
        let theme = ui.ctx().armas_theme();
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };

        let height = self.custom_height.unwrap_or_else(|| self.size.height());
        let padding_x = self.size.padding_x();

        // Width: use content_width if set, otherwise square
        let inner_width = self.content_width.unwrap_or(height - padding_x * 2.0);
        let button_width = if self.full_width {
            ui.available_width()
        } else if let Some(min_w) = self.min_width {
            (inner_width + padding_x * 2.0).max(min_w)
        } else {
            inner_width + padding_x * 2.0
        };

        let button_size = Vec2::new(button_width, height);
        let (rect, mut response) = ui.allocate_exact_size(button_size, sense);

        if self.enabled && response.hovered() {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered() && self.enabled;
            let (_, text_color) =
                draw_button_frame(ui, rect, &theme, self.variant, self.enabled, hovered);

            // Create child UI for custom content
            let content_rect = rect.shrink2(Vec2::new(padding_x, 0.0));
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            child_ui.style_mut().visuals.override_text_color = Some(text_color);

            let ctx = ContentContext {
                color: text_color,
                font_size: self.size.font_size(&theme.typography),
                is_active: false,
            };
            content(&mut child_ui, &ctx);
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
