//! Tabs Component
//!
//! Tab navigation styled like shadcn/ui Tabs.
//! Features a muted background container with animated active indicator.

use crate::ext::ArmasContextExt;
use egui::{Pos2, Ui, Vec2};

// Default constants
const DEFAULT_HEIGHT: f32 = 28.0;
const DEFAULT_PADDING: f32 = 2.0;
const DEFAULT_LIST_RADIUS: f32 = 6.0;
const DEFAULT_TRIGGER_RADIUS: f32 = 4.0;
const DEFAULT_TRIGGER_PADDING_X: f32 = 8.0;
const DEFAULT_GAP: f32 = 4.0;
const DEFAULT_FONT_SIZE: f32 = 13.5;

/// Response from the tabs component
#[derive(Debug, Clone)]
pub struct TabsResponse {
    /// The underlying egui response
    pub response: egui::Response,
    /// The newly selected tab index, if changed this frame
    pub selected: Option<usize>,
    /// Whether the selection changed this frame
    pub changed: bool,
}

/// Tabs component for switching between content sections
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::Tabs;
///
/// let mut tabs = Tabs::new(vec!["Account", "Password"])
///     .height(24.0)
///     .font_size(12.0);
/// let response = tabs.show(ui);
/// if response.changed {
///     // Tab changed to response.selected
/// }
/// # }
/// ```
pub struct Tabs {
    labels: Vec<String>,
    active_index: usize,
    animate: bool,
    indicator_pos: f32,
    persist_state: bool,
    // Visual config
    width: f32,
    height: f32,
    padding: f32,
    list_radius: f32,
    trigger_radius: f32,
    trigger_padding_x: f32,
    gap: f32,
    font_size: f32,
}

impl Tabs {
    /// Create new tabs with labels
    #[must_use]
    pub fn new(labels: Vec<impl Into<String>>) -> Self {
        Self {
            labels: labels.into_iter().map(std::convert::Into::into).collect(),
            active_index: 0,
            animate: true,
            indicator_pos: 0.0,
            persist_state: true,
            width: 0.0,
            height: DEFAULT_HEIGHT,
            padding: DEFAULT_PADDING,
            list_radius: DEFAULT_LIST_RADIUS,
            trigger_radius: DEFAULT_TRIGGER_RADIUS,
            trigger_padding_x: DEFAULT_TRIGGER_PADDING_X,
            gap: DEFAULT_GAP,
            font_size: DEFAULT_FONT_SIZE,
        }
    }

