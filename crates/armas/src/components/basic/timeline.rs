use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Ui, Vec2};

/// Timeline item with title, description, and optional time
pub struct TimelineItem {
    /// Item title
    pub title: String,
    /// Item description
    pub description: String,
    /// Optional timestamp or date
    pub time: Option<String>,
    /// Icon or emoji
    pub icon: Option<String>,
    /// Custom icon color
    pub icon_color: Option<Color32>,
    /// Whether this item is highlighted/active
    pub highlighted: bool,
}

impl TimelineItem {
    /// Create a new timeline item
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            time: None,
            icon: None,
            icon_color: None,
            highlighted: false,
        }
    }

    /// Set timestamp
    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.time = Some(time.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set icon color
    pub fn icon_color(mut self, color: Color32) -> Self {
        self.icon_color = Some(color);
        self
    }

    /// Set highlighted state
    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }
}

/// Vertical timeline component
///
/// Displays a chronological sequence of events with connecting lines.
/// Perfect for activity logs, progress tracking, and history displays.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::Timeline;
///
/// Timeline::new()
///     .dot_size(12.0)
///     .show(ui, |timeline| {
///         timeline.item("Started", "Project initiated")
///             .time("2 hours ago")
///             .icon("🚀");
///         timeline.item("In Progress", "Working on features")
///             .time("1 hour ago")
///             .highlighted(true);
///     });
/// # }
/// ```
pub struct Timeline {
    /// Dot size
    dot_size: f32,
    /// Line width
    line_width: f32,
    /// Gap between items
    item_gap: f32,
    /// Show connecting lines
    show_lines: bool,
}

impl Timeline {
    /// Create a new timeline
    pub fn new() -> Self {
        Self {
            dot_size: 12.0,
            line_width: 2.0,
            item_gap: 48.0,
            show_lines: true,
        }
    }


    /// Set dot size
    pub fn dot_size(mut self, size: f32) -> Self {
        self.dot_size = size;
        self
    }

    /// Set line width
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// Set gap between items
    pub fn item_gap(mut self, gap: f32) -> Self {
        self.item_gap = gap;
        self
    }

    /// Enable or disable connecting lines
    pub fn show_lines(mut self, show: bool) -> Self {
        self.show_lines = show;
        self
    }

    /// Show the timeline with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut TimelineBuilder) -> R,
    ) {
        let theme = ui.ctx().armas_theme();

        let mut builder = TimelineBuilder {
            ui,
            dot_size: self.dot_size,
            line_width: self.line_width,
            item_gap: self.item_gap,
            show_lines: self.show_lines,
            item_index: 0,
            items: Vec::new(),
        };

        // Collect items first
        content(&mut builder);

        // Now render all items
        builder.render_all(&theme);
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding timeline items
pub struct TimelineBuilder<'a> {
    ui: &'a mut Ui,
    dot_size: f32,
    line_width: f32,
    item_gap: f32,
    show_lines: bool,
    item_index: usize,
    items: Vec<TimelineItemData>,
}

struct TimelineItemData {
    title: String,
    description: String,
    time: Option<String>,
    icon: Option<String>,
    icon_color: Option<Color32>,
    highlighted: bool,
}

impl<'a> TimelineBuilder<'a> {
    /// Add a timeline item
    pub fn item(&mut self, title: &str, description: &str) -> TimelineItemBuilder<'_> {
        let item_data = TimelineItemData {
            title: title.to_string(),
            description: description.to_string(),
            time: None,
            icon: None,
            icon_color: None,
            highlighted: false,
        };

        self.items.push(item_data);
        let current_index = self.item_index;
        self.item_index += 1;

