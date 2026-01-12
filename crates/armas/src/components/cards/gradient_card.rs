use crate::ext::ArmasContextExt;
use crate::animation::Interpolate;
use crate::effects::{GlowConfig, GlowEffect};
use crate::context::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Ui, Vec2};
use std::f32::consts::PI;

/// Card with animated gradient border
///
/// A card component with an eye-catching animated gradient border effect,
/// inspired by modern web design trends. Perfect for highlighting premium
/// content or creating visual interest.
///
/// # Example
///
/// ```rust,no_run
/// use armas::{Theme, components::GradientCard};
///
/// fn ui(ui: &mut egui::Ui, card: &mut GradientCard) {
///     let theme = Theme::dark();
///
///     card.show(ui, &theme, |ui| {
///         ui.heading("Premium Content");
///         ui.label("This card has an animated gradient border");
///     });
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GradientCard {
    /// Width of the card (None = fill available width)
    pub width: Option<f32>,
    /// Height of the card (None = auto-size to content)
    pub height: Option<f32>,
    /// Border width in pixels
    pub border_width: f32,
    /// Corner radius
    pub corner_radius: f32,
    /// Gradient colors (at least 2 required)
    pub gradient_colors: Vec<Color32>,
    /// Rotation speed (radians per second)
    pub rotation_speed: f32,
    /// Whether to animate the gradient
    pub animate: bool,
    /// Background color of the card content area
    pub background_color: Option<Color32>,
    /// Glow effect on hover
    pub glow_on_hover: bool,
    /// Glow intensity (0.0 to 1.0)
    glow_intensity: f32,
}

impl Default for GradientCard {
    fn default() -> Self {
        Self::new()
    }
}

impl GradientCard {
    /// Create a new gradient card with default settings
    /// Gradient colors will be derived from theme when shown
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            border_width: 2.0,
            corner_radius: 8.0,
            gradient_colors: Vec::new(), // Will use theme.gradient()
            rotation_speed: PI / 4.0, // 45 degrees per second
            animate: true,
            background_color: None,
            glow_on_hover: true,
            glow_intensity: 0.0,
        }
    }

    /// Set the card width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the card height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Set the gradient colors
    pub fn gradient_colors(mut self, colors: Vec<Color32>) -> Self {
        if colors.len() >= 2 {
            self.gradient_colors = colors;
        }
        self
    }

    /// Set the rotation speed
    pub fn rotation_speed(mut self, speed: f32) -> Self {
        self.rotation_speed = speed;
        self
    }

    /// Enable or disable animation
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Enable or disable glow on hover
    pub fn glow_on_hover(mut self, glow: bool) -> Self {
        self.glow_on_hover = glow;
        self
    }

    /// Show the gradient card with custom content
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> (R, Response) {
        let theme = ui.ctx().armas_theme();
        // Initialize gradient colors from theme if not set
        if self.gradient_colors.is_empty() {
            let [c1, c2, c3] = theme.gradient();
            self.gradient_colors = vec![c1, c2, c3];
        }

        // Determine card size
        let available = ui.available_size();
        let desired_width = self.width.unwrap_or(available.x);
        let desired_height = self.height.unwrap_or(0.0); // Will auto-size if 0

        // Allocate space for the card
        let (outer_rect, mut response) = if self.height.is_some() {
            ui.allocate_exact_size(
                Vec2::new(desired_width, desired_height),
                egui::Sense::hover(),
            )
        } else {
            // Auto-size height
            let (id, rect) = ui.allocate_space(Vec2::new(desired_width, 0.0));
            let response = ui.interact(rect, id, egui::Sense::hover());
            (rect, response)
        };

        let is_hovered = response.hovered();

        // Update animations
        let time = ui.input(|i| i.time) as f32;
        let dt = ui.input(|i| i.stable_dt);

        let gradient_angle = if self.animate {
            (time * self.rotation_speed) % (2.0 * PI)
        } else {
            0.0
        };

        // Animate glow on hover
        let target_glow = if self.glow_on_hover && is_hovered {
            1.0
        } else {
            0.0
        };
        let glow_speed = 5.0; // Smooth transition
        self.glow_intensity += (target_glow - self.glow_intensity) * glow_speed * dt;
        self.glow_intensity = self.glow_intensity.clamp(0.0, 1.0);

        // Render the card
        let inner_rect = outer_rect.shrink(self.border_width);

        // Draw gradient border
        self.draw_gradient_border(ui, outer_rect, inner_rect, gradient_angle);

        // Draw glow effect if hovered
        if self.glow_intensity > 0.01 {
            self.draw_glow(ui, outer_rect);
        }

        // Draw background
        let bg_color = self.background_color.unwrap_or(theme.surface());
        ui.painter().rect_filled(
            inner_rect,
            self.corner_radius - self.border_width / 2.0,
            bg_color,
        );

        // Render content
        let mut content_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner_rect)
                .layout(*ui.layout()),
        );
        let content_result = content(&mut content_ui);

        // Update response rect if we auto-sized
        if self.height.is_none() {
            let content_rect = content_ui.min_rect();
            let total_height = content_rect.height() + self.border_width * 2.0;
            response.rect =
                Rect::from_min_size(outer_rect.min, Vec2::new(desired_width, total_height));
        }

        // Request repaint for animation
        if self.animate || self.glow_intensity > 0.01 {
            ui.ctx().request_repaint();
        }

        (content_result, response)
    }

    /// Draw the animated gradient border
    fn draw_gradient_border(&self, ui: &mut Ui, outer_rect: Rect, inner_rect: Rect, gradient_angle: f32) {
        let painter = ui.painter();

        // Draw gradient border by rendering many small quad segments around the perimeter
        // This creates a smooth gradient effect around the entire border
        let segments = 120; // More segments = smoother gradient

        for i in 0..segments {
            let t1 = i as f32 / segments as f32;
            let t2 = (i + 1) as f32 / segments as f32;

            let color1 = self.get_gradient_color(t1, gradient_angle);
            let color2 = self.get_gradient_color(t2, gradient_angle);

            // Get points on outer and inner perimeter
            let outer1 = self.get_perimeter_point(outer_rect, t1);
            let outer2 = self.get_perimeter_point(outer_rect, t2);
            let inner1 = self.get_perimeter_point(inner_rect, t1);
            let inner2 = self.get_perimeter_point(inner_rect, t2);

            // Draw a quad for this segment of the border
            // Use a mesh for proper gradient rendering
            use egui::epaint::{Mesh, Vertex};

            let mut mesh = Mesh::default();
            mesh.vertices = vec![
                Vertex {
                    pos: outer1,
                    uv: Pos2::ZERO,
                    color: color1,
                },
                Vertex {
                    pos: outer2,
                    uv: Pos2::ZERO,
                    color: color2,
                },
                Vertex {
                    pos: inner2,
                    uv: Pos2::ZERO,
                    color: color2,
                },
                Vertex {
                    pos: inner1,
                    uv: Pos2::ZERO,
                    color: color1,
                },
            ];
            mesh.indices = vec![0, 1, 2, 0, 2, 3]; // Two triangles forming a quad

            painter.add(mesh);
        }
    }

    /// Get a point on the perimeter of a rectangle at position t (0.0 to 1.0)
    fn get_perimeter_point(&self, rect: Rect, t: f32) -> Pos2 {
        let t = t.fract(); // Ensure t is in [0, 1)
        let perimeter = 2.0 * (rect.width() + rect.height());
        let pos = t * perimeter;

        let top_len = rect.width();
        let right_len = rect.height();
        let bottom_len = rect.width();

        if pos < top_len {
            // Top edge (left to right)
            Pos2::new(rect.min.x + pos, rect.min.y)
        } else if pos < top_len + right_len {
            // Right edge (top to bottom)
            Pos2::new(rect.max.x, rect.min.y + (pos - top_len))
        } else if pos < top_len + right_len + bottom_len {
            // Bottom edge (right to left)
            Pos2::new(rect.max.x - (pos - top_len - right_len), rect.max.y)
        } else {
            // Left edge (bottom to top)
            Pos2::new(
                rect.min.x,
                rect.max.y - (pos - top_len - right_len - bottom_len),
            )
        }
    }

    /// Get color from gradient at position t (0.0 to 1.0)
    fn get_gradient_color(&self, t: f32, _gradient_angle: f32) -> Color32 {
        let num_colors = self.gradient_colors.len();
        let position = t * num_colors as f32;
        let index = position.floor() as usize % num_colors;
        let next_index = (index + 1) % num_colors;
        let blend = position.fract();

        // Use animation system's Interpolate trait for color blending
        Color32::interpolate(self.gradient_colors[index], self.gradient_colors[next_index], blend)
    }

    /// Draw glow effect around the card using unified GlowEffect
    fn draw_glow(&self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();

        // Use the first gradient color for glow
        let glow_color = Color32::from_rgba_unmultiplied(
            self.gradient_colors[0].r(),
            self.gradient_colors[0].g(),
            self.gradient_colors[0].b(),
            (self.glow_intensity * 40.0) as u8,
        );

        let glow = GlowEffect::new(
            GlowConfig::new(glow_color)
                .layers(5)
                .expansion(10.0)
                .intensity(self.glow_intensity),
        );
        glow.render_rect(&painter, rect, self.corner_radius, 2.0);
    }
}

