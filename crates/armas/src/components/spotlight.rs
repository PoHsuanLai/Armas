use crate::Theme;
use egui::{Color32, Pos2, Rect, Response, Ui};

/// Mouse-tracking spotlight effect
///
/// Creates a radial gradient that follows the mouse cursor, perfect for
/// highlighting interactive areas or creating dramatic visual effects.
///
/// # Example
///
/// ```rust,no_run
/// use armas::{Theme, components::Spotlight};
///
/// fn ui(ui: &mut egui::Ui, spotlight: &mut Spotlight) {
///     let theme = Theme::dark();
///
///     spotlight.show(ui, &theme, |ui| {
///         ui.heading("Content with spotlight");
///         ui.label("Move your mouse to see the effect");
///     });
/// }
/// ```
pub struct Spotlight {
    /// Current spotlight position (smoothly follows cursor)
    current_pos: Pos2,
    /// Target position (actual cursor position)
    target_pos: Option<Pos2>,
    /// Spotlight radius in pixels
    pub radius: f32,
    /// Spotlight color (typically with transparency)
    pub color: Color32,
    /// Smoothing factor (0.0 = instant, 1.0 = very smooth)
    /// Higher values create more lag but smoother motion
    pub smoothing: f32,
    /// Intensity of the spotlight (0.0 to 1.0)
    pub intensity: f32,
    /// Number of gradient steps for smooth rendering
    pub gradient_steps: usize,
    /// Whether to show spotlight only on hover
    pub only_on_hover: bool,
}

// Note: Default removed as Spotlight now requires theme parameter

impl Spotlight {
    /// Create a new spotlight with theme-based defaults
    pub fn new(theme: &Theme) -> Self {
        let primary = theme.primary();
        Self {
            current_pos: Pos2::ZERO,
            target_pos: None,
            radius: 200.0,
            color: Color32::from_rgba_unmultiplied(primary.r(), primary.g(), primary.b(), 40),
            smoothing: 0.15,
            intensity: 1.0,
            gradient_steps: 30,
            only_on_hover: false,
        }
    }

    /// Set the spotlight radius
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Set the spotlight color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set the smoothing factor (0.0 = instant, 1.0 = very smooth)
    pub fn smoothing(mut self, smoothing: f32) -> Self {
        self.smoothing = smoothing.clamp(0.0, 1.0);
        self
    }

    /// Set the spotlight intensity
    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set the number of gradient steps
    pub fn gradient_steps(mut self, steps: usize) -> Self {
        self.gradient_steps = steps.max(5);
        self
    }

    /// Set whether to show spotlight only on hover
    pub fn only_on_hover(mut self, only_on_hover: bool) -> Self {
        self.only_on_hover = only_on_hover;
        self
    }

    /// Show content with the spotlight effect
    ///
    /// Returns a tuple of (inner content result, response)
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        _theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> (R, Response) {
        let available_rect = ui.available_rect_before_wrap();

        // Allocate space and render content
        let mut content_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(available_rect)
                .layout(*ui.layout()),
        );

        let content_result = content(&mut content_ui);
        let response = content_ui.response();
        let rect = response.rect;

        // Get mouse position
        let pointer_pos = ui.input(|i| i.pointer.hover_pos());
        let is_hovered = response.hovered();

        // Update target position if cursor is over the area
        if is_hovered {
            if let Some(pos) = pointer_pos {
                self.target_pos = Some(pos);
            }
        }

        // Update current position with smoothing
        if let Some(target) = self.target_pos {
            let dt = ui.input(|i| i.stable_dt);
            let lerp_factor = 1.0 - self.smoothing.powf(dt * 60.0); // Normalize to 60fps

            self.current_pos = Pos2::new(
                self.current_pos.x + (target.x - self.current_pos.x) * lerp_factor,
                self.current_pos.y + (target.y - self.current_pos.y) * lerp_factor,
            );
        } else {
            // Initialize position to center on first frame
            self.current_pos = rect.center();
            self.target_pos = Some(rect.center());
        }

        // Only draw spotlight if conditions are met
        let should_draw = if self.only_on_hover { is_hovered } else { true };

        if should_draw {
            // Draw the radial gradient spotlight
            self.draw_spotlight(ui, rect);

            // Request repaint for smooth animation
            ui.ctx().request_repaint();
        }

        (content_result, response)
    }

    /// Draw the radial gradient spotlight effect
    fn draw_spotlight(&self, ui: &mut Ui, _rect: Rect) {
        let painter = ui.painter();
        let center = self.current_pos;

        // Calculate effective alpha based on intensity
        let base_alpha = (self.color.a() as f32 * self.intensity) as u8;

        // Draw concentric circles with decreasing opacity for smooth gradient
        for i in 0..self.gradient_steps {
            let progress = i as f32 / self.gradient_steps as f32;
            let radius = self.radius * (1.0 - progress);

            // Easing function for more natural falloff (quadratic ease-out)
            let alpha_multiplier = 1.0 - progress * progress;
            let alpha = (base_alpha as f32 * alpha_multiplier) as u8;

            let circle_color = Color32::from_rgba_unmultiplied(
                self.color.r(),
                self.color.g(),
                self.color.b(),
                alpha,
            );

            painter.circle_filled(center, radius, circle_color);
        }
    }

    /// Alternative rendering using mesh for potentially better performance
    #[allow(dead_code)]
    fn draw_spotlight_mesh(&self, ui: &mut Ui, _rect: Rect) {
        use egui::epaint::{CircleShape, Shape};

        let center = self.current_pos;
        let painter = ui.painter();

        // Create multiple circle shapes with decreasing opacity
        let shapes: Vec<Shape> = (0..self.gradient_steps)
            .map(|i| {
                let progress = i as f32 / self.gradient_steps as f32;
                let radius = self.radius * (1.0 - progress);

                // Quadratic ease-out for alpha
                let alpha_multiplier = 1.0 - progress * progress;
                let base_alpha = (self.color.a() as f32 * self.intensity) as u8;
                let alpha = (base_alpha as f32 * alpha_multiplier) as u8;

                let circle_color = Color32::from_rgba_unmultiplied(
                    self.color.r(),
                    self.color.g(),
                    self.color.b(),
                    alpha,
                );

                Shape::Circle(CircleShape::filled(center, radius, circle_color))
            })
            .collect();

        painter.extend(shapes);
    }
}

/// Builder for creating multiple spotlights with different colors
pub struct MultiSpotlight {
    spotlights: Vec<(Spotlight, Theme)>,
}

impl MultiSpotlight {
    /// Create a new multi-spotlight effect
    pub fn new() -> Self {
        Self {
            spotlights: Vec::new(),
        }
    }

    /// Add a spotlight with a specific color
    pub fn add_spotlight(mut self, spotlight: Spotlight) -> Self {
        self.spotlights.push((spotlight, Theme::dark()));
        self
    }

    /// Show content with multiple spotlight effects
    ///
    /// Returns a tuple of (inner content result, response)
    pub fn show<R>(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> (R, Response) {
        let available_rect = ui.available_rect_before_wrap();

        // Render content first
        let mut content_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(available_rect)
                .layout(*ui.layout()),
        );

        let content_result = content(&mut content_ui);
        let response = content_ui.response();

        // Render all spotlights on top
        for (spotlight, _) in &mut self.spotlights {
            let _ = spotlight.show(ui, theme, |_ui| {
                // Empty content - we already rendered it above
            });
        }

        (content_result, response)
    }
}

impl Default for MultiSpotlight {
    fn default() -> Self {
        Self::new()
    }
}
