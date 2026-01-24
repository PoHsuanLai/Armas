//! Sidebar
//!
//! Animated sidebar with icons and smooth expand/collapse.
//! Styled to match shadcn/ui conventions with spring-based animations.

use crate::animation::SpringAnimation;
use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

// shadcn sidebar dimensions
const SIDEBAR_WIDTH: f32 = 256.0; // 16rem
const SIDEBAR_WIDTH_ICON: f32 = 48.0; // 3rem

// shadcn item dimensions
const ITEM_HEIGHT: f32 = 32.0; // h-8
const ITEM_HEIGHT_SM: f32 = 28.0; // h-7 for sub-items
const ITEM_GAP: f32 = 4.0; // gap-1
const ITEM_PADDING: f32 = 8.0; // p-2
const ICON_SIZE: f32 = 16.0; // size-4
const CORNER_RADIUS: f32 = 6.0; // rounded-md
const GROUP_PADDING: f32 = 8.0; // p-2 for groups

// Spring animation parameters (snappy but smooth)
const SPRING_STIFFNESS: f32 = 300.0;
const SPRING_DAMPING: f32 = 25.0;

// ============================================================================
// SIDEBAR STATE (for external control)
// ============================================================================

/// Sidebar state that can be stored externally for controlled mode
#[derive(Clone, Debug)]
pub struct SidebarState {
    /// Whether the sidebar is expanded
    pub open: bool,
    /// Width spring animation
    width_spring: SpringAnimation,
    /// Expanded groups
    expanded_groups: std::collections::HashMap<String, bool>,
    /// Currently active item index
    active_index: Option<usize>,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self::new(true)
    }
}

impl SidebarState {
    /// Create new sidebar state
    pub fn new(open: bool) -> Self {
        let target = if open { SIDEBAR_WIDTH } else { SIDEBAR_WIDTH_ICON };
        Self {
            open,
            width_spring: SpringAnimation::new(target, target).params(SPRING_STIFFNESS, SPRING_DAMPING),
            expanded_groups: std::collections::HashMap::new(),
            active_index: None,
        }
    }

    /// Toggle the sidebar open/closed
    pub fn toggle(&mut self) {
        self.open = !self.open;
        let target = if self.open { SIDEBAR_WIDTH } else { SIDEBAR_WIDTH_ICON };
        self.width_spring.set_target(target);
    }

    /// Set the sidebar open state
    pub fn set_open(&mut self, open: bool) {
        if self.open != open {
            self.open = open;
            let target = if open { SIDEBAR_WIDTH } else { SIDEBAR_WIDTH_ICON };
            self.width_spring.set_target(target);
        }
    }

    /// Check if sidebar is expanded
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Get current animated width
    pub fn width(&self) -> f32 {
        self.width_spring.value
    }

    /// Check if animation is still running
    pub fn is_animating(&self) -> bool {
        !self.width_spring.is_settled(0.5, 0.5)
    }
}

// ============================================================================
// SIDEBAR VARIANT
// ============================================================================

/// Sidebar visual variant
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SidebarVariant {
    /// Standard sidebar with border
    #[default]
    Sidebar,
    /// Floating sidebar with rounded corners and shadow
    Floating,
    /// Inset sidebar (similar to floating but for inset layouts)
    Inset,
}

/// Collapsible mode for the sidebar
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CollapsibleMode {
    /// Collapse to icon-only view
    #[default]
    Icon,
    /// Slide completely off screen
    Offcanvas,
    /// Not collapsible
    None,
}

// ============================================================================
// INTERNAL TYPES
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
    is_group_label: bool,
}

/// Builder for configuring individual sidebar items
///
/// This holds a mutable reference to the item in the list, so modifications
/// like `.active()` and `.badge()` are applied directly.
pub struct SidebarItemBuilder<'a> {
    item: &'a mut InternalSidebarItem,
}

impl<'a> SidebarItemBuilder<'a> {
    /// Mark this item as active
    pub fn active(self, active: bool) -> Self {
        self.item.active = active;
        self
    }

