//! Toggle & Toggle Group Components
//!
//! Toggle: A single pressable button with on/off state (shadcn/ui Toggle).
//! Toggle Group: A group of pressable toggle buttons for selection (shadcn/ui Toggle Group).

use crate::ext::ArmasContextExt;
use egui::{pos2, vec2, Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

// ============================================================================
// TOGGLE — shadcn/ui pressable button with on/off state
// ============================================================================

/// Toggle visual variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleVariant {
    /// Transparent background, muted bg when pressed
    #[default]
    Default,
    /// Bordered
    Outline,
}

/// Toggle size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    /// Small: 28px height, 28px min width
    Sm,
    /// Default: 32px height, 32px min width
    #[default]
    Default,
    /// Large: 36px height, 36px min width
    Lg,
}

impl ToggleSize {
    const fn height(self) -> f32 {
        match self {
            Self::Sm => 28.0,
            Self::Default => 32.0,
            Self::Lg => 36.0,
        }
    }

    const fn font_size(self) -> f32 {
        match self {
            Self::Sm => 12.8,
            Self::Default => 14.0,
            Self::Lg => 14.0,
        }
    }

    const fn padding_x(self) -> f32 {
        match self {
            Self::Sm => 8.0,
            Self::Default => 10.0,
            Self::Lg => 12.0,
        }
    }

    const fn corner_radius(self) -> f32 {
        match self {
            Self::Sm => 5.0,
            Self::Default => 6.0,
            Self::Lg => 6.0,
        }
    }
}

/// Response from toggle interaction
pub struct ToggleResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether the toggle state changed this frame
    pub changed: bool,
}

/// A pressable button with on/off state (shadcn/ui Toggle)
///
/// # Example
///
/// ```ignore
/// let mut pressed = false;
/// Toggle::new("Bold")
///     .variant(ToggleVariant::Outline)
///     .show(ui, &mut pressed);
/// ```
pub struct Toggle {
    id: Option<egui::Id>,
    label: String,
    variant: ToggleVariant,
    size: ToggleSize,
    disabled: bool,
}

impl Toggle {
    /// Create a new toggle with the given label
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: None,
            label: label.into(),
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            disabled: false,
        }
    }

    /// Set ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the visual variant
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

    /// Set disabled state
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the toggle button
    ///
    /// `pressed` tracks whether the toggle is in the on/off state.
    pub fn show(self, ui: &mut Ui, pressed: &mut bool) -> ToggleResponse {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("toggle_state");
            let stored: bool = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(*pressed));
            *pressed = stored;
        }

        let old_pressed = *pressed;

        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding_x = self.size.padding_x();
        let item_radius = self.size.corner_radius();

        // Measure text to determine width
        let text_galley = ui.painter().layout_no_wrap(
            self.label.clone(),
            egui::FontId::proportional(font_size),
            theme.foreground(),
        );
        let item_width = text_galley.size().x + padding_x * 2.0;

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(item_width, height),
            if self.disabled {
                Sense::hover()
            } else {
                Sense::click()
            },
        );

        if response.clicked() && !self.disabled {
            *pressed = !*pressed;
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let hovered = response.hovered() && !self.disabled;
            let corner_radius = CornerRadius::same(item_radius as u8);

            // Background color
            let bg_color = if self.disabled {
                Color32::TRANSPARENT
            } else if *pressed {
                theme.muted()
            } else if hovered {
                theme.muted()
            } else {
                Color32::TRANSPARENT
            };

            painter.rect_filled(rect, corner_radius, bg_color);

            // Border for outline variant
            if self.variant == ToggleVariant::Outline {
                let border_color = if self.disabled {
                    theme.border().linear_multiply(0.5)
                } else {
                    theme.input()
                };
                painter.rect_stroke(
                    rect,
                    corner_radius,
                    Stroke::new(1.0, border_color),
                    egui::StrokeKind::Inside,
                );
            }

            // Text
            let text_color = if self.disabled {
                theme.muted_foreground().linear_multiply(0.5)
            } else if *pressed {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };

            let text_galley = painter.layout_no_wrap(
                self.label.clone(),
                egui::FontId::proportional(font_size),
                text_color,
            );
            let text_pos = rect.center() - text_galley.size() / 2.0;
            painter.galley(pos2(text_pos.x, text_pos.y), text_galley, text_color);

            // Focus ring
            if response.has_focus() && !self.disabled {
                painter.rect_stroke(
                    rect.expand(2.0),
                    corner_radius,
                    Stroke::new(2.0, theme.ring()),
                    egui::StrokeKind::Outside,
                );
            }
        }

        let changed = old_pressed != *pressed;

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("toggle_state");
            ui.ctx().data_mut(|d| d.insert_temp(state_id, *pressed));
        }

        ToggleResponse { response, changed }
    }
}

