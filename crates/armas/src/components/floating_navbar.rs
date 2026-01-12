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

/// Floating navbar component
///
/// A navbar that floats above content with smooth morphing background
/// that highlights the active item.
pub struct FloatingNavbar {
    id: egui::Id,
    items: Vec<NavItem>,
    position: NavbarPosition,
    width: Option<f32>,
    show_backdrop: bool,

    // Animation state
    active_position_animation: Animation<f32>,
    active_width_animation: Animation<f32>,
    current_active: Option<usize>,
}

impl FloatingNavbar {
    /// Create a new floating navbar
    pub fn new(items: Vec<NavItem>) -> Self {
        // Find initial active index
        let current_active = items.iter().position(|item| item.active);

        Self {
            id: egui::Id::new("floating_navbar"),
            items,
            position: NavbarPosition::Top,
            width: None,
            show_backdrop: false,
            active_position_animation: Animation::new(0.0, 0.0, 0.3)
                .with_easing(EasingFunction::EaseOut),
            active_width_animation: Animation::new(0.0, 0.0, 0.3)
                .with_easing(EasingFunction::EaseOut),
            current_active,
        }
    }

    /// Set a unique ID (required for multiple instances)
    pub fn with_id(mut self, id: impl std::hash::Hash) -> Self {
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
    pub fn with_backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    /// Show the floating navbar
    pub fn show(&mut self, ctx: &egui::Context) -> NavbarResponse {
        let theme = ctx.armas_theme();
        let dt = ctx.input(|i| i.stable_dt);

        // Update animations
        self.active_position_animation.update(dt);
        self.active_width_animation.update(dt);

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
                    let (rect, response) = ui.allocate_exact_size(
                        viewport_rect.size(),
                        Sense::click(),
                    );
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        Color32::from_black_alpha(180), // Semi-transparent dark overlay
                    );
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
                    let painter = ui.painter();

                    // Draw navbar background using Armas standard style
                    let surface = theme.surface_variant();
                    let bg_color = Color32::from_rgba_unmultiplied(surface.r(), surface.g(), surface.b(), 200);

                    painter.rect_filled(
                        rect,
                        theme.spacing.corner_radius_small,
                        bg_color,
                    );

                    // Border
                    let border_color = Color32::from_gray(80);
                    painter.rect_stroke(
                        rect,
                        theme.spacing.corner_radius_small,
                        egui::Stroke::new(1.0, border_color),
                        egui::StrokeKind::Outside,
                    );

                    // Draw close button (X) in top-right corner
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

                    // Draw close button background
                    let close_bg_color = if close_response.hovered() {
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

                    // Draw diagonal lines forming an X
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

                    if close_response.clicked() {
                        close_clicked = true;
                    }

                    // Calculate item positions and sizes
                    let content_width = navbar_width - (padding * 2.0);
                    let item_count = self.items.len();
                    let total_spacing = (item_count - 1) as f32 * item_spacing;
                    let item_width = (content_width - total_spacing) / item_count as f32;

                    // Draw active indicator background (morphing pill)
                    if let Some(_active_idx) = self.current_active {
                        let indicator_x = self.active_position_animation.value();
                        let indicator_width = self.active_width_animation.value();

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

                    // Draw items
                    for (index, item) in self.items.iter().enumerate() {
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

                        if item_response.hovered() {
                            hovered_index = Some(index);
                        }

                        if item_response.clicked() {
                            clicked_index = Some(index);

                            // Update active state and trigger animation
                            self.current_active = Some(index);
                            self.active_position_animation.start =
                                self.active_position_animation.value();
                            self.active_position_animation.end = item_x;
                            self.active_position_animation.elapsed = 0.0;
                            self.active_position_animation.start();

                            self.active_width_animation.start = self.active_width_animation.value();
                            self.active_width_animation.end = item_width;
                            self.active_width_animation.elapsed = 0.0;
                            self.active_width_animation.start();
                        }

                        // Draw item content
                        let text_color = if item.active {
                            theme.on_surface()
                        } else if item_response.hovered() {
                            theme.on_surface()
                        } else {
                            theme.on_surface_variant()
                        };

                        let mut text_y_offset = 0.0;

                        // Draw icon if present
                        if let Some(icon) = &item.icon {
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
                            &item.label,
                            egui::FontId::proportional(14.0),
                            text_color,
                        );
                    }

                    // Initialize animation on first frame if there's an active item
                    if self.active_position_animation.value() == 0.0
                        && self.current_active.is_some()
                    {
                        if let Some(active_idx) = self.current_active {
                            let item_x = padding + active_idx as f32 * (item_width + item_spacing);
                            self.active_position_animation.start = item_x;
                            self.active_position_animation.end = item_x;
                            self.active_width_animation.start = item_width;
                            self.active_width_animation.end = item_width;
                        }
                    }
                }

                // Request repaint if animations are running
                if !self.active_position_animation.is_complete()
                    || !self.active_width_animation.is_complete()
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

        let mut final_response = navbar_response.inner;
        final_response.backdrop_clicked = backdrop_clicked;
        final_response
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

    #[test]
    fn test_floating_navbar_creation() {
        let items = vec![
            NavItem::new("Home").active(true),
            NavItem::new("About"),
            NavItem::new("Contact"),
        ];
        let navbar = FloatingNavbar::new(items);
        assert_eq!(navbar.items.len(), 3);
        assert_eq!(navbar.current_active, Some(0));
        assert_eq!(navbar.position, NavbarPosition::Top);
    }

    #[test]
    fn test_navbar_position() {
        let items = vec![NavItem::new("Home")];
        let navbar = FloatingNavbar::new(items).position(NavbarPosition::Bottom);
        assert_eq!(navbar.position, NavbarPosition::Bottom);
    }
}