    /// Set badge text (e.g., notification count)
    pub fn badge(self, badge: impl Into<String>) -> Self {
        self.item.badge = Some(badge.into());
        self
    }
}

/// Builder for adding items to the sidebar
pub struct SidebarBuilder<'a> {
    items: &'a mut Vec<InternalSidebarItem>,
    current_depth: usize,
    expanded_groups: &'a mut std::collections::HashMap<String, bool>,
}

impl<'a> SidebarBuilder<'a> {
    /// Add an item to the sidebar
    ///
    /// Returns a builder that can be used to configure the item with chained methods.
    /// The builder holds a mutable reference to the item, so changes like `.badge()` and `.active()`
    /// are applied directly to the item in the list.
    pub fn item(&mut self, icon: &str, label: &str) -> SidebarItemBuilder<'_> {
        let id = format!("item_{}_{}", self.current_depth, label);
        let item = InternalSidebarItem {
            id,
            icon: icon.to_string(),
            label: label.to_string(),
            active: false,
            badge: None,
            depth: self.current_depth,
            is_group_header: false,
            is_group_label: false,
        };
        self.items.push(item);
        let idx = self.items.len() - 1;
        SidebarItemBuilder {
            item: &mut self.items[idx],
        }
    }

    /// Add a group label (non-interactive header text)
    pub fn group_label(&mut self, label: &str) {
        let id = format!("group_label_{}", label);
        let item = InternalSidebarItem {
            id,
            icon: String::new(),
            label: label.to_string(),
            active: false,
            badge: None,
            depth: self.current_depth,
            is_group_header: false,
            is_group_label: true,
        };
        self.items.push(item);
    }

    /// Add a collapsible group with nested items
    pub fn group(&mut self, icon: &str, label: &str, content: impl FnOnce(&mut Self)) {
        let group_id = format!("group_{}_{}", self.current_depth, label);

        // Get expanded state
        let is_expanded = self.expanded_groups.get(&group_id).copied().unwrap_or(false);

        // Add the group header as an item
        let group_item = InternalSidebarItem {
            id: group_id.clone(),
            icon: icon.to_string(),
            label: label.to_string(),
            active: false,
            badge: None,
            depth: self.current_depth,
            is_group_header: true,
            is_group_label: false,
        };
        self.items.push(group_item);

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
    /// Index of the hovered item, if any
    pub hovered: Option<usize>,
    /// Whether the sidebar is currently expanded
    pub is_expanded: bool,
}

/// Animated sidebar component styled to match shadcn/ui
///
/// Supports both controlled mode (with external SidebarState) and uncontrolled mode.
///
/// # Controlled Mode Example
///
/// ```ignore
/// // Store state somewhere persistent
/// let mut sidebar_state = SidebarState::new(true);
///
/// // In your UI code:
/// Sidebar::new()
///     .state(&mut sidebar_state)
///     .show(ui, |sidebar| {
///         sidebar.item("🏠", "Home").active(true);
///     });
///
/// // Toggle from anywhere:
/// if some_button_clicked {
///     sidebar_state.toggle();
/// }
/// ```
///
/// # Uncontrolled Mode Example
///
/// ```ignore
/// Sidebar::new()
///     .collapsed(false)
///     .show(ui, |sidebar| {
///         sidebar.group_label("Platform");
///         sidebar.item("🏠", "Home").active(true);
///         sidebar.item("📧", "Messages").badge("5");
///     })
/// ```
pub struct Sidebar<'a> {
    /// External state for controlled mode
    external_state: Option<&'a mut SidebarState>,
    /// Initial expanded state (for uncontrolled mode)
    initial_open: bool,
    /// Collapsed width override
    collapsed_width: Option<f32>,
    /// Expanded width override
    expanded_width: Option<f32>,
    /// Collapsible mode
    collapsible: CollapsibleMode,
    /// Show icons
    show_icons: bool,
    /// Visual variant
    variant: SidebarVariant,
}