// ============================================================================
// TOGGLE GROUP — shadcn/ui style pressable button group
// ============================================================================

/// Toggle group selection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupType {
    /// Only one item can be selected at a time
    Single,
    /// Multiple items can be selected
    Multiple,
}

/// Toggle group visual variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupVariant {
    /// Transparent background, muted bg when pressed
    #[default]
    Default,
    /// Bordered items
    Outline,
}

/// Toggle group item size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupSize {
    /// Small: 28px height
    Sm,
    /// Default: 32px height
    #[default]
    Default,
    /// Large: 36px height
    Lg,
}

impl ToggleGroupSize {
    const fn height(self) -> f32 {
        match self {
            Self::Sm => 28.0,
            Self::Default => 32.0,
            Self::Lg => 36.0,
        }
    }

    const fn font_size(self) -> f32 {
        match self {
            Self::Sm => 12.8,
            Self::Default => 14.0,
            Self::Lg => 14.0,
        }
    }

    const fn padding_x(self) -> f32 {
        match self {
            Self::Sm => 6.0,
            Self::Default => 8.0,
            Self::Lg => 10.0,
        }
    }

    const fn corner_radius(self) -> f32 {
        match self {
            Self::Sm => 5.0,
            Self::Default => 6.0,
            Self::Lg => 6.0,
        }
    }
}

/// Response from toggle group interaction
pub struct ToggleGroupResponse {
    /// The underlying egui response
    pub response: Response,
    /// Whether the selection changed this frame
    pub changed: bool,
}

/// A group of pressable toggle buttons for selection (shadcn/ui Toggle Group)
///
/// Supports single selection (radio-like) or multiple selection (checkbox-like).
///
/// # Example
///
/// ```ignore
/// // Single selection — clicking one deselects the others
/// let mut selected = vec![true, false, false];
/// ToggleGroup::new(ToggleGroupType::Single)
///     .variant(ToggleGroupVariant::Outline)
///     .show(ui, &["Bold", "Italic", "Underline"], &mut selected);
///
/// // Multiple selection — each item toggles independently
/// let mut selected = vec![false, false, false];
/// ToggleGroup::new(ToggleGroupType::Multiple)
///     .show(ui, &["Bold", "Italic", "Underline"], &mut selected);
/// ```
pub struct ToggleGroup {
    id: Option<egui::Id>,
    group_type: ToggleGroupType,
    variant: ToggleGroupVariant,
    size: ToggleGroupSize,
    spacing: f32,
    vertical: bool,
    disabled: bool,
}

impl ToggleGroup {
    /// Create a new toggle group
    #[must_use]
    pub fn new(group_type: ToggleGroupType) -> Self {
        Self {
            id: None,
            group_type,
            variant: ToggleGroupVariant::Default,
            size: ToggleGroupSize::Default,
            spacing: 0.0,
            vertical: false,
            disabled: false,
        }
    }

