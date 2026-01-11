//! Floating Dock Component
//!
//! macOS-style dock with icon magnification on hover

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{pos2, vec2, Color32, CornerRadius, CursorIcon, Rect, Response, Sense, Stroke, Ui};
use std::f32::consts::PI;

/// A dock item that can be clicked
#[derive(Clone)]
pub struct DockItem {
    /// Display label
    pub label: String,
    /// Icon text (emoji or single character)
    pub icon: String,
    /// Optional callback ID for handling clicks
    pub id: Option<String>,
}

impl DockItem {
    /// Create a new dock item
    pub fn new(label: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
            id: None,
        }
    }

    /// Set the callback ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// Floating dock component
pub struct FloatingDock {
    items: Vec<DockItem>,
    magnification: f32,
    item_size: f32,
    spacing: f32,
    background_color: Color32,
    hover_index: Option<usize>,
    position: DockPosition,
}

/// Dock position on screen
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DockPosition {
    Bottom,
    Top,
    Left,
    Right,
}

impl FloatingDock {
    /// Create a new floating dock with theme-based defaults
    pub fn new(items: Vec<DockItem>, theme: &Theme) -> Self {
        let surface = theme.surface();
        Self {
            items,
            magnification: 1.5,
            item_size: 48.0,
            spacing: 8.0,
            background_color: Color32::from_rgba_unmultiplied(
                surface.r(),
                surface.g(),
                surface.b(),
                200,
            ),
            hover_index: None,
            position: DockPosition::Bottom,
        }
    }

    /// Set magnification factor (default: 1.5)
    pub fn with_magnification(mut self, factor: f32) -> Self {
        self.magnification = factor.max(1.0);
        self
    }

    /// Set item size in pixels (default: 48.0)
    pub fn with_item_size(mut self, size: f32) -> Self {
        self.item_size = size.max(16.0);
        self
    }