        TimelineItemBuilder {
            items: &mut self.items,
            item_index: current_index,
        }
    }

    fn render_all(&mut self, theme: &crate::Theme) {
        if self.items.is_empty() {
            return;
        }

        let available_width = self.ui.available_width();
        let left_margin = 50.0;
        let content_width = available_width - left_margin - 20.0;

        for (index, item) in self.items.iter().enumerate() {
            let is_last = index == self.items.len() - 1;

            // Calculate item height
            let title_height = 24.0;
            let desc_height = {
                let galley = self.ui.painter().layout_no_wrap(
                    item.description.clone(),
                    egui::FontId::proportional(14.0),
                    theme.on_surface_variant(),
                );
                galley.rect.height().min(100.0)
            };
            let time_height = if item.time.is_some() { 20.0 } else { 0.0 };
            let item_height = title_height + desc_height + time_height + self.item_gap;

            let (rect, _) = self.ui.allocate_exact_size(
                Vec2::new(available_width, item_height),
                egui::Sense::hover(),
            );

            let dot_center = Pos2::new(rect.min.x + 24.0, rect.min.y + 12.0);

            // Draw connecting line to next item
            if self.show_lines && !is_last {
                let line_start = Pos2::new(dot_center.x, dot_center.y + self.dot_size / 2.0);
                let line_end = Pos2::new(dot_center.x, rect.min.y + item_height);

                self.ui.painter().line_segment(
                    [line_start, line_end],
                    egui::Stroke::new(self.line_width, theme.outline_variant()),
                );
            }

            // Draw dot/icon
            let dot_color = if item.highlighted {
                theme.primary()
            } else {
                theme.outline()
            };

            if let Some(icon) = &item.icon {
                // Draw circle background
                self.ui.painter().circle_filled(
                    dot_center,
                    self.dot_size,
                    item.icon_color.unwrap_or(dot_color),
                );

                // Draw icon
                self.ui.painter().text(
                    dot_center,
                    egui::Align2::CENTER_CENTER,
                    icon,
                    egui::FontId::proportional(self.dot_size * 1.2),
                    Color32::WHITE,
                );
            } else {
                // Simple dot
                let dot_size = if item.highlighted {
                    self.dot_size
                } else {
                    self.dot_size * 0.6
                };

                self.ui.painter().circle_filled(dot_center, dot_size, dot_color);

                if item.highlighted {
                    // Outer ring for highlighted items
                    self.ui.painter().circle_stroke(
                        dot_center,
                        dot_size + 3.0,
                        egui::Stroke::new(2.0, dot_color.gamma_multiply(0.5)),
                    );
                }
            }

            // Content area
            let content_x = rect.min.x + left_margin;
            let mut content_y = rect.min.y;

            // Time (if present, show at top right)
            if let Some(time) = &item.time {
                self.ui.painter().text(
                    Pos2::new(rect.max.x - 10.0, content_y),
                    egui::Align2::RIGHT_TOP,
                    time,
                    egui::FontId::proportional(12.0),
                    theme.on_surface_variant(),
                );
            }

            // Title
            let title_color = if item.highlighted {
                theme.on_surface()
            } else {
                theme.on_surface_variant()
            };

            self.ui.painter().text(
                Pos2::new(content_x, content_y),
                egui::Align2::LEFT_TOP,
                &item.title,
                egui::FontId::proportional(16.0),
                title_color,
            );

            content_y += title_height + 4.0;

            // Description
            let desc_galley = self.ui.painter().layout(
                item.description.clone(),
                egui::FontId::proportional(14.0),
                theme.on_surface_variant(),
                content_width,
            );

            self.ui.painter().galley(
                Pos2::new(content_x, content_y),
                desc_galley,
                theme.on_surface_variant(),
            );

            // Add spacing between items
            if !is_last {
                self.ui.add_space(self.item_gap - item_height);
            }
        }
    }
}

/// Builder for chaining item modifiers
pub struct TimelineItemBuilder<'a> {
    items: &'a mut Vec<TimelineItemData>,
    item_index: usize,
}

impl<'a> TimelineItemBuilder<'a> {
    /// Set timestamp
    pub fn time(self, time: &str) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.time = Some(time.to_string());
        }
        self
    }

    /// Set icon
    pub fn icon(self, icon: &str) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.icon = Some(icon.to_string());
        }
        self
    }

    /// Set icon color
    pub fn icon_color(self, color: Color32) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.icon_color = Some(color);
        }
        self
    }

    /// Set highlighted state
    pub fn highlighted(self, highlighted: bool) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.highlighted = highlighted;
        }
        self
    }
}