    /// Set ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the visual variant
    #[must_use]
    pub const fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size
    #[must_use]
    pub const fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }

    /// Set spacing between items (0 = joined, >0 = separated)
    #[must_use]
    pub const fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set vertical orientation
    #[must_use]
    pub const fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// Set disabled state
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the toggle group
    ///
    /// `selected` is a bool per item. Will be resized to match `items.len()`.
    /// - `Single`: clicking an item deselects all others (radio behavior)
    /// - `Multiple`: each item toggles independently
    pub fn show(
        self,
        ui: &mut Ui,
        items: &[&str],
        selected: &mut Vec<bool>,
    ) -> ToggleGroupResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = false;

        selected.resize(items.len(), false);

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("toggle_group_state");
            let stored: Vec<bool> = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or_else(|| selected.clone()));
            if stored.len() == selected.len() {
                *selected = stored;
            }
        }

        let layout = if self.vertical {
            egui::Layout::top_down(egui::Align::LEFT)
        } else {
            egui::Layout::left_to_right(egui::Align::Center)
        };

        let response = ui
            .with_layout(layout, |ui| {
                if self.spacing > 0.0 {
                    ui.spacing_mut().item_spacing = if self.vertical {
                        vec2(0.0, self.spacing)
                    } else {
                        vec2(self.spacing, 0.0)
                    };
                } else {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                }

                // Pre-measure all items to find uniform width
                let font_size = self.size.font_size();
                let padding_x = self.size.padding_x();
                let max_text_width = items
                    .iter()
                    .map(|label| {
                        ui.painter()
                            .layout_no_wrap(
                                label.to_string(),
                                egui::FontId::proportional(font_size),
                                theme.foreground(),
                            )
                            .size()
                            .x
                    })
                    .fold(0.0_f32, f32::max);
                let uniform_width = max_text_width + padding_x * 2.0;

                for (i, label) in items.iter().enumerate() {
                    let is_selected = selected[i];
                    let item_response = self.draw_item(
                        ui,
                        label,
                        is_selected,
                        i,
                        items.len(),
                        uniform_width,
                        &theme,
                    );

                    if item_response.clicked() && !self.disabled {
                        match self.group_type {
                            ToggleGroupType::Single => {
                                if selected[i] {
                                    // Deselect current
                                    selected[i] = false;
                                } else {
                                    // Deselect all, then select clicked
                                    for s in selected.iter_mut() {
                                        *s = false;
                                    }
                                    selected[i] = true;
                                }
                            }
                            ToggleGroupType::Multiple => {
                                selected[i] = !selected[i];
                            }
                        }
                        changed = true;
                    }
                }
            })
            .response;

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("toggle_group_state");
            ui.ctx()
                .data_mut(|d| d.insert_temp(state_id, selected.clone()));
        }

        ToggleGroupResponse { response, changed }
    }

    /// Draw a single toggle group item
    fn draw_item(
        &self,
        ui: &mut Ui,
        label: &str,
        is_selected: bool,
        index: usize,
        total: usize,
        item_width: f32,
        theme: &crate::Theme,
    ) -> Response {
        let height = self.size.height();
        let font_size = self.size.font_size();
        let item_radius = self.size.corner_radius();

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(item_width, height),
            if self.disabled {
                Sense::hover()
            } else {
                Sense::click()
            },
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let hovered = response.hovered() && !self.disabled;

            // Calculate corner rounding based on spacing and position
            let corner_radius = if self.spacing > 0.0 {
                // Separated: all items get full rounding
                CornerRadius::same(item_radius as u8)
            } else {
                // Joined: only first/last get rounding on their outer edges
                let is_first = index == 0;
                let is_last = index == total - 1;

                if self.vertical {
                    CornerRadius {
                        nw: if is_first { item_radius as u8 } else { 0 },
                        ne: if is_first { item_radius as u8 } else { 0 },
                        sw: if is_last { item_radius as u8 } else { 0 },
                        se: if is_last { item_radius as u8 } else { 0 },
                    }
                } else {
                    CornerRadius {
                        nw: if is_first { item_radius as u8 } else { 0 },
                        sw: if is_first { item_radius as u8 } else { 0 },
                        ne: if is_last { item_radius as u8 } else { 0 },
                        se: if is_last { item_radius as u8 } else { 0 },
                    }
                }
            };

            // Background color
            let bg_color = if self.disabled {
                Color32::TRANSPARENT
            } else if is_selected {
                theme.muted()
            } else if hovered {
                theme.muted()
            } else {
                Color32::TRANSPARENT
            };

            // Draw background
            painter.rect_filled(rect, corner_radius, bg_color);

            // Border for outline variant
            if self.variant == ToggleGroupVariant::Outline {
                let border_color = if self.disabled {
                    theme.border().linear_multiply(0.5)
                } else {
                    theme.input()
                };

                if self.spacing > 0.0 {
                    // Separated: full border on each item
                    painter.rect_stroke(
                        rect,
                        corner_radius,
                        Stroke::new(1.0, border_color),
                        egui::StrokeKind::Inside,
                    );
                } else {
                    // Joined: draw borders carefully to avoid double borders
                    painter.rect_stroke(
                        rect,
                        corner_radius,
                        Stroke::new(1.0, border_color),
                        egui::StrokeKind::Inside,
                    );

                    // Draw inner divider to cover double border between items
                    if index > 0 {
                        let divider_stroke = Stroke::new(1.0, border_color);
                        if self.vertical {
                            painter
                                .line_segment([rect.left_top(), rect.right_top()], divider_stroke);
                        } else {
                            painter.line_segment(
                                [rect.left_top(), rect.left_bottom()],
                                divider_stroke,
                            );
                        }
                    }
                }
            }

            // Text
            let text_color = if self.disabled {
                theme.muted_foreground().linear_multiply(0.5)
            } else if is_selected {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };

            let text_galley = painter.layout_no_wrap(
                label.to_string(),
                egui::FontId::proportional(font_size),
                text_color,
            );
            let text_pos = rect.center() - text_galley.size() / 2.0;
            painter.galley(pos2(text_pos.x, text_pos.y), text_galley, text_color);

            // Focus ring
            if response.has_focus() && !self.disabled {
                painter.rect_stroke(
                    rect.expand(2.0),
                    corner_radius,
                    Stroke::new(2.0, theme.ring()),
                    egui::StrokeKind::Outside,
                );
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_creation() {
        let toggle = Toggle::new("Bold");
        assert_eq!(toggle.label, "Bold");
        assert_eq!(toggle.variant, ToggleVariant::Default);
        assert_eq!(toggle.size, ToggleSize::Default);
        assert!(!toggle.disabled);
    }

    #[test]
    fn test_toggle_builder() {
        let toggle = Toggle::new("Bold")
            .variant(ToggleVariant::Outline)
            .size(ToggleSize::Lg)
            .disabled(true);

        assert_eq!(toggle.variant, ToggleVariant::Outline);
        assert_eq!(toggle.size, ToggleSize::Lg);
        assert!(toggle.disabled);
    }

    #[test]
    fn test_toggle_size_heights() {
        assert_eq!(ToggleSize::Sm.height(), 28.0);
        assert_eq!(ToggleSize::Default.height(), 32.0);
        assert_eq!(ToggleSize::Lg.height(), 36.0);
    }

    #[test]
    fn test_toggle_group_creation() {
        let group = ToggleGroup::new(ToggleGroupType::Single)
            .variant(ToggleGroupVariant::Outline)
            .size(ToggleGroupSize::Sm)
            .spacing(4.0)
            .vertical(true)
            .disabled(true);

        assert_eq!(group.group_type, ToggleGroupType::Single);
        assert_eq!(group.variant, ToggleGroupVariant::Outline);
        assert_eq!(group.size, ToggleGroupSize::Sm);
        assert_eq!(group.spacing, 4.0);
        assert!(group.vertical);
        assert!(group.disabled);
    }

    #[test]
    fn test_toggle_group_size_heights() {
        assert_eq!(ToggleGroupSize::Sm.height(), 28.0);
        assert_eq!(ToggleGroupSize::Default.height(), 32.0);
        assert_eq!(ToggleGroupSize::Lg.height(), 36.0);
    }

    #[test]
    fn test_toggle_group_defaults() {
        let group = ToggleGroup::new(ToggleGroupType::Multiple);
        assert_eq!(group.group_type, ToggleGroupType::Multiple);
        assert_eq!(group.variant, ToggleGroupVariant::Default);
        assert_eq!(group.size, ToggleGroupSize::Default);
        assert_eq!(group.spacing, 0.0);
        assert!(!group.vertical);
        assert!(!group.disabled);
    }
}
