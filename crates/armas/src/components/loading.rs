use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Rect, Response, Ui, Vec2};
use std::f32::consts::PI;

/// Rotating spinner with multiple bars
///
/// A classic loading spinner with 12 rotating bars that have staggered opacity
/// for a smooth animation effect.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::Spinner;
///
/// fn ui(ui: &mut egui::Ui, spinner: &mut Spinner) {
///     spinner.show(ui);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Spinner {
    /// Size of the spinner in pixels
    pub size: f32,
    /// Rotation speed in radians per second
    pub speed: f32,
    /// Color of the spinner bars
    pub color: Color32,
    /// Current rotation angle
    rotation: f32,
    /// Number of bars in the spinner
    pub bar_count: usize,
    /// Width of each bar
    pub bar_width: f32,
}

impl Spinner {
    /// Create a new spinner with default settings
    pub fn new() -> Self {
        Self {
            size: 40.0,
            speed: 2.0 * PI, // One rotation per second
            color: Color32::from_rgb(99, 102, 241), // Default primary blue
            rotation: 0.0,
            bar_count: 12,
            bar_width: 2.0,
        }
    }

    /// Set the spinner size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the spinner color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the rotation speed (in radians per second)
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the number of bars
    pub fn bar_count(mut self, count: usize) -> Self {
        self.bar_count = count.max(3);
        self
    }

    /// Show the spinner
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), egui::Sense::hover());

        // Update rotation
        let dt = ui.input(|i| i.stable_dt);
        self.rotation += self.speed * dt;
        self.rotation %= 2.0 * PI;

        // Draw the spinner
        self.draw_spinner(ui, rect);

        // Request repaint for animation
        ui.ctx().request_repaint();

        response
    }

    /// Draw the rotating spinner
    fn draw_spinner(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        let center = rect.center();
        let radius = self.size / 2.0;
        let bar_length = radius * 0.3;
        let bar_start = radius * 0.5;

        for i in 0..self.bar_count {
            let angle = (i as f32 / self.bar_count as f32) * 2.0 * PI + self.rotation;

            // Calculate opacity with stagger effect (bars further in rotation are more opaque)
            let opacity_index = (self.bar_count - i) as f32 / self.bar_count as f32;
            let alpha = (opacity_index * 255.0) as u8;

            let color = Color32::from_rgba_unmultiplied(
                self.color.r(),
                self.color.g(),
                self.color.b(),
                alpha,
            );

            // Calculate bar start and end positions
            let start = Pos2::new(
                center.x + angle.cos() * bar_start,
                center.y + angle.sin() * bar_start,
            );
            let end = Pos2::new(
                center.x + angle.cos() * (bar_start + bar_length),
                center.y + angle.sin() * (bar_start + bar_length),
            );

            painter.line_segment([start, end], egui::Stroke::new(self.bar_width, color));
        }
    }
}

/// Blinking dots loader
///
/// Three dots that blink in sequence, creating a classic loading animation.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::LoadingDots;
///
/// fn ui(ui: &mut egui::Ui, dots: &mut LoadingDots) {
///     dots.show(ui);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct LoadingDots {
    /// Size of each dot in pixels
    pub dot_size: f32,
    /// Spacing between dots in pixels
    pub spacing: f32,
    /// Color of the dots
    pub color: Color32,
    /// Animation phase (0.0 to 1.0)
    phase: f32,
    /// Animation speed (cycles per second)
    pub speed: f32,
    /// Number of dots
    pub dot_count: usize,
}

impl LoadingDots {
    /// Create new loading dots with default settings
    pub fn new() -> Self {
        Self {
            dot_size: 8.0,
            spacing: 12.0,
            color: Color32::from_rgb(99, 102, 241), // Default primary blue
            phase: 0.0,
            speed: 1.5,
            dot_count: 3,
        }
    }

    /// Set the dot size
    pub fn dot_size(mut self, size: f32) -> Self {
        self.dot_size = size;
        self
    }

    /// Set the dot color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the spacing between dots
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the animation speed
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the number of dots
    pub fn dot_count(mut self, count: usize) -> Self {
        self.dot_count = count.max(2);
        self
    }

    /// Show the loading dots
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let width = (self.dot_count as f32 - 1.0) * self.spacing + self.dot_size;
        let height = self.dot_size;

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());

        // Update animation phase
        let dt = ui.input(|i| i.stable_dt);
        self.phase += self.speed * dt;
        self.phase %= 1.0;

        // Draw the dots
        self.draw_dots(ui, rect);

        // Request repaint for animation
        ui.ctx().request_repaint();

        response
    }

    /// Draw the blinking dots
    fn draw_dots(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        let y = rect.center().y;

        for i in 0..self.dot_count {
            let x = rect.min.x + self.dot_size / 2.0 + i as f32 * self.spacing;

            // Calculate opacity for this dot based on phase
            // Each dot blinks at a different time in the cycle
            let dot_phase = (self.phase + i as f32 / self.dot_count as f32) % 1.0;

            // Use a sine wave for smooth blinking (0.2 to 1.0 opacity)
            let opacity = 0.2 + 0.8 * (dot_phase * 2.0 * PI).sin().abs();
            let alpha = (opacity * 255.0) as u8;

            let color = Color32::from_rgba_unmultiplied(
                self.color.r(),
                self.color.g(),
                self.color.b(),
                alpha,
            );

            painter.circle_filled(Pos2::new(x, y), self.dot_size / 2.0, color);
        }
    }
}

