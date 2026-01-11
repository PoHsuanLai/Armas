use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Pos2, Response, Ui, Vec2};

/// An accordion item with title and collapsible content
pub struct AccordionItem {
    /// Item title
    title: String,
    /// Optional ID for state persistence
    id: Option<egui::Id>,
    /// Whether the item is open
    is_open: bool,
    /// Show chevron icon
    show_icon: bool,
    /// Animate expansion
    animate: bool,
    /// Animation progress (0.0 to 1.0)
    animation_t: f32,
}

impl AccordionItem {
    /// Create a new accordion item
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            id: None,
            is_open: false,
            show_icon: true,
            animate: true,
            animation_t: 0.0,
        }
    }

    /// Set ID for state persistence (useful for demos where item is recreated each frame)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set initial open state
    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self.animation_t = if open { 1.0 } else { 0.0 };
        self
    }

    /// Enable or disable chevron icon
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Enable or disable animation
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Show the accordion item
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> (bool, R) {
        let theme = ui.ctx().armas_theme();

        // Load state from memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("accordion_state");
            let (stored_open, stored_anim): (bool, f32) = ui.ctx().data_mut(|d| {
                d.get_temp(state_id)
                    .unwrap_or((self.is_open, self.animation_t))
            });
            self.is_open = stored_open;
            self.animation_t = stored_anim;
        }

        let header_response = self.show_header(ui, &theme);

        // Toggle on click
        if header_response.clicked() {
            self.is_open = !self.is_open;
        }

        // Update animation
        let dt = ui.input(|i| i.stable_dt);
        let target = if self.is_open { 1.0 } else { 0.0 };

        if self.animate {
            let speed = 8.0;
            self.animation_t += (target - self.animation_t) * speed * dt;
            self.animation_t = self.animation_t.clamp(0.0, 1.0);
        } else {
            self.animation_t = target;
        }

        // Show content with animation
        let content_result = if self.animation_t > 0.01 {
            let max_height = 1000.0; // Maximum content height for animation
            let visible_height = max_height * self.animation_t;

            // Create a clipped area for the content
            let content_rect = ui.available_rect_before_wrap();
            let clipped_rect = egui::Rect::from_min_size(
                content_rect.min,
                Vec2::new(content_rect.width(), visible_height),
            );

            let mut content_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(clipped_rect)
                    .layout(*ui.layout()),
            );
            content_ui.set_clip_rect(clipped_rect);
            content_ui.add_space(8.0);

            let result = content(&mut content_ui);

            content_ui.add_space(8.0);
            result
        } else {
            // Don't render content at all when fully closed
            // We need to return a default value, so we create a temporary UI
            let temp_rect = egui::Rect::from_min_size(Pos2::ZERO, Vec2::ZERO);
            let mut content_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(temp_rect)
                    .layout(*ui.layout()),
            );
            content_ui.set_clip_rect(temp_rect);
            content(&mut content_ui)
        };

        // Request repaint if animating
        if (self.animation_t - target).abs() > 0.01 {
            ui.ctx().request_repaint();
        }

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("accordion_state");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, (self.is_open, self.animation_t));
            });
        }

        (header_response.clicked(), content_result)
    }

    /// Show the header
    fn show_header(&self, ui: &mut Ui, theme: &Theme) -> Response {
        let desired_size = Vec2::new(ui.available_width(), 44.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        let is_hovered = response.hovered();

        // Background
        let bg_color = if is_hovered {
            theme.hover()
        } else {
            theme.surface()
        };

        ui.painter().rect_filled(rect, 0.0, bg_color);

        // Bottom border
        let border_y = rect.max.y;
        ui.painter().line_segment(
            [
                Pos2::new(rect.min.x, border_y),
                Pos2::new(rect.max.x, border_y),
            ],
            egui::Stroke::new(1.0, theme.outline_variant()),
        );

        // Chevron icon
        if self.show_icon {
            let icon_x = rect.min.x + 16.0;
            let icon_y = rect.min.y + 22.0;

            // Rotate chevron based on animation
            let rotation = self.animation_t * std::f32::consts::PI / 2.0;
            self.draw_chevron(ui, Pos2::new(icon_x, icon_y), rotation, theme);
        }

        // Title
        let title_x = if self.show_icon {
            rect.min.x + 40.0
        } else {
            rect.min.x + 16.0
        };

        ui.painter().text(
            Pos2::new(title_x, rect.min.y + 22.0),
            egui::Align2::LEFT_CENTER,
            &self.title,
            egui::FontId::proportional(16.0),
            theme.on_surface(),
        );

        response
    }

    /// Draw chevron icon
    fn draw_chevron(&self, ui: &mut Ui, center: Pos2, rotation: f32, theme: &Theme) {
        let size = 6.0;
        let painter = ui.painter();

        // Chevron points (right-pointing)
        let points = [
            Vec2::new(0.0, -size),
            Vec2::new(size, 0.0),
            Vec2::new(0.0, size),
        ];

        // Rotate and translate points
        let cos = rotation.cos();
        let sin = rotation.sin();

        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];

            // Rotate
            let rotated1 = Vec2::new(p1.x * cos - p1.y * sin, p1.x * sin + p1.y * cos);
            let rotated2 = Vec2::new(p2.x * cos - p2.y * sin, p2.x * sin + p2.y * cos);

            // Translate
            let pos1 = center + rotated1;
            let pos2 = center + rotated2;

            painter.line_segment([pos1, pos2], egui::Stroke::new(2.0, theme.on_surface()));
        }
    }
}

/// Accordion container for multiple items
pub struct Accordion {
    /// Allow multiple items open at once
    allow_multiple: bool,
    /// Show icons
    show_icons: bool,
    /// Animate expansions
    animate: bool,
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}

impl Accordion {
    /// Create a new accordion
    pub fn new() -> Self {
        Self {
            allow_multiple: false,
            show_icons: true,
            animate: true,
        }
    }

    /// Allow multiple items to be open at once
    pub fn allow_multiple(mut self, allow: bool) -> Self {
        self.allow_multiple = allow;
        self
    }

    /// Enable or disable icons
    pub fn show_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Enable or disable animation
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Show accordion with items
    pub fn show(
        &self,
        ui: &mut Ui,
        items: &mut [AccordionItem],
        mut content_fn: impl FnMut(&mut Ui, usize),
    ) {
        let mut clicked_index: Option<usize> = None;

        for (index, item) in items.iter_mut().enumerate() {
            // Apply accordion settings
            item.show_icon = self.show_icons;
            item.animate = self.animate;

            let (was_clicked, _) = item.show(ui, |ui| content_fn(ui, index));

            if was_clicked {
                clicked_index = Some(index);
            }
        }

        // Handle single-open mode
        if !self.allow_multiple {
            if let Some(clicked) = clicked_index {
                for (i, item) in items.iter_mut().enumerate() {
                    if i != clicked && item.is_open {
                        item.is_open = false;
                    }
                }
            }
        }
    }
}
