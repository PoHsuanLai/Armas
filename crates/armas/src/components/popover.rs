//! Popover Component
//!
//! Floating panels anchored to elements with animations

use crate::{Animation, Card, EasingFunction, Theme};
use egui::{pos2, vec2, Id, Pos2, Rect, Sense, Stroke, Ui, Vec2};

/// Popover position relative to anchor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverPosition {
    /// Above the anchor
    Top,
    /// Below the anchor
    Bottom,
    /// To the left of the anchor
    Left,
    /// To the right of the anchor
    Right,
    /// Automatically choose based on space
    Auto,
}

/// Popover visual style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverStyle {
    /// Default style with soft border and shadow
    Default,
    /// Elevated style with stronger shadow
    Elevated,
    /// Bordered style with stronger border
    Bordered,
    /// Flat style with no shadow or border
    Flat,
}

/// Popover color themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverColor {
    /// Default surface color
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

/// Popover component
#[derive(Clone)]
pub struct Popover {
    id: Id,
    position: PopoverPosition,
    style: PopoverStyle,
    color: PopoverColor,
    offset: Vec2,
    width: Option<f32>,
    max_width: f32,
    show_arrow: bool,
    animation: Animation<f32>,
    is_open: bool,
    // Internal state management
    external_is_open: Option<bool>, // None = use internal state, Some = external control
}

