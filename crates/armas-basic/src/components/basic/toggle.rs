//! Toggle/Switch Components
//!
//! Animated toggle switches and checkboxes styled like shadcn/ui Switch.
//! Provides smooth spring animations and supports:
//! - Switch style (default)
//! - Checkbox style
//! - Labels and descriptions
//! - Disabled state

use crate::animation::SpringAnimation;
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{pos2, vec2, Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

// shadcn Switch dimensions
const SWITCH_WIDTH: f32 = 44.0; // w-11
const SWITCH_HEIGHT: f32 = 24.0; // h-6
const SWITCH_THUMB_SIZE: f32 = 20.0; // h-5 w-5

/// Toggle switch variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleVariant {
    /// Standard toggle switch
    Switch,
    /// Checkbox style
    Checkbox,
}

/// Toggle size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleSize {
    /// Small toggle
    Small,
    /// Medium toggle (default)
    Medium,
    /// Large toggle
    Large,
}

impl ToggleSize {
    const fn dimensions(&self, variant: ToggleVariant) -> (f32, f32) {
        match variant {
            ToggleVariant::Switch => match self {
                Self::Small => (36.0, 20.0),
                Self::Medium => (SWITCH_WIDTH, SWITCH_HEIGHT), // shadcn default
                Self::Large => (52.0, 28.0),
            },
            ToggleVariant::Checkbox => match self {
                Self::Small => (16.0, 16.0),
                Self::Medium => (20.0, 20.0),
                Self::Large => (24.0, 24.0),
            },
        }
    }
}

/// Animated toggle switch component
pub struct Toggle {
    id: Option<egui::Id>,
    variant: ToggleVariant,
    size: ToggleSize,
    label: Option<String>,
    description: Option<String>,
    disabled: bool,
    // Use spring animation for smooth, physics-based toggle animation
    toggle_spring: SpringAnimation,
}

