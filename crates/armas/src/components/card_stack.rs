//! Card Stack Effect
//!
//! Auto-rotating stack of cards with staggered depth

use crate::animation::EasingFunction;
use crate::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// A single card in the stack
#[derive(Clone)]
pub struct StackCard {
    pub title: String,
    pub description: String,
    pub color: Color32,
}

/// Internal state for CardStack (stored in egui memory)
#[derive(Clone)]
struct CardStackState {
    active_index: usize,
    last_rotation_time: f32,
}

/// Card stack with auto-rotation
///
/// Displays multiple cards in a stack with automatic rotation
/// and smooth transitions between cards.
pub struct CardStack {
    cards: Vec<StackCard>,
    rotation_interval: f32,
    transition_duration: f32,
    width: f32,
    height: f32,
    auto_rotate: bool,
    id: egui::Id,
}

impl CardStack {
    /// Create a new card stack
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            cards: Vec::new(),
            rotation_interval: 5.0,
            transition_duration: 0.5,
            width,
            height,
            auto_rotate: true,
            id: egui::Id::new("card_stack"),
        }
    }

    /// Create a new card stack with a unique ID
    pub fn with_id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = egui::Id::new(id);
        self
    }

    /// Add a card to the stack
    pub fn add_card(mut self, card: StackCard) -> Self {
        self.cards.push(card);
        self
    }

    /// Set rotation interval (seconds)
    pub fn with_rotation_interval(mut self, interval: f32) -> Self {
        self.rotation_interval = interval;
        self
    }

    /// Enable/disable auto-rotation
    pub fn with_auto_rotate(mut self, enabled: bool) -> Self {
        self.auto_rotate = enabled;
        self
    }

    /// Set transition duration
    pub fn with_transition_duration(mut self, duration: f32) -> Self {
        self.transition_duration = duration;
        self
    }

    /// Manually advance to next card
    fn next(state: &mut CardStackState, time: f32, num_cards: usize) {
        if num_cards > 0 {
            state.active_index = (state.active_index + 1) % num_cards;
            state.last_rotation_time = time;
        }
    }

    /// Show the card stack
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), Sense::click());

        let time = ui.input(|i| i.time) as f32;

        // Get or initialize state from egui memory
        let mut state = ui.data_mut(|d| {
            d.get_temp::<CardStackState>(self.id).unwrap_or(CardStackState {
                active_index: 0,
                last_rotation_time: time,
            })
        });

        if response.clicked() && !self.auto_rotate {
            Self::next(&mut state, time, self.cards.len());
        }

        // Update timing
        if self.auto_rotate && !self.cards.is_empty() {
            let time_since_rotation = time - state.last_rotation_time;

            if time_since_rotation >= self.rotation_interval {
                Self::next(&mut state, time, self.cards.len());
            }
        }

        // Store state back
        ui.data_mut(|d| d.insert_temp(self.id, state.clone()));

        if self.cards.is_empty() {
            return response;
        }

        // Draw stacked cards (back to front)
        let num_visible = 3.min(self.cards.len());

        // Draw back cards first (higher depth), then front card last
        for stack_pos in (0..num_visible).rev() {
            let card_index = (state.active_index + stack_pos) % self.cards.len();
            let card = &self.cards[card_index];

            // Calculate offset and scale for stack effect
            // stack_pos=0 is the front (active) card, higher = deeper in stack
            let depth = stack_pos as f32;
            let y_offset = -depth * 15.0; // Negative so back cards appear above front card
            let scale = 1.0 - (depth * 0.05);

            // Apply transition animation to active (front) card
            // The active card is when stack_pos == 0 (front of stack)
            let is_active = stack_pos == 0;
            let (final_scale, final_y_offset) = if is_active {
                let time_since_rotation = time - state.last_rotation_time;
                let transition_progress = (time_since_rotation / self.transition_duration).min(1.0);
                let t = EasingFunction::CubicOut.apply(transition_progress);
                let start_scale = 0.95;
                let start_y = -15.0; // Start from back position
                (
                    start_scale + (scale - start_scale) * t,
                    start_y + (y_offset - start_y) * t,
                )
            } else {
                (scale, y_offset)
            };

            // Calculate card rect
            let card_width = self.width * final_scale;
            let card_height = self.height * final_scale;
            let card_x = rect.left() + (self.width - card_width) / 2.0;
            let card_y = rect.top() + final_y_offset;

            let card_rect = Rect::from_min_size(
                Pos2::new(card_x, card_y),
                Vec2::new(card_width, card_height),
            );

            // Draw shadow
            let shadow_alpha = (50.0 * (1.0 - depth * 0.2)) as u8;
            ui.painter().rect_filled(
                card_rect.translate(Vec2::new(2.0, 4.0)),
                8.0,
                Color32::from_black_alpha(shadow_alpha),
            );

            // Draw card
            ui.painter().rect_filled(card_rect, 8.0, card.color);

            // Draw card border - use theme outline with transparency
            let outline = theme.outline();
            ui.painter().rect_stroke(
                card_rect,
                8.0,
                egui::Stroke::new(
                    1.0,
                    Color32::from_rgba_unmultiplied(outline.r(), outline.g(), outline.b(), 40),
                ),
                egui::epaint::StrokeKind::Middle,
            );

            // Draw content for active card
            if is_active {
                let text_rect = card_rect.shrink(20.0);
                let mut child_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(text_rect)
                        .layout(*ui.layout()),
                );

                child_ui.vertical(|ui| {
                    ui.heading(&card.title);
                    ui.add_space(theme.spacing.sm);
                    ui.label(&card.description);
                });
            }
        }

        // Draw progress indicator
        if self.auto_rotate {
            let time_since_rotation = time - state.last_rotation_time;
            let progress = (time_since_rotation / self.rotation_interval).min(1.0);
            let indicator_width = self.width * 0.8;
            let indicator_height = 4.0;
            let indicator_x = rect.left() + (self.width - indicator_width) / 2.0;
            let indicator_y = rect.bottom() - 20.0;

            let bg_rect = Rect::from_min_size(
                Pos2::new(indicator_x, indicator_y),
                Vec2::new(indicator_width, indicator_height),
            );

            let surface = theme.surface_variant();
            ui.painter().rect_filled(
                bg_rect,
                2.0,
                Color32::from_rgba_unmultiplied(surface.r(), surface.g(), surface.b(), 60),
            );

            let progress_rect = Rect::from_min_size(
                Pos2::new(indicator_x, indicator_y),
                Vec2::new(indicator_width * progress, indicator_height),
            );

            ui.painter()
                .rect_filled(progress_rect, 2.0, theme.primary());
        }

        // Request repaint for animation
        ui.ctx().request_repaint();

        response
    }
}

/// Ease-out cubic easing function

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_stack_creation() {
        let stack = CardStack::new(300.0, 200.0);
        assert_eq!(stack.cards.len(), 0);
    }

    #[test]
    fn test_card_stack_add() {
        let card = StackCard {
            title: "Test".to_string(),
            description: "Description".to_string(),
            color: Color32::RED,
        };

        let stack = CardStack::new(300.0, 200.0).add_card(card);
        assert_eq!(stack.cards.len(), 1);
    }
}