impl<'a> Sidebar<'a> {
    /// Create a new sidebar with shadcn defaults
    pub fn new() -> Self {
        Self {
            external_state: None,
            initial_open: true,
            collapsed_width: None,
            expanded_width: None,
            collapsible: CollapsibleMode::Icon,
            show_icons: true,
            variant: SidebarVariant::Sidebar,
        }
    }

    /// Use external state for controlled mode
    ///
    /// This allows you to control the sidebar from outside and persist state.
    pub fn state(mut self, state: &'a mut SidebarState) -> Self {
        self.external_state = Some(state);
        self
    }

    /// Set whether the sidebar starts collapsed (uncontrolled mode only)
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.initial_open = !collapsed;
        self
    }

    /// Set the collapsed width (default: 48px / 3rem)
    pub fn collapsed_width(mut self, width: f32) -> Self {
        self.collapsed_width = Some(width);
        self
    }

    /// Set the expanded width (default: 256px / 16rem)
    pub fn expanded_width(mut self, width: f32) -> Self {
        self.expanded_width = Some(width);
        self
    }

    /// Set the collapsible mode
    pub fn collapsible(mut self, mode: CollapsibleMode) -> Self {
        self.collapsible = mode;
        self
    }

    /// Set whether to show icons
    pub fn show_icons(mut self, show_icons: bool) -> Self {
        self.show_icons = show_icons;
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: SidebarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Show the sidebar
    pub fn show<R>(
        mut self,
        ui: &mut Ui,
        content: impl FnOnce(&mut SidebarBuilder) -> R,
    ) -> SidebarResponse {
        let theme = ui.ctx().armas_theme();
        let dt = ui.input(|i| i.stable_dt);

        // Get width bounds
        let collapsed_width = self.collapsed_width.unwrap_or(SIDEBAR_WIDTH_ICON);
        let expanded_width = self.expanded_width.unwrap_or(SIDEBAR_WIDTH);

        // Handle state (controlled vs uncontrolled)
        let sidebar_id = ui.id().with("sidebar_state");

        // Get or create internal state for uncontrolled mode
        let mut internal_state: SidebarState = if self.external_state.is_none() {
            ui.ctx().data_mut(|d| {
                d.get_temp(sidebar_id).unwrap_or_else(|| {
                    let mut state = SidebarState::new(self.initial_open);
                    // Apply custom widths
                    let target = if self.initial_open { expanded_width } else { collapsed_width };
                    state.width_spring = SpringAnimation::new(target, target)
                        .params(SPRING_STIFFNESS, SPRING_DAMPING);
                    state
                })
            })
        } else {
            SidebarState::default() // Won't be used
        };

        // Get mutable reference to the actual state we're using
        let state = if let Some(ref mut ext) = self.external_state {
            ext
        } else {
            &mut internal_state
        };

        // Update spring animation
        state.width_spring.update(dt);

        // Collect items from closure
        let mut items = Vec::new();
        {
            let mut builder = SidebarBuilder {
                items: &mut items,
                current_depth: 0,
                expanded_groups: &mut state.expanded_groups,
            };
            content(&mut builder);
        }

        let current_width = state.width_spring.value;

        // For floating/inset variants, add padding to the outer dimensions
        let floating_padding = if matches!(self.variant, SidebarVariant::Floating | SidebarVariant::Inset) {
            8.0
        } else {
            0.0
        };

        // Calculate sidebar height based on content
        let mut total_height = GROUP_PADDING;

        if self.collapsible != CollapsibleMode::None {
            total_height += ITEM_HEIGHT + ITEM_GAP;
        }

        for item in &items {
            if item.is_group_label {
                total_height += ITEM_HEIGHT + ITEM_GAP;
            } else if item.depth > 0 {
                total_height += ITEM_HEIGHT_SM + ITEM_GAP;
            } else {
                total_height += ITEM_HEIGHT + ITEM_GAP;
            }
        }

        total_height += GROUP_PADDING;

        // Add padding to outer rect for floating variants
        let outer_width = current_width + floating_padding * 2.0;
        let outer_height = total_height + floating_padding * 2.0;

        let rect = Rect::from_min_size(ui.cursor().min, Vec2::new(outer_width, outer_height));
        ui.advance_cursor_after_rect(rect);

        let mut clicked_id: Option<String> = None;
        let mut hovered_index: Option<usize> = None;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Content rect is where items are drawn
            let content_rect = if floating_padding > 0.0 {
                rect.shrink(floating_padding)
            } else {
                rect
            };

            // Draw sidebar background based on variant
            match self.variant {
                SidebarVariant::Sidebar => {
                    painter.rect_filled(rect, 0.0, theme.sidebar());
                    painter.line_segment(
                        [rect.right_top(), rect.right_bottom()],
                        Stroke::new(1.0, theme.border()),
                    );
                }
                SidebarVariant::Floating | SidebarVariant::Inset => {
                    // Shadow
                    painter.rect_filled(
                        content_rect.translate(Vec2::new(0.0, 2.0)),
                        CORNER_RADIUS + 2.0,
                        Color32::from_black_alpha(20),
                    );
                    // Background
                    painter.rect_filled(content_rect, CORNER_RADIUS + 2.0, theme.sidebar());
                    // Border
                    painter.rect_stroke(
                        content_rect,
                        CORNER_RADIUS + 2.0,
                        Stroke::new(1.0, theme.sidebar_border()),
                        egui::StrokeKind::Inside,
                    );
                }
            }

            // Use current_width for item sizing (the animated content width)
            let content_width = current_width;

            // Calculate expansion ratio once for consistent icon positioning
            let expansion_ratio = ((content_width - collapsed_width)
                / (expanded_width - collapsed_width))
                .clamp(0.0, 1.0);

            // Calculate icon position based on expansion
            // When collapsed: center in available width
            // When expanded: left-align with padding
            // Use smooth interpolation to avoid jumps
            let icon_left_aligned_x = content_rect.left() + ITEM_PADDING + ITEM_PADDING + ICON_SIZE / 2.0;
            let icon_centered_x = content_rect.left() + content_width / 2.0;

            // Smoothly interpolate between centered and left-aligned
            let icon_x_base = if expansion_ratio < 0.5 {
                // More collapsed than expanded - use centered position
                icon_centered_x
            } else {
                // More expanded - interpolate from centered to left-aligned
                let t = (expansion_ratio - 0.5) * 2.0; // 0 to 1 as expansion goes from 0.5 to 1.0
                icon_centered_x + (icon_left_aligned_x - icon_centered_x) * t
            };

            let mut current_y = content_rect.top() + GROUP_PADDING;

            // Draw toggle button if collapsible
            if self.collapsible != CollapsibleMode::None {
                let toggle_rect = Rect::from_min_size(
                    Pos2::new(content_rect.left() + ITEM_PADDING, current_y),
                    Vec2::new(content_width - ITEM_PADDING * 2.0, ITEM_HEIGHT),
                );

                let toggle_response = ui.interact(
                    toggle_rect,
                    ui.id().with("toggle"),
                    Sense::click().union(Sense::hover()),
                );

                if toggle_response.clicked() {
                    state.toggle();
                }

                if toggle_response.hovered() {
                    painter.rect_filled(toggle_rect, CORNER_RADIUS, theme.sidebar_accent());
                }

                painter.text(
                    Pos2::new(icon_x_base, toggle_rect.center().y),
                    egui::Align2::CENTER_CENTER,
                    "☰",
                    egui::FontId::proportional(ICON_SIZE),
                    if toggle_response.hovered() {
                        theme.sidebar_accent_foreground()
                    } else {
                        theme.sidebar_foreground()
                    },
                );

                current_y += ITEM_HEIGHT + ITEM_GAP;
            }

            // Draw items
            for (index, item) in items.iter().enumerate() {
                let item_height = if item.is_group_label {
                    ITEM_HEIGHT
                } else if item.depth > 0 {
                    ITEM_HEIGHT_SM
                } else {
                    ITEM_HEIGHT
                };

                // Group labels
                if item.is_group_label {
                    draw_group_label(
                        &painter,
                        &theme,
                        &content_rect,
                        current_y,
                        content_width,
                        collapsed_width,
                        expanded_width,
                        &item.label,
                    );
                    current_y += item_height + ITEM_GAP;
                    continue;
                }

                // Calculate indent for sub-items
                let indent = if item.depth > 0 {
                    14.0 + (item.depth - 1) as f32 * 12.0
                } else {
                    0.0
                };

                let item_rect = Rect::from_min_size(
                    Pos2::new(content_rect.left() + ITEM_PADDING + indent, current_y),
                    Vec2::new(content_width - ITEM_PADDING * 2.0 - indent, item_height),
                );

                // Draw left border for sub-items
                if item.depth > 0 {
                    let border_x = content_rect.left() + ITEM_PADDING + 14.0;
                    painter.line_segment(
                        [
                            Pos2::new(border_x, current_y),
                            Pos2::new(border_x, current_y + item_height),
                        ],
                        Stroke::new(1.0, theme.sidebar_border()),
                    );
                }

                let item_response = ui.interact(
                    item_rect,
                    ui.id().with(&item.id),
                    Sense::click().union(Sense::hover()),
                );

                if item_response.hovered() {
                    hovered_index = Some(index);
                }

                if item_response.clicked() {
                    if item.is_group_header {
                        let was_expanded = state.expanded_groups.get(&item.id).copied().unwrap_or(false);
                        state.expanded_groups.insert(item.id.clone(), !was_expanded);
                    } else {
                        clicked_id = Some(item.id.clone());
                        state.active_index = Some(index);
                    }
                }

                let is_active = item.active || state.active_index == Some(index);
                let is_hovered = item_response.hovered();

                if is_active || is_hovered {
                    painter.rect_filled(item_rect, CORNER_RADIUS, theme.sidebar_accent());
                }

                let text_color = if is_active || is_hovered {
                    theme.sidebar_accent_foreground()
                } else {
                    theme.sidebar_foreground()
                };

                // Draw icon using the same position as toggle for consistency
                // Adjust for indent if this is a sub-item
                let icon_center = if self.show_icons && !item.icon.is_empty() {
                    // For sub-items, offset from base position
                    let item_icon_x = if item.depth > 0 {
                        // Sub-items: always left-aligned with indent
                        item_rect.left() + ITEM_PADDING + ICON_SIZE / 2.0
                    } else {
                        // Top-level items: use same animated position as toggle
                        icon_x_base
                    };
                    painter.text(
                        Pos2::new(item_icon_x, item_rect.center().y),
                        egui::Align2::CENTER_CENTER,
                        &item.icon,
                        egui::FontId::proportional(ICON_SIZE),
                        text_color,
                    );
                    Some(Pos2::new(item_icon_x, item_rect.center().y))
                } else {
                    None
                };

                if expansion_ratio > 0.3 {
                    let label_opacity = ((expansion_ratio - 0.3) / 0.7).clamp(0.0, 1.0);
                    let label_color = Color32::from_rgba_unmultiplied(
                        text_color.r(),
                        text_color.g(),
                        text_color.b(),
                        (text_color.a() as f32 * label_opacity) as u8,
                    );

                    let label_x = if self.show_icons && !item.icon.is_empty() {
                        item_rect.left() + ITEM_PADDING + ICON_SIZE + 8.0
                    } else {
                        item_rect.left() + ITEM_PADDING
                    };

                    let font = if is_active {
                        egui::FontId::new(14.0, egui::FontFamily::Proportional)
                    } else {
                        egui::FontId::proportional(14.0)
                    };

                    painter.text(
                        Pos2::new(label_x, item_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        &item.label,
                        font,
                        label_color,
                    );

                    if item.is_group_header {
                        let is_group_expanded =
                            state.expanded_groups.get(&item.id).copied().unwrap_or(false);
                        let chevron = if is_group_expanded { "▼" } else { "▶" };
                        painter.text(
                            Pos2::new(item_rect.right() - ITEM_PADDING - 8.0, item_rect.center().y),
                            egui::Align2::CENTER_CENTER,
                            chevron,
                            egui::FontId::proportional(10.0),
                            label_color.gamma_multiply(0.7),
                        );
                    }

                    if let Some(badge) = &item.badge {
                        if !item.is_group_header {
                            draw_badge(&painter, &theme, &item_rect, badge, label_opacity);
                        }
                    }
                } else if let Some(badge) = &item.badge {
                    // When collapsed, show badge indicator on icon
                    if !item.is_group_header {
                        if let Some(icon_pos) = icon_center {
                            draw_collapsed_badge(&painter, &theme, icon_pos, badge);
                        }
                    }
                }

                current_y += item_height + ITEM_GAP;
            }
        }

        // Request repaint if animating
        if state.is_animating() {
            ui.ctx().request_repaint();
        }

        let is_expanded = state.open;

        // Save internal state if using uncontrolled mode
        if self.external_state.is_none() {
            ui.ctx().data_mut(|d| {
                d.insert_temp(sidebar_id, internal_state);
            });
        }

        let response = ui.interact(rect, ui.id().with("sidebar"), Sense::hover());

        SidebarResponse {
            response,
            clicked: clicked_id,
            hovered: hovered_index,
            is_expanded,
        }
    }

}

