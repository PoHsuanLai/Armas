//! Select Component
//!
//! Searchable dropdown menu styled like shadcn/ui Select.
//! Features:
//! - Keyboard navigation (arrow keys, enter, escape)
//! - Search/filter functionality
//! - Support for icons and descriptions
//! - Disabled options
//! - State persistence

use crate::Theme;
use egui::{
    vec2, Color32, CornerRadius, Key, Painter, Rect, Response, Sense, Stroke, TextEdit, Ui,
};

// ============================================================================
// Constants
// ============================================================================

const TRIGGER_HEIGHT: f32 = 36.0;
const ITEM_HEIGHT: f32 = 32.0;
const ITEM_HEIGHT_WITH_DESC: f32 = 48.0;
const CORNER_RADIUS: u8 = 6;
const CORNER_RADIUS_SM: u8 = 4;
const PADDING: f32 = 8.0;
const ICON_WIDTH: f32 = 24.0;

// ============================================================================
// SelectOption
// ============================================================================

/// A selectable option in a dropdown
#[derive(Clone, Debug)]
pub struct SelectOption {
    /// Option value (internal identifier)
    pub value: String,
    /// Option label (displayed text)
    pub label: String,
    /// Optional icon identifier
    pub icon: Option<String>,
    /// Optional description text
    pub description: Option<String>,
    /// Whether this option is disabled
    pub disabled: bool,
}

impl SelectOption {
    /// Create a new select option
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            description: None,
            disabled: false,
        }
    }

    /// Set an icon for this option
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set a description for this option
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set whether this option is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// ============================================================================
// Select
// ============================================================================

/// Searchable dropdown select component
pub struct Select {
    id: Option<egui::Id>,
    options: Vec<SelectOption>,
    selected_value: Option<String>,
    is_open: bool,
    search_text: String,
    filtered_indices: Vec<usize>,
    highlighted_index: Option<usize>,
    label: Option<String>,
    placeholder: String,
    width: Option<f32>,
    custom_height: Option<f32>,
    max_height: f32,
    searchable: bool,
}

impl Select {
    /// Create a new Select component with the given options
    pub fn new(options: Vec<SelectOption>) -> Self {
        let filtered_indices: Vec<usize> = (0..options.len()).collect();
        Self {
            id: None,
            options,
            selected_value: None,
            is_open: false,
            search_text: String::new(),
            filtered_indices,
            highlighted_index: None,
            label: None,
            placeholder: "Select an option...".to_string(),
            width: None,
            custom_height: None,
            max_height: 300.0,
            searchable: true,
        }
    }

    /// Build a Select using a closure-based API (prefer using `Select::new()`)
    pub fn build(builder: impl FnOnce(&mut SelectBuilder)) -> Self {
        let mut select_builder = SelectBuilder {
            options: Vec::new(),
        };
        builder(&mut select_builder);
        Self::new(select_builder.options)
    }

