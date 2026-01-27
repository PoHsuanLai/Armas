//! Tooltip Component
//!
//! Contextual help tooltips styled like shadcn/ui Tooltip.
//! Appears on hover with configurable delay and position.

use crate::ext::ArmasContextExt;
use egui::{pos2, vec2, Color32, FontId, Rect, Response, Shape, Stroke, Ui, Vec2};

// shadcn Tooltip constants
const CORNER_RADIUS: f32 = 6.0; // rounded-md
const PADDING_X: f32 = 12.0; // px-3
const PADDING_Y: f32 = 6.0; // py-1.5
const FONT_SIZE: f32 = 12.0; // text-xs
const ARROW_SIZE: f32 = 5.0; // size-2.5 (10px / 2 for triangle)

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
}

/// Tooltip component that shows contextual help on hover
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::Tooltip;
///
/// let response = ui.button("Hover me");
/// Tooltip::new("This is helpful information").show(ui, &response);
/// # }
/// ```
pub struct Tooltip {
    text: String,
    position: Option<TooltipPosition>,
    max_width: f32,
    delay_ms: u64,
    show_arrow: bool,
}

impl Tooltip {
    /// Create a new tooltip
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position: None, // Auto by default
            max_width: 300.0,
            delay_ms: 0, // shadcn default: no delay
            show_arrow: true,
        }
    }

    /// Set the tooltip position (default: auto-detect)
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = Some(position);
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
    pub fn arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    /// Show tooltip for a UI element
    pub fn show(self, ui: &mut Ui, target_response: &Response) -> bool {
        let theme = ui.ctx().armas_theme();
        let is_hovered = target_response.hovered();

        // Use egui's memory to track hover start time
        let hover_id = target_response.id.with("tooltip_hover");
        let current_time = ui.ctx().input(|i| i.time);

        let hover_start: Option<f64> = ui.ctx().data(|d| d.get_temp(hover_id));

        if is_hovered {
            // Start tracking hover time
            if hover_start.is_none() {
                ui.ctx().data_mut(|d| d.insert_temp(hover_id, current_time));
                if self.delay_ms > 0 {
                    ui.ctx().request_repaint();
                    return false;
                }
            }

            // Check if delay has elapsed
            if let Some(start) = hover_start {
                let elapsed_ms = ((current_time - start) * 1000.0) as u64;
                if elapsed_ms < self.delay_ms {
                    ui.ctx().request_repaint();
                    return false;
                }
            }
        } else {
            // Clear hover state when not hovering
            ui.ctx().data_mut(|d| d.remove::<f64>(hover_id));
            return false;
        }

        // Calculate tooltip content size
        let font_id = FontId::proportional(FONT_SIZE);
        let padding = vec2(PADDING_X, PADDING_Y);

        // shadcn uses inverted colors: bg-foreground text-background
        let bg_color = theme.foreground();
        let text_color = theme.background();

        let text_galley = ui.painter().layout(
            self.text.clone(),
            font_id,
            text_color,
            self.max_width - padding.x * 2.0,
        );

        let text_size = text_galley.size();
        let tooltip_size = text_size + padding * 2.0;
        let arrow_offset = if self.show_arrow {
            ARROW_SIZE + 2.0
        } else {
            4.0
        };

        // Determine position
        let target_rect = target_response.rect;
        let position = self.determine_position(ui, target_rect, tooltip_size, arrow_offset);
        let tooltip_rect =
            self.calculate_tooltip_rect(target_rect, tooltip_size, arrow_offset, position);

        // Draw tooltip as an overlay (above everything else)
        let layer_id = egui::LayerId::new(
            egui::Order::Tooltip,
            target_response.id.with("tooltip_layer"),
        );
        let painter = ui.ctx().layer_painter(layer_id);

        // Background
        painter.rect_filled(tooltip_rect, CORNER_RADIUS, bg_color);

        // Arrow
        if self.show_arrow {
            self.draw_arrow(&painter, bg_color, target_rect, tooltip_rect, position);
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
        arrow_offset: f32,
    ) -> TooltipPosition {
        if let Some(pos) = self.position {
            return pos;
        }

        let screen_rect = ui.clip_rect();
        let spacing = arrow_offset;

        // Check available space in each direction
        let space_above = target_rect.top() - screen_rect.top();
        let space_below = screen_rect.bottom() - target_rect.bottom();
        let space_left = target_rect.left() - screen_rect.left();
        let space_right = screen_rect.right() - target_rect.right();

        let needed_vertical = tooltip_size.y + spacing;
        let needed_horizontal = tooltip_size.x + spacing;

        // Prefer top, then bottom, then right, then left
        if space_above >= needed_vertical {
            TooltipPosition::Top
        } else if space_below >= needed_vertical {
            TooltipPosition::Bottom
        } else if space_right >= needed_horizontal {
            TooltipPosition::Right
        } else if space_left >= needed_horizontal {
            TooltipPosition::Left
        } else {
            TooltipPosition::Top
        }
    }

    /// Calculate the tooltip rectangle based on position
    fn calculate_tooltip_rect(
        &self,
        target_rect: Rect,
        tooltip_size: Vec2,
        arrow_offset: f32,
        position: TooltipPosition,
    ) -> Rect {
        let target_center_x = target_rect.center().x;
        let target_center_y = target_rect.center().y;

        let min_pos = match position {
            TooltipPosition::Top => pos2(
                target_center_x - tooltip_size.x / 2.0,
                target_rect.top() - tooltip_size.y - arrow_offset,
            ),
            TooltipPosition::Bottom => pos2(
                target_center_x - tooltip_size.x / 2.0,
                target_rect.bottom() + arrow_offset,
            ),
            TooltipPosition::Left => pos2(
                target_rect.left() - tooltip_size.x - arrow_offset,
                target_center_y - tooltip_size.y / 2.0,
            ),
            TooltipPosition::Right => pos2(
                target_rect.right() + arrow_offset,
                target_center_y - tooltip_size.y / 2.0,
            ),
        };

        Rect::from_min_size(min_pos, tooltip_size)
    }

    /// Draw the arrow pointing to the target
    fn draw_arrow(
        &self,
        painter: &egui::Painter,
        bg_color: Color32,
        _target_rect: Rect,
        tooltip_rect: Rect,
        position: TooltipPosition,
    ) {
        let size = ARROW_SIZE;

        let (tip, base1, base2) = match position {
            TooltipPosition::Top => {
                // Arrow points down (at bottom of tooltip)
                let tip = pos2(tooltip_rect.center().x, tooltip_rect.bottom() + size);
                let base1 = pos2(tip.x - size, tooltip_rect.bottom());
                let base2 = pos2(tip.x + size, tooltip_rect.bottom());
                (tip, base1, base2)
            }
            TooltipPosition::Bottom => {
                // Arrow points up (at top of tooltip)
                let tip = pos2(tooltip_rect.center().x, tooltip_rect.top() - size);
                let base1 = pos2(tip.x - size, tooltip_rect.top());
                let base2 = pos2(tip.x + size, tooltip_rect.top());
                (tip, base1, base2)
            }
            TooltipPosition::Left => {
                // Arrow points right (at right of tooltip)
                let tip = pos2(tooltip_rect.right() + size, tooltip_rect.center().y);
                let base1 = pos2(tooltip_rect.right(), tip.y - size);
                let base2 = pos2(tooltip_rect.right(), tip.y + size);
                (tip, base1, base2)
            }
            TooltipPosition::Right => {
                // Arrow points left (at left of tooltip)
                let tip = pos2(tooltip_rect.left() - size, tooltip_rect.center().y);
                let base1 = pos2(tooltip_rect.left(), tip.y - size);
                let base2 = pos2(tooltip_rect.left(), tip.y + size);
                (tip, base1, base2)
            }
        };

        // Draw filled triangle
        painter.add(Shape::convex_polygon(
            vec![tip, base1, base2],
            bg_color,
            Stroke::NONE,
        ));
    }
}

/// Helper function to show a simple tooltip on any UI element
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::tooltip;
///
/// let response = ui.button("Hover me");
/// tooltip(ui, &response, "This is a tooltip!");
/// # }
/// ```
pub fn tooltip(ui: &mut Ui, response: &Response, text: impl Into<String>) {
    Tooltip::new(text).show(ui, response);
}

/// Show tooltip with custom configuration
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::{tooltip_with, TooltipPosition};
///
/// let response = ui.button("Hover me");
/// tooltip_with(ui, &response, "Custom tooltip", |t| {
///     t.position(TooltipPosition::Bottom).delay(500)
/// });
/// # }
/// ```
pub fn tooltip_with(
    ui: &mut Ui,
    response: &Response,
    text: impl Into<String>,
    configure: impl FnOnce(Tooltip) -> Tooltip,
) {
    configure(Tooltip::new(text)).show(ui, response);
}

// Keep these for backwards compatibility but mark as deprecated
#[doc(hidden)]
pub type TooltipStyle = ();
#[doc(hidden)]
pub type TooltipColor = ();
