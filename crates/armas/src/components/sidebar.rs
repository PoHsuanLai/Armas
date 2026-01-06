//! Sidebar
//!
//! Animated sidebar with icons and smooth expand/collapse

use crate::animation::{Animation, EasingFunction};
use crate::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Individual sidebar item
#[derive(Clone)]
pub struct SidebarItem {
    /// Item label
    pub label: String,
    /// Icon
    pub icon: String,
    /// Whether this item is active
    pub active: bool,
    /// Optional badge text (e.g., notification count)
    pub badge: Option<String>,
}

impl SidebarItem {
    /// Create a new sidebar item
    pub fn new(icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
            active: false,
            badge: None,
        }
    }

    /// Set active state
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set badge text
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}

/// Response from sidebar interaction
pub struct SidebarResponse {
    /// The overall sidebar response
    pub response: Response,
    /// Index of the clicked item, if any
    pub clicked: Option<usize>,
    /// Index of the hovered item, if any
    pub hovered: Option<usize>,
    /// Whether the sidebar is currently expanded
    pub is_expanded: bool,
}

/// Animated sidebar component
///
/// A sidebar that can expand to show labels or collapse to show only icons.
/// Includes smooth animations for expand/collapse and item highlighting.
pub struct Sidebar {
    items: Vec<SidebarItem>,
    is_expanded: bool,
    collapsed_width: f32,
    expanded_width: f32,

    // Animation state
    width_animation: Animation<f32>,
    active_position_animation: Animation<f32>,
    current_active: Option<usize>,
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new(items: Vec<SidebarItem>) -> Self {
        // Find initial active index
        let current_active = items.iter().position(|item| item.active);

        Self {
            items,
            is_expanded: true,
            collapsed_width: 70.0,
            expanded_width: 240.0,
            width_animation: Animation::new(240.0, 240.0, 0.3).with_easing(EasingFunction::EaseOut),
            active_position_animation: Animation::new(0.0, 0.0, 0.3)
                .with_easing(EasingFunction::EaseOut),
            current_active,
        }
    }

