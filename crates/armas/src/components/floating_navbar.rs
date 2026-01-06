//! Floating Navbar
//!
//! A navbar that floats above content with smooth morphing animations

use crate::animation::{Animation, EasingFunction};
use crate::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

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
    items: Vec<NavItem>,
    position: NavbarPosition,
    width: Option<f32>,

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
            items,
            position: NavbarPosition::Top,
            width: None,
            active_position_animation: Animation::new(0.0, 0.0, 0.3)
                .with_easing(EasingFunction::EaseOut),
            active_width_animation: Animation::new(0.0, 0.0, 0.3)
                .with_easing(EasingFunction::EaseOut),
            current_active,
        }
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

    /// Show the floating navbar
    pub fn show(&mut self, ctx: &egui::Context, theme: &Theme) -> NavbarResponse {
        let dt = ctx.input(|i| i.stable_dt);

        // Update animations
        self.active_position_animation.update(dt);
        self.active_width_animation.update(dt);

        let navbar_height = 60.0;
        let padding = 20.0;
        let item_spacing = 10.0;

        // Calculate navbar width
        let navbar_width = self.width.unwrap_or(800.0);

        // Position the navbar using egui::Area for floating behavior
        let area_id = egui::Id::new("floating_navbar");

        let navbar_response = egui::Area::new(area_id)
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

                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();

                    // Draw navbar background with blur effect
                    painter.rect(
                        rect,
                        16.0,
                        Color32::from_rgba_unmultiplied(20, 20, 25, 200),
                        egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 20)),
                        egui::StrokeKind::Outside,
                    );

                    // Calculate item positions and sizes
                    let content_width = navbar_width - (padding * 2.0);
                    let item_count = self.items.len();
                    let total_spacing = (item_count - 1) as f32 * item_spacing;
                    let item_width = (content_width - total_spacing) / item_count as f32;

                    // Draw active indicator background (morphing pill)
                    if let Some(active_idx) = self.current_active {
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
                            Color32::WHITE
                        } else if item_response.hovered() {
                            Color32::from_gray(220)
                        } else {
                            Color32::from_gray(160)
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
                }
            });

        navbar_response.inner
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
