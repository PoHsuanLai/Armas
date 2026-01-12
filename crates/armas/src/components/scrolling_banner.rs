use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Pos2, Rect, Response, Ui, Vec2};

/// Direction for scrolling animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Scroll from right to left
    Left,
    /// Scroll from left to right
    Right,
    /// Scroll from bottom to top
    Up,
    /// Scroll from top to bottom
    Down,
}

/// Infinite scrolling banner/marquee component
///
/// Displays content that scrolls continuously in a loop. Perfect for logos,
/// announcements, or any repeating content.
///
/// # Example
///
/// ```rust,no_run
/// use armas::{Theme, components::ScrollingBanner, components::ScrollDirection};
///
/// fn ui(ui: &mut egui::Ui, banner: &mut ScrollingBanner) {
///     let theme = Theme::dark();
///
///     banner.show(ui, &theme, |ui, _index| {
///         ui.label("Item 1");
///         ui.label("Item 2");
///         ui.label("Item 3");
///     });
/// }
/// ```
pub struct ScrollingBanner {
    /// Scroll speed in pixels per second
    pub speed: f32,
    /// Direction of scrolling
    pub direction: ScrollDirection,
    /// Gap between repeated content in pixels
    pub gap: f32,
    /// Whether the animation is paused
    paused: bool,
    /// Pause on hover
    pub pause_on_hover: bool,
    /// Show fade effect at edges
    pub show_fade: bool,
    /// Fade width in pixels
    pub fade_width: f32,
}

impl Default for ScrollingBanner {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollingBanner {
    /// Create a new scrolling banner with default settings
    pub fn new() -> Self {
        Self {
            speed: 50.0,
            direction: ScrollDirection::Left,
            gap: 32.0,
            paused: false,
            pause_on_hover: true,
            show_fade: true,
            fade_width: 40.0,
        }
    }

    /// Set the scroll speed in pixels per second
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the scroll direction
    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the gap between repeated content
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Enable or disable pause on hover
    pub fn pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;
        self
    }

    /// Enable or disable fade effect at edges
    pub fn show_fade(mut self, show: bool) -> Self {
        self.show_fade = show;
        self
    }

    /// Set the fade width
    pub fn fade_width(mut self, width: f32) -> Self {
        self.fade_width = width;
        self
    }

    /// Pause the scrolling animation
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume the scrolling animation
    pub fn resume(&mut self) {
        self.paused = false;
    }


    /// Show the scrolling banner with custom content
    ///
    /// The content function receives the UI and the current repetition index.
    /// The content will be rendered multiple times to create the infinite loop effect.
    pub fn show<F>(self, ui: &mut Ui, content: F) -> Response
    where
        F: Fn(&mut Ui, usize),
    {
        let theme = ui.ctx().armas_theme();
        // First, measure content size to know how much space we need
        let content_size = self.measure_content(ui, &content);

        // Determine the size to allocate based on available space and content
        let available_width = ui.available_width();
        let desired_height = content_size.y.max(40.0); // Ensure minimum height
        let desired_size = egui::vec2(available_width, desired_height);

        // Allocate space for the banner - this advances the cursor properly
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

        // Calculate animation offset from absolute time
        let time = ui.input(|i| i.time) as f32;
        let is_hovered = response.hovered();

        let offset = if !self.paused && (!self.pause_on_hover || !is_hovered) {
            time * self.speed
        } else {
            0.0
        };

        // Calculate how many repetitions we need to fill the visible area
        let (primary_axis_size, _secondary_axis_size) = match self.direction {
            ScrollDirection::Left | ScrollDirection::Right => (rect.width(), rect.height()),
            ScrollDirection::Up | ScrollDirection::Down => (rect.height(), rect.width()),
        };

        let total_content_size = match self.direction {
            ScrollDirection::Left | ScrollDirection::Right => content_size.x,
            ScrollDirection::Up | ScrollDirection::Down => content_size.y,
        };

        let repeat_distance = total_content_size + self.gap;

        // Wrap offset to create infinite loop
        let offset = if repeat_distance > 0.0 {
            offset % repeat_distance
        } else {
            offset
        };

        // Calculate number of repetitions needed (always at least 2 for seamless loop)
        let repetitions = ((primary_axis_size / repeat_distance).ceil() as usize + 2).max(2);

        // Render content with clipping
        let clip_rect = rect;
        ui.set_clip_rect(clip_rect);

        for i in 0..repetitions {
            let offset_multiplier = i as f32;
            let position_offset = match self.direction {
                ScrollDirection::Left => {
                    Vec2::new(-offset + offset_multiplier * repeat_distance, 0.0)
                }
                ScrollDirection::Right => {
                    Vec2::new(offset - offset_multiplier * repeat_distance, 0.0)
                }
                ScrollDirection::Up => {
                    Vec2::new(0.0, -offset + offset_multiplier * repeat_distance)
                }
                ScrollDirection::Down => {
                    Vec2::new(0.0, offset - offset_multiplier * repeat_distance)
                }
            };

            let content_rect = Rect::from_min_size(rect.min + position_offset, content_size);

            let layout = match self.direction {
                ScrollDirection::Left | ScrollDirection::Right => {
                    egui::Layout::left_to_right(egui::Align::Center)
                }
                ScrollDirection::Up | ScrollDirection::Down => {
                    egui::Layout::top_down(egui::Align::Center)
                }
            };

            let mut item_ui =
                ui.new_child(egui::UiBuilder::new().max_rect(content_rect).layout(layout));

            content(&mut item_ui, i);
        }

        // Draw fade effects at edges
        if self.show_fade {
            self.draw_fade_effects(ui, rect, &theme);
        }

        // Request repaint for animation
        if !self.paused && (!self.pause_on_hover || !is_hovered) {
            ui.ctx().request_repaint();
        }

        response
    }

