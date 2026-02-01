//! Loading Components
//!
//! Loading indicators styled for consistent UX.
//! Includes:
//! - Spinner: Classic rotating bar spinner
//! - Skeleton: Content placeholder with shimmer

use egui::{Color32, Pos2, Rect, Response, Ui, Vec2};
use std::f32::consts::PI;

const SPINNER_SIZE: f32 = 40.0;
const SPINNER_BAR_COUNT: usize = 12;
const SPINNER_BAR_WIDTH: f32 = 2.0;

const SKELETON_CORNER_RADIUS: f32 = 6.0; // rounded-md
const SKELETON_SHIMMER_WIDTH: f32 = 0.3;

/// Rotating spinner with multiple bars
///
/// A classic loading spinner with 12 rotating bars that have staggered opacity
/// for a smooth animation effect.
///
/// # Example
///
/// ```rust,no_run
/// use armas_basic::components::Spinner;
/// use armas_basic::ext::ArmasContextExt;
///
/// fn ui(ui: &mut egui::Ui, spinner: &mut Spinner) {
///     let theme = ui.ctx().armas_theme();
///     spinner.show(ui, &theme);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Spinner {
    /// Size of the spinner in pixels
    pub size: f32,
    /// Rotation speed in radians per second
    pub speed: f32,
    /// Color of the spinner bars (None = use theme primary color)
    color: Option<Color32>,
    /// Current rotation angle
    rotation: f32,
    /// Number of bars in the spinner
    pub bar_count: usize,
    /// Width of each bar
    pub bar_width: f32,
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Spinner {
    /// Create a new spinner with default settings
    /// Color defaults to theme primary color
    #[must_use] 
    pub fn new() -> Self {
        Self {
            size: SPINNER_SIZE,
            speed: 2.0 * PI,
            color: None, // Will use theme.primary()
            rotation: 0.0,
            bar_count: SPINNER_BAR_COUNT,
            bar_width: SPINNER_BAR_WIDTH,
        }
    }

    /// Set the spinner size
    #[must_use] 
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the spinner color (overrides theme)
    #[must_use] 
    pub const fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the rotation speed (in radians per second)
    #[must_use] 
    pub const fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the number of bars
    #[must_use] 
    pub fn bar_count(mut self, count: usize) -> Self {
        self.bar_count = count.max(3);
        self
    }

    /// Show the spinner
    pub fn show(&mut self, ui: &mut Ui, theme: &crate::Theme) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), egui::Sense::hover());

        // Calculate rotation from time for stateless animation
        let time = ui.input(|i| i.time) as f32;
        self.rotation = (time * self.speed) % (2.0 * PI);

        // Draw the spinner
        self.draw_spinner(ui, rect, theme);

        // Request repaint for animation
        ui.ctx().request_repaint();

        response
    }

    /// Draw the rotating spinner
    fn draw_spinner(&self, ui: &mut Ui, rect: Rect, theme: &crate::Theme) {
        let painter = ui.painter();
        let center = rect.center();
        let radius = self.size / 2.0;
        let bar_length = radius * 0.3;
        let bar_start = radius * 0.5;

        // Use custom color or theme primary
        let base_color = self.color.unwrap_or_else(|| theme.primary());

        for i in 0..self.bar_count {
            let angle = (i as f32 / self.bar_count as f32) * 2.0 * PI + self.rotation;

            // Calculate opacity with stagger effect (bars further in rotation are more opaque)
            let opacity_index = (self.bar_count - i) as f32 / self.bar_count as f32;
            let alpha = (opacity_index * 255.0) as u8;

            let color = Color32::from_rgba_unmultiplied(
                base_color.r(),
                base_color.g(),
                base_color.b(),
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

/// Skeleton loader for placeholder content
///
/// A shimmer effect that animates across a rectangular area,
/// useful for indicating loading content.
///
/// # Example
///
/// ```rust,no_run
/// use armas_basic::components::Skeleton;
/// use armas_basic::ext::ArmasContextExt;
///
/// fn ui(ui: &mut egui::Ui, skeleton: &mut Skeleton) {
///     let theme = ui.ctx().armas_theme();
///     skeleton.show(ui, &theme);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Skeleton {
    /// Width of the skeleton
    pub width: f32,
    /// Height of the skeleton
    pub height: f32,
    /// Base color of the skeleton (None = use theme `surface_variant`)
    base_color: Option<Color32>,
    /// Highlight color for the shimmer (None = use theme surface)
    highlight_color: Option<Color32>,
    /// Animation position (0.0 to 1.0)
    shimmer_pos: f32,
    /// Animation speed
    pub speed: f32,
    /// Corner radius (None = use theme spacing.xs)
    corner_radius: Option<f32>,
    /// Width of the shimmer effect
    pub shimmer_width: f32,
}

impl Skeleton {
    /// Create a new skeleton loader
    /// Colors default to theme `surface_variant` and surface
    #[must_use] 
    pub const fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            base_color: None,      // Will use theme.muted()
            highlight_color: None, // Will use theme.card()
            shimmer_pos: 0.0,
            speed: 0.5,
            corner_radius: Some(SKELETON_CORNER_RADIUS),
            shimmer_width: SKELETON_SHIMMER_WIDTH,
        }
    }

    /// Set the base color (overrides theme)
    #[must_use] 
    pub const fn base_color(mut self, color: Color32) -> Self {
        self.base_color = Some(color);
        self
    }

    /// Set the highlight color (overrides theme)
    #[must_use] 
    pub const fn highlight_color(mut self, color: Color32) -> Self {
        self.highlight_color = Some(color);
        self
    }

    /// Set the animation speed
    #[must_use] 
    pub const fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the corner radius (overrides theme)
    #[must_use] 
    pub const fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Set the shimmer width (as a fraction of total width)
    #[must_use] 
    pub const fn shimmer_width(mut self, width: f32) -> Self {
        self.shimmer_width = width.clamp(0.1, 1.0);
        self
    }

    /// Show the skeleton loader
    pub fn show(&mut self, ui: &mut Ui, theme: &crate::Theme) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), egui::Sense::hover());

        // Calculate shimmer position from time for stateless animation
        let time = ui.input(|i| i.time) as f32;
        self.shimmer_pos = (time * self.speed) % 1.0;

        // Draw the skeleton
        self.draw_skeleton(ui, rect, theme);

        // Request repaint for animation
        ui.ctx().request_repaint();

        response
    }

    /// Draw the skeleton with shimmer effect
    fn draw_skeleton(&self, ui: &mut Ui, rect: Rect, theme: &crate::Theme) {
        let painter = ui.painter();

        // Use custom colors or theme defaults
        let base_color = self.base_color.unwrap_or_else(|| theme.muted());
        let highlight_color = self.highlight_color.unwrap_or_else(|| theme.card());
        let corner_radius = self.corner_radius.unwrap_or(theme.spacing.xs);

        // Draw base rectangle
        painter.rect_filled(rect, corner_radius, base_color);

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
                let alpha = (f32::from(highlight_color.a()) * alpha_multiplier) as u8;

                let shimmer_color = Color32::from_rgba_unmultiplied(
                    highlight_color.r(),
                    highlight_color.g(),
                    highlight_color.b(),
                    alpha,
                );

                let step_width = shimmer_pixel_width / steps as f32;
                let shimmer_rect = Rect::from_min_max(
                    Pos2::new(x, rect.min.y),
                    Pos2::new((x + step_width).min(rect.max.x), rect.max.y),
                );

                painter.rect_filled(shimmer_rect, corner_radius, shimmer_color);
            }
        }
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new(200.0, 20.0)
    }
}