    /// Set a unique identifier for this select component
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the initially selected value
    pub fn selected(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    /// Set a label for the select component
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set placeholder text shown when no option is selected
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the width of the select component
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the trigger height (overrides default 36px)
    pub fn height(mut self, height: f32) -> Self {
        self.custom_height = Some(height);
        self
    }

    /// Set the maximum height of the dropdown menu
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Enable or disable search functionality
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// Get the currently selected value
    pub fn selected_value(&self) -> Option<&str> {
        self.selected_value.as_deref()
    }

    /// Set the selected value programmatically
    pub fn set_selected(&mut self, value: Option<String>) {
        self.selected_value = value;
    }

    // ========================================================================
    // Main show method
    // ========================================================================

    /// Show the Select component
    pub fn show(&mut self, ui: &mut Ui, theme: &crate::Theme) -> SelectResponse {
        let width = self.width.unwrap_or(200.0);
        let mut changed = false;
        let mut new_value = None;

        self.load_state(ui);

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme.spacing.xs;

            self.show_label(ui, theme);
            let (button_rect, response) = self.show_trigger(ui, theme, width);

            if response.clicked() {
                self.toggle_dropdown();
            }

            if self.is_open {
                let dropdown_response = self.show_dropdown(ui, theme, button_rect, width);
                if let Some(value) = dropdown_response.selected_value {
                    self.selected_value = Some(value.clone());
                    new_value = Some(value);
                    changed = true;
                    self.is_open = false;
                }
                if dropdown_response.should_close {
                    self.is_open = false;
                }
            }

            self.save_state(ui);

            SelectResponse {
                response,
                changed,
                selected_value: new_value,
                is_open: self.is_open,
            }
        })
        .inner
    }

    // ========================================================================
    // State persistence
    // ========================================================================

    fn load_state(&mut self, ui: &Ui) {
        let Some(id) = self.id else { return };
        let state_id = id.with("select_state");
        let stored: Option<(Option<String>, bool, String, Option<usize>)> =
            ui.ctx().data_mut(|d| d.get_temp(state_id));

        if let Some((selected_value, is_open, search_text, highlighted_index)) = stored {
            self.selected_value = selected_value;
            self.is_open = is_open;
            self.search_text = search_text;
            self.highlighted_index = highlighted_index;
            if !self.search_text.is_empty() {
                self.update_filter();
            }
        }
    }

    fn save_state(&self, ui: &Ui) {
        let Some(id) = self.id else { return };
        let state_id = id.with("select_state");
        ui.ctx().data_mut(|d| {
            d.insert_temp(
                state_id,
                (
                    self.selected_value.clone(),
                    self.is_open,
                    self.search_text.clone(),
                    self.highlighted_index,
                ),
            );
        });
    }

    // ========================================================================
    // Trigger button
    // ========================================================================

    fn show_label(&self, ui: &mut Ui, theme: &Theme) {
        if let Some(label) = &self.label {
            ui.label(
                egui::RichText::new(label)
                    .size(14.0)
                    .color(theme.foreground()),
            );
        }
    }

    fn show_trigger(&self, ui: &mut Ui, theme: &Theme, width: f32) -> (Rect, Response) {
        let height = self.custom_height.unwrap_or(TRIGGER_HEIGHT);
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());

        if ui.is_rect_visible(rect) {
            self.paint_trigger(ui.painter(), rect, &response, theme, height);
        }

        (rect, response)
    }

    fn paint_trigger(&self, painter: &Painter, rect: Rect, response: &Response, theme: &Theme, height: f32) {
        let hovered = response.hovered();
        let is_focused = self.is_open;
        let corner_radius = CornerRadius::same(CORNER_RADIUS);

        // Background
        let bg_color = if hovered && !is_focused {
            let input = theme.input();
            Color32::from_rgba_unmultiplied(input.r(), input.g(), input.b(), 128)
        } else {
            Color32::TRANSPARENT
        };
        painter.rect_filled(rect, corner_radius, bg_color);

        // Border
        let border_color = if is_focused {
            theme.ring()
        } else {
            theme.input()
        };
        painter.rect_stroke(
            rect,
            corner_radius,
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        // Focus ring
        if is_focused {
            let ring_color = {
                let r = theme.ring();
                Color32::from_rgba_unmultiplied(r.r(), r.g(), r.b(), 128)
            };
            painter.rect_stroke(
                rect.expand(2.0),
                CornerRadius::same(8),
                Stroke::new(2.0, ring_color),
                egui::StrokeKind::Outside,
            );
        }

        // Scale font and padding for small heights
        let font_size = if height < 30.0 {
            (height * 0.55).max(8.0)
        } else {
            14.0
        };
        let padding_x = if height < 30.0 {
            (height * 0.3).max(4.0)
        } else {
            12.0
        };

        // Display text
        let display_text = self.get_display_text();
        let text_color = if self.selected_value.is_some() {
            theme.foreground()
        } else {
            theme.muted_foreground()
        };
        painter.text(
            rect.left_center() + vec2(padding_x, 0.0),
            egui::Align2::LEFT_CENTER,
            display_text,
            egui::FontId::proportional(font_size),
            text_color,
        );

        // Solid triangle indicator
        let tri_size = if height < 30.0 {
            (height * 0.15).max(2.5)
        } else {
            4.0
        };
        let center = rect.right_center() - vec2(padding_x + tri_size, 0.0);
        let triangle = if self.is_open {
            // Pointing up
            vec![
                egui::pos2(center.x, center.y - tri_size),
                egui::pos2(center.x - tri_size, center.y + tri_size * 0.6),
                egui::pos2(center.x + tri_size, center.y + tri_size * 0.6),
            ]
        } else {
            // Pointing down
            vec![
                egui::pos2(center.x, center.y + tri_size),
                egui::pos2(center.x - tri_size, center.y - tri_size * 0.6),
                egui::pos2(center.x + tri_size, center.y - tri_size * 0.6),
            ]
        };
        painter.add(egui::Shape::convex_polygon(
            triangle,
            theme.muted_foreground(),
            Stroke::NONE,
        ));
    }

    fn get_display_text(&self) -> &str {
        if let Some(selected) = &self.selected_value {
            self.options
                .iter()
                .find(|opt| opt.value == *selected)
                .map(|opt| opt.label.as_str())
                .unwrap_or(&self.placeholder)
        } else {
            &self.placeholder
        }
    }

    fn toggle_dropdown(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.search_text.clear();
            self.update_filter();
            self.highlighted_index = self.filtered_indices.first().copied();
        }
    }

    // ========================================================================
    // Dropdown
    // ========================================================================

    fn show_dropdown(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        button_rect: Rect,
        width: f32,
    ) -> DropdownResponse {
        let mut selected_value = None;
        let mut should_close = false;

        let dropdown_id = ui.id().with("dropdown");
        let area_response = egui::Area::new(dropdown_id)
            .fixed_pos(button_rect.left_bottom() + vec2(0.0, 4.0))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(theme.popover())
                    .stroke(Stroke::new(1.0, theme.border()))
                    .corner_radius(CornerRadius::same(CORNER_RADIUS))
                    .inner_margin(4.0)
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 4],
                        blur: 8,
                        spread: 0,
                        color: Color32::from_black_alpha(60),
                    })
                    .show(ui, |ui| {
                        ui.set_width(width - 8.0);

                        if self.searchable {
                            should_close |= self.show_search_box(ui, width);
                            self.show_separator(ui, theme, width);
                        }

                        self.show_options_list(ui, theme, width, &mut selected_value);
                    });
            });

        should_close |= self.handle_keyboard_input(ui, &mut selected_value);
        should_close |=
            self.should_close_on_click_outside(ui, &area_response.response, button_rect);

        DropdownResponse {
            selected_value,
            should_close,
        }
    }

    fn show_search_box(&mut self, ui: &mut Ui, width: f32) -> bool {
        let search_response = ui.add(
            TextEdit::singleline(&mut self.search_text)
                .hint_text("Search...")
                .desired_width(width - 16.0)
                .frame(true),
        );

        if search_response.changed() {
            self.update_filter();
            self.highlighted_index = self.filtered_indices.first().copied();
        }

        ui.input(|i| i.key_pressed(Key::Escape))
    }

    fn show_separator(&self, ui: &mut Ui, theme: &Theme, width: f32) {
        ui.add_space(4.0);
        let sep_rect = ui.available_rect_before_wrap();
        let sep_rect = Rect::from_min_size(sep_rect.min, vec2(width - 16.0, 1.0));
        ui.painter().rect_filled(sep_rect, 0.0, theme.border());
        ui.allocate_space(vec2(width - 16.0, 1.0));
        ui.add_space(4.0);
    }

    fn show_options_list(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        width: f32,
        selected_value: &mut Option<String>,
    ) {
        egui::ScrollArea::vertical()
            .max_height(self.max_height)
            .show(ui, |ui| {
                if self.filtered_indices.is_empty() {
                    ui.label(
                        egui::RichText::new("No results found.")
                            .color(theme.muted_foreground())
                            .size(14.0),
                    );
                    return;
                }

                let indices = self.filtered_indices.clone();
                for option_idx in indices {
                    let option = self.options[option_idx].clone();

                    if option.disabled {
                        self.show_disabled_option(ui, &option, theme, width);
                    } else if let Some(value) =
                        self.show_option(ui, &option, option_idx, theme, width)
                    {
                        *selected_value = Some(value);
                    }
                }
            });
    }

    fn item_height(&self) -> f32 {
        self.custom_height.unwrap_or(ITEM_HEIGHT)
    }

    fn item_font_size(&self) -> f32 {
        let h = self.item_height();
        if h < 30.0 { (h * 0.55).max(8.0) } else { 14.0 }
    }

    fn show_disabled_option(&self, ui: &mut Ui, option: &SelectOption, theme: &Theme, width: f32) {
        let (rect, _) = ui.allocate_exact_size(vec2(width - 16.0, self.item_height()), Sense::hover());

        if !ui.is_rect_visible(rect) {
            return;
        }

        let content_rect = rect.shrink2(vec2(PADDING, 0.0));
        let color = theme.muted_foreground().linear_multiply(0.5);
        let mut label_x = 0.0;

        let font_size = self.item_font_size();
        if let Some(icon) = &option.icon {
            ui.painter().text(
                content_rect.left_center(),
                egui::Align2::LEFT_CENTER,
                icon,
                egui::FontId::proportional(font_size),
                color,
            );
            label_x = ICON_WIDTH;
        }

        ui.painter().text(
            content_rect.left_center() + vec2(label_x, 0.0),
            egui::Align2::LEFT_CENTER,
            &option.label,
            egui::FontId::proportional(font_size),
            color,
        );
    }

    fn show_option(
        &mut self,
        ui: &mut Ui,
        option: &SelectOption,
        option_idx: usize,
        theme: &Theme,
        width: f32,
    ) -> Option<String> {
        let is_highlighted = self.highlighted_index == Some(option_idx);
        let base_height = self.item_height();
        let height = if option.description.is_some() {
            base_height + (ITEM_HEIGHT_WITH_DESC - ITEM_HEIGHT)
        } else {
            base_height
        };

        let (rect, response) = ui.allocate_exact_size(vec2(width - 16.0, height), Sense::click());

        if !ui.is_rect_visible(rect) {
            return None;
        }

        let is_active = is_highlighted || response.hovered();

        // Background
        if is_active {
            ui.painter()
                .rect_filled(rect, CornerRadius::same(CORNER_RADIUS_SM), theme.accent());
        }

        // Content
        let text_color = if is_active {
            theme.accent_foreground()
        } else {
            theme.popover_foreground()
        };
        self.paint_option_content(ui.painter(), rect, option, text_color, theme);

        // Update highlight on hover
        if response.hovered() {
            self.highlighted_index = Some(option_idx);
        }

        if response.clicked() {
            Some(option.value.clone())
        } else {
            None
        }
    }

    fn paint_option_content(
        &self,
        painter: &Painter,
        rect: Rect,
        option: &SelectOption,
        text_color: Color32,
        theme: &Theme,
    ) {
        let font_size = self.item_font_size();
        let content_rect = rect.shrink2(vec2(PADDING, 0.0));
        let mut label_x = 0.0;

        // Icon
        if let Some(icon) = &option.icon {
            painter.text(
                content_rect.left_center(),
                egui::Align2::LEFT_CENTER,
                icon,
                egui::FontId::proportional(font_size),
                text_color,
            );
            label_x = ICON_WIDTH;
        }

        // Label and description
        if let Some(description) = &option.description {
            let label_pos = content_rect.left_top() + vec2(label_x, 6.0);
            painter.text(
                label_pos,
                egui::Align2::LEFT_TOP,
                &option.label,
                egui::FontId::proportional(font_size),
                text_color,
            );
            painter.text(
                label_pos + vec2(0.0, font_size + 4.0),
                egui::Align2::LEFT_TOP,
                description,
                egui::FontId::proportional((font_size - 2.0).max(8.0)),
                theme.muted_foreground(),
            );
        } else {
            painter.text(
                content_rect.left_center() + vec2(label_x, 0.0),
                egui::Align2::LEFT_CENTER,
                &option.label,
                egui::FontId::proportional(font_size),
                text_color,
            );
        }

    }

    // ========================================================================
    // Keyboard & input handling
    // ========================================================================

    fn handle_keyboard_input(&mut self, ui: &Ui, selected_value: &mut Option<String>) -> bool {
        let mut should_close = false;

        ui.input(|i| {
            if i.key_pressed(Key::ArrowDown) {
                self.move_highlight(1);
            }
            if i.key_pressed(Key::ArrowUp) {
                self.move_highlight(-1);
            }
            if i.key_pressed(Key::Enter) {
                if let Some(idx) = self.highlighted_index {
                    let option = &self.options[idx];
                    if !option.disabled {
                        *selected_value = Some(option.value.clone());
                    }
                }
            }
            if i.key_pressed(Key::Escape) {
                should_close = true;
            }
        });

        should_close
    }

    fn move_highlight(&mut self, delta: i32) {
        let Some(current) = self.highlighted_index else {
            self.highlighted_index = self.filtered_indices.first().copied();
            return;
        };

        let Some(pos) = self.filtered_indices.iter().position(|&idx| idx == current) else {
            return;
        };

        let new_pos =
            (pos as i32 + delta).clamp(0, self.filtered_indices.len() as i32 - 1) as usize;
        self.highlighted_index = Some(self.filtered_indices[new_pos]);
    }

    fn should_close_on_click_outside(
        &self,
        ui: &Ui,
        area_response: &Response,
        button_rect: Rect,
    ) -> bool {
        let clicked = ui.input(|i| i.pointer.any_click());
        let pointer_pos = ui.input(|i| i.pointer.interact_pos()).unwrap_or_default();

        clicked && !area_response.rect.contains(pointer_pos) && !button_rect.contains(pointer_pos)
    }

    // ========================================================================
    // Filtering
    // ========================================================================

    fn update_filter(&mut self) {
        if self.search_text.is_empty() {
            self.filtered_indices = (0..self.options.len()).collect();
        } else {
            let search_lower = self.search_text.to_lowercase();
            self.filtered_indices = self
                .options
                .iter()
                .enumerate()
                .filter(|(_, opt)| {
                    opt.label.to_lowercase().contains(&search_lower)
                        || opt.value.to_lowercase().contains(&search_lower)
                        || opt
                            .description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&search_lower))
                            .unwrap_or(false)
                })
                .map(|(idx, _)| idx)
                .collect();
        }

        // Reset highlight if no longer visible
        if let Some(idx) = self.highlighted_index {
            if !self.filtered_indices.contains(&idx) {
                self.highlighted_index = self.filtered_indices.first().copied();
            }
        }
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Response from showing a Select component
pub struct SelectResponse {
    /// The egui Response for the trigger button
    pub response: Response,
    /// Whether the selected value changed this frame
    pub changed: bool,
    /// The newly selected value, if changed
    pub selected_value: Option<String>,
    /// Whether the dropdown is currently open
    pub is_open: bool,
}