    /// Set whether the sidebar starts expanded
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        let target_width = if expanded {
            self.expanded_width
        } else {
            self.collapsed_width
        };
        self.width_animation.start = target_width;
        self.width_animation.end = target_width;
        self
    }

    /// Set the collapsed width
    pub fn collapsed_width(mut self, width: f32) -> Self {
        self.collapsed_width = width;
        self
    }

    /// Set the expanded width
    pub fn expanded_width(mut self, width: f32) -> Self {
        self.expanded_width = width;
        self
    }

    /// Toggle the sidebar expansion
    pub fn toggle(&mut self) {
        self.is_expanded = !self.is_expanded;
        self.width_animation.start = self.width_animation.value();
        self.width_animation.end = if self.is_expanded {
            self.expanded_width
        } else {
            self.collapsed_width
        };
        self.width_animation.elapsed = 0.0;
        self.width_animation.start();
    }

    /// Show the sidebar
    pub fn show(&mut self, ui: &mut Ui, theme: &Theme) -> SidebarResponse {
        let dt = ui.input(|i| i.stable_dt);

        // Update animations
        self.width_animation.update(dt);
        self.active_position_animation.update(dt);

        let current_width = self.width_animation.value();
        let item_height = 56.0;
        let padding = 12.0;
        let icon_size = 24.0;

        let sidebar_height = ui.available_height();

        // Get the rect we'll draw into
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::new(current_width, sidebar_height));

        // Manually reserve space (but don't consume interactions)
        ui.advance_cursor_after_rect(rect);

        let mut clicked_index: Option<usize> = None;
        let mut hovered_index: Option<usize> = None;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw sidebar background
            painter.rect(
                rect,
                0.0,
                Color32::from_rgb(18, 18, 22),
                egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 10)),
                egui::StrokeKind::Outside,
            );

            // Draw toggle button at top
            let toggle_rect = Rect::from_min_size(
                Pos2::new(rect.left() + padding, rect.top() + padding),
                Vec2::new(current_width - padding * 2.0, item_height),
            );

            let toggle_response = ui.interact(
                toggle_rect,
                ui.id().with("toggle"),
                Sense::click().union(Sense::hover()),
            );

            if toggle_response.clicked() {
                self.toggle();
            }

            // Draw toggle button background
            let toggle_bg_color = if toggle_response.hovered() {
                Color32::from_rgba_unmultiplied(255, 255, 255, 20)
            } else {
                Color32::from_rgba_unmultiplied(255, 255, 255, 8)
            };

            painter.rect(
                toggle_rect,
                8.0,
                toggle_bg_color,
                egui::Stroke::NONE,
                egui::StrokeKind::Outside,
            );

            // Draw toggle icon
            let toggle_icon = if self.is_expanded { "☰" } else { "☰" };
            painter.text(
                Pos2::new(
                    toggle_rect.left() + (self.collapsed_width - padding * 2.0) / 2.0,
                    toggle_rect.center().y,
                ),
                egui::Align2::CENTER_CENTER,
                toggle_icon,
                egui::FontId::proportional(20.0),
                Color32::from_gray(200),
            );

            // Draw items
            let items_start_y = rect.top() + padding * 2.0 + item_height + padding;

            for (index, item) in self.items.iter().enumerate() {
                let item_y = items_start_y + index as f32 * (item_height + padding);
                let item_rect = Rect::from_min_size(
                    Pos2::new(rect.left() + padding, item_y),
                    Vec2::new(current_width - padding * 2.0, item_height),
                );

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
                    self.active_position_animation.start = self.active_position_animation.value();
                    self.active_position_animation.end = item_y - items_start_y;
                    self.active_position_animation.elapsed = 0.0;
                    self.active_position_animation.start();
                }

                // Draw active indicator
                if item.active || Some(index) == self.current_active {
                    let indicator_rect = if Some(index) == self.current_active {
                        // Animated position
                        Rect::from_min_size(
                            Pos2::new(
                                rect.left() + padding,
                                items_start_y + self.active_position_animation.value(),
                            ),
                            Vec2::new(current_width - padding * 2.0, item_height),
                        )
                    } else {
                        item_rect
                    };

                    painter.rect(
                        indicator_rect,
                        8.0,
                        theme.primary().linear_multiply(0.2),
                        egui::Stroke::NONE,
                        egui::StrokeKind::Outside,
                    );
                }

                // Draw hover effect
                if item_response.hovered() && Some(index) != self.current_active {
                    painter.rect(
                        item_rect,
                        8.0,
                        Color32::from_rgba_unmultiplied(255, 255, 255, 15),
                        egui::Stroke::NONE,
                        egui::StrokeKind::Outside,
                    );
                }

                let text_color = if item.active || Some(index) == self.current_active {
                    Color32::WHITE
                } else if item_response.hovered() {
                    Color32::from_gray(220)
                } else {
                    Color32::from_gray(160)
                };

                // Draw icon
                let icon_pos = Pos2::new(
                    item_rect.left() + (self.collapsed_width - padding * 2.0) / 2.0,
                    item_rect.center().y,
                );
                painter.text(
                    icon_pos,
                    egui::Align2::CENTER_CENTER,
                    &item.icon,
                    egui::FontId::proportional(icon_size),
                    text_color,
                );

                // Draw label when expanded
                if self.is_expanded && current_width > self.collapsed_width + 20.0 {
                    let label_opacity = ((current_width - self.collapsed_width)
                        / (self.expanded_width - self.collapsed_width))
                        .clamp(0.0, 1.0);

                    let label_color = Color32::from_rgba_unmultiplied(
                        text_color.r(),
                        text_color.g(),
                        text_color.b(),
                        (text_color.a() as f32 * label_opacity) as u8,
                    );

                    painter.text(
                        Pos2::new(
                            item_rect.left() + self.collapsed_width - padding,
                            item_rect.center().y,
                        ),
                        egui::Align2::LEFT_CENTER,
                        &item.label,
                        egui::FontId::proportional(14.0),
                        label_color,
                    );
                }

                // Draw badge if present
                if let Some(badge) = &item.badge {
                    let badge_size = 18.0;
                    let badge_pos =
                        if self.is_expanded && current_width > self.collapsed_width + 20.0 {
                            Pos2::new(
                                item_rect.right() - badge_size / 2.0 - 4.0,
                                item_rect.top() + 8.0,
                            )
                        } else {
                            Pos2::new(icon_pos.x + icon_size / 2.0, item_rect.top() + 8.0)
                        };

                    painter.circle(
                        badge_pos,
                        badge_size / 2.0,
                        theme.error(),
                        egui::Stroke::new(2.0, Color32::from_rgb(18, 18, 22)),
                    );

                    painter.text(
                        badge_pos,
                        egui::Align2::CENTER_CENTER,
                        badge,
                        egui::FontId::proportional(10.0),
                        Color32::WHITE,
                    );
                }
            }

            // Initialize animation on first frame if there's an active item
            if self.active_position_animation.value() == 0.0 && self.current_active.is_some() {
                if let Some(active_idx) = self.current_active {
                    let item_y = active_idx as f32 * (item_height + padding);
                    self.active_position_animation.start = item_y;
                    self.active_position_animation.end = item_y;
                }
            }
        }

        // Request repaint if animations are running
        if !self.width_animation.is_complete() || !self.active_position_animation.is_complete() {
            ui.ctx().request_repaint();
        }

        // Create a response for the whole sidebar area
        let response = ui.interact(rect, ui.id().with("sidebar"), Sense::hover());

        SidebarResponse {
            response,
            clicked: clicked_index,
            hovered: hovered_index,
            is_expanded: self.is_expanded,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_item_creation() {
        let item = SidebarItem::new("🏠", "Home");
        assert_eq!(item.label, "Home");
        assert_eq!(item.icon, "🏠");
        assert!(!item.active);
        assert!(item.badge.is_none());
    }

    #[test]
    fn test_sidebar_item_with_badge() {
        let item = SidebarItem::new("📧", "Messages").badge("5");
        assert_eq!(item.badge, Some("5".to_string()));
    }

    #[test]
    fn test_sidebar_creation() {
        let items = vec![
            SidebarItem::new("🏠", "Home").active(true),
            SidebarItem::new("📧", "Messages"),
            SidebarItem::new("⚙️", "Settings"),
        ];
        let sidebar = Sidebar::new(items);
        assert_eq!(sidebar.items.len(), 3);
        assert_eq!(sidebar.current_active, Some(0));
        assert!(sidebar.is_expanded);
    }

    #[test]
    fn test_sidebar_toggle() {
        let items = vec![SidebarItem::new("🏠", "Home")];
        let mut sidebar = Sidebar::new(items);
        assert!(sidebar.is_expanded);

        sidebar.toggle();
        assert!(!sidebar.is_expanded);

        sidebar.toggle();
        assert!(sidebar.is_expanded);
    }
}
