//! Sidebar
//!
//! Animated sidebar with icons and smooth expand/collapse

use crate::animation::{Animation, EasingFunction};
use crate::ext::ArmasContextExt;
use egui::{Color32, Id, Pos2, Rect, Response, Sense, Ui, Vec2};

// ============================================================================
// NEW CLOSURE-BASED API
// ============================================================================

/// Internal representation of a sidebar item for rendering
#[derive(Clone)]
struct InternalSidebarItem {
    id: String,
    icon: String,
    label: String,
    active: bool,
    badge: Option<String>,
    depth: usize,
    is_group_header: bool,
}

/// Builder for configuring individual sidebar items
pub struct SidebarItemBuilder {
    item: InternalSidebarItem,
}

impl SidebarItemBuilder {
    fn new(id: String, icon: String, label: String, depth: usize) -> Self {
        Self {
            item: InternalSidebarItem {
                id,
                icon,
                label,
                active: false,
                badge: None,
                depth,
                is_group_header: false,
            },
        }
    }

    /// Mark this item as active
    pub fn active(mut self, active: bool) -> Self {
        self.item.active = active;
        self
    }

    /// Set badge text (e.g., notification count)
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.item.badge = Some(badge.into());
        self
    }
}

/// Builder for adding items to the sidebar
pub struct SidebarBuilder<'a> {
    items: &'a mut Vec<InternalSidebarItem>,
    current_depth: usize,
    ui: &'a mut Ui,
    expanded_groups: &'a mut std::collections::HashMap<String, bool>,
}

impl<'a> SidebarBuilder<'a> {
    /// Add an item to the sidebar
    pub fn item(&mut self, icon: &str, label: &str) -> SidebarItemBuilder {
        let id = format!("item_{}_{}", self.current_depth, label);
        let builder = SidebarItemBuilder::new(
            id,
            icon.to_string(),
            label.to_string(),
            self.current_depth,
        );
        self.items.push(builder.item.clone());
        builder
    }

    /// Add a group with nested items
    pub fn group(
        &mut self,
        icon: &str,
        label: &str,
        content: impl FnOnce(&mut Self),
    ) {
        let group_id = format!("group_{}_{}", self.current_depth, label);

        // Load expanded state from memory
        let state_id = Id::new(&group_id);
        let is_expanded = self.ui.ctx().data_mut(|d| {
            d.get_temp::<bool>(state_id)
                .or_else(|| self.expanded_groups.get(&group_id).copied())
                .unwrap_or(false)
        });

        // Add the group header as an item
        let group_item = InternalSidebarItem {
            id: group_id.clone(),
            icon: icon.to_string(),
            label: label.to_string(),
            active: false,
            badge: Some(if is_expanded { "▼".to_string() } else { "▶".to_string() }),
            depth: self.current_depth,
            is_group_header: true,
        };
        self.items.push(group_item);

        // Store expanded state
        self.expanded_groups.insert(group_id.clone(), is_expanded);

        // If expanded, add nested items with increased depth
        if is_expanded {
            self.current_depth += 1;
            content(self);
            self.current_depth -= 1;
        }
    }
}

/// Response from sidebar interaction
pub struct SidebarResponse {
    /// The overall sidebar response
    pub response: Response,
    /// ID of the clicked item, if any
    pub clicked: Option<String>,
    /// Index of the hovered item, if any (for backwards compatibility)
    pub hovered: Option<usize>,
    /// Whether the sidebar is currently expanded
    pub is_expanded: bool,
}

/// Animated sidebar component (new closure-based API)
///
/// A sidebar that can expand to show labels or collapse to show only icons.
/// Includes smooth animations for expand/collapse and item highlighting.
///
/// # Example
///
/// ```ignore
/// Sidebar::new()
///     .collapsed(false)
///     .show(ui, |sidebar| {
///         sidebar.item("🏠", "Home").active(true);
///         sidebar.item("📧", "Messages").badge("5");
///
///         sidebar.group("⚙️", "Settings", |group| {
///             group.item("👤", "Profile");
///             group.item("🔒", "Privacy");
///         });
///     })
/// ```
pub struct Sidebar {
    is_expanded: bool,
    collapsed_width: f32,
    expanded_width: f32,
    collapsible: bool,
    show_icons: bool,
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new() -> Self {
        Self {
            is_expanded: true,
            collapsed_width: 70.0,
            expanded_width: 240.0,
            collapsible: true,
            show_icons: true,
        }
    }