/// Internal response for dropdown interactions
struct DropdownResponse {
    selected_value: Option<String>,
    should_close: bool,
}

// ============================================================================
// Builder (internal helper)
// ============================================================================

/// Internal builder for Select options (prefer using `Select::new()` directly)
#[doc(hidden)]
pub struct SelectBuilder {
    options: Vec<SelectOption>,
}

impl SelectBuilder {
    /// Add an option to the builder
    pub fn option(&mut self, value: &str, label: &str) -> SelectOptionBuilder<'_> {
        self.options.push(SelectOption::new(value, label));
        let idx = self.options.len() - 1;
        SelectOptionBuilder {
            options: &mut self.options,
            option_index: idx,
        }
    }
}

/// Internal builder for configuring a SelectOption
#[doc(hidden)]
pub struct SelectOptionBuilder<'a> {
    options: &'a mut Vec<SelectOption>,
    option_index: usize,
}

impl<'a> SelectOptionBuilder<'a> {
    /// Set an icon for this option
    pub fn icon(self, icon: &str) -> Self {
        if let Some(opt) = self.options.get_mut(self.option_index) {
            opt.icon = Some(icon.to_string());
        }
        self
    }

    /// Set a description for this option
    pub fn description(self, description: &str) -> Self {
        if let Some(opt) = self.options.get_mut(self.option_index) {
            opt.description = Some(description.to_string());
        }
        self
    }