    /// Set active tab index
    #[must_use]
    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index.min(self.labels.len().saturating_sub(1));
        self.indicator_pos = self.active_index as f32;
        self.persist_state = false;
        self
    }

    /// Enable or disable animation
    #[must_use]
    pub const fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Set width (0 = fit content, which is the default)
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set overall height
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set inner padding
    #[must_use]
    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set outer container corner radius
    #[must_use]
    pub const fn list_radius(mut self, radius: f32) -> Self {
        self.list_radius = radius;
        self
    }

    /// Set active indicator corner radius
    #[must_use]
    pub const fn trigger_radius(mut self, radius: f32) -> Self {
        self.trigger_radius = radius;
        self
    }

    /// Set horizontal padding inside each tab trigger
    #[must_use]
    pub const fn trigger_padding_x(mut self, padding: f32) -> Self {
        self.trigger_padding_x = padding;
        self
    }

    /// Set gap between tabs
    #[must_use]
    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set font size
    #[must_use]
    pub const fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Show the tabs and return the response
    pub fn show(&mut self, ui: &mut Ui) -> TabsResponse {
        let theme = ui.ctx().armas_theme();
        if self.labels.is_empty() {
            let (_, empty_response) =
                ui.allocate_exact_size(egui::Vec2::new(0.0, self.height), egui::Sense::hover());
            return TabsResponse {
                response: empty_response,
                selected: None,
                changed: false,
            };
        }

        // Load state if persisting
        if self.persist_state {
            let tabs_id = ui.id().with("tabs_state");
            let (stored_active, stored_indicator): (usize, f32) = ui.ctx().data_mut(|d| {
                d.get_persisted(tabs_id)
                    .unwrap_or((self.active_index, self.active_index as f32))
            });

            if self.active_index == 0 && stored_active > 0 {
                self.active_index = stored_active;
            }
            self.indicator_pos = stored_indicator;
        }

        // Animate
        let dt = ui.input(|i| i.stable_dt);
        if self.animate {
            let target = self.active_index as f32;
            let speed = 12.0;
            self.indicator_pos += (target - self.indicator_pos) * speed * dt;

            if (self.indicator_pos - target).abs() > 0.01 {
                ui.ctx().request_repaint();
            }
        } else {
            self.indicator_pos = self.active_index as f32;
        }

        // Resolve width: 0 = fit content, >0 = fixed/fill
        let font_id = egui::FontId::proportional(self.font_size);
        let explicit_width = if self.width > 0.0 { self.width } else { 0.0 };

        let n = self.labels.len();
        let total_gap = self.gap * n.saturating_sub(1) as f32;

        let tab_widths: Vec<f32> = if explicit_width > 0.0 {
            // Distribute evenly across available space
            let inner = explicit_width - self.padding * 2.0 - total_gap;
            let per_tab = inner / n as f32;
            vec![per_tab; n]
        } else {
            // Fit to content
            let char_width = self.font_size * 0.6;
            self.labels
                .iter()
                .map(|label| {
                    let text_width = char_width * label.len() as f32;
                    text_width + self.trigger_padding_x * 2.0
                })
                .collect()
        };

        let total_width = if explicit_width > 0.0 {
            explicit_width
        } else {
            tab_widths.iter().sum::<f32>() + total_gap + self.padding * 2.0
        };

        // Allocate space for the TabsList container
        let (list_rect, list_response) =
            ui.allocate_exact_size(Vec2::new(total_width, self.height), egui::Sense::hover());

        // Draw TabsList background
        ui.painter()
            .rect_filled(list_rect, self.list_radius, theme.muted());

        let mut selected = None;
        let inner_height = self.height - self.padding * 2.0;

        // Calculate cumulative x positions
        let mut x_positions: Vec<f32> = Vec::with_capacity(self.labels.len());
        let mut current_x = list_rect.min.x + self.padding;
        for (i, width) in tab_widths.iter().enumerate() {
            x_positions.push(current_x);
            current_x += width;
            if i < self.labels.len() - 1 {
                current_x += self.gap;
            }
        }

        // Draw animated active indicator background
        if !tab_widths.is_empty() {
            let floor_idx = (self.indicator_pos.floor() as usize).min(tab_widths.len() - 1);
            let ceil_idx = (self.indicator_pos.ceil() as usize).min(tab_widths.len() - 1);
            let t = self.indicator_pos.fract();

            let start_x =
                x_positions[floor_idx] + (x_positions[ceil_idx] - x_positions[floor_idx]) * t;
            let width = tab_widths[floor_idx] + (tab_widths[ceil_idx] - tab_widths[floor_idx]) * t;

            let active_rect = egui::Rect::from_min_size(
                Pos2::new(start_x, list_rect.min.y + self.padding),
                Vec2::new(width, inner_height),
            );

            ui.painter()
                .rect_filled(active_rect, self.trigger_radius, theme.background());
        }

        // Draw tab triggers
        for (index, label) in self.labels.iter().enumerate() {
            let tab_rect = egui::Rect::from_min_size(
                Pos2::new(x_positions[index], list_rect.min.y + self.padding),
                Vec2::new(tab_widths[index], inner_height),
            );

            let is_active = index == self.active_index;
            let is_hovered = ui.rect_contains_pointer(tab_rect);

            let text_color = if is_active {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };

            ui.painter().text(
                tab_rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                font_id.clone(),
                text_color,
            );

            if is_hovered && ui.input(|i| i.pointer.primary_clicked()) {
                selected = Some(index);
                self.active_index = index;
            }
        }

        let changed = selected.is_some();
        if let Some(new_index) = selected {
            self.active_index = new_index;
        }

        // Persist state
        if self.persist_state {
            let tabs_id = ui.id().with("tabs_state");
            ui.ctx().data_mut(|d| {
                d.insert_persisted(tabs_id, (self.active_index, self.indicator_pos));
            });
        }

        TabsResponse {
            response: list_response,
            selected,
            changed,
        }
    }
}
