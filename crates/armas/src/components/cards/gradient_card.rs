use crate::animation::Interpolate;
use crate::Theme;
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
/// fn ui(ui: &mut egui::Ui) {
///     let theme = Theme::dark();
///
///     GradientCard::new().show(ui, &theme, |ui| {
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
    /// Current gradient rotation angle
    gradient_angle: f32,
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
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            border_width: 2.0,
            corner_radius: 8.0,
            gradient_colors: vec![
                Color32::from_rgb(59, 130, 246), // Blue
                Color32::from_rgb(147, 51, 234), // Purple
                Color32::from_rgb(236, 72, 153), // Pink
            ],
            gradient_angle: 0.0,
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
        mut self,
        ui: &mut Ui,
        theme: &Theme,
        content: impl Fn(&mut Ui) -> R,
    ) -> (R, Response) {
        // Determine card size
        let available_width = ui.available_width();
        let desired_width = self.width.unwrap_or(available_width);

        // Use absolute time for animation
        let time = ui.input(|i| i.time) as f32;

        if self.animate {
            self.gradient_angle = (time * self.rotation_speed) % (2.0 * PI);
        }

        // For auto-sizing, we need to measure content first
        let auto_size = self.height.is_none();

        // Measure content if auto-sizing
        let content_size = if auto_size {
            let layout = *ui.layout();
            let measure_response = ui.scope(|ui| {
                ui.set_invisible();
                ui.with_layout(layout, |ui| {
                    let inner_rect = Rect::from_min_size(
                        ui.cursor().min,
                        Vec2::new(desired_width - self.border_width * 2.0, 1000.0),
                    );
                    let mut content_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(inner_rect)
                            .layout(layout),
                    );
                    content(&mut content_ui);
                })
                .response
            });
            measure_response.inner.rect.size()
        } else {
            Vec2::ZERO
        };

        let desired_height = if auto_size {
            content_size.y.max(60.0) + self.border_width * 2.0
        } else {
            self.height.unwrap()
        };

        // Allocate space for the card
        let (outer_rect, response) = ui.allocate_exact_size(
            Vec2::new(desired_width, desired_height),
            egui::Sense::hover(),
        );

        let is_hovered = response.hovered();

        // Animate glow on hover using absolute time
        let target_glow = if self.glow_on_hover && is_hovered { 1.0 } else { 0.0 };
        self.glow_intensity = target_glow; // Instant for stateless rendering

        // Render the card
        let inner_rect = outer_rect.shrink(self.border_width);

        // Draw gradient border
        self.draw_gradient_border(ui, outer_rect, inner_rect, theme);

        // Draw glow effect if hovered
        if self.glow_intensity > 0.01 {
            self.draw_glow(ui, outer_rect, theme);
        }

        // Draw background
        let bg_color = self.background_color.unwrap_or(theme.surface());
        ui.painter().rect_filled(
            inner_rect,
            (theme.spacing.corner_radius as f32 - self.border_width / 2.0).max(0.0),
            bg_color,
        );

        // Render content
        let mut content_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner_rect)
                .layout(*ui.layout()),
        );
        let content_result = content(&mut content_ui);

        // Request repaint for animation
        if self.animate {
            ui.ctx().request_repaint();
        }

        (content_result, response)
    }

    /// Draw the animated gradient border
    fn draw_gradient_border(&self, ui: &mut Ui, outer_rect: Rect, inner_rect: Rect, _theme: &Theme) {
        let painter = ui.painter();

        // Draw gradient border by rendering many small quad segments around the perimeter
        // This creates a smooth gradient effect around the entire border
        let segments = 120; // More segments = smoother gradient

        for i in 0..segments {
            let t1 = i as f32 / segments as f32;
            let t2 = (i + 1) as f32 / segments as f32;

            let color1 = self.get_gradient_color(t1);
            let color2 = self.get_gradient_color(t2);

            // Get points on outer and inner perimeter
            let outer1 = self.get_perimeter_point(outer_rect, t1);
            let outer2 = self.get_perimeter_point(outer_rect, t2);
            let inner1 = self.get_perimeter_point(inner_rect, t1);
            let inner2 = self.get_perimeter_point(inner_rect, t2);

            // Draw a quad for this segment of the border
            // Use a mesh for proper gradient rendering
            use egui::epaint::{Mesh, Vertex};

            let mesh = Mesh {
                vertices: vec![
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
                ],
                indices: vec![0, 1, 2, 0, 2, 3], // Two triangles forming a quad
                ..Default::default()
            };

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
    fn get_gradient_color(&self, t: f32) -> Color32 {
        let num_colors = self.gradient_colors.len();
        let position = t * num_colors as f32;
        let index = position.floor() as usize % num_colors;
        let next_index = (index + 1) % num_colors;
        let blend = position.fract();

        // Use animation system's Interpolate trait for color blending
        self.gradient_colors[index].interpolate(&self.gradient_colors[next_index], blend)
    }

    /// Draw glow effect around the card
    fn draw_glow(&self, ui: &mut Ui, rect: Rect, theme: &Theme) {
        let painter = ui.painter();
        let glow_layers = 5;
        let max_glow_distance = theme.spacing.sm;

        for i in 0..glow_layers {
            let distance = (i + 1) as f32 / glow_layers as f32 * max_glow_distance;
            let alpha = ((1.0 - i as f32 / glow_layers as f32) * self.glow_intensity * 40.0) as u8;

            // Use the first gradient color for glow
            let glow_color = Color32::from_rgba_unmultiplied(
                self.gradient_colors[0].r(),
                self.gradient_colors[0].g(),
                self.gradient_colors[0].b(),
                alpha,
            );

            let glow_rect = rect.expand(distance);
            painter.rect_stroke(
                glow_rect,
                theme.spacing.corner_radius as f32 + distance,
                egui::Stroke::new(2.0, glow_color),
                egui::StrokeKind::Middle,
            );
        }
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