impl Popover {
    /// Create a new popover
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            position: PopoverPosition::Bottom,
            style: PopoverStyle::Default,
            color: PopoverColor::Surface,
            offset: vec2(0.0, 12.0),
            width: None,
            max_width: 400.0,
            show_arrow: true,
            animation: Animation::new(0.0, 1.0, 0.2).with_easing(EasingFunction::CubicOut),
            is_open: false,
            external_is_open: None, // Use internal state by default
        }
    }

    /// Set the popover to be open (for external control)
    pub fn open(mut self, is_open: bool) -> Self {
        self.external_is_open = Some(is_open);
        self
    }

    /// Set the open state (mutable version for updating existing popovers)
    pub fn set_open(&mut self, is_open: bool) {
        self.external_is_open = Some(is_open);
    }

    /// Set the popover position
    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the popover style
    pub fn style(mut self, style: PopoverStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the popover color
    pub fn color(mut self, color: PopoverColor) -> Self {
        self.color = color;
        self
    }

    /// Set the offset from the anchor
    pub fn offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    /// Set a fixed width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set maximum width
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Show or hide the arrow
    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    /// Show the popover
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        anchor_rect: Rect,
        content: impl FnOnce(&mut Ui),
    ) -> PopoverResponse {
        let mut response = PopoverResponse {
            clicked_outside: false,
            should_close: false,
        };

        // Load state from egui memory if not externally controlled
        let state_id = self.id.with("popover_state");
        let mut is_open = if let Some(external_open) = self.external_is_open {
            external_open
        } else {
            ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false))
        };

        // Handle opening/closing
        if is_open && !self.is_open {
            // Opening
            self.animation.reset();
            self.animation.start();
            self.is_open = true;
        } else if !is_open && self.is_open {
            // Closing
            self.is_open = false;
        }

        if !self.is_open && !self.animation.is_running() {
            return response;
        }

        // Update animation
        if self.animation.is_running() {
            self.animation.update(ctx.input(|i| i.unstable_dt));
            ctx.request_repaint();
        }

        let progress = self.animation.value();

        // Don't show if fully closed
        if progress <= 0.0 {
            return response;
        }

        // Determine position
        let position = self.determine_position(ctx, anchor_rect);
        let arrow_size = if self.show_arrow { 8.0 } else { 0.0 };

        // Calculate popover rect (estimate size first)
        let estimated_size = vec2(
            self.width.unwrap_or(self.max_width),
            100.0, // Will be adjusted
        );

        let popover_pos =
            self.calculate_position(anchor_rect, estimated_size, arrow_size, position);

        // Draw backdrop to catch clicks outside - ONLY when popover is actually open
        if is_open {
            let backdrop_id = self.id.with("backdrop");
            let _backdrop_response = egui::Area::new(backdrop_id)
                .order(egui::Order::Middle)
                .interactable(true)
                .show(ctx, |ui| {
                    let screen_rect = ctx.viewport_rect();
                    let backdrop = ui.allocate_response(screen_rect.size(), Sense::click());

                    if backdrop.clicked() {
                        is_open = false;
                        response.clicked_outside = true;
                        response.should_close = true;
                    }

                    backdrop
                });
        }

        // Persist state if not externally controlled
        if self.external_is_open.is_none() {
            ctx.data_mut(|d| d.insert_temp(state_id, is_open));
        }

        // Get colors based on color theme
        let (bg_color, border_color) = match self.color {
            PopoverColor::Surface => (theme.surface(), theme.outline()),
            PopoverColor::Primary => {
                let base = theme.primary();
                (
                    egui::Color32::from_rgba_premultiplied(
                        (theme.surface().r() as f32 * 0.85 + base.r() as f32 * 0.15) as u8,
                        (theme.surface().g() as f32 * 0.85 + base.g() as f32 * 0.15) as u8,
                        (theme.surface().b() as f32 * 0.85 + base.b() as f32 * 0.15) as u8,
                        255,
                    ),
                    base,
                )
            }
            PopoverColor::Success => {
                let base = theme.success();
                (
                    egui::Color32::from_rgba_premultiplied(
                        (theme.surface().r() as f32 * 0.85 + base.r() as f32 * 0.15) as u8,
                        (theme.surface().g() as f32 * 0.85 + base.g() as f32 * 0.15) as u8,
                        (theme.surface().b() as f32 * 0.85 + base.b() as f32 * 0.15) as u8,
                        255,
                    ),
                    base,
                )
            }
            PopoverColor::Warning => {
                let base = theme.warning();
                (
                    egui::Color32::from_rgba_premultiplied(
                        (theme.surface().r() as f32 * 0.85 + base.r() as f32 * 0.15) as u8,
                        (theme.surface().g() as f32 * 0.85 + base.g() as f32 * 0.15) as u8,
                        (theme.surface().b() as f32 * 0.85 + base.b() as f32 * 0.15) as u8,
                        255,
                    ),
                    base,
                )
            }
            PopoverColor::Error => {
                let base = theme.error();
                (
                    egui::Color32::from_rgba_premultiplied(
                        (theme.surface().r() as f32 * 0.85 + base.r() as f32 * 0.15) as u8,
                        (theme.surface().g() as f32 * 0.85 + base.g() as f32 * 0.15) as u8,
                        (theme.surface().b() as f32 * 0.85 + base.b() as f32 * 0.15) as u8,
                        255,
                    ),
                    base,
                )
            }
            PopoverColor::Info => {
                let base = theme.info();
                (
                    egui::Color32::from_rgba_premultiplied(
                        (theme.surface().r() as f32 * 0.85 + base.r() as f32 * 0.15) as u8,
                        (theme.surface().g() as f32 * 0.85 + base.g() as f32 * 0.15) as u8,
                        (theme.surface().b() as f32 * 0.85 + base.b() as f32 * 0.15) as u8,
                        255,
                    ),
                    base,
                )
            }
        };

        // Get style parameters
        let (stroke_width, rounding, padding) = match self.style {
            PopoverStyle::Default => (1.0, 12.0, 16.0),
            PopoverStyle::Elevated => (0.5, 16.0, 20.0),
            PopoverStyle::Bordered => (2.0, 8.0, 16.0),
            PopoverStyle::Flat => (0.0, 8.0, 16.0),
        };

        // Draw popover content using Card
        egui::Area::new(self.id)
            .order(egui::Order::Foreground)
            .fixed_pos(popover_pos)
            .show(ctx, |ui| {
                // Apply animation opacity
                ui.set_opacity(progress);

                let content_width = self
                    .width
                    .unwrap_or_else(|| ui.available_width().min(self.max_width));

                ui.set_max_width(content_width);

                // Use Card component which properly handles colors
                Card::new()
                    .fill(bg_color)
                    .stroke(border_color)
                    .elevation(if stroke_width > 0.0 { 1 } else { 0 })
                    .corner_radius(rounding as f32)
                    .inner_margin(padding)
                    .width(content_width)
                    .show(ui, theme, |ui| {
                        content(ui);
                    });

                // Draw arrow
                if self.show_arrow && progress > 0.5 {
                    self.draw_arrow(ui, theme, anchor_rect, position, bg_color, border_color);
                }
            });

        response
    }

    fn determine_position(&self, ctx: &egui::Context, anchor_rect: Rect) -> PopoverPosition {
        if self.position != PopoverPosition::Auto {
            return self.position;
        }

        let screen_rect = ctx.content_rect();
        let spacing = 50.0; // Minimum space needed

        let space_above = anchor_rect.top() - screen_rect.top();
        let space_below = screen_rect.bottom() - anchor_rect.bottom();
        let space_left = anchor_rect.left() - screen_rect.left();
        let space_right = screen_rect.right() - anchor_rect.right();

        // Prefer bottom, then top, then sides
        if space_below >= spacing {
            PopoverPosition::Bottom
        } else if space_above >= spacing {
            PopoverPosition::Top
        } else if space_right >= spacing {
            PopoverPosition::Right
        } else if space_left >= spacing {
            PopoverPosition::Left
        } else {
            PopoverPosition::Bottom
        }
    }

    fn calculate_position(
        &self,
        anchor_rect: Rect,
        size: Vec2,
        arrow_size: f32,
        position: PopoverPosition,
    ) -> Pos2 {
        let spacing = arrow_size + self.offset.length();

        match position {
            PopoverPosition::Top => pos2(
                anchor_rect.center().x - size.x / 2.0,
                anchor_rect.top() - size.y - spacing,
            ),
            PopoverPosition::Bottom => pos2(
                anchor_rect.center().x - size.x / 2.0,
                anchor_rect.bottom() + spacing,
            ),
            PopoverPosition::Left => pos2(
                anchor_rect.left() - size.x - spacing,
                anchor_rect.center().y - size.y / 2.0,
            ),
            PopoverPosition::Right => pos2(
                anchor_rect.right() + spacing,
                anchor_rect.center().y - size.y / 2.0,
            ),
            PopoverPosition::Auto => unreachable!(),
        }
    }

    fn draw_arrow(
        &self,
        ui: &mut Ui,
        _theme: &Theme,
        _anchor_rect: Rect,
        position: PopoverPosition,
        bg_color: egui::Color32,
        border_color: egui::Color32,
    ) {
        let painter = ui.painter();
        let arrow_size = 8.0;

        let popover_rect = ui.min_rect();

        let (tip, base1, base2) = match position {
            PopoverPosition::Top => {
                let tip = pos2(popover_rect.center().x, popover_rect.bottom());
                let base1 = pos2(tip.x - arrow_size, tip.y - arrow_size);
                let base2 = pos2(tip.x + arrow_size, tip.y - arrow_size);
                (tip, base1, base2)
            }
            PopoverPosition::Bottom => {
                let tip = pos2(popover_rect.center().x, popover_rect.top());
                let base1 = pos2(tip.x - arrow_size, tip.y + arrow_size);
                let base2 = pos2(tip.x + arrow_size, tip.y + arrow_size);
                (tip, base1, base2)
            }
            PopoverPosition::Left => {
                let tip = pos2(popover_rect.right(), popover_rect.center().y);
                let base1 = pos2(tip.x - arrow_size, tip.y - arrow_size);
                let base2 = pos2(tip.x - arrow_size, tip.y + arrow_size);
                (tip, base1, base2)
            }
            PopoverPosition::Right => {
                let tip = pos2(popover_rect.left(), popover_rect.center().y);
                let base1 = pos2(tip.x + arrow_size, tip.y - arrow_size);
                let base2 = pos2(tip.x + arrow_size, tip.y + arrow_size);
                (tip, base1, base2)
            }
            PopoverPosition::Auto => unreachable!(),
        };

        // Fill
        painter.add(egui::Shape::convex_polygon(
            vec![tip, base1, base2],
            bg_color,
            Stroke::NONE,
        ));

        // Border (make it subtle like the main border)
        let subtle_border = border_color.linear_multiply(0.3);
        painter.line_segment([base1, tip], Stroke::new(1.0, subtle_border));
        painter.line_segment([tip, base2], Stroke::new(1.0, subtle_border));
    }
}

/// Response from a popover
#[derive(Debug, Clone, Copy)]
pub struct PopoverResponse {
    /// Whether the user clicked outside the popover
    pub clicked_outside: bool,
    /// Whether the popover should be closed (for external state management)
    pub should_close: bool,
}
