//! Focus Cards
//!
//! Interactive card grid where hovering a card focuses it while blurring others

use crate::animation::{Animation, EasingFunction};
use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Response from a focus card interaction
pub struct FocusCardResponse {
    /// The overall grid response
    pub response: Response,
    /// Index of the clicked card, if any
    pub clicked: Option<usize>,
    /// Index of the hovered card, if any
    pub hovered: Option<usize>,
}

/// Focus cards component
///
/// Displays a grid of cards where hovering one card focuses it (sharp)
/// while blurring all others. Uses Animation for smooth blur transitions.
///
/// This is a general-purpose component that accepts any content through closures.
/// Cards are rendered in a grid layout and apply a blur effect to non-focused cards.
///
/// # Example
///
/// ```ignore
/// use armas::{FocusCards, Card, CardVariant};
///
/// FocusCards::new()
///     .columns(3)
///     .card_size(300.0, 400.0)
///     .show(ui, &theme, |cards| {
///         cards.card(|ui, theme, opacity| {
///             Card::new()
///                 .variant(CardVariant::Filled)
///                 .show(ui, theme, |ui| {
///                     ui.label("Card content");
///                 });
///         });
///
///         cards.card(|ui, theme, opacity| {
///             Card::new()
///                 .variant(CardVariant::Outlined)
///                 .show(ui, theme, |ui| {
///                     ui.label("Another card");
///                 });
///         });
///     });
/// ```
pub struct FocusCards {
    card_width: f32,
    card_height: f32,
    spacing: f32,
    columns: usize,
}

impl FocusCards {
    /// Create a new focus cards grid
    pub fn new() -> Self {
        Self {
            card_width: 300.0,
            card_height: 400.0,
            spacing: 20.0,
            columns: 3,
        }
    }

    /// Set the card dimensions
    pub fn card_size(mut self, width: f32, height: f32) -> Self {
        self.card_width = width;
        self.card_height = height;
        self
    }

    /// Set the spacing between cards
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the number of columns
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    /// Show the focus cards grid with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        theme: &Theme,
        content: impl FnOnce(&mut FocusCardsBuilder) -> R,
    ) -> FocusCardResponse {
        let dt = ui.input(|i| i.stable_dt);

        // Collect cards from closure
        let mut cards = Vec::new();
        let mut builder = FocusCardsBuilder {
            cards: &mut cards,
        };
        content(&mut builder);

        let card_count = cards.len();

        // Get or initialize animation state from memory
        let cards_id = ui.id().with("focus_cards_state");
        let (mut blur_animations, mut focused_index): (Vec<Animation<f32>>, Option<usize>) =
            ui.ctx().data_mut(|d| {
                d.get_temp(cards_id).unwrap_or_else(|| {
                    (
                        (0..card_count)
                            .map(|_| {
                                Animation::new(0.0, 0.0, 0.3).easing(EasingFunction::EaseOut)
                            })
                            .collect(),
                        None,
                    )
                })
            });

        // Ensure animations match card count
        if blur_animations.len() != card_count {
            blur_animations = (0..card_count)
                .map(|_| Animation::new(0.0, 0.0, 0.3).easing(EasingFunction::EaseOut))
                .collect();
        }

        // Calculate grid dimensions
        let rows = card_count.div_ceil(self.columns);
        let grid_width = self.columns as f32 * self.card_width
            + (self.columns - 1) as f32 * self.spacing;
        let grid_height =
            rows as f32 * self.card_height + (rows - 1) as f32 * self.spacing;

        let (rect, mut response) =
            ui.allocate_exact_size(Vec2::new(grid_width, grid_height), Sense::hover());

        let mut clicked_index: Option<usize> = None;
        let mut hovered_index: Option<usize> = None;

        if ui.is_rect_visible(rect) {
            // Draw each card
            for (index, card_fn) in cards.into_iter().enumerate() {
                let row = index / self.columns;
                let col = index % self.columns;

                let card_pos = Pos2::new(
                    rect.min.x + col as f32 * (self.card_width + self.spacing),
                    rect.min.y + row as f32 * (self.card_height + self.spacing),
                );

                let card_rect = Rect::from_min_size(
                    card_pos,
                    Vec2::new(self.card_width, self.card_height),
                );

                // Get current blur amount
                let blur_amount = blur_animations[index].value();

                // Calculate opacity based on blur (0.0 = no blur/full opacity, 1.0 = full blur/reduced opacity)
                let opacity_factor = 1.0 - (blur_amount * 0.6); // Max 60% opacity reduction

                // Create a child UI for this card with clipping
                let mut child_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(card_rect)
                        .layout(egui::Layout::top_down(egui::Align::Min)),
                );

                // Call the user's card content function with opacity
                // The user can use the opacity parameter to adjust their content
                card_fn(&mut child_ui, theme, opacity_factor);

                // Add card hover and click sense
                let card_response = ui.interact(
                    card_rect,
                    ui.id().with(index),
                    Sense::click().union(Sense::hover()),
                );
                if card_response.hovered() {
                    hovered_index = Some(index);
                }
                if card_response.clicked() {
                    clicked_index = Some(index);
                }

                // Add subtle highlight for focused card
                if Some(index) == hovered_index {
                    let scale_offset = theme.spacing.xs / 2.0;
                    let highlight_rect = card_rect.expand(scale_offset);
                    let painter = ui.painter();
                    painter.rect(
                        highlight_rect,
                        theme.spacing.corner_radius,
                        Color32::TRANSPARENT,
                        egui::Stroke::new(2.0, theme.primary().linear_multiply(0.8)),
                        egui::StrokeKind::Outside,
                    );
                }
            }
        }

        // Update focused state and animations
        if hovered_index != focused_index {
            focused_index = hovered_index;

            // Update all blur animations
            for (i, anim) in blur_animations.iter_mut().enumerate() {
                if let Some(focused) = hovered_index {
                    // Blur all cards except the focused one
                    let target_blur = if i == focused { 0.0 } else { 1.0 };
                    anim.start = anim.value();
                    anim.end = target_blur;
                    anim.elapsed = 0.0;
                    anim.start();
                } else {
                    // No focus - remove blur from all
                    anim.start = anim.value();
                    anim.end = 0.0;
                    anim.elapsed = 0.0;
                    anim.start();
                }
            }
        }

        // Update all animations
        for anim in &mut blur_animations {
            anim.update(dt);
        }

        // Request repaint if any animations are running
        if blur_animations.iter().any(|anim| !anim.is_complete()) {
            ui.ctx().request_repaint();
        }

        // Save state
        ui.ctx().data_mut(|d| {
            d.insert_temp(cards_id, (blur_animations, focused_index));
        });

        if clicked_index.is_some() {
            response.mark_changed();
        }

        FocusCardResponse {
            response,
            clicked: clicked_index,
            hovered: hovered_index,
        }
    }
}

impl Default for FocusCards {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding cards to the focus cards grid
pub struct FocusCardsBuilder<'a> {
    cards: &'a mut Vec<Box<dyn FnOnce(&mut Ui, &Theme, f32) + 'static>>,
}

impl<'a> FocusCardsBuilder<'a> {
    /// Add a card with custom content
    ///
    /// The closure receives the UI, theme, and an opacity factor (0.0 = fully blurred, 1.0 = fully focused).
    /// You can use this opacity to adjust the appearance of your card content.
    pub fn card<F>(&mut self, card_content: F)
    where
        F: FnOnce(&mut Ui, &Theme, f32) + 'static,
    {
        self.cards.push(Box::new(card_content));
    }
}