    /// Set whether the sidebar starts collapsed
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.is_expanded = !collapsed;
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

    /// Set whether the sidebar can be collapsed
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set whether to show icons
    pub fn show_icons(mut self, show_icons: bool) -> Self {
        self.show_icons = show_icons;
        self
    }

    /// Show the sidebar with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut SidebarBuilder) -> R,
    ) -> SidebarResponse {
        let theme = ui.ctx().armas_theme();
        let dt = ui.input(|i| i.stable_dt);

        // Get or initialize animation state from memory
        let sidebar_id = ui.id().with("sidebar_state");
        let (mut width_anim, mut active_pos_anim, mut current_active, mut expanded_groups): (
            Animation<f32>,
            Animation<f32>,
            Option<String>,
            std::collections::HashMap<String, bool>,
        ) = ui.ctx().data_mut(|d| {
            d.get_temp(sidebar_id).unwrap_or_else(|| {
                let target_width = if self.is_expanded {
                    self.expanded_width
                } else {
                    self.collapsed_width
                };
                (
                    Animation::new(target_width, target_width, 0.3).easing(EasingFunction::EaseOut),
                    Animation::new(0.0, 0.0, 0.3).easing(EasingFunction::EaseOut),
                    None,
                    std::collections::HashMap::new(),
                )
            })
        });

        // Collect items from closure
        let mut items = Vec::new();
        let mut builder = SidebarBuilder {
            items: &mut items,
            current_depth: 0,
            ui,
            expanded_groups: &mut expanded_groups,
        };
        content(&mut builder);

        // Update animations
        width_anim.update(dt);
        active_pos_anim.update(dt);

        let current_width = width_anim.value();
        let item_height = 56.0;
        let padding = 12.0;
        let icon_size = 24.0;
        let child_indent = 32.0;

        // Calculate height based on actual content
        let toggle_height = if self.collapsible {
            item_height + padding
        } else {
            0.0
        };
        let content_height = items.len() as f32 * (item_height + padding);
        let sidebar_height = padding + toggle_height + content_height + padding;

        let rect = Rect::from_min_size(ui.cursor().min, Vec2::new(current_width, sidebar_height));

        ui.advance_cursor_after_rect(rect);

        let mut clicked_id: Option<String> = None;
        let mut hovered_index: Option<usize> = None;
        let mut is_expanded = self.is_expanded;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw sidebar background
            painter.rect_filled(rect, 0.0, theme.surface());

            // Conditionally draw toggle button if collapsible
            let items_start_y = if self.collapsible {
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
                    is_expanded = !is_expanded;
                    width_anim.start = width_anim.value();
                    width_anim.end = if is_expanded {
                        self.expanded_width
                    } else {
                        self.collapsed_width
                    };
                    width_anim.elapsed = 0.0;
                    width_anim.start();
                }

                // Draw toggle button background
                let toggle_bg_color = if toggle_response.hovered() {
                    theme.hover()
                } else {
                    theme.surface_variant()
                };

                painter.rect(
                    toggle_rect,
                    theme.spacing.corner_radius_small,
                    toggle_bg_color,
                    egui::Stroke::NONE,
                    egui::StrokeKind::Outside,
                );

                // Draw toggle icon
                painter.text(
                    Pos2::new(
                        toggle_rect.left() + (self.collapsed_width - padding * 2.0) / 2.0,
                        toggle_rect.center().y,
                    ),
                    egui::Align2::CENTER_CENTER,
                    "☰",
                    egui::FontId::proportional(20.0),
                    theme.on_surface(),
                );

                rect.top() + padding * 2.0 + item_height + padding
            } else {
                rect.top() + padding
            };

            // Draw items
            let mut current_y = items_start_y;

            for (index, item) in items.iter().enumerate() {
                let indent = item.depth as f32 * child_indent;
                let item_rect = Rect::from_min_size(
                    Pos2::new(rect.left() + padding + indent, current_y),
                    Vec2::new(current_width - padding * 2.0 - indent, item_height),
                );

                // Item interaction
                let item_response = ui.interact(
                    item_rect,
                    ui.id().with(&item.id),
                    Sense::click().union(Sense::hover()),
                );

                if item_response.hovered() {
                    hovered_index = Some(index);
                }

                if item_response.clicked() {
                    // Check if this is a group header
                    if item.is_group_header {
                        // Toggle group expansion
                        let state_id = Id::new(&item.id);
                        let is_expanded = expanded_groups.get(&item.id).copied().unwrap_or(false);
                        expanded_groups.insert(item.id.clone(), !is_expanded);
                        ui.ctx().data_mut(|d| {
                            d.insert_temp(state_id, !is_expanded);
                        });
                    } else {
                        clicked_id = Some(item.id.clone());
                        current_active = Some(item.id.clone());
                        active_pos_anim.start = active_pos_anim.value();
                        active_pos_anim.end = current_y - items_start_y;
                        active_pos_anim.elapsed = 0.0;
                        active_pos_anim.start();
                    }
                }

                // Draw active indicator
                if item.active || current_active.as_ref() == Some(&item.id) {
                    let indicator_rect = if current_active.as_ref() == Some(&item.id) {
                        Rect::from_min_size(
                            Pos2::new(
                                rect.left() + padding + indent,
                                items_start_y + active_pos_anim.value(),
                            ),
                            Vec2::new(current_width - padding * 2.0 - indent, item_height),
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
                if item_response.hovered() && current_active.as_ref() != Some(&item.id) {
                    painter.rect(
                        item_rect,
                        theme.spacing.corner_radius_small,
                        theme.hover(),
                        egui::Stroke::NONE,
                        egui::StrokeKind::Outside,
                    );
                }

                let text_color = if item.active
                    || current_active.as_ref() == Some(&item.id)
                    || item_response.hovered()
                {
                    theme.on_surface()
                } else {
                    theme.on_surface_variant()
                };

                // Calculate icon position
                let icon_pos = Pos2::new(
                    item_rect.left() + (self.collapsed_width - padding * 2.0) / 2.0 - indent,
                    item_rect.center().y,
                );

                // Draw icon if enabled
                if self.show_icons {
                    painter.text(
                        icon_pos,
                        egui::Align2::CENTER_CENTER,
                        &item.icon,
                        egui::FontId::proportional(icon_size),
                        text_color,
                    );
                }

                // Draw label when expanded
                if is_expanded && current_width > self.collapsed_width + 20.0 {
                    let label_opacity = ((current_width - self.collapsed_width)
                        / (self.expanded_width - self.collapsed_width))
                        .clamp(0.0, 1.0);

                    let label_color = Color32::from_rgba_unmultiplied(
                        text_color.r(),
                        text_color.g(),
                        text_color.b(),
                        (text_color.a() as f32 * label_opacity) as u8,
                    );

                    let label_x = if self.show_icons {
                        item_rect.left() + self.collapsed_width - padding - indent
                    } else {
                        item_rect.left() + padding
                    };

                    painter.text(
                        Pos2::new(label_x, item_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        &item.label,
                        egui::FontId::proportional(14.0),
                        label_color,
                    );
                }

                // Draw badge or chevron
                if let Some(badge) = &item.badge {
                    if item.is_group_header {
                        // Draw chevron for group
                        if is_expanded && current_width > self.collapsed_width + 20.0 {
                            painter.text(
                                Pos2::new(item_rect.right() - 16.0, item_rect.center().y),
                                egui::Align2::CENTER_CENTER,
                                badge,
                                egui::FontId::proportional(10.0),
                                theme.on_surface_variant(),
                            );
                        }
                    } else {
                        // Draw badge circle
                        let badge_size = 18.0;
                        let badge_pos = if is_expanded && current_width > self.collapsed_width + 20.0 {
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
                            egui::Stroke::new(2.0, theme.surface()),
                        );

                        painter.text(
                            badge_pos,
                            egui::Align2::CENTER_CENTER,
                            badge,
                            egui::FontId::proportional(10.0),
                            theme.on_surface(),
                        );
                    }
                }

                current_y += item_height + padding;
            }
        }

        // Request repaint if animations are running
        if !width_anim.is_complete() || !active_pos_anim.is_complete() {
            ui.ctx().request_repaint();
        }

        // Save state
        ui.ctx().data_mut(|d| {
            d.insert_temp(
                sidebar_id,
                (width_anim, active_pos_anim, current_active, expanded_groups),
            );
        });

        let response = ui.interact(rect, ui.id().with("sidebar"), Sense::hover());

        SidebarResponse {
            response,
            clicked: clicked_id,
            hovered: hovered_index,
            is_expanded,
        }
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