    /// Set whether this option is disabled
    pub fn disabled(self, disabled: bool) -> Self {
        if let Some(opt) = self.options.get_mut(self.option_index) {
            opt.disabled = disabled;
        }
        self
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option_creation() {
        let option = SelectOption::new("value1", "Label 1")
            .icon("x")
            .description("This is a description");

        assert_eq!(option.value, "value1");
        assert_eq!(option.label, "Label 1");
        assert_eq!(option.icon, Some("x".to_string()));
        assert_eq!(
            option.description,
            Some("This is a description".to_string())
        );
        assert!(!option.disabled);
    }

    #[test]
    fn test_select_creation() {
        let options = vec![
            SelectOption::new("1", "Option 1"),
            SelectOption::new("2", "Option 2"),
        ];

        let select = Select::new(options);
        assert_eq!(select.options.len(), 2);
        assert!(select.selected_value.is_none());
        assert!(!select.is_open);
    }

    #[test]
    fn test_select_filtering() {
        let options = vec![
            SelectOption::new("apple", "Apple"),
            SelectOption::new("banana", "Banana"),
            SelectOption::new("cherry", "Cherry"),
        ];

        let mut select = Select::new(options);
        select.search_text = "app".to_string();
        select.update_filter();

        assert_eq!(select.filtered_indices.len(), 1);
        assert_eq!(select.filtered_indices[0], 0);
    }
}