impl Default for Sidebar<'_> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// STANDALONE DRAWING FUNCTIONS
// ============================================================================

#[allow(clippy::too_many_arguments)]
fn draw_group_label(
    painter: &egui::Painter,
    theme: &crate::Theme,
    content_rect: &Rect,
    y: f32,
    current_width: f32,
    collapsed_width: f32,
    expanded_width: f32,
    label: &str,
) {
    let expansion_ratio = ((current_width - collapsed_width)
        / (expanded_width - collapsed_width))
        .clamp(0.0, 1.0);

    if expansion_ratio > 0.5 {
        let opacity = ((expansion_ratio - 0.5) / 0.5).clamp(0.0, 1.0);
        let color = theme.sidebar_foreground().gamma_multiply(0.7 * opacity);

        painter.text(
            Pos2::new(content_rect.left() + ITEM_PADDING, y + ITEM_HEIGHT / 2.0),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(12.0),
            color,
        );
    }
}

/// Draw badge when collapsed (small indicator on icon)
fn draw_collapsed_badge(
    painter: &egui::Painter,
    theme: &crate::Theme,
    icon_center: Pos2,
    _badge: &str,
) {
    // Draw a small dot indicator at top-right of icon
    let badge_pos = Pos2::new(
        icon_center.x + ICON_SIZE / 2.0 - 2.0,
        icon_center.y - ICON_SIZE / 2.0 + 2.0,
    );
    let badge_radius = 4.0;

    // Background circle (destructive color for notification feel)
    painter.circle_filled(badge_pos, badge_radius, theme.destructive());
    // Border
    painter.circle_stroke(
        badge_pos,
        badge_radius,
        Stroke::new(1.0, theme.sidebar()),
    );
}

fn draw_badge(
    painter: &egui::Painter,
    theme: &crate::Theme,
    item_rect: &Rect,
    badge: &str,
    opacity: f32,
) {
    let badge_height = 18.0;
    let badge_padding_x = 6.0;
    let badge_min_width = 18.0;

    // Calculate text width more accurately
    let text_width = badge.len() as f32 * 6.0 + badge_padding_x * 2.0;
    let badge_width = text_width.max(badge_min_width);

    let badge_rect = Rect::from_min_size(
        Pos2::new(
            item_rect.right() - ITEM_PADDING - badge_width,
            item_rect.center().y - badge_height / 2.0,
        ),
        Vec2::new(badge_width, badge_height),
    );

    // Use a more visible background - muted foreground color
    let bg_color = theme.muted().gamma_multiply(opacity);
    let text_color = theme.muted_foreground().gamma_multiply(opacity);

    painter.rect_filled(badge_rect, badge_height / 2.0, bg_color);
    painter.text(
        badge_rect.center(),
        egui::Align2::CENTER_CENTER,
        badge,
        egui::FontId::proportional(11.0),
        text_color,
    );
}
