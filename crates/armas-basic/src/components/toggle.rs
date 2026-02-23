//! Toggle & Toggle Group Components
//!
//! Toggle: A single pressable button with on/off state (shadcn/ui Toggle).
//! Toggle Group: A group of pressable toggle buttons for selection (shadcn/ui Toggle Group).

use super::content::ContentContext;
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

    const fn font_size(self, typo: &crate::theme::Typography) -> f32 {
        match self {
            Self::Sm => typo.sm,
            Self::Default | Self::Lg => typo.base,
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
            Self::Default | Self::Lg => 6.0,
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
    custom_content_width: Option<f32>,
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
            custom_content_width: None,
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

    /// Set explicit content area width for custom content.
    ///
    /// When using [`show_ui`](Self::show_ui), this controls the inner width.
    /// If not set, defaults to a square layout (width = height).
    #[must_use]
    pub const fn content_width(mut self, width: f32) -> Self {
        self.custom_content_width = Some(width);
        self
    }

    /// Load toggle state from memory if ID is set, update `pressed`.
    fn load_state(&self, ui: &Ui, pressed: &mut bool) {
        if let Some(id) = self.id {
            let state_id = id.with("toggle_state");
            let stored: bool = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(*pressed));
            *pressed = stored;
        }
    }

    /// Save toggle state to memory if ID is set.
    fn save_state(&self, ui: &Ui, pressed: bool) {
        if let Some(id) = self.id {
            let state_id = id.with("toggle_state");
            ui.ctx().data_mut(|d| d.insert_temp(state_id, pressed));
        }
    }

    /// Draw the toggle frame (background, border, focus ring).
    /// Returns `(text_color, hovered)`.
    fn draw_frame(
        &self,
        ui: &Ui,
        rect: egui::Rect,
        response: &Response,
        pressed: bool,
        theme: &crate::Theme,
    ) -> Color32 {
        let painter = ui.painter();
        let hovered = response.hovered() && !self.disabled;
        let item_radius = self.size.corner_radius();
        let corner_radius = CornerRadius::same(item_radius as u8);

        let bg_color = if self.disabled {
            Color32::TRANSPARENT
        } else if pressed || hovered {
            theme.muted()
        } else {
            Color32::TRANSPARENT
        };

        painter.rect_filled(rect, corner_radius, bg_color);

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

        // Focus ring
        if response.has_focus() && !self.disabled {
            painter.rect_stroke(
                rect.expand(2.0),
                corner_radius,
                Stroke::new(2.0, theme.ring()),
                egui::StrokeKind::Outside,
            );
        }

        // Return text color
        if self.disabled {
            theme.muted_foreground().linear_multiply(0.5)
        } else if pressed {
            theme.foreground()
        } else {
            theme.muted_foreground()
        }
    }

    /// Show the toggle button
    ///
    /// `pressed` tracks whether the toggle is in the on/off state.
    pub fn show(self, ui: &mut Ui, pressed: &mut bool) -> ToggleResponse {
        let theme = ui.ctx().armas_theme();

        self.load_state(ui, pressed);
        let old_pressed = *pressed;

        let height = self.size.height();
        let font_size = self.size.font_size(&theme.typography);
        let padding_x = self.size.padding_x();

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
            let text_color = self.draw_frame(ui, rect, &response, *pressed, &theme);

            let text_galley = ui.painter().layout_no_wrap(
                self.label.clone(),
                egui::FontId::proportional(font_size),
                text_color,
            );
            let text_pos = rect.center() - text_galley.size() / 2.0;
            ui.painter()
                .galley(pos2(text_pos.x, text_pos.y), text_galley, text_color);
        }

        let changed = old_pressed != *pressed;
        self.save_state(ui, *pressed);

        ToggleResponse { response, changed }
    }

    /// Show the toggle with custom content instead of a text label.
    ///
    /// The closure receives a `&mut Ui` (with override text color set) and a
    /// [`ContentContext`] with the state-dependent color and font size.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut pressed = false;
    /// Toggle::new("")
    ///     .variant(ToggleVariant::Outline)
    ///     .show_ui(ui, &mut pressed, |ui, ctx| {
    ///         // Render an icon using ctx.color
    ///     });
    /// ```
    pub fn show_ui(
        self,
        ui: &mut Ui,
        pressed: &mut bool,
        content: impl FnOnce(&mut Ui, &ContentContext),
    ) -> ToggleResponse {
        let theme = ui.ctx().armas_theme();

        self.load_state(ui, pressed);
        let old_pressed = *pressed;

        let height = self.size.height();
        let padding_x = self.size.padding_x();

        // Width: use content_width if set, otherwise square
        let inner_width = self
            .custom_content_width
            .unwrap_or(height - padding_x * 2.0);
        let item_width = inner_width + padding_x * 2.0;

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
            let text_color = self.draw_frame(ui, rect, &response, *pressed, &theme);

            let content_rect = rect.shrink2(Vec2::new(padding_x, 0.0));
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            child_ui.style_mut().visuals.override_text_color = Some(text_color);

            let ctx = ContentContext {
                color: text_color,
                font_size: self.size.font_size(&theme.typography),
                is_active: *pressed,
            };
            content(&mut child_ui, &ctx);
        }

        let changed = old_pressed != *pressed;
        self.save_state(ui, *pressed);

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

    const fn font_size(self, typo: &crate::theme::Typography) -> f32 {
        match self {
            Self::Sm => typo.sm,
            Self::Default | Self::Lg => typo.base,
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
            Self::Default | Self::Lg => 6.0,
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
    padding: Option<f32>,
    vertical: bool,
    disabled: bool,
    item_width: Option<f32>,
}

impl ToggleGroup {
    /// Create a new toggle group
    #[must_use]
    pub const fn new(group_type: ToggleGroupType) -> Self {
        Self {
            id: None,
            group_type,
            variant: ToggleGroupVariant::Default,
            size: ToggleGroupSize::Default,
            spacing: 0.0,
            padding: None,
            vertical: false,
            disabled: false,
            item_width: None,
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

    /// Override horizontal padding around each item's text.
    /// When set, this takes precedence over the size-based default padding.
    #[must_use]
    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
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

    /// Set explicit uniform item width.
    ///
    /// Required when using [`show_ui`](Self::show_ui) for proper layout.
    /// For text-based [`show`](Self::show), items auto-size to the widest label.
    #[must_use]
    pub const fn item_width(mut self, width: f32) -> Self {
        self.item_width = Some(width);
        self
    }

    /// Load state from memory if ID is set.
    fn load_state(&self, ui: &Ui, selected: &mut Vec<bool>) {
        if let Some(id) = self.id {
            let state_id = id.with("toggle_group_state");
            let stored: Vec<bool> = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or_else(|| selected.clone()));
            if stored.len() == selected.len() {
                *selected = stored;
            }
        }
    }

    /// Save state to memory if ID is set.
    fn save_state(&self, ui: &Ui, selected: &[bool]) {
        if let Some(id) = self.id {
            let state_id = id.with("toggle_group_state");
            ui.ctx()
                .data_mut(|d| d.insert_temp(state_id, selected.to_vec()));
        }
    }

    /// Handle selection logic when an item is clicked.
    fn handle_click(&self, selected: &mut [bool], index: usize) {
        match self.group_type {
            ToggleGroupType::Single => {
                if selected[index] {
                    selected[index] = false;
                } else {
                    for s in selected.iter_mut() {
                        *s = false;
                    }
                    selected[index] = true;
                }
            }
            ToggleGroupType::Multiple => {
                selected[index] = !selected[index];
            }
        }
    }

    /// Draw the frame (background, border, focus ring) for a single item.
    /// Returns the text color.
    fn draw_item_frame(
        &self,
        ui: &Ui,
        rect: egui::Rect,
        response: &Response,
        is_selected: bool,
        index: usize,
        total: usize,
        theme: &crate::Theme,
    ) -> Color32 {
        let painter = ui.painter();
        let hovered = response.hovered() && !self.disabled;
        let item_radius = self.size.corner_radius();

        // Calculate corner rounding based on spacing and position
        let corner_radius = if self.spacing > 0.0 {
            CornerRadius::same(item_radius as u8)
        } else {
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

        // Background
        let bg_color = if self.disabled {
            Color32::TRANSPARENT
        } else if is_selected || hovered {
            theme.muted()
        } else {
            Color32::TRANSPARENT
        };
        painter.rect_filled(rect, corner_radius, bg_color);

        // Border for outline variant
        if self.variant == ToggleGroupVariant::Outline {
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
            if self.spacing == 0.0 && index > 0 {
                let divider_stroke = Stroke::new(1.0, border_color);
                if self.vertical {
                    painter.line_segment([rect.left_top(), rect.right_top()], divider_stroke);
                } else {
                    painter.line_segment([rect.left_top(), rect.left_bottom()], divider_stroke);
                }
            }
        }

        // Focus ring
        if response.has_focus() && !self.disabled {
            painter.rect_stroke(
                rect.expand(2.0),
                corner_radius,
                Stroke::new(2.0, theme.ring()),
                egui::StrokeKind::Outside,
            );
        }

        // Return text color
        if self.disabled {
            theme.muted_foreground().linear_multiply(0.5)
        } else if is_selected {
            theme.foreground()
        } else {
            theme.muted_foreground()
        }
    }

    /// Set up the group layout, run the inner closure, restore spacing.
    fn with_group_layout<R>(&self, ui: &mut Ui, inner: impl FnOnce(&mut Ui) -> R) -> (Response, R) {
        let layout = if self.vertical {
            egui::Layout::top_down(egui::Align::LEFT)
        } else {
            egui::Layout::left_to_right(egui::Align::Center)
        };

        let prev_spacing = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing = Vec2::ZERO;

        let result = ui.with_layout(layout, |ui| {
            if self.spacing > 0.0 {
                ui.spacing_mut().item_spacing = if self.vertical {
                    vec2(0.0, self.spacing)
                } else {
                    vec2(self.spacing, 0.0)
                };
            } else {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
            }
            inner(ui)
        });

        ui.spacing_mut().item_spacing = prev_spacing;

        (result.response, result.inner)
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
        self.load_state(ui, selected);

        let (response, ()) = self.with_group_layout(ui, |ui| {
            let font_size = self.size.font_size(&theme.typography);
            let padding_x = self.padding.unwrap_or_else(|| self.size.padding_x());
            let height = self.size.height();

            // Pre-measure all items to find uniform width
            let uniform_width = self.item_width.unwrap_or_else(|| {
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
                max_text_width + padding_x * 2.0
            });

            for (i, label) in items.iter().enumerate() {
                let is_selected = selected[i];

                let (rect, item_response) = ui.allocate_exact_size(
                    Vec2::new(uniform_width, height),
                    if self.disabled {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );

                if ui.is_rect_visible(rect) {
                    let text_color = self.draw_item_frame(
                        ui,
                        rect,
                        &item_response,
                        is_selected,
                        i,
                        items.len(),
                        &theme,
                    );

                    let text_galley = ui.painter().layout_no_wrap(
                        label.to_string(),
                        egui::FontId::proportional(font_size),
                        text_color,
                    );
                    let text_pos = rect.center() - text_galley.size() / 2.0;
                    ui.painter()
                        .galley(pos2(text_pos.x, text_pos.y), text_galley, text_color);
                }

                if item_response.clicked() && !self.disabled {
                    self.handle_click(selected, i);
                    changed = true;
                }
            }
        });

        self.save_state(ui, selected);

        ToggleGroupResponse { response, changed }
    }

    /// Show the toggle group with custom content for each item.
    ///
    /// The closure receives the item index, a `&mut Ui`, and a [`ContentContext`].
    /// Use [`item_width`](Self::item_width) to set uniform item width.
    /// If not set, items default to square (height x height).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut selected = vec![false, false, false];
    /// ToggleGroup::new(ToggleGroupType::Single)
    ///     .item_width(40.0)
    ///     .show_ui(ui, 3, &mut selected, |index, ui, ctx| {
    ///         // Render icon for item `index` using ctx.color
    ///     });
    /// ```
    pub fn show_ui(
        self,
        ui: &mut Ui,
        count: usize,
        selected: &mut Vec<bool>,
        render_item: impl Fn(usize, &mut Ui, &ContentContext),
    ) -> ToggleGroupResponse {
        let theme = ui.ctx().armas_theme();
        let mut changed = false;

        selected.resize(count, false);
        self.load_state(ui, selected);

        let height = self.size.height();
        let padding_x = self.padding.unwrap_or_else(|| self.size.padding_x());
        let uniform_width = self.item_width.unwrap_or(height);

        let (response, ()) = self.with_group_layout(ui, |ui| {
            for i in 0..count {
                let is_selected = selected[i];

                let (rect, item_response) = ui.allocate_exact_size(
                    Vec2::new(uniform_width, height),
                    if self.disabled {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );

                if ui.is_rect_visible(rect) {
                    let text_color = self.draw_item_frame(
                        ui,
                        rect,
                        &item_response,
                        is_selected,
                        i,
                        count,
                        &theme,
                    );

                    let content_rect = rect.shrink2(Vec2::new(padding_x, 0.0));
                    let mut child_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(content_rect)
                            .layout(egui::Layout::left_to_right(egui::Align::Center)),
                    );
                    child_ui.style_mut().visuals.override_text_color = Some(text_color);

                    let ctx = ContentContext {
                        color: text_color,
                        font_size: self.size.font_size(&theme.typography),
                        is_active: is_selected,
                    };
                    render_item(i, &mut child_ui, &ctx);
                }

                if item_response.clicked() && !self.disabled {
                    self.handle_click(selected, i);
                    changed = true;
                }
            }
        });

        self.save_state(ui, selected);

        ToggleGroupResponse { response, changed }
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
    fn test_toggle_empty_label() {
        let toggle = Toggle::new("");
        assert_eq!(toggle.label, "");
        assert!(toggle.custom_content_width.is_none());
    }

    #[test]
    fn test_toggle_content_width() {
        let toggle = Toggle::new("").content_width(80.0);
        assert_eq!(toggle.custom_content_width, Some(80.0));
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
        assert!(group.item_width.is_none());
    }

    #[test]
    fn test_toggle_group_item_width() {
        let group = ToggleGroup::new(ToggleGroupType::Single).item_width(60.0);
        assert_eq!(group.item_width, Some(60.0));
    }
}
