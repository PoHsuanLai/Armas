//! Step Sequencer Component
//!
//! Grid of toggle buttons for rhythm programming and pattern creation.
//! Perfect for drum machines and pattern-based sequencers.

use crate::ext::ArmasContextExt;
use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Visual style variant for step sequencer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepSequencerVariant {
    /// Filled steps with solid backgrounds
    Filled,
    /// Outlined steps with transparent backgrounds
    Outlined,
    /// Elevated steps with shadow effect
    Elevated,
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
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::StepSequencer;
///
/// let mut steps = vec![false; 16]; // 16 steps, all off
/// steps[0] = true;  // First step on
/// steps[4] = true;  // Fifth step on
///
/// let response = StepSequencer::new(&mut steps)
///     .steps(16)
///     .current_step(Some(2))
///     .show(ui);
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
    variant: StepSequencerVariant,
    step_width: f32,
    step_height: f32,
    gap: f32,
    accent_color: Option<Color32>,
    show_step_numbers: bool,
    glow_intensity: f32,
}

impl<'a> StepSequencer<'a> {
    /// Create a new step sequencer
    pub fn new(steps: &'a mut Vec<bool>) -> Self {
        Self {
            steps,
            num_steps: 16,
            current_step: None,
            variant: StepSequencerVariant::Filled,
            step_width: 40.0,
            step_height: 40.0,
            gap: 4.0,
            accent_color: None,
            show_step_numbers: false,
            glow_intensity: 0.8,
        }
    }

    /// Set number of steps to display
    pub fn steps(mut self, num_steps: usize) -> Self {
        self.num_steps = num_steps.max(1);
        self
    }

    /// Set current playback step (for visual feedback)
    pub fn current_step(mut self, step: Option<usize>) -> Self {
        self.current_step = step;
        self
    }

    /// Set visual variant
    pub fn variant(mut self, variant: StepSequencerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set step size
    pub fn step_size(mut self, width: f32, height: f32) -> Self {
        self.step_width = width.max(20.0);
        self.step_height = height.max(20.0);
        self
    }

    /// Set gap between steps
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Set accent color for active steps
    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    /// Show step numbers
    pub fn show_step_numbers(mut self, show: bool) -> Self {
        self.show_step_numbers = show;
        self
    }

    /// Set glow intensity
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Show the step sequencer
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();

        // Ensure steps vec has correct size
        self.steps.resize(self.num_steps, false);

        // Calculate total size
        let total_width = self.num_steps as f32 * self.step_width
            + (self.num_steps - 1) as f32 * self.gap;
        let total_height = self.step_height;
        let desired_size = Vec2::new(total_width, total_height);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        let mut changed = false;

        if ui.is_rect_visible(rect) {
            // Draw each step
            for i in 0..self.num_steps {
                let step_x = rect.min.x + i as f32 * (self.step_width + self.gap);
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
                    &theme,
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

        response
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
        let corner_radius = theme.spacing.corner_radius_small as f32;

        // Determine colors
        let (bg_color, border_color) = if is_active {
            let accent = self.accent_color.unwrap_or(theme.primary());
            let bg = if is_hovered {
                accent.gamma_multiply(1.2)
            } else {
                accent
            };
            (bg, theme.on_surface())
        } else {
            let bg = if is_hovered {
                theme.surface_variant().gamma_multiply(1.2)
            } else {
                theme.surface_variant()
            };
            (bg, theme.outline_variant())
        };

        // Draw based on variant
        match self.variant {
            StepSequencerVariant::Filled => {
                painter.rect_filled(rect, corner_radius, bg_color);
                painter.rect_stroke(
                    rect,
                    corner_radius,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Outside,
                );
            }
            StepSequencerVariant::Outlined => {
                painter.rect_filled(rect, corner_radius, theme.surface());
                let border_width = if is_active { 2.0 } else { 1.5 };
                painter.rect_stroke(
                    rect,
                    corner_radius,
                    egui::Stroke::new(border_width, if is_active { bg_color } else { theme.outline() }),
                    egui::StrokeKind::Outside,
                );
            }
            StepSequencerVariant::Elevated => {
                // Shadow
                if !is_active {
                    for i in 0..2 {
                        let offset = (i + 1) as f32 * 0.5;
                        let shadow_rect = rect.translate(Vec2::new(0.0, offset));
                        let alpha = (15.0 - i as f32 * 5.0) as u8;
                        let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, alpha);
                        painter.rect_filled(shadow_rect, corner_radius, shadow_color);
                    }
                }

                painter.rect_filled(rect, corner_radius, bg_color);
                painter.rect_stroke(
                    rect,
                    corner_radius,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Outside,
                );
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
                theme.on_surface()
            } else {
                theme.on_surface_variant()
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
