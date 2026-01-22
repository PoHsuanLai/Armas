//! Select/Dropdown Components
//!
//! Searchable dropdown menus with keyboard navigation

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{vec2, Color32, CornerRadius, Key, Response, Sense, Stroke, TextEdit, Ui};

/// A selectable option in a dropdown
#[derive(Clone, Debug)]
pub struct SelectOption {
    /// The value/ID of this option
    pub value: String,
    /// Display label
    pub label: String,
    /// Optional icon (emoji or character)
    pub icon: Option<String>,
    /// Optional description
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

    /// Set an icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set a description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Mark as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

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
    max_height: f32,
    searchable: bool,
}

impl Select {
    /// Create a new select component with options
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
            max_height: 300.0,
            searchable: true,
        }
    }

    /// Create a new select component with closure-based API
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use egui::Ui;
    /// # fn example(ui: &mut Ui) {
    /// use armas::Select;
    ///
    /// Select::build(|select| {
    ///     select.option("apple", "Apple").icon("🍎");
    ///     select.option("banana", "Banana").icon("🍌");
    ///     select.option("cherry", "Cherry")
    ///         .icon("🍒")
    ///         .description("Sweet red fruit");
    /// })
    /// .label("Choose a fruit")
    /// .selected("apple")
    /// .show(ui);
    /// # }
    /// ```
    pub fn build(builder: impl FnOnce(&mut SelectBuilder)) -> Self {
        let mut select_builder = SelectBuilder {
            options: Vec::new(),
        };
        builder(&mut select_builder);
        Self::new(select_builder.options)
    }

    /// Set ID for state persistence (useful for demos where select is recreated each frame)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the selected value
    pub fn selected(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    /// Set a label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set maximum dropdown height
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Enable or disable search
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

    /// Update filtered options based on search text
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

        // Reset highlight if current selection is no longer visible
        if let Some(highlighted_idx) = self.highlighted_index {
            if !self.filtered_indices.contains(&highlighted_idx) {
                self.highlighted_index = self.filtered_indices.first().copied();
            }
        }
    }

    /// Show the select component
    pub fn show(&mut self, ui: &mut Ui) -> SelectResponse {
        let theme = ui.ctx().armas_theme();
        let width = self.width.unwrap_or(200.0);
        let mut changed = false;
        let mut new_value = None;

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("select_state");
            let stored_state: Option<(Option<String>, bool, String, Option<usize>)> =
                ui.ctx().data_mut(|d| d.get_temp(state_id));

            if let Some((selected_value, is_open, search_text, highlighted_index)) = stored_state {
                self.selected_value = selected_value;
                self.is_open = is_open;
                self.search_text = search_text;
                self.highlighted_index = highlighted_index;
                // Update filter based on loaded search text
                if !self.search_text.is_empty() {
                    self.update_filter();
                }
            }
        }

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = theme.spacing.xs;
            // Label
            if let Some(label) = &self.label {
                ui.label(
                    egui::RichText::new(label)
                        .size(14.0)
                        .color(theme.foreground()),
                );
            }

            // Select button
            let button_height = 40.0;
            let desired_size = vec2(width, button_height);
            let (button_rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

            if ui.is_rect_visible(button_rect) {
                let hovered = response.hovered();
                let bg_color = if hovered {
                    let hover = theme.accent();
                    Color32::from_rgba_unmultiplied(hover.r(), hover.g(), hover.b(), 200)
                } else {
                    let surface = theme.muted();
                    Color32::from_rgba_unmultiplied(surface.r(), surface.g(), surface.b(), 180)
                };

                // Background
                ui.painter().rect_filled(
                    button_rect,
                    CornerRadius::same(theme.spacing.corner_radius_small),
                    bg_color,
                );

                // Border
                let border_color = if self.is_open {
                    theme.primary()
                } else if hovered {
                    theme.border()
                } else {
                    theme.border()
                };

                ui.painter().rect_stroke(
                    button_rect,
                    CornerRadius::same(theme.spacing.corner_radius_small),
                    Stroke::new(1.0, border_color),
                    egui::StrokeKind::Outside,
                );

                // Display text
                let display_text = if let Some(selected) = &self.selected_value {
                    self.options
                        .iter()
                        .find(|opt| opt.value == *selected)
                        .map(|opt| opt.label.as_str())
                        .unwrap_or(&self.placeholder)
                } else {
                    &self.placeholder
                };

                let text_color = if self.selected_value.is_some() {
                    theme.foreground()
                } else {
                    theme.muted_foreground()
                };

                let text_pos = button_rect.left_center() + vec2(12.0, 0.0);
                ui.painter().text(
                    text_pos,
                    egui::Align2::LEFT_CENTER,
                    display_text,
                    egui::FontId::proportional(14.0),
                    text_color,
                );

                // Dropdown arrow
                let arrow = if self.is_open { "▲" } else { "▼" };
                let arrow_pos = button_rect.right_center() - vec2(12.0, 0.0);
                ui.painter().text(
                    arrow_pos,
                    egui::Align2::RIGHT_CENTER,
                    arrow,
                    egui::FontId::proportional(12.0),
                    theme.muted_foreground(),
                );
            }

            // Toggle dropdown on click
            if response.clicked() {
                self.is_open = !self.is_open;
                if self.is_open {
                    self.search_text.clear();
                    self.update_filter();
                    self.highlighted_index = self.filtered_indices.first().copied();
                }
            }

            // Show dropdown if open
            if self.is_open {
                let dropdown_response = self.show_dropdown(ui, &theme, button_rect, width);
                if let Some(selected_value) = dropdown_response.selected_value {
                    self.selected_value = Some(selected_value.clone());
                    new_value = Some(selected_value);
                    changed = true;
                    self.is_open = false;
                }
                if dropdown_response.should_close {
                    self.is_open = false;
                }
            }

            // Save state to memory if ID is set
            if let Some(id) = self.id {
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

            SelectResponse {
                response,
                changed,
                selected_value: new_value,
            }
        })
        .inner
    }

    /// Show the dropdown menu
    fn show_dropdown(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        button_rect: egui::Rect,
        width: f32,
    ) -> DropdownResponse {
        let mut selected_value = None;
        let mut should_close = false;

        let dropdown_id = ui.id().with("dropdown");
        let area_response = egui::Area::new(dropdown_id)
            .fixed_pos(button_rect.left_bottom() + vec2(0.0, 4.0))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                let surface = theme.card();
                let bg_color =
                    Color32::from_rgba_unmultiplied(surface.r(), surface.g(), surface.b(), 250);

                egui::Frame::new()
                    .fill(bg_color)
                    .stroke(Stroke::new(1.0, theme.border()))
                    .corner_radius(CornerRadius::same(theme.spacing.corner_radius_small))
                    .inner_margin(theme.spacing.sm)
                    .show(ui, |ui| {
                        ui.set_width(width - 16.0);

                        // Search box (if enabled)
                        if self.searchable {
                            let search_response = ui.add(
                                TextEdit::singleline(&mut self.search_text)
                                    .hint_text("Search...")
                                    .desired_width(width - 32.0)
                                    .frame(true),
                            );

                            if search_response.changed() {
                                self.update_filter();
                                self.highlighted_index = self.filtered_indices.first().copied();
                            }

                            // Auto-focus search on open
                            if ui.input(|i| i.key_pressed(Key::Escape)) {
                                should_close = true;
                            }

                            ui.add_space(theme.spacing.xs);
                            ui.separator();
                            ui.add_space(theme.spacing.xs);
                        }

                        // Options list
                        egui::ScrollArea::vertical()
                            .max_height(self.max_height)
                            .show(ui, |ui| {
                                if self.filtered_indices.is_empty() {
                                    ui.label(
                                        egui::RichText::new("No matches found")
                                            .color(theme.muted_foreground())
                                            .size(14.0),
                                    );
                                } else {
                                    for &option_idx in self.filtered_indices.iter() {
                                        let option = &self.options[option_idx];

                                        if option.disabled {
                                            // Show disabled option
                                            ui.horizontal(|ui| {
                                                ui.spacing_mut().item_spacing.x = theme.spacing.sm;
                                                if let Some(icon) = &option.icon {
                                                    ui.label(
                                                        egui::RichText::new(icon).color(
                                                            theme
                                                                .muted_foreground()
                                                                .linear_multiply(0.5),
                                                        ),
                                                    );
                                                }
                                                ui.label(
                                                    egui::RichText::new(&option.label)
                                                        .color(
                                                            theme
                                                                .muted_foreground()
                                                                .linear_multiply(0.5),
                                                        )
                                                        .size(14.0),
                                                );
                                            });
                                            continue;
                                        }

                                        let is_selected = self
                                            .selected_value
                                            .as_ref()
                                            .map(|v| v == &option.value)
                                            .unwrap_or(false);
                                        let is_highlighted =
                                            self.highlighted_index == Some(option_idx);

                                        let option_height = if option.description.is_some() {
                                            50.0
                                        } else {
                                            36.0
                                        };

                                        let (option_rect, option_response) = ui
                                            .allocate_exact_size(
                                                vec2(width - 32.0, option_height),
                                                Sense::click(),
                                            );

                                        if ui.is_rect_visible(option_rect) {
                                            // Background
                                            if is_highlighted || option_response.hovered() {
                                                ui.painter().rect_filled(
                                                    option_rect,
                                                    CornerRadius::same(
                                                        theme.spacing.corner_radius_small,
                                                    ),
                                                    theme.accent(),
                                                );
                                            }

                                            // Content
                                            let mut content_rect =
                                                option_rect.shrink2(vec2(8.0, 4.0));

                                            // Icon
                                            if let Some(icon) = &option.icon {
                                                let icon_pos = content_rect.left_center();
                                                ui.painter().text(
                                                    icon_pos,
                                                    egui::Align2::LEFT_CENTER,
                                                    icon,
                                                    egui::FontId::proportional(16.0),
                                                    theme.foreground(),
                                                );
                                                content_rect.min.x += 24.0;
                                            }

                                            // Label and description
                                            if let Some(description) = &option.description {
                                                let label_pos = content_rect.left_top();
                                                ui.painter().text(
                                                    label_pos,
                                                    egui::Align2::LEFT_TOP,
                                                    &option.label,
                                                    egui::FontId::proportional(14.0),
                                                    theme.foreground(),
                                                );

                                                let desc_pos = label_pos + vec2(0.0, 18.0);
                                                ui.painter().text(
                                                    desc_pos,
                                                    egui::Align2::LEFT_TOP,
                                                    description,
                                                    egui::FontId::proportional(12.0),
                                                    theme.muted_foreground(),
                                                );
                                            } else {
                                                let label_pos = content_rect.left_center();
                                                ui.painter().text(
                                                    label_pos,
                                                    egui::Align2::LEFT_CENTER,
                                                    &option.label,
                                                    egui::FontId::proportional(14.0),
                                                    theme.foreground(),
                                                );
                                            }

                                            // Checkmark if selected
                                            if is_selected {
                                                let check_pos =
                                                    option_rect.right_center() - vec2(8.0, 0.0);
                                                ui.painter().text(
                                                    check_pos,
                                                    egui::Align2::RIGHT_CENTER,
                                                    "✓",
                                                    egui::FontId::proportional(14.0),
                                                    theme.primary(),
                                                );
                                            }

                                            // Handle click
                                            if option_response.clicked() {
                                                selected_value = Some(option.value.clone());
                                            }

                                            // Update highlight on hover
                                            if option_response.hovered() {
                                                self.highlighted_index = Some(option_idx);
                                            }
                                        }

                                        ui.add_space(2.0);
                                    }
                                }
                            });
                    });
            });

        // Handle keyboard navigation
        ui.input(|i| {
            if i.key_pressed(Key::ArrowDown) {
                if let Some(current) = self.highlighted_index {
                    if let Some(pos) = self.filtered_indices.iter().position(|&idx| idx == current)
                    {
                        if pos + 1 < self.filtered_indices.len() {
                            self.highlighted_index = Some(self.filtered_indices[pos + 1]);
                        }
                    }
                } else {
                    self.highlighted_index = self.filtered_indices.first().copied();
                }
            }

            if i.key_pressed(Key::ArrowUp) {
                if let Some(current) = self.highlighted_index {
                    if let Some(pos) = self.filtered_indices.iter().position(|&idx| idx == current)
                    {
                        if pos > 0 {
                            self.highlighted_index = Some(self.filtered_indices[pos - 1]);
                        }
                    }
                }
            }

            if i.key_pressed(Key::Enter) {
                if let Some(highlighted) = self.highlighted_index {
                    let option = &self.options[highlighted];
                    if !option.disabled {
                        selected_value = Some(option.value.clone());
                    }
                }
            }

            if i.key_pressed(Key::Escape) {
                should_close = true;
            }
        });

        // Close on click outside
        if ui.input(|i| i.pointer.any_click())
            && !area_response
                .response
                .rect
                .contains(ui.input(|i| i.pointer.interact_pos()).unwrap_or_default())
            && !button_rect.contains(ui.input(|i| i.pointer.interact_pos()).unwrap_or_default())
        {
            should_close = true;
        }

        DropdownResponse {
            selected_value,
            should_close,
        }
    }
}