impl Toggle {
    /// Create a new toggle
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            id: None,
            variant: ToggleVariant::Switch,
            size: ToggleSize::Medium,
            label: None,
            description: None,
            disabled: false,
            // Smooth spring animation for natural toggle feel
            toggle_spring: SpringAnimation::new(0.0, 0.0).params(800.0, 30.0),
        }
    }

    /// Set ID for state persistence (useful for demos where toggle is recreated each frame)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the variant
    #[must_use] 
    pub const fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size
    #[must_use] 
    pub const fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Set a label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set a description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set disabled state
    #[must_use] 
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the toggle and return whether it changed
    pub fn show(
        &mut self,
        ui: &mut Ui,
        checked: &mut bool,
        theme: &crate::Theme,
    ) -> ToggleResponse {
        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("toggle_state");
            let (stored_checked, stored_anim): (bool, f32) = ui.ctx().data_mut(|d| {
                d.get_temp(state_id)
                    .unwrap_or((*checked, if *checked { 1.0 } else { 0.0 }))
            });
            *checked = stored_checked;
            self.toggle_spring.value = stored_anim;
        }

        let old_checked = *checked;

        // Update spring animation to match checked state
        let target = if *checked { 1.0 } else { 0.0 };
        self.toggle_spring.set_target(target);

        let dt = ui.input(|i| i.stable_dt);
        self.toggle_spring.update(dt);

        // Request repaint while animating
        if !self.toggle_spring.is_settled(0.001, 0.001) {
            ui.ctx().request_repaint();
        }

        let response = ui
            .horizontal(|ui| {
                // Toggle control
                let (width, height) = self.size.dimensions(self.variant);
                let (rect, mut response) = ui.allocate_exact_size(
                    Vec2::new(width, height),
                    if self.disabled {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );

                if ui.is_rect_visible(rect) {
                    match self.variant {
                        ToggleVariant::Switch => {
                            self.draw_switch(ui, rect, *checked, theme);
                        }
                        ToggleVariant::Checkbox => {
                            self.draw_checkbox(ui, rect, *checked, theme);
                        }
                    }
                }

                // Handle click
                if response.clicked() && !self.disabled {
                    *checked = !*checked;
                    response.mark_changed();
                }

                // Label and description
                if self.label.is_some() || self.description.is_some() {
                    ui.add_space(theme.spacing.sm);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = theme.spacing.xs;
                        if let Some(label) = &self.label {
                            let label_color = if self.disabled {
                                theme.muted_foreground().linear_multiply(0.5)
                            } else {
                                theme.foreground()
                            };

                            ui.label(egui::RichText::new(label).size(14.0).color(label_color));
                        }

                        if let Some(description) = &self.description {
                            ui.label(
                                egui::RichText::new(description)
                                    .size(12.0)
                                    .color(theme.muted_foreground()),
                            );
                        }
                    });
                }

                response
            })
            .inner;

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("toggle_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, (*checked, self.toggle_spring.value));
            });
        }

        ToggleResponse {
            response,
            changed: old_checked != *checked,
        }
    }

    /// Draw a switch-style toggle (shadcn/ui style)
    fn draw_switch(&self, ui: &mut Ui, rect: egui::Rect, checked: bool, theme: &Theme) {
        let painter = ui.painter();
        let t = self.toggle_spring.value;

        // Background track - shadcn uses input color when unchecked, primary when checked
        let bg_color = if self.disabled {
            theme.muted().gamma_multiply(0.5)
        } else if checked {
            theme.primary()
        } else {
            theme.input()
        };

        // Full rounded corners (pill shape)
        let track_radius = rect.height() / 2.0;
        painter.rect_filled(rect, CornerRadius::same(track_radius as u8), bg_color);

        // Focus ring on hover (shadcn style)
        let response = ui.interact(rect, ui.id().with("switch_hover"), Sense::hover());
        if response.hovered() && !self.disabled {
            painter.rect_stroke(
                rect.expand(2.0),
                CornerRadius::same((track_radius + 2.0) as u8),
                Stroke::new(2.0, theme.ring()),
                egui::StrokeKind::Outside,
            );
        }

        // Thumb (sliding circle) - shadcn uses background color for thumb
        let thumb_radius = SWITCH_THUMB_SIZE / 2.0;
        let thumb_padding = 2.0;
        let thumb_travel = rect.width() - SWITCH_THUMB_SIZE - thumb_padding * 2.0;
        let thumb_x = rect.min.x + thumb_padding + thumb_radius + thumb_travel * t;
        let thumb_center = pos2(thumb_x, rect.center().y);

        let thumb_color = if self.disabled {
            theme.muted_foreground()
        } else {
            theme.background()
        };

        // Shadow under thumb
        if !self.disabled {
            painter.circle_filled(
                thumb_center + vec2(0.0, 1.0),
                thumb_radius,
                Color32::from_rgba_unmultiplied(0, 0, 0, 20),
            );
        }

        painter.circle_filled(thumb_center, thumb_radius, thumb_color);
    }

    /// Draw a checkbox-style toggle (shadcn/ui Checkbox style)
    fn draw_checkbox(&self, ui: &mut Ui, rect: egui::Rect, checked: bool, theme: &Theme) {
        let painter = ui.painter();
        let t = self.toggle_spring.value;

        // Background - shadcn uses primary when checked, transparent when unchecked
        let bg_color = if self.disabled {
            theme.muted().gamma_multiply(0.5)
        } else if checked {
            theme.primary()
        } else {
            Color32::TRANSPARENT
        };

        let corner_radius = 4u8; // rounded-sm for checkbox

        painter.rect_filled(rect, CornerRadius::same(corner_radius), bg_color);

        // Border - always visible when unchecked
        let border_color = if self.disabled {
            theme.border()
        } else if checked {
            theme.primary()
        } else {
            theme.border()
        };

        painter.rect_stroke(
            rect,
            CornerRadius::same(corner_radius),
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        // Focus ring on hover
        let response = ui.interact(rect, ui.id().with("checkbox_hover"), Sense::hover());
        if response.hovered() && !self.disabled {
            painter.rect_stroke(
                rect.expand(2.0),
                CornerRadius::same(corner_radius + 2),
                Stroke::new(2.0, theme.ring()),
                egui::StrokeKind::Outside,
            );
        }

        // Checkmark - white on primary background
        if t > 0.0 {
            let scale = t;
            let center = rect.center();
            let size = rect.height() * 0.5 * scale;

            // Draw checkmark as two lines
            let check_start = center + vec2(-size * 0.35, 0.0);
            let check_middle = center + vec2(-size * 0.05, size * 0.3);
            let check_end = center + vec2(size * 0.35, -size * 0.35);

            let check_color = if self.disabled {
                theme.muted_foreground()
            } else {
                theme.primary_foreground() // White on primary
            };

            let stroke_width = if self.size == ToggleSize::Small {
                1.5
            } else {
                2.0
            };

            painter.line_segment(
                [check_start, check_middle],
                Stroke::new(stroke_width, check_color),
            );
            painter.line_segment(
                [check_middle, check_end],
                Stroke::new(stroke_width, check_color),
            );
        }
    }
}

impl Default for Toggle {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from toggle interaction
pub struct ToggleResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether the toggle state changed
    pub changed: bool,
}

// ============================================================================
// NEW CLOSURE-BASED API FOR TOGGLE GROUP
// ============================================================================

/// External state for `ToggleGroup`
///
/// Must be stored by the user and passed to `ToggleGroup::new()`.
/// This is necessary because toggle states must persist across frames
/// and be accessible outside the closure.
#[derive(Default, Clone)]
pub struct ToggleGroupState {
    checked: std::collections::HashMap<String, bool>,
}

impl ToggleGroupState {
    /// Create a new toggle group state
    #[must_use] 
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a toggle is checked
    #[must_use] 
    pub fn is_checked(&self, id: &str) -> bool {
        self.checked.get(id).copied().unwrap_or(false)
    }

    /// Set the checked state of a toggle
    pub fn set_checked(&mut self, id: &str, checked: bool) {
        self.checked.insert(id.to_string(), checked);
    }

    /// Get all toggle states
    #[must_use] 
    pub fn get_all(&self) -> Vec<(String, bool)> {
        self.checked.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }
}

