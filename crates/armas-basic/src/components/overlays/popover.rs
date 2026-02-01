//! Popover Component
//!
//! Floating panels anchored to elements.

use crate::{Card, CardVariant, Theme};
use egui::{pos2, vec2, Color32, Id, Pos2, Rect, Ui, Vec2};

// ============================================================================
// Constants
// ============================================================================

const MIN_SPACE_FOR_POSITION: f32 = 50.0;

// ============================================================================
// Enums
// ============================================================================

/// Popover position relative to anchor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverPosition {
    /// Above the anchor
    Top,
    /// Below the anchor
    #[default]
    Bottom,
    /// To the left of the anchor
    Left,
    /// To the right of the anchor
    Right,
    /// Automatically choose based on available space
    Auto,
}

/// Popover visual style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverStyle {
    /// Default style with soft border
    #[default]
    Default,
    /// Elevated style with stronger shadow
    Elevated,
    /// Bordered style with stronger border
    Bordered,
    /// Flat style with no shadow or border
    Flat,
}

/// Popover color themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverColor {
    /// Default surface color
    #[default]
    Surface,
    /// Primary theme color
    Primary,
    /// Success/positive (green)
    Success,
    /// Warning/caution (yellow)
    Warning,
    /// Error/danger (red)
    Error,
    /// Informational (blue)
    Info,
}

// ============================================================================
// Response
// ============================================================================

/// Response from showing a popover
#[derive(Debug, Clone, Copy, Default)]
pub struct PopoverResponse {
    /// Whether the user clicked outside the popover
    pub clicked_outside: bool,
    /// Whether the popover should be closed (for external state management)
    pub should_close: bool,
}

// ============================================================================
// Popover Component
// ============================================================================

/// Popover component for floating panels anchored to elements
#[derive(Clone)]
pub struct Popover {
    id: Id,
    position: PopoverPosition,
    style: PopoverStyle,
    color: PopoverColor,
    offset: Vec2,
    width: Option<f32>,
    max_width: f32,
    padding: Option<f32>,
    external_is_open: Option<bool>,
}

/// Style parameters for popover rendering
struct PopoverRenderStyle {
    bg_color: Color32,
    border_color: Color32,
    rounding: f32,
    padding: f32,
    card_variant: CardVariant,
}

