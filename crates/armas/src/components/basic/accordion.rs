//! Accordion Component
//!
//! Collapsible content sections. Styled to match shadcn/ui accordion.

use crate::animation::SpringAnimation;
use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Pos2, Ui, Vec2};

const TRIGGER_PADDING_Y: f32 = 16.0;
const CONTENT_PADDING_BOTTOM: f32 = 16.0;
const FONT_SIZE: f32 = 14.0;
const CHEVRON_SIZE: f32 = 16.0;

/// Response from showing an accordion
pub struct AccordionResponse {
    pub clicked: Option<usize>,
    pub open: Vec<usize>,
}

/// Accordion with collapsible sections
pub struct Accordion {
    id: egui::Id,
    titles: Vec<String>,
    allow_multiple: bool,
}

impl Accordion {
    pub fn new(id: impl Into<egui::Id>, titles: Vec<impl Into<String>>) -> Self {
        Self {
            id: id.into(),
            titles: titles.into_iter().map(|t| t.into()).collect(),
            allow_multiple: false,
        }
    }

    pub fn allow_multiple(mut self, allow: bool) -> Self {
        self.allow_multiple = allow;
        self
    }

    pub fn show(self, ui: &mut Ui, mut content_fn: impl FnMut(&mut Ui, usize)) -> AccordionResponse {
        let theme = ui.ctx().armas_theme();
        let dt = ui.input(|i| i.stable_dt);

        // Load state
        let state_id = self.id.with("accordion_state");
        let mut open_indices: Vec<usize> = ui.ctx().data_mut(|d| d.get_temp(state_id).unwrap_or_default());

        // Load spring animations for each item
        let mut springs: Vec<SpringAnimation> = ui.ctx().data_mut(|d| {
            d.get_temp(self.id.with("accordion_springs"))
                .unwrap_or_else(|| {
                    self.titles.iter().map(|_| SpringAnimation::new(0.0, 0.0).params(180.0, 22.0)).collect()
                })
        });

        // Ensure springs vec matches titles length
        while springs.len() < self.titles.len() {
            springs.push(SpringAnimation::new(0.0, 0.0).params(180.0, 22.0));
        }

        let mut clicked = None;
        let mut needs_repaint = false;

        for (idx, title) in self.titles.iter().enumerate() {
            let is_open = open_indices.contains(&idx);

            // Trigger
            let trigger_clicked = self.show_trigger(ui, title, springs[idx].value, &theme);

            if trigger_clicked {
                clicked = Some(idx);
                if is_open {
                    open_indices.retain(|&i| i != idx);
                } else {
                    if !self.allow_multiple {
                        open_indices.clear();
                    }
                    open_indices.push(idx);
                }
            }

            // Update spring target and simulate
            let target = if open_indices.contains(&idx) { 1.0 } else { 0.0 };
            springs[idx].set_target(target);
            springs[idx].update(dt);

            // Check if still animating (use looser threshold for smooth closing)
            let is_animating = !springs[idx].is_settled(0.005, 0.1);
            if is_animating {
                needs_repaint = true;
            }

            // Content - show while animating OR while open
            let anim_value = springs[idx].value.clamp(0.0, 1.0);
            let should_show = anim_value > 0.001 || is_animating;
            if should_show && anim_value > 0.0 {
                let content_id = self.id.with(("content_height", idx));
                let stored_height: f32 = ui.ctx().data_mut(|d| d.get_temp(content_id).unwrap_or(50.0));

                let animated_height = (stored_height + CONTENT_PADDING_BOTTOM) * anim_value;

                let response = egui::Frame::new()
                    .show(ui, |ui| {
                        ui.set_max_height(animated_height);
                        ui.set_clip_rect(ui.max_rect());

                        content_fn(ui, idx);
                        ui.add_space(CONTENT_PADDING_BOTTOM);

                        ui.min_rect().height()
                    });

                let actual_height = response.inner / anim_value.max(0.01);
                ui.ctx().data_mut(|d| d.insert_temp(content_id, actual_height));
            }

            // Bottom border
            let rect = ui.available_rect_before_wrap();
            ui.painter().hline(rect.x_range(), rect.top(), egui::Stroke::new(1.0, theme.border()));
            ui.allocate_space(Vec2::new(0.0, 1.0));
        }

        if needs_repaint {
            ui.ctx().request_repaint();
        }

        // Save state
        ui.ctx().data_mut(|d| {
            d.insert_temp(state_id, open_indices.clone());
            d.insert_temp(self.id.with("accordion_springs"), springs);
        });

        AccordionResponse {
            clicked,
            open: open_indices,
        }
    }

    fn show_trigger(&self, ui: &mut Ui, title: &str, anim_value: f32, theme: &Theme) -> bool {
        let available_width = ui.available_width();
        let text_galley = ui.painter().layout_no_wrap(
            title.to_string(),
            egui::FontId::proportional(FONT_SIZE),
            theme.foreground(),
        );
        let text_height = text_galley.rect.height();
        let trigger_height = text_height + TRIGGER_PADDING_Y * 2.0;

        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(available_width, trigger_height),
            egui::Sense::click(),
        );

        if ui.is_rect_visible(rect) {
            let text_pos = Pos2::new(rect.left(), rect.center().y - text_height / 2.0);
            ui.painter().galley(text_pos, text_galley.clone(), theme.foreground());

            // Underline on hover
            if response.hovered() {
                ui.painter().hline(
                    text_pos.x..=text_pos.x + text_galley.rect.width(),
                    text_pos.y + text_height,
                    egui::Stroke::new(1.0, theme.foreground()),
                );
            }

            // Chevron (rotates 180deg based on spring value)
            self.draw_chevron(ui, Pos2::new(rect.right() - CHEVRON_SIZE / 2.0, rect.center().y), anim_value, theme);
        }

        response.clicked()
    }

    fn draw_chevron(&self, ui: &mut Ui, center: Pos2, anim_value: f32, theme: &Theme) {
        let size = CHEVRON_SIZE / 3.0;
        let rotation = anim_value * std::f32::consts::PI;

        let points = [
            Vec2::new(-size, -size / 2.0),
            Vec2::new(0.0, size / 2.0),
            Vec2::new(size, -size / 2.0),
        ];

        let (cos, sin) = (rotation.cos(), rotation.sin());
        let rotate = |v: Vec2| center + Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos);

        ui.painter().line_segment([rotate(points[0]), rotate(points[1])], egui::Stroke::new(1.5, theme.muted_foreground()));
        ui.painter().line_segment([rotate(points[1]), rotate(points[2])], egui::Stroke::new(1.5, theme.muted_foreground()));
    }
}
