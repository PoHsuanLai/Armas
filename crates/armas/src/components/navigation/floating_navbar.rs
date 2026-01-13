//! Floating Navbar
//!
//! A navbar that floats above content with smooth morphing animations

use crate::animation::{Animation, EasingFunction};
use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Vec2};

/// Individual navigation item
#[derive(Clone)]
pub struct NavItem {
    /// Item label
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether this item is active
    pub active: bool,
}

impl NavItem {
    /// Create a new nav item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            active: false,
        }
    }

    /// Set the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set active state
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

/// Response from navbar interaction
pub struct NavbarResponse {
    /// The overall navbar response
    pub response: Response,
    /// Index of the clicked item, if any
    pub clicked: Option<usize>,
    /// Index of the hovered item, if any
    pub hovered: Option<usize>,
    /// Whether the close button was clicked
    pub close_clicked: bool,
    /// Whether the backdrop was clicked
    pub backdrop_clicked: bool,
}

/// Position of the floating navbar
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavbarPosition {
    /// Top of the screen
    Top,
    /// Bottom of the screen
    Bottom,
}

/// Internal state for navbar animations
#[derive(Clone)]
struct NavbarState {
    active_position_animation: Animation<f32>,
    active_width_animation: Animation<f32>,
    current_active: Option<usize>,
    item_count: usize,
}

/// Floating navbar component
///
/// A navbar that floats above content with smooth morphing background
/// that highlights the active item.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Context;
/// # fn example(ctx: &Context) {
/// use armas::{FloatingNavbar, NavbarPosition};
///
/// FloatingNavbar::new()
///     .backdrop(true)
///     .position(NavbarPosition::Top)
///     .show(ctx, |navbar| {
///         navbar.item("Home", Some("🏠")).active(true);
///         navbar.item("About", Some("ℹ️"));
///         navbar.item("Contact", Some("📧"));
///     });
/// # }
/// ```
pub struct FloatingNavbar {
    id: egui::Id,
    position: NavbarPosition,
    width: Option<f32>,
    show_backdrop: bool,
}

impl FloatingNavbar {
    /// Create a new floating navbar
    pub fn new() -> Self {
        Self {
            id: egui::Id::new("floating_navbar"),
            position: NavbarPosition::Top,
            width: None,
            show_backdrop: false,
        }
    }

