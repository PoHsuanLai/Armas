//! Popover Component
//!
//! Floating panels anchored to elements with animations

use crate::ext::ArmasContextExt;
use crate::animation::{Animation, EasingFunction};
use crate::context::ArmasContextExt;
use crate::traits::ArmasModifiers;
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

/// Popover component
pub struct Popover {
    id: Id,
    position: PopoverPosition,
    offset: Vec2,
    width: Option<f32>,
    max_width: f32,
    show_arrow: bool,
    animation: Animation<f32>,
    is_open: bool,
}

impl Popover {
    /// Create a new popover
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            position: PopoverPosition::Bottom,
            offset: vec2(0.0, 8.0),
            width: None,
            max_width: 300.0,
            show_arrow: true,
            animation: Animation::new(0.0, 1.0, 0.15).easing(EasingFunction::CubicOut),
            is_open: false,
        }
    }

    /// Set the popover position
    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
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

    /// Bind popover with state and content for modifier usage
    pub fn bind<'a, F>(
        &'a mut self,
        anchor_rect: Rect,
        is_open: &'a mut bool,
        content: F,
    ) -> PopoverBound<'a, F>
    where
        F: FnOnce(&mut Ui),
    {
        PopoverBound {
            popover: self,
            anchor_rect,
            is_open,
            content: Some(content),
        }
    }

    /// Show the popover
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        anchor_rect: Rect,
        is_open: &mut bool,
        content: impl FnOnce(&mut Ui),
    ) -> PopoverResponse {
        let theme = ctx.armas_theme();
        let mut clicked_outside = false;

        // Handle opening/closing
        if *is_open && !self.is_open {
            // Opening
            self.animation.reset();
            self.animation.start();
            self.is_open = true;
        } else if !*is_open && self.is_open {
            // Closing
            self.is_open = false;
        }

        if !self.is_open && !self.animation.is_running() {
            // Create a dummy response for when popover is closed
            let dummy_response = egui::Area::new(self.id.with("not_open"))
                .order(egui::Order::Background)
                .show(ctx, |ui| {
                    ui.allocate_response(vec2(0.0, 0.0), Sense::hover())
                })
                .inner;
            return PopoverResponse {
                response: dummy_response,
                clicked_outside: false,
            };
        }

        // Update animation
        if self.animation.is_running() {
            self.animation.update(ctx.input(|i| i.unstable_dt));
            ctx.request_repaint();
        }

        let progress = self.animation.value();

        // Don't show if fully closed
        if progress <= 0.0 {
            let dummy_response = egui::Area::new(self.id.with("closed"))
                .order(egui::Order::Background)
                .show(ctx, |ui| {
                    ui.allocate_response(vec2(0.0, 0.0), Sense::hover())
                })
                .inner;
            return PopoverResponse {
                response: dummy_response,
                clicked_outside: false,
            };
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

        // Draw backdrop to catch clicks outside
        let backdrop_id = self.id.with("backdrop");
        let backdrop_response = egui::Area::new(backdrop_id)
            .order(egui::Order::Middle)
            .interactable(true)
            .show(ctx, |ui| {
                let screen_rect = ctx.content_rect();
                let backdrop = ui.allocate_response(screen_rect.size(), Sense::click());

                if backdrop.clicked() && *is_open {
                    *is_open = false;
                    clicked_outside = true;
                }

                backdrop
            })
            .inner;

        // Draw popover content
        egui::Area::new(self.id)
            .order(egui::Order::Foreground)
            .fixed_pos(popover_pos)
            .show(ctx, |ui| {
                // Apply animation: scale and opacity
                let scale = 0.95 + (progress * 0.05);
                ui.set_opacity(progress);

                let content_width = self
                    .width
                    .unwrap_or_else(|| ui.available_width().min(self.max_width));

                ui.set_width(content_width);

                // Background frame
                let frame = egui::Frame::NONE
                    .fill(theme.surface())
                    .stroke(Stroke::new(1.0, theme.outline().linear_multiply(0.3)))
                    .shadow(egui::Shadow {
                        offset: [0, 4],
                        blur: 12,
                        spread: 0,
                        color: egui::Color32::from_black_alpha(180),
                    })
                    .corner_radius(8.0)
                    .inner_margin(12.0);

                frame.show(ui, |ui| {
                    // Scale the content slightly
                    let content_rect = ui.available_rect_before_wrap();
                    let center = content_rect.center();
                    let scaled_rect = Rect::from_center_size(center, content_rect.size() * scale);

                    ui.allocate_ui_at_rect(scaled_rect, |ui| {
                        content(ui);
                    });
                });

                // Draw arrow
                if self.show_arrow && progress > 0.5 {
                    self.draw_arrow(ui, anchor_rect, position);
                }
            });

        PopoverResponse {
            response: backdrop_response,
            clicked_outside,
        }
    }

    fn determine_position(&self, ctx: &egui::Context, anchor_rect: Rect) -> PopoverPosition {
        if self.position != PopoverPosition::Auto {
            return self.position;
        }

        let screen_rect = ctx.screen_rect();
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

    fn draw_arrow(&self, ui: &mut Ui, anchor_rect: Rect, position: PopoverPosition) {
        let theme = ui.ctx().armas_theme();
        let painter = ui.painter();
        let arrow_size = 8.0;
        let color = theme.surface();
        let border_color = theme.outline().linear_multiply(0.3);

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
            color,
            Stroke::NONE,
        ));

        // Border
        painter.line_segment([base1, tip], Stroke::new(1.0, border_color));
        painter.line_segment([tip, base2], Stroke::new(1.0, border_color));
    }
}

impl ArmasModifiers for Popover {}

/// Response from a popover
#[derive(Debug, Clone)]
pub struct PopoverResponse {
    /// The underlying egui response
    pub response: egui::Response,
    /// Whether the user clicked outside the popover
    pub clicked_outside: bool,
}

impl std::ops::Deref for PopoverResponse {
    type Target = egui::Response;
    fn deref(&self) -> &egui::Response {
        &self.response
    }
}

/// Bound wrapper for Popover to enable modifier usage
pub struct PopoverBound<'a, F> {
    popover: &'a mut Popover,
    anchor_rect: Rect,
    is_open: &'a mut bool,
    content: Option<F>,
}

impl<'a, F> crate::traits::ArmasViewMut for PopoverBound<'a, F>
where
    F: FnOnce(&mut Ui),
{
    type Output = PopoverResponse;

    fn ui(&mut self, ui: &mut Ui) -> PopoverResponse {
        let content = self.content.take().expect("PopoverBound.ui() called more than once");
        self.popover.show(ui.ctx(), self.anchor_rect, self.is_open, content)
    }
}

impl<'a, F> ArmasModifiers for PopoverBound<'a, F> {}
