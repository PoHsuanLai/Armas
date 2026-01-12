use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Ui, Vec2};

/// Tab style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStyle {
    /// Underline style (default)
    Underline,
    /// Pill/button style
    Pill,
    /// Segment style (connected buttons)
    Segment,
}

/// Animated tabs component
///
/// A tab navigation component with smooth animations and multiple style variants.
/// Perfect for organizing content into switchable sections.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::{AnimatedTabs, TabStyle};
///
/// let mut tabs = AnimatedTabs::new(vec!["Tab 1".to_string(), "Tab 2".to_string()]);
///
/// if let Some(new_index) = tabs.show(ui) {
///     // Tab changed to new_index
/// }
/// # }
/// ```
pub struct AnimatedTabs {
    /// Tab labels
    labels: Vec<String>,
    /// Active tab index
    pub active_index: usize,
    /// Tab style
    style: TabStyle,
    /// Animate indicator
    animate: bool,
    /// Indicator position (0.0 to num_tabs as f32)
    indicator_pos: f32,
    /// Indicator width multiplier
    indicator_width: f32,
    /// Whether to persist state internally (disable when state is managed externally)
    persist_state: bool,
}

impl AnimatedTabs {
    /// Create new animated tabs
    pub fn new(labels: Vec<impl Into<String>>) -> Self {
        Self {
            labels: labels.into_iter().map(|l| l.into()).collect(),
            active_index: 0,
            style: TabStyle::Underline,
            animate: true,
            indicator_pos: 0.0,
            indicator_width: 1.0,
            persist_state: true, // Enable by default
        }
    }