/// Skeleton loader for placeholder content
///
/// A shimmer effect that animates across a rectangular area,
/// useful for indicating loading content.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::Skeleton;
///
/// fn ui(ui: &mut egui::Ui, skeleton: &mut Skeleton) {
///     skeleton.show(ui);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Skeleton {
    /// Width of the skeleton
    pub width: f32,
    /// Height of the skeleton
    pub height: f32,
    /// Base color of the skeleton
    pub base_color: Color32,
    /// Highlight color for the shimmer
    pub highlight_color: Color32,
    /// Animation position (0.0 to 1.0)
    shimmer_pos: f32,
    /// Animation speed
    pub speed: f32,
    /// Corner radius
    pub corner_radius: f32,
    /// Width of the shimmer effect
    pub shimmer_width: f32,
}

impl Skeleton {
    /// Create a new skeleton loader
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            base_color: Color32::from_gray(40),
            highlight_color: Color32::from_gray(60),
            shimmer_pos: 0.0,
            speed: 0.5,
            corner_radius: 4.0,
            shimmer_width: 0.3, // 30% of width
        }
    }

    /// Set the base color
    pub fn base_color(mut self, color: Color32) -> Self {
        self.base_color = color;
        self
    }

    /// Set the highlight color
    pub fn highlight_color(mut self, color: Color32) -> Self {
        self.highlight_color = color;
        self
    }

    /// Set the animation speed
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set the shimmer width (as a fraction of total width)
    pub fn shimmer_width(mut self, width: f32) -> Self {
        self.shimmer_width = width.clamp(0.1, 1.0);
        self
    }

    /// Show the skeleton loader
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), egui::Sense::hover());

        // Update shimmer position
        let dt = ui.input(|i| i.stable_dt);
        self.shimmer_pos += self.speed * dt;
        self.shimmer_pos %= 1.0;

        // Draw the skeleton
        self.draw_skeleton(ui, rect);

        // Request repaint for animation
        ui.ctx().request_repaint();

        response
    }

    /// Draw the skeleton with shimmer effect
    fn draw_skeleton(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();

        // Draw base rectangle
        painter.rect_filled(rect, self.corner_radius, self.base_color);

        // Draw shimmer effect as a gradient
        let shimmer_pixel_width = self.width * self.shimmer_width;
        let shimmer_center = rect.min.x + self.shimmer_pos * (self.width + shimmer_pixel_width)
            - shimmer_pixel_width / 2.0;

        // Draw shimmer as multiple rectangles with varying opacity
        let steps = 20;
        for i in 0..steps {
            let offset_from_center = (i as f32 - steps as f32 / 2.0) / (steps as f32 / 2.0);
            let x = shimmer_center + offset_from_center * shimmer_pixel_width / 2.0;

            // Only draw if within bounds
            if x >= rect.min.x && x < rect.max.x {
                let alpha_multiplier = 1.0 - offset_from_center.abs();
                let alpha = (self.highlight_color.a() as f32 * alpha_multiplier) as u8;

                let shimmer_color = Color32::from_rgba_unmultiplied(
                    self.highlight_color.r(),
                    self.highlight_color.g(),
                    self.highlight_color.b(),
                    alpha,
                );

                let step_width = shimmer_pixel_width / steps as f32;
                let shimmer_rect = Rect::from_min_max(
                    Pos2::new(x, rect.min.y),
                    Pos2::new((x + step_width).min(rect.max.x), rect.max.y),
                );

                painter.rect_filled(shimmer_rect, self.corner_radius, shimmer_color);
            }
        }
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new(200.0, 20.0)
    }
}

/// Circular progress spinner
///
/// A circular arc that rotates and grows/shrinks to indicate loading.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::CircularProgress};
///
/// fn ui(ui: &mut egui::Ui, progress: &mut CircularProgress) {
///     progress.show(ui);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CircularProgress {
    /// Size of the circular progress
    pub size: f32,
    /// Color of the arc
    pub color: Color32,
    /// Stroke width
    pub stroke_width: f32,
    /// Current rotation angle
    rotation: f32,
    /// Current arc length (0.0 to 1.0)
    arc_length: f32,
    /// Animation phase
    phase: f32,
}

impl CircularProgress {
    /// Create a new circular progress indicator with default settings
    pub fn new() -> Self {
        Self {
            size: 40.0,
            color: Color32::from_rgb(99, 102, 241), // Default primary blue
            stroke_width: 3.0,
            rotation: 0.0,
            arc_length: 0.25,
            phase: 0.0,
        }
    }

    /// Set the size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the stroke width
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    /// Show the circular progress
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), egui::Sense::hover());

        // Update animation
        let dt = ui.input(|i| i.stable_dt);
        self.rotation += 3.0 * dt; // Rotate
        self.phase += 2.0 * dt; // Phase for arc length animation

        // Animate arc length (breathing effect)
        self.arc_length = 0.1 + 0.6 * ((self.phase * PI).sin().abs());

        // Draw the arc
        self.draw_arc(ui, rect);

        // Request repaint for animation
        ui.ctx().request_repaint();

        response
    }

    /// Draw the rotating arc
    fn draw_arc(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();
        let center = rect.center();
        let radius = (self.size - self.stroke_width) / 2.0;

        // Draw arc using multiple line segments
        let segments = 32;
        let arc_angle = self.arc_length * 2.0 * PI;
        let start_angle = self.rotation;

        for i in 0..segments {
            let t = i as f32 / segments as f32;
            if t > self.arc_length {
                break;
            }

            let angle1 = start_angle + t * arc_angle;
            let angle2 =
                start_angle + ((i + 1) as f32 / segments as f32).min(self.arc_length) * arc_angle;

            let p1 = Pos2::new(
                center.x + angle1.cos() * radius,
                center.y + angle1.sin() * radius,
            );
            let p2 = Pos2::new(
                center.x + angle2.cos() * radius,
                center.y + angle2.sin() * radius,
            );

            painter.line_segment([p1, p2], egui::Stroke::new(self.stroke_width, self.color));
        }
    }
}
