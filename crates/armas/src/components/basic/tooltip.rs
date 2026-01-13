//! Tooltip Component
//!
//! Contextual help tooltips that appear on hover

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{pos2, vec2, Color32, FontId, Rect, Response, Shape, Stroke, StrokeKind, Ui, Vec2};

/// Tooltip position relative to the target
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TooltipPosition {
    /// Above the target
    Top,
    /// Below the target
    Bottom,
    /// To the left of the target
    Left,
    /// To the right of the target
    Right,
    /// Automatically choose based on available space
    Auto,
}

/// Tooltip style variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipStyle {
    /// Default tooltip (simple, minimal)
    Default,
    /// Rich tooltip with elevated appearance
    Rich,
}

/// Tooltip color variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipColor {
    /// Surface color (default)
    Surface,
    /// Primary color
    Primary,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Error color
    Error,
    /// Info color
    Info,
}

impl TooltipColor {
    /// Get the background color from theme
    pub fn background_color(&self, theme: &Theme) -> Color32 {
        match self {
            TooltipColor::Surface => theme.surface_variant(),
            TooltipColor::Primary => theme.primary(),
            TooltipColor::Success => theme.success(),
            TooltipColor::Warning => theme.warning(),
            TooltipColor::Error => theme.error(),
            TooltipColor::Info => theme.info(),
        }
    }

    /// Get the text color from theme (ensures contrast)
    pub fn text_color(&self, theme: &Theme) -> Color32 {
        match self {
            TooltipColor::Surface => theme.on_surface(),
            _ => Color32::WHITE, // High contrast on colored backgrounds
        }
    }
}

/// Tooltip component that shows contextual help on hover
pub struct Tooltip {
    text: String,
    position: TooltipPosition,
    max_width: f32,
    delay_ms: u64,
    show_arrow: bool,
    hover_start: Option<f64>, // egui time in seconds
    style: TooltipStyle,
    color: TooltipColor,
}