/// Response from the select component
pub struct SelectResponse {
    /// The underlying response
    pub response: Response,
    /// Whether the selection changed
    pub changed: bool,
    /// The newly selected value (if changed)
    pub selected_value: Option<String>,
}

/// Internal response from dropdown
struct DropdownResponse {
    selected_value: Option<String>,
    should_close: bool,
}

/// Builder for creating select options with closure-based API
pub struct SelectBuilder {
    options: Vec<SelectOption>,
}

impl SelectBuilder {
    /// Add an option to the select
    pub fn option(&mut self, value: &str, label: &str) -> SelectOptionBuilder<'_> {
        let option = SelectOption::new(value, label);
        self.options.push(option);
        let current_index = self.options.len() - 1;

        SelectOptionBuilder {
            options: &mut self.options,
            option_index: current_index,
        }
    }
}

/// Builder for chaining option modifiers
pub struct SelectOptionBuilder<'a> {
    options: &'a mut Vec<SelectOption>,
    option_index: usize,
}

impl<'a> SelectOptionBuilder<'a> {
    /// Set an icon for this option
    pub fn icon(self, icon: &str) -> Self {
        if let Some(option) = self.options.get_mut(self.option_index) {
            option.icon = Some(icon.to_string());
        }
        self
    }

    /// Set a description for this option
    pub fn description(self, description: &str) -> Self {
        if let Some(option) = self.options.get_mut(self.option_index) {
            option.description = Some(description.to_string());
        }
        self
    }

    /// Mark this option as disabled
    pub fn disabled(self, disabled: bool) -> Self {
        if let Some(option) = self.options.get_mut(self.option_index) {
            option.disabled = disabled;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option_creation() {
        let option = SelectOption::new("value1", "Label 1")
            .icon("📄")
            .description("This is a description");

        assert_eq!(option.value, "value1");
        assert_eq!(option.label, "Label 1");
        assert_eq!(option.icon, Some("📄".to_string()));
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
