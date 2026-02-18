//! Carousel Component (shadcn/ui style)
//!
//! A scrollable content strip with prev/next navigation and snap-to-item behavior.
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas_basic::prelude::*;
//!
//! let mut carousel = Carousel::new("demo");
//! carousel.show(ui, 5, |ui, index| {
//!     ui.label(format!("Slide {}", index + 1));
//! });
//! # }
//! ```

use crate::animation::SpringAnimation;
use egui::{vec2, Id, Pos2, Rect, Sense, Stroke, Ui};

// Constants
const BUTTON_SIZE: f32 = 32.0;
const BUTTON_RADIUS: f32 = 16.0;
const BUTTON_ICON_SIZE: f32 = 16.0;
const DEFAULT_GAP: f32 = 16.0;
const DEFAULT_HEIGHT: f32 = 200.0;

/// Carousel orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarouselOrientation {
    /// Horizontal scrolling (left/right).
    Horizontal,
    /// Vertical scrolling (up/down).
    Vertical,
}

/// Carousel — a scrollable content strip with snap-to-item behavior.
pub struct Carousel {
    id: Id,
    orientation: CarouselOrientation,
    loop_mode: bool,
    item_basis: f32,
    gap: f32,
    show_buttons: bool,
    height: f32,
}

/// Response from a carousel.
pub struct CarouselResponse {
    /// The UI response.
    pub response: egui::Response,
    /// The currently active (centered) item index.
    pub active_index: usize,
    /// Whether the active index changed this frame.
    pub changed: bool,
}