/// Preset gradient card styles
impl GradientCard {
    /// Blue to purple gradient (default)
    pub fn blue_purple() -> Self {
        Self::new()
    }

    /// Rainbow gradient
    pub fn rainbow() -> Self {
        Self::new().gradient_colors(vec![
            Color32::from_rgb(239, 68, 68),  // Red
            Color32::from_rgb(251, 191, 36), // Yellow
            Color32::from_rgb(34, 197, 94),  // Green
            Color32::from_rgb(59, 130, 246), // Blue
            Color32::from_rgb(168, 85, 247), // Purple
        ])
    }

    /// Warm gradient (red to orange to yellow)
    pub fn warm() -> Self {
        Self::new().gradient_colors(vec![
            Color32::from_rgb(239, 68, 68),  // Red
            Color32::from_rgb(249, 115, 22), // Orange
            Color32::from_rgb(251, 191, 36), // Yellow
        ])
    }

    /// Cool gradient (cyan to blue to purple)
    pub fn cool() -> Self {
        Self::new().gradient_colors(vec![
            Color32::from_rgb(6, 182, 212),  // Cyan
            Color32::from_rgb(59, 130, 246), // Blue
            Color32::from_rgb(147, 51, 234), // Purple
        ])
    }

    /// Neon gradient (bright, saturated colors)
    pub fn neon() -> Self {
        Self::new()
            .gradient_colors(vec![
                Color32::from_rgb(255, 0, 255), // Magenta
                Color32::from_rgb(0, 255, 255), // Cyan
                Color32::from_rgb(255, 255, 0), // Yellow
            ])
            .border_width(3.0)
    }

    /// Gold gradient
    pub fn gold() -> Self {
        Self::new().gradient_colors(vec![
            Color32::from_rgb(251, 191, 36), // Yellow
            Color32::from_rgb(217, 119, 6),  // Amber
            Color32::from_rgb(161, 98, 7),   // Dark gold
        ])
    }
}