/// Builder for configuring individual toggles in a group
pub struct ToggleBuilder {
    label: String,
    description: Option<String>,
}

impl ToggleBuilder {
    fn new(_id: String, label: String) -> Self {
        Self {
            label,
            description: None,
        }
    }

    /// Set toggle description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Builder for adding toggles to the group
pub struct ToggleGroupBuilder<'a> {
    state: &'a mut ToggleGroupState,
    ui: &'a mut Ui,
    changed: &'a mut Vec<(String, bool)>,
}

impl ToggleGroupBuilder<'_> {
    /// Add a toggle to the group
    pub fn toggle(&mut self, id: &str, label: &str) -> ToggleBuilder {
        let builder = ToggleBuilder::new(id.to_string(), label.to_string());

        // Get current checked state
        let mut checked = self.state.is_checked(id);

        // Create toggle and show it
        let mut toggle = Toggle::new().label(&builder.label);

        if let Some(desc) = &builder.description {
            toggle = toggle.description(desc);
        }

        let theme = self.ui.ctx().armas_theme();
        let response = toggle.show(self.ui, &mut checked, &theme);

        // Update state if changed
        if response.changed {
            self.state.set_checked(id, checked);
            self.changed.push((id.to_string(), checked));
        }

        builder
    }
}

/// Response from toggle group
pub struct ToggleGroupResponse {
    /// List of toggles that changed: (id, `new_state`)
    pub changed: Vec<(String, bool)>,
}

/// Toggle group for managing multiple related toggles (new closure-based API)
///
/// # Example
///
/// ```ignore
/// let mut state = ToggleGroupState::default();
/// ToggleGroup::new(&mut state)
///     .label("Settings")
///     .show(ui, |group| {
///         group.toggle("option1", "Option 1");
///         group.toggle("option2", "Option 2")
///             .description("Enable advanced features");
///     })
/// ```
pub struct ToggleGroup<'a> {
    state: &'a mut ToggleGroupState,
    label: Option<String>,
}

impl<'a> ToggleGroup<'a> {
    /// Create a new toggle group with external state
    pub const fn new(state: &'a mut ToggleGroupState) -> Self {
        Self { state, label: None }
    }

    /// Set a label for the group
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show the toggle group with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut ToggleGroupBuilder) -> R,
    ) -> ToggleGroupResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = Vec::new();

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme.spacing.sm;

            // Group label
            if let Some(label) = &self.label {
                ui.label(
                    egui::RichText::new(label)
                        .size(14.0)
                        .strong()
                        .color(theme.foreground()),
                );
            }

            // Build toggles from closure
            let mut builder = ToggleGroupBuilder {
                state: self.state,
                ui,
                changed: &mut changed,
            };
            content(&mut builder);
        });

        ToggleGroupResponse { changed }
    }
}

// ============================================================================
// OLD API (DEPRECATED - kept for backwards compatibility)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_creation() {
        let toggle = Toggle::new();
        assert_eq!(toggle.variant, ToggleVariant::Switch);
        assert_eq!(toggle.size, ToggleSize::Medium);
        assert!(!toggle.disabled);
    }

    #[test]
    fn test_toggle_builder() {
        let toggle = Toggle::new()
            .variant(ToggleVariant::Checkbox)
            .size(ToggleSize::Large)
            .label("Enable feature")
            .disabled(true);

        assert_eq!(toggle.variant, ToggleVariant::Checkbox);
        assert_eq!(toggle.size, ToggleSize::Large);
        assert_eq!(toggle.label, Some("Enable feature".to_string()));
        assert!(toggle.disabled);
    }

    #[test]
    fn test_toggle_size_dimensions() {
        assert_eq!(
            ToggleSize::Small.dimensions(ToggleVariant::Switch),
            (36.0, 20.0)
        );
        assert_eq!(
            ToggleSize::Medium.dimensions(ToggleVariant::Checkbox),
            (20.0, 20.0)
        );
    }

    #[test]
    fn test_toggle_group() {
        // Test the new closure-based API
        let mut state = ToggleGroupState::new();

        // Simulate setting up toggles
        state.set_checked("option1", false);
        state.set_checked("option2", true);

        let all_states = state.get_all();
        assert_eq!(all_states.len(), 2);

        // Check that both toggles are present with correct values
        assert!(all_states
            .iter()
            .any(|(id, checked)| id == "option1" && !checked));
        assert!(all_states
            .iter()
            .any(|(id, checked)| id == "option2" && *checked));

        state.set_checked("option1", true);
        let all_states = state.get_all();
        assert!(all_states
            .iter()
            .any(|(id, checked)| id == "option1" && *checked));
    }

    #[test]
    fn test_toggle_group_state() {
        let mut state = ToggleGroupState::default();

        // Initially, toggles should be unchecked
        assert!(!state.is_checked("test"));

        // Set checked
        state.set_checked("test", true);
        assert!(state.is_checked("test"));

        // Set unchecked
        state.set_checked("test", false);
        assert!(!state.is_checked("test"));
    }
}