    /// Set active tab index
    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index.min(self.labels.len().saturating_sub(1));
        self.indicator_pos = self.active_index as f32;
        self.persist_state = false; // Disable internal persistence when externally managed
        self
    }

    /// Set tab style
    pub fn style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    /// Enable or disable animation
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Show the tabs and return selected index if changed
    pub fn show(&mut self, ui: &mut Ui) -> Option<usize> {
        let theme = ui.ctx().armas_theme();
        if self.labels.is_empty() {
            return None;
        }

        // Load previous state from egui memory only if internal persistence is enabled
        if self.persist_state {
            let tabs_id = ui.id().with("tabs_state");

            let (stored_active, stored_indicator): (usize, f32) = ui.ctx().data_mut(|d| {
                d.get_persisted(tabs_id)
                    .unwrap_or((self.active_index, self.active_index as f32))
            });

            // Use stored state if no explicit active was set
            if self.active_index == 0 && stored_active > 0 {
                self.active_index = stored_active;
            }
            self.indicator_pos = stored_indicator;
        }

        // Update animation
        let dt = ui.input(|i| i.stable_dt);
        if self.animate {
            let target = self.active_index as f32;
            let speed = 10.0;
            self.indicator_pos += (target - self.indicator_pos) * speed * dt;

            if (self.indicator_pos - target).abs() > 0.01 {
                ui.ctx().request_repaint();
            }
        } else {
            self.indicator_pos = self.active_index as f32;
        }

        let result = match self.style {
            TabStyle::Underline => self.show_underline(ui, &theme),
            TabStyle::Pill => self.show_pill(ui, &theme),
            TabStyle::Segment => self.show_segment(ui, &theme),
        };

        // Store state if changed
        if let Some(new_index) = result {
            self.active_index = new_index;
        }

        // Persist state to egui memory only if internal persistence is enabled
        if self.persist_state {
            let tabs_id = ui.id().with("tabs_state");
            ui.ctx().data_mut(|d| {
                d.insert_persisted(tabs_id, (self.active_index, self.indicator_pos));
            });
        }

        result
    }

    /// Show underline style tabs
    fn show_underline(&mut self, ui: &mut Ui, theme: &Theme) -> Option<usize> {
        let mut selected = None;
        let num_tabs = self.labels.len();
        let available_width = ui.available_width();
        let tab_width = available_width / num_tabs as f32;
        let height = 44.0;

        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(available_width, height), egui::Sense::hover());

        // Draw tabs
        for (index, label) in self.labels.iter().enumerate() {
            let x = rect.min.x + index as f32 * tab_width;
            let tab_rect = egui::Rect::from_min_size(
                Pos2::new(x, rect.min.y),
                Vec2::new(tab_width, height - 2.0),
            );

            let is_active = index == self.active_index;
            let is_hovered = ui.rect_contains_pointer(tab_rect);

            // Background
            if is_hovered && !is_active {
                ui.painter().rect_filled(tab_rect, 0.0, theme.hover());
            }

            // Label
            let label_color = if is_active {
                theme.primary()
            } else {
                theme.on_surface_variant()
            };

            ui.painter().text(
                tab_rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(15.0),
                label_color,
            );

            // Check for click
            if is_hovered && ui.input(|i| i.pointer.primary_clicked()) {
                selected = Some(index);
                self.active_index = index;
            }
        }

        // Animated indicator
        let indicator_x = rect.min.x + self.indicator_pos * tab_width;
        let indicator_width = tab_width * self.indicator_width;
        let indicator_height = 2.0;

        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                Pos2::new(indicator_x, rect.max.y - indicator_height),
                Vec2::new(indicator_width, indicator_height),
            ),
            0.0,
            theme.primary(),
        );

        selected
    }

    /// Show pill style tabs
    fn show_pill(&mut self, ui: &mut Ui, theme: &Theme) -> Option<usize> {
        let mut selected = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for (index, label) in self.labels.iter().enumerate() {
                let is_active = index == self.active_index;

                let (rect, response) =
                    ui.allocate_exact_size(Vec2::new(100.0, 36.0), egui::Sense::click());

                let is_hovered = response.hovered();

                // Background
                let bg_color = if is_active {
                    theme.primary()
                } else if is_hovered {
                    theme.hover()
                } else {
                    theme.surface()
                };

                ui.painter().rect_filled(rect, 18.0, bg_color);

                // Label
                let label_color = if is_active {
                    Color32::WHITE
                } else {
                    theme.on_surface()
                };

                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(14.0),
                    label_color,
                );

                if response.clicked() {
                    selected = Some(index);
                    self.active_index = index;
                }
            }
        });

        selected
    }

    /// Show segment style tabs
    fn show_segment(&mut self, ui: &mut Ui, theme: &Theme) -> Option<usize> {
        let mut selected = None;
        let num_tabs = self.labels.len();
        let segment_width = 120.0;
        let height = 36.0;
        let total_width = segment_width * num_tabs as f32;

        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(total_width, height), egui::Sense::hover());

        // Draw container background
        ui.painter().rect_filled(rect, 4.0, theme.surface());

        // Draw container border
        ui.painter().rect_stroke(
            rect,
            4.0,
            egui::Stroke::new(1.0, theme.outline()),
            egui::StrokeKind::Middle,
        );

        // Draw segments
        for (index, label) in self.labels.iter().enumerate() {
            let x = rect.min.x + index as f32 * segment_width;
            let segment_rect = egui::Rect::from_min_size(
                Pos2::new(x, rect.min.y),
                Vec2::new(segment_width, height),
            );

            let is_active = index == self.active_index;
            let is_hovered = ui.rect_contains_pointer(segment_rect);

            // Active background with animation
            if is_active {
                let active_x = rect.min.x + self.indicator_pos * segment_width;
                let active_rect = egui::Rect::from_min_size(
                    Pos2::new(active_x, rect.min.y + 2.0),
                    Vec2::new(segment_width, height - 4.0),
                );

                ui.painter().rect_filled(active_rect, 3.0, theme.primary());
            }

            // Hover effect
            if is_hovered && !is_active {
                ui.painter()
                    .rect_filled(segment_rect.shrink(2.0), 3.0, theme.hover());
            }

            // Divider (except for last segment) - drawn manually
            if index < num_tabs - 1 {
                let divider_x = segment_rect.max.x;
                ui.painter().line_segment(
                    [
                        Pos2::new(divider_x, rect.min.y + 8.0),
                        Pos2::new(divider_x, rect.max.y - 8.0),
                    ],
                    egui::Stroke::new(1.0, theme.outline_variant()),
                );
            }

            // Label
            let label_color = if is_active {
                Color32::WHITE
            } else {
                theme.on_surface()
            };

            ui.painter().text(
                segment_rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(14.0),
                label_color,
            );

            // Check for click
            if is_hovered && ui.input(|i| i.pointer.primary_clicked()) {
                selected = Some(index);
                self.active_index = index;
            }
        }

        selected
    }
}