    /// Set a unique ID (required for multiple instances)
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = egui::Id::new(id);
        self
    }

    /// Set the navbar position
    pub fn position(mut self, position: NavbarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set a fixed width (otherwise uses available width)
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show a darkened backdrop behind the navbar
    pub fn backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    /// Show the floating navbar with closure-based API
    pub fn show<R>(
        self,
        ctx: &egui::Context,
        content: impl FnOnce(&mut FloatingNavbarBuilder) -> R,
    ) -> NavbarResponse {
        let theme = ctx.armas_theme();

        // Load or initialize animation state from memory
        let state_id = self.id.with("navbar_state");
        let mut state: NavbarState = ctx.data_mut(|d| {
            d.get_temp(state_id).unwrap_or(NavbarState {
                active_position_animation: Animation::new(0.0, 0.0, 0.3)
                    .easing(EasingFunction::EaseOut),
                active_width_animation: Animation::new(0.0, 0.0, 0.3)
                    .easing(EasingFunction::EaseOut),
                current_active: None,
                item_count: 0,
            })
        });

        let dt = ctx.input(|i| i.stable_dt);

        // Update animations
        state.active_position_animation.update(dt);
        state.active_width_animation.update(dt);

        let navbar_height = 60.0;
        let padding = 20.0;
        let item_spacing = 10.0;

        // Calculate navbar width
        let navbar_width = self.width.unwrap_or(800.0);

        // Draw backdrop if enabled and handle backdrop clicks
        let mut backdrop_clicked = false;
        if self.show_backdrop {
            let backdrop_id = self.id.with("backdrop");
            let backdrop_response = egui::Area::new(backdrop_id)
                .fixed_pos(Pos2::ZERO)
                .order(egui::Order::Background)
                .interactable(true)
                .show(ctx, |ui| {
                    let viewport_rect = ctx.viewport_rect();
                    let (rect, response) =
                        ui.allocate_exact_size(viewport_rect.size(), Sense::click());
                    ui.painter()
                        .rect_filled(rect, 0.0, Color32::from_black_alpha(180));
                    response
                });

            if backdrop_response.inner.clicked() {
                backdrop_clicked = true;
            }
        }

        // Position the navbar using egui::Area for floating behavior
        let navbar_response = egui::Area::new(self.id)
            .fixed_pos(match self.position {
                NavbarPosition::Top => {
                    Pos2::new((ctx.viewport_rect().width() - navbar_width) / 2.0, 20.0)
                }
                NavbarPosition::Bottom => Pos2::new(
                    (ctx.viewport_rect().width() - navbar_width) / 2.0,
                    ctx.viewport_rect().height() - navbar_height - 20.0,
                ),
            })
            .order(egui::Order::Foreground)
            .interactable(true)
            .show(ctx, |ui| {
                let (rect, response) =
                    ui.allocate_exact_size(Vec2::new(navbar_width, navbar_height), Sense::hover());

                let mut clicked_index: Option<usize> = None;
                let mut hovered_index: Option<usize> = None;
                let mut close_clicked = false;

                if ui.is_rect_visible(rect) {
                    // Draw close button (X) in top-right corner - do interaction first
                    let close_button_size = 32.0;
                    let close_button_rect = Rect::from_min_size(
                        Pos2::new(rect.right() - close_button_size - 8.0, rect.top() + 14.0),
                        Vec2::new(close_button_size, close_button_size),
                    );

                    let close_response = ui.interact(
                        close_button_rect,
                        ui.id().with("close_button"),
                        Sense::click().union(Sense::hover()),
                    );

                    let close_hovered = close_response.hovered();
                    if close_response.clicked() {
                        close_clicked = true;
                    }

                    // Build items using closure - do this before getting painter
                    let mut builder = FloatingNavbarBuilder {
                        ui,
                        rect,
                        padding,
                        item_spacing,
                        navbar_width,
                        navbar_height,
                        item_index: 0,
                        items_data: Vec::new(),
                        state: &mut state,
                    };

                    content(&mut builder);

                    // Calculate item positions and sizes
                    let item_count = builder.items_data.len();
                    let content_width = navbar_width - (padding * 2.0);
                    let total_spacing = (item_count - 1) as f32 * item_spacing;
                    let item_width = (content_width - total_spacing) / item_count as f32;

                    // Collect item interaction data first (before getting painter)
                    let mut item_interactions = Vec::new();
                    for (index, item_data) in builder.items_data.iter().enumerate() {
                        let item_x = padding + index as f32 * (item_width + item_spacing);
                        let item_pos = Pos2::new(rect.left() + item_x, rect.top());
                        let item_rect =
                            Rect::from_min_size(item_pos, Vec2::new(item_width, navbar_height));

                        // Item interaction
                        let item_response = ui.interact(
                            item_rect,
                            ui.id().with(index),
                            Sense::click().union(Sense::hover()),
                        );

                        let is_hovered = item_response.hovered();
                        let is_clicked = item_response.clicked();

                        if is_hovered {
                            hovered_index = Some(index);
                        }

                        if is_clicked {
                            clicked_index = Some(index);

                            // Update active state and trigger animation
                            state.current_active = Some(index);
                            state.active_position_animation.start =
                                state.active_position_animation.value();
                            state.active_position_animation.end = item_x;
                            state.active_position_animation.elapsed = 0.0;
                            state.active_position_animation.start();

                            state.active_width_animation.start =
                                state.active_width_animation.value();
                            state.active_width_animation.end = item_width;
                            state.active_width_animation.elapsed = 0.0;
                            state.active_width_animation.start();
                        }

                        item_interactions.push((item_rect, is_hovered, item_data.clone()));
                    }

                    // Now get painter for all drawing operations
                    let painter = ui.painter();

                    // Draw navbar background
                    let surface = theme.surface_variant();
                    let bg_color =
                        Color32::from_rgba_unmultiplied(surface.r(), surface.g(), surface.b(), 200);

                    painter.rect_filled(rect, theme.spacing.corner_radius_small, bg_color);

                    // Border
                    painter.rect_stroke(
                        rect,
                        theme.spacing.corner_radius_small,
                        egui::Stroke::new(1.0, theme.outline_variant()),
                        egui::StrokeKind::Outside,
                    );

                    // Draw close button background
                    let close_bg_color = if close_hovered {
                        let error = theme.error();
                        Color32::from_rgba_unmultiplied(error.r(), error.g(), error.b(), 150)
                    } else {
                        let hover = theme.hover();
                        Color32::from_rgba_unmultiplied(hover.r(), hover.g(), hover.b(), 100)
                    };

                    painter.rect_filled(
                        close_button_rect,
                        theme.spacing.corner_radius_small,
                        close_bg_color,
                    );

                    // Draw X icon with two lines
                    let line_length = 10.0;
                    let center = close_button_rect.center();
                    let offset = line_length / 2.0;

                    painter.line_segment(
                        [
                            Pos2::new(center.x - offset, center.y - offset),
                            Pos2::new(center.x + offset, center.y + offset),
                        ],
                        egui::Stroke::new(2.0, theme.on_surface()),
                    );
                    painter.line_segment(
                        [
                            Pos2::new(center.x + offset, center.y - offset),
                            Pos2::new(center.x - offset, center.y + offset),
                        ],
                        egui::Stroke::new(2.0, theme.on_surface()),
                    );

                    // Draw active indicator background (morphing pill)
                    if state.current_active.is_some() {
                        let indicator_x = state.active_position_animation.value();
                        let indicator_width = state.active_width_animation.value();

                        let indicator_rect = Rect::from_min_size(
                            Pos2::new(rect.left() + padding + indicator_x, rect.top() + 10.0),
                            Vec2::new(indicator_width, navbar_height - 20.0),
                        );

                        painter.rect(
                            indicator_rect,
                            12.0,
                            theme.primary(),
                            egui::Stroke::NONE,
                            egui::StrokeKind::Outside,
                        );
                    }

                    // Draw items using collected interaction data
                    for (item_rect, is_hovered, item_data) in item_interactions {
                        let text_color = if item_data.is_active || is_hovered {
                            theme.on_surface()
                        } else {
                            theme.on_surface_variant()
                        };

                        let mut text_y_offset = 0.0;

                        // Draw icon if present
                        if let Some(icon) = &item_data.icon {
                            let icon_pos = Pos2::new(item_rect.center().x, item_rect.top() + 15.0);
                            painter.text(
                                icon_pos,
                                egui::Align2::CENTER_TOP,
                                icon,
                                egui::FontId::proportional(20.0),
                                text_color,
                            );
                            text_y_offset = 25.0;
                        }

                        // Draw label
                        let label_pos =
                            Pos2::new(item_rect.center().x, item_rect.top() + 15.0 + text_y_offset);
                        painter.text(
                            label_pos,
                            egui::Align2::CENTER_TOP,
                            &item_data.label,
                            egui::FontId::proportional(14.0),
                            text_color,
                        );
                    }

                    // Initialize animation on first frame if there's an active item
                    if state.active_position_animation.value() == 0.0
                        && state.current_active.is_some()
                    {
                        if let Some(active_idx) = state.current_active {
                            let item_x = padding + active_idx as f32 * (item_width + item_spacing);
                            state.active_position_animation.start = item_x;
                            state.active_position_animation.end = item_x;
                            state.active_width_animation.start = item_width;
                            state.active_width_animation.end = item_width;
                        }
                    }

                    // Update item count in state
                    state.item_count = item_count;
                }

                // Request repaint if animations are running
                if !state.active_position_animation.is_complete()
                    || !state.active_width_animation.is_complete()
                {
                    ui.ctx().request_repaint();
                }

                NavbarResponse {
                    response,
                    clicked: clicked_index,
                    hovered: hovered_index,
                    close_clicked,
                    backdrop_clicked: false,
                }
            });

        // Save state back to memory
        ctx.data_mut(|d| d.insert_temp(state_id, state));

        let mut final_response = navbar_response.inner;
        final_response.backdrop_clicked = backdrop_clicked;
        final_response
    }
}

