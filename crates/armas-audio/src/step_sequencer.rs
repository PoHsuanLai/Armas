//! Step Sequencer Component
//!
//! Grid of toggle buttons for rhythm programming and pattern creation.
//! Designed for drum machines and pattern-based sequencers.

use armas::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Response from the step sequencer
#[derive(Debug, Clone)]
pub struct StepSequencerResponse {
    /// The UI response
    pub response: Response,
    /// Whether any steps were modified this frame
    pub changed: bool,
}

impl StepSequencerResponse {
    /// Check if any steps were modified this frame
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.changed
    }
}

/// Step Sequencer component
///
/// A grid of toggle buttons for programming rhythmic patterns.
/// Each step can be on/off, commonly used for drum programming.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas_audio::StepSequencer;
///
/// let mut steps = vec![false; 16]; // 16 steps, all off
/// steps[0] = true;  // First step on
/// steps[4] = true;  // Fifth step on
///
/// let response = StepSequencer::new(&mut steps)
///     .steps(16)
///     .current_step(Some(2))
///     .show(ui, theme);
///
/// if response.changed() {
///     println!("Pattern changed!");
/// }
/// # }
/// ```
pub struct StepSequencer<'a> {
    steps: &'a mut Vec<bool>,
    num_steps: usize,
    current_step: Option<usize>,
    step_width: f32,
    step_height: f32,
    gap: f32,
    accent_color: Option<Color32>,
    show_step_numbers: bool,
    glow_intensity: f32,
    /// Color for active (ON) steps
    step_on_color: Option<Color32>,
    /// Color for inactive (OFF) steps
    step_off_color: Option<Color32>,
    /// Color for the current playback step
    current_step_color: Option<Color32>,
    /// Velocity data for each step (0.0-1.0), optional
    velocities: Option<&'a Vec<f32>>,
    /// Show measure accents (every N steps)
    measure_accent: Option<usize>,
}

impl<'a> StepSequencer<'a> {
    /// Create a new step sequencer with sophisticated professional styling
    pub const fn new(steps: &'a mut Vec<bool>) -> Self {
        Self {
            steps,
            num_steps: 16,
            current_step: None,
            step_width: 40.0,
            step_height: 40.0,
            gap: 4.0,
            accent_color: None,
            show_step_numbers: false,
            glow_intensity: 0.8,
            step_on_color: None,
            step_off_color: None,
            current_step_color: None,
            velocities: None,
            measure_accent: None,
        }
    }

    /// Set number of steps to display
    #[must_use]
    pub fn steps(mut self, num_steps: usize) -> Self {
        self.num_steps = num_steps.max(1);
        self
    }

    /// Set current playback step (for visual feedback)
    #[must_use]
    pub const fn current_step(mut self, step: Option<usize>) -> Self {
        self.current_step = step;
        self
    }

    /// Set step size
    #[must_use]
    pub const fn step_size(mut self, width: f32, height: f32) -> Self {
        self.step_width = width.max(20.0);
        self.step_height = height.max(20.0);
        self
    }

    /// Set gap between steps
    #[must_use]
    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Set accent color for active steps
    #[must_use]
    pub const fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    /// Show step numbers
    #[must_use]
    pub const fn show_step_numbers(mut self, show: bool) -> Self {
        self.show_step_numbers = show;
        self
    }

    /// Set glow intensity
    #[must_use]
    pub const fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set color for active (ON) steps
    #[must_use]
    pub const fn step_on_color(mut self, color: Color32) -> Self {
        self.step_on_color = Some(color);
        self
    }

    /// Set color for inactive (OFF) steps
    #[must_use]
    pub const fn step_off_color(mut self, color: Color32) -> Self {
        self.step_off_color = Some(color);
        self
    }

    /// Set color for the current playback step
    #[must_use]
    pub const fn current_step_color(mut self, color: Color32) -> Self {
        self.current_step_color = Some(color);
        self
    }

    /// Set velocity data for each step (for visualization)
    #[must_use]
    pub const fn velocities(mut self, velocities: &'a Vec<f32>) -> Self {
        self.velocities = Some(velocities);
        self
    }

    /// Show measure accents every N steps (e.g., 4 for quarter notes)
    #[must_use]
    pub fn measure_accent(mut self, every_n_steps: usize) -> Self {
        self.measure_accent = Some(every_n_steps.max(1));
        self
    }

    /// Show the step sequencer
    pub fn show(self, ui: &mut Ui, theme: &armas::Theme) -> StepSequencerResponse {
        // Ensure steps vec has correct size
        self.steps.resize(self.num_steps, false);

        // Calculate total size
        let total_width =
            (self.num_steps as f32).mul_add(self.step_width, (self.num_steps - 1) as f32 * self.gap);
        let total_height = self.step_height;
        let desired_size = Vec2::new(total_width, total_height);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        let mut changed = false;

        if ui.is_rect_visible(rect) {
            // Draw each step
            for i in 0..self.num_steps {
                let step_x = (i as f32).mul_add(self.step_width + self.gap, rect.min.x);
                let step_rect = Rect::from_min_size(
                    Pos2::new(step_x, rect.min.y),
                    Vec2::new(self.step_width, self.step_height),
                );

                let step_response = ui.allocate_rect(step_rect, Sense::click());

                if step_response.clicked() {
                    self.steps[i] = !self.steps[i];
                    changed = true;
                }

                let is_active = self.steps[i];
                let is_current = self.current_step == Some(i);
                let is_hovered = step_response.hovered();

                self.draw_step(
                    ui.painter(),
                    theme,
                    step_rect,
                    is_active,
                    is_current,
                    is_hovered,
                    i,
                );
            }
        }

        if changed {
            ui.ctx().request_repaint();
        }

        StepSequencerResponse { response, changed }
    }

