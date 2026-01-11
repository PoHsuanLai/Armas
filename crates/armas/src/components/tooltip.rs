//! Tooltip Component
//!
//! Contextual help tooltips that appear on hover

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{pos2, vec2, FontId, Rect, Response, Shape, Stroke, StrokeKind, Ui, Vec2};
use std::time::Instant;

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

/// Tooltip component that shows contextual help on hover
pub struct Tooltip {
    text: String,
    position: TooltipPosition,
    max_width: f32,
    delay_ms: u64,
    show_arrow: bool,
    hover_start: Option<Instant>,
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

    /// Show tooltip for a UI element
    ///
    /// Returns true if the tooltip is currently visible
    pub fn show(&mut self, ui: &mut Ui, target_response: &Response) -> bool {
        let theme = ui.ctx().armas_theme();
        let is_hovered = target_response.hovered();

        // Track hover state
        if is_hovered {
            if self.hover_start.is_none() {
                self.hover_start = Some(Instant::now());
            }
        } else {
            self.hover_start = None;
            return false;
        }

        // Check if delay has elapsed
        let hover_start = self.hover_start.unwrap();
        let elapsed = hover_start.elapsed().as_millis() as u64;
        if elapsed < self.delay_ms {
            ui.ctx().request_repaint();
            return false;
        }

        // Calculate tooltip content size
        let font_id = FontId::proportional(13.0);
        let text_galley = ui.painter().layout(
            self.text.clone(),
            font_id,
            theme.on_surface(),
            self.max_width - 16.0, // Account for padding
        );

        let text_size = text_galley.size();
        let padding = vec2(8.0, 6.0);
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

        // Background
        painter.rect_filled(tooltip_rect, 4.0, theme.surface_variant());

        // Border
        painter.rect_stroke(
            tooltip_rect,
            4.0,
            Stroke::new(1.0, theme.outline().linear_multiply(0.3)),
            StrokeKind::Outside,
        );

        // Arrow
        if self.show_arrow {
            self.draw_arrow(&painter, &theme, target_rect, tooltip_rect, position);
        }

        // Text
        painter.galley(tooltip_rect.min + padding, text_galley, theme.on_surface());

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

        let screen_rect = ui.ctx().screen_rect();
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
        theme: &Theme,
        _target_rect: Rect,
        tooltip_rect: Rect,
        position: TooltipPosition,
    ) {
        let arrow_size = 6.0;
        let color = theme.surface_variant();
        let border_color = theme.outline().linear_multiply(0.3);

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
            color,
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
pub fn tooltip(ui: &mut Ui, theme: &Theme, response: &Response, text: impl Into<String>) {
    let mut tooltip = Tooltip::new(text);
    tooltip.show(ui, response);
}

/// Show tooltip with custom configuration
pub fn tooltip_with(
    ui: &mut Ui,
    theme: &Theme,
    response: &Response,
    text: impl Into<String>,
    configure: impl FnOnce(Tooltip) -> Tooltip,
) {
    let mut tooltip = configure(Tooltip::new(text));
    tooltip.show(ui, response);
}