impl Default for FloatingNavbar {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding navbar items
pub struct FloatingNavbarBuilder<'a> {
    #[allow(dead_code)]
    ui: &'a mut egui::Ui,
    #[allow(dead_code)]
    rect: Rect,
    #[allow(dead_code)]
    padding: f32,
    #[allow(dead_code)]
    item_spacing: f32,
    #[allow(dead_code)]
    navbar_width: f32,
    #[allow(dead_code)]
    navbar_height: f32,
    item_index: usize,
    items_data: Vec<ItemData>,
    state: &'a mut NavbarState,
}

#[derive(Clone)]
struct ItemData {
    label: String,
    icon: Option<String>,
    is_active: bool,
}

impl<'a> FloatingNavbarBuilder<'a> {
    /// Add a navbar item
    pub fn item(&mut self, label: &str, icon: Option<&str>) -> ItemBuilder<'_> {
        let item_data = ItemData {
            label: label.to_string(),
            icon: icon.map(|s| s.to_string()),
            is_active: false,
        };

        self.items_data.push(item_data);
        let current_index = self.item_index;
        self.item_index += 1;

        ItemBuilder {
            items_data: &mut self.items_data,
            item_index: current_index,
            state: self.state,
        }
    }
}

/// Builder for chaining item modifiers
pub struct ItemBuilder<'a> {
    items_data: &'a mut Vec<ItemData>,
    item_index: usize,
    state: &'a mut NavbarState,
}

impl<'a> ItemBuilder<'a> {
    /// Mark this item as active
    pub fn active(self, active: bool) -> Self {
        if let Some(item) = self.items_data.get_mut(self.item_index) {
            item.is_active = active;
            if active {
                self.state.current_active = Some(self.item_index);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nav_item_creation() {
        let item = NavItem::new("Home");
        assert_eq!(item.label, "Home");
        assert!(item.icon.is_none());
        assert!(!item.active);
    }

    #[test]
    fn test_nav_item_with_icon() {
        let item = NavItem::new("Home").icon("🏠");
        assert_eq!(item.icon, Some("🏠".to_string()));
    }
}