impl Popover {
    /// Create a new popover with the given ID
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            position: PopoverPosition::default(),
            style: PopoverStyle::default(),
            color: PopoverColor::default(),
            offset: vec2(0.0, 8.0),
            width: None,
            max_width: 400.0,
            padding: None,
            external_is_open: None,
        }
    }

    /// Set the popover to be open (for external control)
    #[must_use]
    pub const fn open(mut self, is_open: bool) -> Self {
        self.external_is_open = Some(is_open);
        self
    }

    /// Set the open state (mutable version)
    pub const fn set_open(&mut self, is_open: bool) {
        self.external_is_open = Some(is_open);
    }

    /// Set the popover position relative to anchor
    #[must_use]
    pub const fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the popover visual style
    #[must_use]
    pub const fn style(mut self, style: PopoverStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the popover color theme
    #[must_use]
    pub const fn color(mut self, color: PopoverColor) -> Self {
        self.color = color;
        self
    }

    /// Set the offset from the anchor
    #[must_use]
    pub const fn offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    /// Set a fixed width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set maximum width
    #[must_use]
    pub const fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Set custom inner padding (overrides style default)
    #[must_use]
    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Show the popover anchored to the given rect
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        anchor_rect: Rect,
        content: impl FnOnce(&mut Ui),
    ) -> PopoverResponse {
        let mut response = PopoverResponse::default();

        // Check if should be open
        let is_open = self.external_is_open.unwrap_or(false);
        if !is_open {
            return response;
        }

        // Calculate position
        let position = self.determine_position(ctx, anchor_rect);
        let popover_pos = self.calculate_popover_position(anchor_rect, position);

        // Get styling
        let (bg_color, border_color) = self.get_colors(theme);
        let (stroke_width, rounding, padding) = self.get_style_params(theme);
        let card_variant = self.get_card_variant(stroke_width);

        let style = PopoverRenderStyle {
            bg_color,
            border_color,
            rounding,
            padding,
            card_variant,
        };

        // Render the popover
        let area_response = self.render_popover(ctx, theme, popover_pos, &style, content);

        // Handle click outside
        response = self.check_click_outside(ctx, &area_response.response.rect, anchor_rect);

        response
    }

    // ========================================================================
    // Position Calculation
    // ========================================================================

    fn determine_position(&self, ctx: &egui::Context, anchor_rect: Rect) -> PopoverPosition {
        if self.position != PopoverPosition::Auto {
            return self.position;
        }

        let screen_rect = ctx.available_rect();
        let space_above = anchor_rect.top() - screen_rect.top();
        let space_below = screen_rect.bottom() - anchor_rect.bottom();
        let space_left = anchor_rect.left() - screen_rect.left();
        let space_right = screen_rect.right() - anchor_rect.right();

        // Prefer bottom, then top, then sides
        if space_below >= MIN_SPACE_FOR_POSITION {
            PopoverPosition::Bottom
        } else if space_above >= MIN_SPACE_FOR_POSITION {
            PopoverPosition::Top
        } else if space_right >= MIN_SPACE_FOR_POSITION {
            PopoverPosition::Right
        } else if space_left >= MIN_SPACE_FOR_POSITION {
            PopoverPosition::Left
        } else {
            PopoverPosition::Bottom
        }
    }

    fn calculate_popover_position(&self, anchor_rect: Rect, position: PopoverPosition) -> Pos2 {
        let spacing = self.offset.length();
        let estimated_width = self.width.unwrap_or(self.max_width);

        match position {
            PopoverPosition::Top => pos2(
                anchor_rect.center().x - estimated_width / 2.0,
                anchor_rect.top() - spacing,
            ),
            PopoverPosition::Bottom => pos2(
                anchor_rect.center().x - estimated_width / 2.0,
                anchor_rect.bottom() + spacing,
            ),
            PopoverPosition::Left => pos2(
                anchor_rect.left() - estimated_width - spacing,
                anchor_rect.center().y,
            ),
            PopoverPosition::Right => pos2(anchor_rect.right() + spacing, anchor_rect.center().y),
            PopoverPosition::Auto => unreachable!(),
        }
    }

    // ========================================================================
    // Styling
    // ========================================================================

    fn get_colors(&self, theme: &Theme) -> (Color32, Color32) {
        match self.color {
            PopoverColor::Surface => (theme.card(), theme.border()),
            PopoverColor::Primary => blend_with_card(theme, theme.primary()),
            PopoverColor::Success => blend_with_card(theme, theme.chart_2()),
            PopoverColor::Warning => blend_with_card(theme, theme.chart_3()),
            PopoverColor::Error => blend_with_card(theme, theme.destructive()),
            PopoverColor::Info => blend_with_card(theme, theme.chart_4()),
        }
    }

    fn get_style_params(&self, theme: &Theme) -> (f32, f32, f32) {
        let (stroke_width, rounding, default_padding) = match self.style {
            PopoverStyle::Default => (
                1.0,
                f32::from(theme.spacing.corner_radius),
                theme.spacing.md,
            ),
            PopoverStyle::Elevated => (
                0.5,
                f32::from(theme.spacing.corner_radius_large),
                theme.spacing.lg,
            ),
            PopoverStyle::Bordered => (
                2.0,
                f32::from(theme.spacing.corner_radius_small),
                theme.spacing.md,
            ),
            PopoverStyle::Flat => (
                0.0,
                f32::from(theme.spacing.corner_radius_small),
                theme.spacing.md,
            ),
        };
        let padding = self.padding.unwrap_or(default_padding);
        (stroke_width, rounding, padding)
    }

    fn get_card_variant(&self, stroke_width: f32) -> CardVariant {
        match self.style {
            PopoverStyle::Elevated => CardVariant::Elevated,
            PopoverStyle::Bordered => CardVariant::Outlined,
            PopoverStyle::Flat => CardVariant::Filled,
            PopoverStyle::Default => {
                if stroke_width > 0.0 {
                    CardVariant::Outlined
                } else {
                    CardVariant::Filled
                }
            }
        }
    }

    // ========================================================================
    // Rendering
    // ========================================================================

    fn render_popover(
        &self,
        ctx: &egui::Context,
        theme: &Theme,
        popover_pos: Pos2,
        style: &PopoverRenderStyle,
        content: impl FnOnce(&mut Ui),
    ) -> egui::InnerResponse<()> {
        egui::Area::new(self.id)
            .order(egui::Order::Foreground)
            .fixed_pos(popover_pos)
            .show(ctx, |ui| {
                let content_width = self
                    .width
                    .unwrap_or_else(|| ui.available_width().min(self.max_width));

                ui.set_max_width(content_width);

                Card::new()
                    .variant(style.card_variant)
                    .fill(style.bg_color)
                    .stroke(style.border_color)
                    .corner_radius(style.rounding)
                    .inner_margin(style.padding)
                    .width(content_width)
                    .show(ui, theme, |ui| {
                        content(ui);
                    });
            })
    }

    fn check_click_outside(
        &self,
        ctx: &egui::Context,
        popover_rect: &Rect,
        anchor_rect: Rect,
    ) -> PopoverResponse {
        let mut response = PopoverResponse::default();

        if ctx.input(|i| i.pointer.any_click()) {
            if let Some(click_pos) = ctx.input(|i| i.pointer.interact_pos()) {
                if !popover_rect.contains(click_pos) && !anchor_rect.contains(click_pos) {
                    response.clicked_outside = true;
                    response.should_close = true;
                }
            }
        }

        response
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn blend_with_card(theme: &Theme, base: Color32) -> (Color32, Color32) {
    let blended = Color32::from_rgba_premultiplied(
        (f32::from(theme.card().r()) * 0.85 + f32::from(base.r()) * 0.15) as u8,
        (f32::from(theme.card().g()) * 0.85 + f32::from(base.g()) * 0.15) as u8,
        (f32::from(theme.card().b()) * 0.85 + f32::from(base.b()) * 0.15) as u8,
        255,
    );
    (blended, base)
}