    fn draw_step(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        is_active: bool,
        is_current: bool,
        is_hovered: bool,
        step_index: usize,
    ) {
        let corner_radius = f32::from(theme.spacing.corner_radius_small);

        // Determine colors
        let bg_color = if is_active {
            let accent = self.accent_color.unwrap_or_else(|| theme.primary());
            if is_hovered {
                accent.gamma_multiply(1.2)
            } else {
                accent
            }
        } else if is_hovered {
            theme.muted().gamma_multiply(1.2)
        } else {
            theme.muted()
        };

        // Professional sophisticated styling (glassmorphic with subtle depth)

        // Subtle shadow for depth (inactive steps only)
        if !is_active {
            for i in 0..1 {
                let shadow_offset = (i as f32 + 1.0) * 0.3;
                let shadow_rect = rect.translate(Vec2::new(0.0, shadow_offset));
                let shadow_alpha = 8u8;
                let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, shadow_alpha);
                painter.rect_filled(shadow_rect, corner_radius, shadow_color);
            }
        }

        // Main background with glassmorphic effect
        painter.rect_filled(rect, corner_radius, bg_color);

        // Active steps get a subtle inner glow
        if is_active {
            let glow_alpha = (30.0 * self.glow_intensity) as u8;
            let glow_color = Color32::from_rgba_unmultiplied(
                bg_color.r(),
                bg_color.g(),
                bg_color.b(),
                glow_alpha,
            );
            // Inner highlight for depth
            let highlight_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + 1.0, rect.min.y + 1.0),
                Vec2::new(rect.width() - 2.0, rect.height() / 2.5),
            );
            painter.rect_filled(highlight_rect, corner_radius * 0.8, glow_color);
        }

        // Sophisticated border
        let border_width = if is_active { 2.0 } else { 1.0 };
        let border_color = if is_active {
            bg_color.gamma_multiply(1.3)
        } else {
            theme.border()
        };
        painter.rect_stroke(
            rect,
            corner_radius,
            egui::Stroke::new(border_width, border_color),
            egui::StrokeKind::Outside,
        );

        // Velocity visualization (if available)
        if let Some(velocities) = self.velocities {
            if step_index < velocities.len() && is_active {
                let velocity = velocities[step_index].clamp(0.0, 1.0);
                if velocity > 0.0 && velocity < 1.0 {
                    let velocity_height = rect.height() * velocity * 0.6;
                    let velocity_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x + 2.0, rect.max.y - velocity_height - 2.0),
                        Vec2::new(rect.width() - 4.0, velocity_height),
                    );
                    let velocity_color = bg_color.gamma_multiply(0.7);
                    painter.rect_filled(velocity_rect, corner_radius * 0.5, velocity_color);
                }
            }
        }

        // Current step indicator (playhead)
        if is_current {
            let indicator_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, rect.max.y - 3.0),
                Vec2::new(rect.width(), 3.0),
            );
            painter.rect_filled(indicator_rect, 0.0, theme.secondary());

            // Glow effect
            for i in 0..4 {
                let offset = (i + 1) as f32 * 2.0;
                let alpha = ((1.0 - i as f32 / 4.0) * 50.0 * self.glow_intensity) as u8;
                let glow_color = Color32::from_rgba_unmultiplied(
                    theme.secondary().r(),
                    theme.secondary().g(),
                    theme.secondary().b(),
                    alpha,
                );
                painter.rect_stroke(
                    rect.expand(offset),
                    corner_radius,
                    egui::Stroke::new(2.0, glow_color),
                    egui::StrokeKind::Outside,
                );
            }
        }

        // Step numbers
        if self.show_step_numbers {
            let text_color = if is_active {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{}", step_index + 1),
                egui::FontId::proportional(9.0),
                text_color,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_sequencer_creation() {
        let mut steps = vec![false; 16];
        let _seq = StepSequencer::new(&mut steps);
        assert_eq!(steps.len(), 16);
    }

    #[test]
    fn test_step_sequencer_builder() {
        let mut steps = vec![false; 8];
        let seq = StepSequencer::new(&mut steps)
            .steps(8)
            .current_step(Some(2))
            .show_step_numbers(true);

        assert_eq!(seq.num_steps, 8);
        assert_eq!(seq.current_step, Some(2));
        assert!(seq.show_step_numbers);
    }

    #[test]
    fn test_step_resize() {
        let mut steps = vec![false; 8];
        let seq = StepSequencer::new(&mut steps).steps(16);

        // After calling steps(), the builder stores num_steps but doesn't resize yet
        assert_eq!(seq.num_steps, 16);
    }
}