    /// Measure the size of the content
    fn measure_content<F>(&self, ui: &mut Ui, content: &F) -> Vec2
    where
        F: Fn(&mut Ui, usize),
    {
        let layout = match self.direction {
            ScrollDirection::Left | ScrollDirection::Right => {
                egui::Layout::left_to_right(egui::Align::Center)
            }
            ScrollDirection::Up | ScrollDirection::Down => {
                egui::Layout::top_down(egui::Align::Center)
            }
        };

        // Measure content invisibly
        let response = ui.scope(|ui| {
            ui.set_invisible();
            ui.with_layout(layout, |ui| {
                content(ui, 0);
            })
            .response
        });

        response.inner.rect.size()
    }

    /// Draw fade effects at the edges
    fn draw_fade_effects(&self, ui: &mut Ui, rect: Rect, theme: &Theme) {
        let painter = ui.painter();
        let background = theme.surface();

        match self.direction {
            ScrollDirection::Left | ScrollDirection::Right => {
                // Left fade
                let left_rect = Rect::from_min_max(
                    rect.min,
                    Pos2::new(rect.min.x + self.fade_width, rect.max.y),
                );
                self.draw_horizontal_fade(painter, left_rect, background, true);

                // Right fade
                let right_rect = Rect::from_min_max(
                    Pos2::new(rect.max.x - self.fade_width, rect.min.y),
                    rect.max,
                );
                self.draw_horizontal_fade(painter, right_rect, background, false);
            }
            ScrollDirection::Up | ScrollDirection::Down => {
                // Top fade
                let top_rect = Rect::from_min_max(
                    rect.min,
                    Pos2::new(rect.max.x, rect.min.y + self.fade_width),
                );
                self.draw_vertical_fade(painter, top_rect, background, true);

                // Bottom fade
                let bottom_rect = Rect::from_min_max(
                    Pos2::new(rect.min.x, rect.max.y - self.fade_width),
                    rect.max,
                );
                self.draw_vertical_fade(painter, bottom_rect, background, false);
            }
        }
    }

    /// Draw horizontal gradient fade
    fn draw_horizontal_fade(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        color: egui::Color32,
        fade_left: bool,
    ) {
        let steps = 20;
        let width = rect.width();
        let step_width = width / steps as f32;

        for i in 0..steps {
            let x = if fade_left {
                rect.min.x + i as f32 * step_width
            } else {
                rect.max.x - (i + 1) as f32 * step_width
            };

            let alpha = ((steps - i) as f32 / steps as f32 * 255.0) as u8;

            let fade_color =
                egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

            let step_rect = Rect::from_min_max(
                Pos2::new(x, rect.min.y),
                Pos2::new(x + step_width, rect.max.y),
            );

            painter.rect_filled(step_rect, 0.0, fade_color);
        }
    }

    /// Draw vertical gradient fade
    fn draw_vertical_fade(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        color: egui::Color32,
        fade_top: bool,
    ) {
        let steps = 20;
        let height = rect.height();
        let step_height = height / steps as f32;

        for i in 0..steps {
            let y = if fade_top {
                rect.min.y + i as f32 * step_height
            } else {
                rect.max.y - (i + 1) as f32 * step_height
            };

            let alpha = ((steps - i) as f32 / steps as f32 * 255.0) as u8;

            let fade_color =
                egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

            let step_rect = Rect::from_min_max(
                Pos2::new(rect.min.x, y),
                Pos2::new(rect.max.x, y + step_height),
            );

            painter.rect_filled(step_rect, 0.0, fade_color);
        }
    }
}
