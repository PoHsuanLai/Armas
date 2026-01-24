//! Button Group Component
//!
//! Groups multiple buttons together with connected styling.
//! Styled to match shadcn/ui toggle-group with zero spacing.

use crate::ext::ArmasContextExt;
use egui::{Color32, CornerRadius, Sense, Stroke, Ui, Vec2};

const BUTTON_HEIGHT: f32 = 32.0;
const HORIZONTAL_PADDING: f32 = 12.0;
const CORNER_RADIUS: f32 = 6.0;

/// Visual variant for the button group
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ButtonGroupVariant {
    #[default]
    Outline,
    Ghost,
}

/// Response from showing a button group
pub struct ButtonGroupResponse {
    pub clicked: Option<usize>,
    pub selected: Option<usize>,
    pub changed: bool,
}

/// A group of connected buttons with shared styling
pub struct ButtonGroup {
    labels: Vec<String>,
    selected: Option<usize>,
    variant: ButtonGroupVariant,
}

impl ButtonGroup {
    pub fn new(labels: Vec<impl Into<String>>) -> Self {
        Self {
            labels: labels.into_iter().map(|l| l.into()).collect(),
            selected: None,
            variant: ButtonGroupVariant::default(),
        }
    }

    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }

    pub fn variant(mut self, variant: ButtonGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show(self, ui: &mut Ui) -> ButtonGroupResponse {
        let theme = ui.ctx().armas_theme();

        if self.labels.is_empty() {
            return ButtonGroupResponse { clicked: None, selected: None, changed: false };
        }

        // Calculate item widths
        let font_id = egui::TextStyle::Button.resolve(ui.style());
        let item_widths: Vec<f32> = self.labels.iter().map(|label| {
            let galley = ui.painter().layout_no_wrap(label.clone(), font_id.clone(), Color32::PLACEHOLDER);
            galley.rect.width() + HORIZONTAL_PADDING * 2.0
        }).collect();

        let total_width: f32 = item_widths.iter().sum();
        let (group_rect, _) = ui.allocate_exact_size(Vec2::new(total_width, BUTTON_HEIGHT), Sense::hover());

        if !ui.is_rect_visible(group_rect) {
            return ButtonGroupResponse { clicked: None, selected: self.selected, changed: false };
        }

        // Outer border
        if self.variant == ButtonGroupVariant::Outline {
            ui.painter().rect_stroke(
                group_rect,
                CornerRadius::same(CORNER_RADIUS as u8),
                Stroke::new(1.0, theme.input()),
                egui::StrokeKind::Inside,
            );
        }

        // Draw items
        let mut clicked = None;
        let mut x = group_rect.left();
        let item_count = self.labels.len();

        for (idx, (label, &width)) in self.labels.iter().zip(&item_widths).enumerate() {
            let is_first = idx == 0;
            let is_last = idx == item_count - 1;
            let is_selected = self.selected == Some(idx);

            let rect = egui::Rect::from_min_size(egui::pos2(x, group_rect.top()), Vec2::new(width, BUTTON_HEIGHT));
            let response = ui.allocate_rect(rect, Sense::click());
            let hovered = response.hovered();

            // Corner radius
            let corners = match (is_first, is_last) {
                (true, true) => CornerRadius::same(CORNER_RADIUS as u8),
                (true, false) => CornerRadius { nw: CORNER_RADIUS as u8, sw: CORNER_RADIUS as u8, ne: 0, se: 0 },
                (false, true) => CornerRadius { nw: 0, sw: 0, ne: CORNER_RADIUS as u8, se: CORNER_RADIUS as u8 },
                (false, false) => CornerRadius::ZERO,
            };

            // Background
            let bg = if is_selected {
                theme.accent()
            } else if hovered {
                Color32::from_rgba_unmultiplied(theme.accent().r(), theme.accent().g(), theme.accent().b(), 128)
            } else {
                Color32::TRANSPARENT
            };

            if bg != Color32::TRANSPARENT {
                ui.painter().rect_filled(rect, corners, bg);
            }

            // Separator
            if !is_first && self.variant == ButtonGroupVariant::Outline {
                ui.painter().line_segment(
                    [egui::pos2(rect.left(), rect.top() + 6.0), egui::pos2(rect.left(), rect.bottom() - 6.0)],
                    Stroke::new(1.0, theme.border()),
                );
            }

            // Label
            let text_color = if is_selected {
                theme.accent_foreground()
            } else if hovered {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };

            let galley = ui.painter().layout_no_wrap(label.clone(), font_id.clone(), text_color);
            let text_pos = rect.center() - galley.rect.size() / 2.0;
            ui.painter().galley(text_pos, galley, text_color);

            if response.clicked() {
                clicked = Some(idx);
            }

            x += width;
        }

        let new_selected = clicked.or(self.selected);
        ButtonGroupResponse {
            clicked,
            selected: new_selected,
            changed: clicked.is_some() && clicked != self.selected,
        }
    }
}