impl Carousel {
    /// Create a new carousel with a unique ID.
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            orientation: CarouselOrientation::Horizontal,
            loop_mode: false,
            item_basis: 1.0,
            gap: DEFAULT_GAP,
            show_buttons: true,
            height: DEFAULT_HEIGHT,
        }
    }

    /// Set the carousel orientation.
    #[must_use]
    pub const fn orientation(mut self, o: CarouselOrientation) -> Self {
        self.orientation = o;
        self
    }

    /// Enable loop mode (wraps around at ends).
    #[must_use]
    pub const fn loop_mode(mut self, l: bool) -> Self {
        self.loop_mode = l;
        self
    }

    /// Set the fraction of container width each item occupies (e.g. 0.33 = 3 visible items).
    #[must_use]
    pub const fn item_basis(mut self, basis: f32) -> Self {
        self.item_basis = basis;
        self
    }

    /// Set the gap between items in pixels.
    #[must_use]
    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Show or hide prev/next navigation buttons.
    #[must_use]
    pub const fn show_buttons(mut self, show: bool) -> Self {
        self.show_buttons = show;
        self
    }

    /// Set the carousel height in pixels.
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Show the carousel.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        item_count: usize,
        mut content: impl FnMut(&mut Ui, usize),
    ) -> CarouselResponse {
        let theme = crate::ext::ArmasContextExt::armas_theme(ui.ctx());

        if item_count == 0 {
            let (_, response) = ui.allocate_exact_size(vec2(0.0, 0.0), Sense::hover());
            return CarouselResponse {
                response,
                active_index: 0,
                changed: false,
            };
        }

        let is_horizontal = self.orientation == CarouselOrientation::Horizontal;
        let available_width = ui.available_width();
        let container_size = vec2(available_width, self.height);

        // Allocate container
        let (container_rect, container_response) =
            ui.allocate_exact_size(container_size, Sense::drag());

        // Load state
        let spring_id = self.id.with("spring");
        let index_id = self.id.with("index");

        let mut spring: SpringAnimation = ui.ctx().data_mut(|d| {
            d.get_temp(spring_id)
                .unwrap_or(SpringAnimation::new(0.0, 0.0))
        });
        let mut current_index: usize = ui.ctx().data_mut(|d| d.get_temp(index_id).unwrap_or(0));
        let prev_index = current_index;

        // Calculate item dimensions
        let main_extent = if is_horizontal {
            container_rect.width()
        } else {
            container_rect.height()
        };
        let item_extent = main_extent * self.item_basis - self.gap * (1.0 - self.item_basis);
        let step = item_extent + self.gap;
        let max_index = if item_count > 0 { item_count - 1 } else { 0 };

        // Handle drag
        if container_response.dragged() {
            let delta = if is_horizontal {
                container_response.drag_delta().x
            } else {
                container_response.drag_delta().y
            };
            spring.value -= delta;
            spring.velocity = 0.0;
        }

        // Snap on drag release
        if container_response.drag_stopped() {
            let raw_index = (spring.value / step).round().clamp(0.0, max_index as f32);
            current_index = raw_index as usize;
            spring.target = current_index as f32 * step;
        }

        // Handle button clicks
        let mut prev_clicked = false;
        let mut next_clicked = false;

        if self.show_buttons {
            let can_prev = self.loop_mode || current_index > 0;
            let can_next = self.loop_mode || current_index < max_index;

            if is_horizontal {
                // Left button
                if can_prev {
                    let btn_rect = Rect::from_center_size(
                        Pos2::new(
                            container_rect.left() + BUTTON_SIZE / 2.0 + 8.0,
                            container_rect.center().y,
                        ),
                        vec2(BUTTON_SIZE, BUTTON_SIZE),
                    );
                    prev_clicked = self.draw_nav_button(ui, &theme, btn_rect, true);
                }

                // Right button
                if can_next {
                    let btn_rect = Rect::from_center_size(
                        Pos2::new(
                            container_rect.right() - BUTTON_SIZE / 2.0 - 8.0,
                            container_rect.center().y,
                        ),
                        vec2(BUTTON_SIZE, BUTTON_SIZE),
                    );
                    next_clicked = self.draw_nav_button(ui, &theme, btn_rect, false);
                }
            } else {
                // Top button
                if can_prev {
                    let btn_rect = Rect::from_center_size(
                        Pos2::new(
                            container_rect.center().x,
                            container_rect.top() + BUTTON_SIZE / 2.0 + 8.0,
                        ),
                        vec2(BUTTON_SIZE, BUTTON_SIZE),
                    );
                    prev_clicked = self.draw_nav_button(ui, &theme, btn_rect, true);
                }

                // Bottom button
                if can_next {
                    let btn_rect = Rect::from_center_size(
                        Pos2::new(
                            container_rect.center().x,
                            container_rect.bottom() - BUTTON_SIZE / 2.0 - 8.0,
                        ),
                        vec2(BUTTON_SIZE, BUTTON_SIZE),
                    );
                    next_clicked = self.draw_nav_button(ui, &theme, btn_rect, false);
                }
            }
        }

        if prev_clicked {
            if current_index > 0 {
                current_index -= 1;
            } else if self.loop_mode {
                current_index = max_index;
            }
            spring.target = current_index as f32 * step;
        }

        if next_clicked {
            if current_index < max_index {
                current_index += 1;
            } else if self.loop_mode {
                current_index = 0;
            }
            spring.target = current_index as f32 * step;
        }

        // Update spring animation
        let dt = ui.ctx().input(|i| i.unstable_dt);
        spring.update(dt);

        if !spring.is_settled(0.5, 0.5) {
            ui.ctx().request_repaint();
        }

        // Draw items with clipping
        let clip_rect = container_rect;
        let _painter = ui.painter_at(clip_rect);

        for i in 0..item_count {
            let offset = i as f32 * step - spring.value;

            let item_rect = if is_horizontal {
                Rect::from_min_size(
                    Pos2::new(container_rect.left() + offset, container_rect.top()),
                    vec2(item_extent, container_rect.height()),
                )
            } else {
                Rect::from_min_size(
                    Pos2::new(container_rect.left(), container_rect.top() + offset),
                    vec2(container_rect.width(), item_extent),
                )
            };

            // Only render visible items
            if item_rect.intersects(clip_rect) {
                // Create a child UI for this item
                let mut child_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(item_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                child_ui.set_clip_rect(clip_rect);
                content(&mut child_ui, i);
            }
        }

        let changed = current_index != prev_index;

        // Save state
        ui.ctx().data_mut(|d| {
            d.insert_temp(spring_id, spring);
            d.insert_temp(index_id, current_index);
        });

        CarouselResponse {
            response: container_response,
            active_index: current_index,
            changed,
        }
    }

    fn draw_nav_button(
        &self,
        ui: &mut Ui,
        theme: &crate::Theme,
        rect: Rect,
        is_prev: bool,
    ) -> bool {
        let response = ui.interact(
            rect,
            self.id.with(if is_prev { "prev" } else { "next" }),
            Sense::click(),
        );
        let hovered = response.hovered();

        // Button background
        let bg = if hovered {
            theme.accent()
        } else {
            theme.background()
        };
        let fg = if hovered {
            theme.accent_foreground()
        } else {
            theme.foreground()
        };

        ui.painter().rect_filled(rect, BUTTON_RADIUS, bg);
        ui.painter().rect_stroke(
            rect,
            BUTTON_RADIUS,
            Stroke::new(1.0, theme.border()),
            egui::epaint::StrokeKind::Inside,
        );

        // Chevron icon
        let center = rect.center();
        let half = BUTTON_ICON_SIZE * 0.3;
        let is_horizontal = self.orientation == CarouselOrientation::Horizontal;

        if is_horizontal {
            if is_prev {
                // Left chevron: <
                let points = [
                    Pos2::new(center.x + half * 0.5, center.y - half),
                    Pos2::new(center.x - half * 0.5, center.y),
                    Pos2::new(center.x + half * 0.5, center.y + half),
                ];
                ui.painter()
                    .line_segment([points[0], points[1]], Stroke::new(1.5, fg));
                ui.painter()
                    .line_segment([points[1], points[2]], Stroke::new(1.5, fg));
            } else {
                // Right chevron: >
                let points = [
                    Pos2::new(center.x - half * 0.5, center.y - half),
                    Pos2::new(center.x + half * 0.5, center.y),
                    Pos2::new(center.x - half * 0.5, center.y + half),
                ];
                ui.painter()
                    .line_segment([points[0], points[1]], Stroke::new(1.5, fg));
                ui.painter()
                    .line_segment([points[1], points[2]], Stroke::new(1.5, fg));
            }
        } else if is_prev {
            // Up chevron: ^
            let points = [
                Pos2::new(center.x - half, center.y + half * 0.5),
                Pos2::new(center.x, center.y - half * 0.5),
                Pos2::new(center.x + half, center.y + half * 0.5),
            ];
            ui.painter()
                .line_segment([points[0], points[1]], Stroke::new(1.5, fg));
            ui.painter()
                .line_segment([points[1], points[2]], Stroke::new(1.5, fg));
        } else {
            // Down chevron: v
            let points = [
                Pos2::new(center.x - half, center.y - half * 0.5),
                Pos2::new(center.x, center.y + half * 0.5),
                Pos2::new(center.x + half, center.y - half * 0.5),
            ];
            ui.painter()
                .line_segment([points[0], points[1]], Stroke::new(1.5, fg));
            ui.painter()
                .line_segment([points[1], points[2]], Stroke::new(1.5, fg));
        }

        response.clicked()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carousel_creation() {
        let carousel = Carousel::new("test");
        assert_eq!(carousel.orientation, CarouselOrientation::Horizontal);
        assert!(!carousel.loop_mode);
        assert_eq!(carousel.item_basis, 1.0);
        assert!(carousel.show_buttons);
    }

    #[test]
    fn test_carousel_builder() {
        let carousel = Carousel::new("test")
            .orientation(CarouselOrientation::Vertical)
            .loop_mode(true)
            .item_basis(0.33)
            .gap(8.0)
            .show_buttons(false)
            .height(300.0);
        assert_eq!(carousel.orientation, CarouselOrientation::Vertical);
        assert!(carousel.loop_mode);
        assert_eq!(carousel.item_basis, 0.33);
        assert_eq!(carousel.gap, 8.0);
        assert!(!carousel.show_buttons);
        assert_eq!(carousel.height, 300.0);
    }
}
