//! Tabs Component
//!
//! Tab navigation styled like shadcn/ui Tabs.
//! Features a muted background container with animated active indicator.

use egui::{Pos2, Ui, Vec2};

// shadcn Tabs constants
const LIST_HEIGHT: f32 = 36.0; // h-9
const LIST_PADDING: f32 = 3.0; // p-[3px]
const LIST_RADIUS: f32 = 8.0; // rounded-lg
const TRIGGER_RADIUS: f32 = 6.0; // rounded-md
const TRIGGER_PADDING_X: f32 = 8.0; // px-2
const TRIGGER_GAP: f32 = 6.0; // gap-1.5
const FONT_SIZE: f32 = 14.0; // text-sm

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
/// Styled like shadcn/ui Tabs with a muted background and active indicator.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::Tabs;
/// use armas_basic::ext::ArmasContextExt;
///
/// let theme = ui.ctx().armas_theme();
/// let mut tabs = Tabs::new(vec!["Account", "Password"]);
/// let response = tabs.show(ui, &theme);
/// if response.changed {
///     // Tab changed to response.selected
/// }
/// # }
/// ```
pub struct Tabs {
    /// Tab labels
    labels: Vec<String>,
    /// Active tab index
    active_index: usize,
    /// Animate indicator
    animate: bool,
    /// Indicator position for animation
    indicator_pos: f32,
    /// Whether to persist state internally
    persist_state: bool,
}

impl Tabs {
    /// Create new tabs with labels
    pub fn new(labels: Vec<impl Into<String>>) -> Self {
        Self {
            labels: labels.into_iter().map(|l| l.into()).collect(),
            active_index: 0,
            animate: true,
            indicator_pos: 0.0,
            persist_state: true,
        }
    }

    /// Set active tab index
    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index.min(self.labels.len().saturating_sub(1));
        self.indicator_pos = self.active_index as f32;
        self.persist_state = false;
        self
    }

    /// Enable or disable animation
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Show the tabs and return the response
    pub fn show(&mut self, ui: &mut Ui, theme: &crate::Theme) -> TabsResponse {
        if self.labels.is_empty() {
            let (_, empty_response) =
                ui.allocate_exact_size(egui::Vec2::new(0.0, LIST_HEIGHT), egui::Sense::hover());
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

        // Calculate tab widths based on text (approximate: 8px per character)
        let font_id = egui::FontId::proportional(FONT_SIZE);
        let tab_widths: Vec<f32> = self
            .labels
            .iter()
            .map(|label| {
                let text_width = 8.0 * label.len() as f32;
                text_width + TRIGGER_PADDING_X * 2.0
            })
            .collect();

        let total_width: f32 = tab_widths.iter().sum::<f32>()
            + TRIGGER_GAP * (self.labels.len().saturating_sub(1)) as f32
            + LIST_PADDING * 2.0;

        // Allocate space for the TabsList container
        let (list_rect, list_response) =
            ui.allocate_exact_size(Vec2::new(total_width, LIST_HEIGHT), egui::Sense::hover());

        // Draw TabsList background (bg-muted rounded-lg)
        ui.painter()
            .rect_filled(list_rect, LIST_RADIUS, theme.muted());

        let mut selected = None;
        let inner_height = LIST_HEIGHT - LIST_PADDING * 2.0;

        // Calculate cumulative x positions
        let mut x_positions: Vec<f32> = Vec::with_capacity(self.labels.len());
        let mut current_x = list_rect.min.x + LIST_PADDING;
        for (i, width) in tab_widths.iter().enumerate() {
            x_positions.push(current_x);
            current_x += width;
            if i < self.labels.len() - 1 {
                current_x += TRIGGER_GAP;
            }
        }

        // Draw animated active indicator background
        if !tab_widths.is_empty() {
            // Interpolate position and width for smooth animation
            let floor_idx = (self.indicator_pos.floor() as usize).min(tab_widths.len() - 1);
            let ceil_idx = (self.indicator_pos.ceil() as usize).min(tab_widths.len() - 1);
            let t = self.indicator_pos.fract();

            let start_x =
                x_positions[floor_idx] + (x_positions[ceil_idx] - x_positions[floor_idx]) * t;
            let width = tab_widths[floor_idx] + (tab_widths[ceil_idx] - tab_widths[floor_idx]) * t;

            let active_rect = egui::Rect::from_min_size(
                Pos2::new(start_x, list_rect.min.y + LIST_PADDING),
                Vec2::new(width, inner_height),
            );

            // Active tab gets bg-background with subtle shadow
            ui.painter()
                .rect_filled(active_rect, TRIGGER_RADIUS, theme.background());
        }

        // Draw tab triggers
        for (index, label) in self.labels.iter().enumerate() {
            let tab_rect = egui::Rect::from_min_size(
                Pos2::new(x_positions[index], list_rect.min.y + LIST_PADDING),
                Vec2::new(tab_widths[index], inner_height),
            );

            let is_active = index == self.active_index;
            let is_hovered = ui.rect_contains_pointer(tab_rect);

            // Text color: foreground for active, muted-foreground for inactive
            let text_color = if is_active {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };

            // Draw label
            ui.painter().text(
                tab_rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                font_id.clone(),
                text_color,
            );

            // Handle click
            if is_hovered && ui.input(|i| i.pointer.primary_clicked()) {
                selected = Some(index);
                self.active_index = index;
            }
        }

        // Update active if changed
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

// Backwards compatibility alias
#[doc(hidden)]
pub type AnimatedTabs = Tabs;

// Keep TabStyle for backwards compatibility but mark as deprecated
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabStyle {
    #[default]
    Underline,
    Pill,
    Segment,
}