impl Tooltip {
    /// Create a new tooltip
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position: TooltipPosition::Auto,
            max_width: 200.0,
            delay_ms: 500,
            show_arrow: true,
            hover_start: None,
            style: TooltipStyle::Default,
            color: TooltipColor::Surface,
        }
    }

    /// Set the tooltip position
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set maximum width for text wrapping
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Set hover delay in milliseconds before showing tooltip
    pub fn delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    /// Show or hide the arrow pointer
    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    /// Set the tooltip style
    pub fn style(mut self, style: TooltipStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the tooltip color variant
    pub fn color(mut self, color: TooltipColor) -> Self {
        self.color = color;
        self
    }

    /// Show tooltip for a UI element
    ///
    /// Returns true if the tooltip is currently visible
    pub fn show(&mut self, ui: &mut Ui, target_response: &Response) -> bool {
        let theme = ui.ctx().armas_theme();
        let is_hovered = target_response.hovered();
        let current_time = ui.ctx().input(|i| i.time);

        // Track hover state
        if is_hovered {
            if self.hover_start.is_none() {
                self.hover_start = Some(current_time);
            }
        } else {
            self.hover_start = None;
            return false;
        }

        // Check if delay has elapsed
        let hover_start = self.hover_start.unwrap();
        let elapsed_ms = ((current_time - hover_start) * 1000.0) as u64;
        if elapsed_ms < self.delay_ms {
            ui.ctx().request_repaint();
            return false;
        }

        // Calculate tooltip content size
        let (font_id, padding) = match self.style {
            TooltipStyle::Default => (FontId::proportional(13.0), vec2(8.0, 6.0)),
            TooltipStyle::Rich => (FontId::proportional(14.0), vec2(12.0, 8.0)),
        };

        let text_color = self.color.text_color(&theme);
        let text_galley = ui.painter().layout(
            self.text.clone(),
            font_id,
            text_color,
            self.max_width - padding.x * 2.0,
        );

        let text_size = text_galley.size();
        let tooltip_size = text_size + padding * 2.0;
        let arrow_size = if self.show_arrow { 6.0 } else { 0.0 };

        // Determine position
        let target_rect = target_response.rect;
        let position = self.determine_position(ui, target_rect, tooltip_size, arrow_size);
        let tooltip_rect =
            self.calculate_tooltip_rect(target_rect, tooltip_size, arrow_size, position);

        // Draw tooltip as an overlay (above everything else)
        let layer_id = egui::LayerId::new(egui::Order::Tooltip, ui.id().with("tooltip"));
        let painter = ui.ctx().layer_painter(layer_id);

        // Get colors based on color variant
        let bg_color = self.color.background_color(&theme);
        let border_color = match self.color {
            TooltipColor::Surface => theme.outline().linear_multiply(0.3),
            _ => bg_color.linear_multiply(1.2), // Slightly lighter border for colored tooltips
        };

        // Styling based on style variant
        let (rounding, border_width) = match self.style {
            TooltipStyle::Default => (4.0, 1.0),
            TooltipStyle::Rich => (6.0, 2.0),
        };

        // Background
        painter.rect_filled(tooltip_rect, rounding, bg_color);

        // Border
        painter.rect_stroke(
            tooltip_rect,
            rounding,
            Stroke::new(border_width, border_color),
            StrokeKind::Outside,
        );

        // Arrow
        if self.show_arrow {
            self.draw_arrow(
                &painter,
                bg_color,
                border_color,
                target_rect,
                tooltip_rect,
                position,
            );
        }

        // Text
        painter.galley(tooltip_rect.min + padding, text_galley, text_color);

        true
    }

    /// Determine the best position for the tooltip
    fn determine_position(
        &self,
        ui: &Ui,
        target_rect: Rect,
        tooltip_size: Vec2,
        arrow_size: f32,
    ) -> TooltipPosition {
        if self.position != TooltipPosition::Auto {
            return self.position;
        }

        let screen_rect = ui.ctx().content_rect();
        let spacing = 8.0 + arrow_size;

        // Check available space in each direction
        let space_above = target_rect.top() - screen_rect.top();
        let space_below = screen_rect.bottom() - target_rect.bottom();
        let space_left = target_rect.left() - screen_rect.left();
        let space_right = screen_rect.right() - target_rect.right();

        let needed_vertical = tooltip_size.y + spacing;
        let needed_horizontal = tooltip_size.x + spacing;

        // Prefer top/bottom over left/right
        if space_below >= needed_vertical {
            TooltipPosition::Bottom
        } else if space_above >= needed_vertical {
            TooltipPosition::Top
        } else if space_right >= needed_horizontal {
            TooltipPosition::Right
        } else if space_left >= needed_horizontal {
            TooltipPosition::Left
        } else {
            // Default to bottom if no space is sufficient
            TooltipPosition::Bottom
        }
    }

    /// Calculate the tooltip rectangle based on position
    fn calculate_tooltip_rect(
        &self,
        target_rect: Rect,
        tooltip_size: Vec2,
        arrow_size: f32,
        position: TooltipPosition,
    ) -> Rect {
        let spacing = 8.0 + arrow_size;
        let target_center_x = target_rect.center().x;
        let target_center_y = target_rect.center().y;

        let min_pos = match position {
            TooltipPosition::Top => pos2(
                target_center_x - tooltip_size.x / 2.0,
                target_rect.top() - tooltip_size.y - spacing,
            ),
            TooltipPosition::Bottom => pos2(
                target_center_x - tooltip_size.x / 2.0,
                target_rect.bottom() + spacing,
            ),
            TooltipPosition::Left => pos2(
                target_rect.left() - tooltip_size.x - spacing,
                target_center_y - tooltip_size.y / 2.0,
            ),
            TooltipPosition::Right => pos2(
                target_rect.right() + spacing,
                target_center_y - tooltip_size.y / 2.0,
            ),
            TooltipPosition::Auto => unreachable!("Auto should be resolved before this"),
        };

        Rect::from_min_size(min_pos, tooltip_size)
    }

    /// Draw the arrow pointing to the target
    fn draw_arrow(
        &self,
        painter: &egui::Painter,
        bg_color: Color32,
        border_color: Color32,
        _target_rect: Rect,
        tooltip_rect: Rect,
        position: TooltipPosition,
    ) {
        let arrow_size = 6.0;

        let (tip, base1, base2) = match position {
            TooltipPosition::Top => {
                // Arrow points down
                let tip = pos2(tooltip_rect.center().x, tooltip_rect.bottom());
                let base1 = pos2(tip.x - arrow_size, tip.y - arrow_size);
                let base2 = pos2(tip.x + arrow_size, tip.y - arrow_size);
                (tip, base1, base2)
            }
            TooltipPosition::Bottom => {
                // Arrow points up
                let tip = pos2(tooltip_rect.center().x, tooltip_rect.top());
                let base1 = pos2(tip.x - arrow_size, tip.y + arrow_size);
                let base2 = pos2(tip.x + arrow_size, tip.y + arrow_size);
                (tip, base1, base2)
            }
            TooltipPosition::Left => {
                // Arrow points right
                let tip = pos2(tooltip_rect.right(), tooltip_rect.center().y);
                let base1 = pos2(tip.x - arrow_size, tip.y - arrow_size);
                let base2 = pos2(tip.x - arrow_size, tip.y + arrow_size);
                (tip, base1, base2)
            }
            TooltipPosition::Right => {
                // Arrow points left
                let tip = pos2(tooltip_rect.left(), tooltip_rect.center().y);
                let base1 = pos2(tip.x + arrow_size, tip.y - arrow_size);
                let base2 = pos2(tip.x + arrow_size, tip.y + arrow_size);
                (tip, base1, base2)
            }
            TooltipPosition::Auto => unreachable!(),
        };

        // Draw filled triangle
        painter.add(Shape::convex_polygon(
            vec![tip, base1, base2],
            bg_color,
            Stroke::NONE,
        ));

        // Draw border on two sides of the triangle
        painter.line_segment([base1, tip], Stroke::new(1.0, border_color));
        painter.line_segment([tip, base2], Stroke::new(1.0, border_color));
    }
}

/// Helper function to show a simple tooltip on any UI element
///
/// # Example
/// ```ignore
/// let response = ui.button("Hover me");
/// tooltip(ui, theme, &response, "This is a tooltip!");
/// ```
pub fn tooltip(ui: &mut Ui, _theme: &Theme, response: &Response, text: impl Into<String>) {
    let mut tooltip = Tooltip::new(text);
    tooltip.show(ui, response);
}

/// Show tooltip with custom configuration
pub fn tooltip_with(
    ui: &mut Ui,
    _theme: &Theme,
    response: &Response,
    text: impl Into<String>,
    configure: impl FnOnce(Tooltip) -> Tooltip,
) {
    let mut tooltip = configure(Tooltip::new(text));
    tooltip.show(ui, response);
}