    /// Set spacing between items (default: 8.0)
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(0.0);
        self
    }

    /// Set background color
    pub fn with_background_color(mut self, color: Color32) -> Self {
        self.background_color = color;
        self
    }

    /// Set dock position
    pub fn with_position(mut self, position: DockPosition) -> Self {
        self.position = position;
        self
    }

    /// Show the dock at the bottom center of the screen
    pub fn show(&mut self, ui: &mut Ui) -> DockResponse {
        let theme = ui.ctx().armas_theme();
        let mut clicked_item = None;

        // Calculate dock dimensions
        let base_width = self.items.len() as f32 * (self.item_size + self.spacing) + self.spacing;
        let base_height = self.item_size + self.spacing * 2.0;

        let max_scale = self.magnification;
        let max_height = base_height * max_scale;

        // Position at bottom center
        let screen_rect = ui.ctx().content_rect();
        let dock_pos = match self.position {
            DockPosition::Bottom => pos2(
                screen_rect.center().x - base_width / 2.0,
                screen_rect.max.y - max_height - 20.0,
            ),
            DockPosition::Top => pos2(screen_rect.center().x - base_width / 2.0, 20.0),
            DockPosition::Left => pos2(20.0, screen_rect.center().y - base_width / 2.0),
            DockPosition::Right => pos2(
                screen_rect.max.x - max_height - 20.0,
                screen_rect.center().y - base_width / 2.0,
            ),
        };

        let dock_rect = Rect::from_min_size(dock_pos, vec2(base_width, max_height));

        // Background
        let bg_rect = Rect::from_min_size(
            dock_pos + vec2(0.0, max_height - base_height),
            vec2(base_width, base_height),
        );
        ui.painter().rect_filled(
            bg_rect,
            CornerRadius::same(theme.spacing.corner_radius as u8),
            self.background_color,
        );
        ui.painter().rect_stroke(
            bg_rect,
            CornerRadius::same(theme.spacing.corner_radius as u8),
            Stroke::new(1.0, Color32::from_gray(80)),
            egui::StrokeKind::Middle,
        );

        // Get mouse position
        let mouse_pos = ui.input(|i| i.pointer.hover_pos());

        // Calculate item scales and positions
        let mut item_rects = Vec::new();
        let mut current_x = dock_pos.x + self.spacing;
        let base_y = dock_pos.y + max_height - base_height + self.spacing;

        for (idx, _item) in self.items.iter().enumerate() {
            // Calculate scale based on mouse distance
            let scale = if let Some(mouse_pos) = mouse_pos {
                let item_center_x = current_x + self.item_size / 2.0;
                let item_center = pos2(item_center_x, base_y + self.item_size / 2.0);
                let distance = (mouse_pos.x - item_center.x).abs();

                // Magnification effect with smooth falloff
                let influence_radius = self.item_size * 2.0;
                if distance < influence_radius {
                    let t = 1.0 - (distance / influence_radius);
                    // Smooth curve using cosine interpolation
                    let smooth_t = (1.0 - (t * PI).cos()) / 2.0;
                    1.0 + (max_scale - 1.0) * smooth_t
                } else {
                    1.0
                }
            } else {
                1.0
            };

            let scaled_size = self.item_size * scale;
            let y_offset = (scaled_size - self.item_size) / 2.0;

            let item_rect = Rect::from_min_size(
                pos2(current_x, base_y - y_offset),
                vec2(scaled_size, scaled_size),
            );

            item_rects.push((item_rect, scale, idx));
            current_x += self.item_size + self.spacing;
        }

        // Draw items
        for (item_rect, scale, idx) in &item_rects {
            let item = &self.items[*idx];

            // Item background - use theme colors
            let item_color = if Some(*idx) == self.hover_index {
                let hover = theme.hover();
                Color32::from_rgba_unmultiplied(hover.r(), hover.g(), hover.b(), 220)
            } else {
                let variant = theme.surface_variant();
                Color32::from_rgba_unmultiplied(variant.r(), variant.g(), variant.b(), 200)
            };

            ui.painter().rect_filled(
                *item_rect,
                CornerRadius::same((theme.spacing.corner_radius_small * scale) as u8),
                item_color,
            );

            // Icon
            let icon_size = 24.0 * scale;

            ui.painter().text(
                item_rect.center(),
                egui::Align2::CENTER_CENTER,
                &item.icon,
                egui::FontId::proportional(icon_size),
                Color32::WHITE,
            );
        }

        // Handle interactions
        let response = ui.allocate_rect(dock_rect, Sense::hover());

        self.hover_index = None;
        if let Some(mouse_pos) = mouse_pos {
            for (item_rect, _scale, idx) in &item_rects {
                if item_rect.contains(mouse_pos) {
                    self.hover_index = Some(*idx);

                    // Show label on hover
                    let label_pos = pos2(item_rect.center().x, item_rect.min.y - 10.0);
                    ui.painter().text(
                        label_pos,
                        egui::Align2::CENTER_BOTTOM,
                        &self.items[*idx].label,
                        egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );

                    // Handle click
                    if ui.input(|i| i.pointer.primary_clicked()) {
                        clicked_item = Some(*idx);
                    }

                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    break;
                }
            }
        }

        // Request repaint for smooth animations
        if mouse_pos.is_some() {
            ui.ctx().request_repaint();
        }

        DockResponse {
            response,
            clicked_item,
        }
    }
}

/// Response from the dock
pub struct DockResponse {
    /// The underlying response
    pub response: Response,
    /// Index of clicked item, if any
    pub clicked_item: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dock_creation() {
        let theme = Theme::default();
        let items = vec![DockItem::new("Home", "🏠"), DockItem::new("Mail", "📧")];
        let dock = FloatingDock::new(items, &theme);
        assert_eq!(dock.items.len(), 2);
        assert_eq!(dock.magnification, 1.5);
    }

    #[test]
    fn test_dock_config() {
        let theme = Theme::default();
        let items = vec![DockItem::new("Test", "✨")];
        let dock = FloatingDock::new(items, &theme)
            .with_magnification(2.0)
            .with_item_size(64.0)
            .with_position(DockPosition::Top);

        assert_eq!(dock.magnification, 2.0);
        assert_eq!(dock.item_size, 64.0);
        assert_eq!(dock.position, DockPosition::Top);
    }
}
