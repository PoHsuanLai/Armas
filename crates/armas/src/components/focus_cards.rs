//! Focus Cards
//!
//! Interactive card grid where hovering a card focuses it while blurring others

use crate::animation::{Animation, EasingFunction};
use crate::ext::ArmasContextExt;
use egui::{Color32, Image, Pos2, Rect, Response, Sense, TextureHandle, Ui, Vec2};

/// Individual card in a focus cards grid
#[derive(Clone)]
pub struct FocusCard {
    /// Card title
    pub title: String,
    /// Card description
    pub description: String,
    /// Optional image texture
    pub image: Option<TextureHandle>,
    /// Background color (used if no image)
    pub background_color: Color32,
}

impl FocusCard {
    /// Create a new focus card
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            image: None,
            background_color: Color32::from_rgb(40, 40, 50),
        }
    }

    /// Set the card image
    pub fn image(mut self, texture: TextureHandle) -> Self {
        self.image = Some(texture);
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = color;
        self
    }
}

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
pub struct FocusCards {
    cards: Vec<FocusCard>,
    card_width: f32,
    card_height: f32,
    spacing: f32,
    columns: usize,

    // Animation state - tracks blur amount for each card
    blur_animations: Vec<Animation<f32>>,
    focused_index: Option<usize>,
}

impl FocusCards {
    /// Create a new focus cards grid
    pub fn new(cards: Vec<FocusCard>) -> Self {
        let card_count = cards.len();
        Self {
            cards,
            card_width: 300.0,
            card_height: 400.0,
            spacing: 20.0,
            columns: 3,
            blur_animations: (0..card_count)
                .map(|_| Animation::new(0.0, 0.0, 0.3).with_easing(EasingFunction::EaseOut))
                .collect(),
            focused_index: None,
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

    /// Show the focus cards grid
    pub fn show(&mut self, ui: &mut Ui) -> FocusCardResponse {
        let theme = ui.ctx().armas_theme();
        let dt = ui.input(|i| i.stable_dt);

        // Calculate grid dimensions
        let rows = self.cards.len().div_ceil(self.columns);
        let grid_width =
            self.columns as f32 * self.card_width + (self.columns - 1) as f32 * self.spacing;
        let grid_height = rows as f32 * self.card_height + (rows - 1) as f32 * self.spacing;

        let (rect, mut response) =
            ui.allocate_exact_size(Vec2::new(grid_width, grid_height), Sense::hover());

        let mut clicked_index: Option<usize> = None;
        let mut hovered_index: Option<usize> = None;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw each card
            for (index, card) in self.cards.iter().enumerate() {
                let row = index / self.columns;
                let col = index % self.columns;

                let card_pos = Pos2::new(
                    rect.min.x + col as f32 * (self.card_width + self.spacing),
                    rect.min.y + row as f32 * (self.card_height + self.spacing),
                );

                let card_rect =
                    Rect::from_min_size(card_pos, Vec2::new(self.card_width, self.card_height));

                // Get current blur amount
                let blur_amount = self.blur_animations[index].value();

                // Calculate opacity based on blur (0.0 = no blur/full opacity, 1.0 = full blur/reduced opacity)
                let opacity_factor = 1.0 - (blur_amount * 0.6); // Max 60% opacity reduction

                // Draw card background or image
                if let Some(texture) = &card.image {
                    // Draw image with opacity
                    let tint = Color32::from_rgba_unmultiplied(
                        255,
                        255,
                        255,
                        (255.0 * opacity_factor) as u8,
                    );

                    let image = Image::new(texture).tint(tint);
                    image.paint_at(ui, card_rect);

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
                } else {
                    // Draw colored background with opacity
                    let bg_color = Color32::from_rgba_unmultiplied(
                        card.background_color.r(),
                        card.background_color.g(),
                        card.background_color.b(),
                        (card.background_color.a() as f32 * opacity_factor) as u8,
                    );

                    painter.rect(
                        card_rect,
                        8.0,
                        bg_color,
                        egui::Stroke::new(
                            1.0,
                            Color32::from_rgba_unmultiplied(
                                255,
                                255,
                                255,
                                (40.0 * opacity_factor) as u8,
                            ),
                        ),
                        egui::StrokeKind::Outside,
                    );

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
                }

                // Draw text content with opacity
                let content_rect = card_rect.shrink(20.0);

                let text_alpha = (255.0 * opacity_factor) as u8;
                let title_color = Color32::from_rgba_unmultiplied(255, 255, 255, text_alpha);
                let desc_color = Color32::from_rgba_unmultiplied(200, 200, 200, text_alpha);

                painter.text(
                    Pos2::new(content_rect.left(), content_rect.bottom() - 60.0),
                    egui::Align2::LEFT_TOP,
                    &card.title,
                    egui::FontId::proportional(24.0),
                    title_color,
                );

                painter.text(
                    Pos2::new(content_rect.left(), content_rect.bottom() - 35.0),
                    egui::Align2::LEFT_TOP,
                    &card.description,
                    egui::FontId::proportional(14.0),
                    desc_color,
                );

                // Add subtle scale effect for focused card
                if Some(index) == hovered_index {
                    let scale_offset = 2.0;
                    let highlight_rect = card_rect.expand(scale_offset);
                    painter.rect(
                        highlight_rect,
                        8.0,
                        Color32::TRANSPARENT,
                        egui::Stroke::new(2.0, theme.primary().linear_multiply(0.8)),
                        egui::StrokeKind::Outside,
                    );
                }
            }
        }

        // Update focused state and animations
        if hovered_index != self.focused_index {
            self.focused_index = hovered_index;

            // Update all blur animations
            for (i, anim) in self.blur_animations.iter_mut().enumerate() {
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
        for anim in &mut self.blur_animations {
            anim.update(dt);
        }

        // Request repaint if any animations are running
        if self.blur_animations.iter().any(|anim| !anim.is_complete()) {
            ui.ctx().request_repaint();
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_card_creation() {
        let card = FocusCard::new("Title", "Description");
        assert_eq!(card.title, "Title");
        assert_eq!(card.description, "Description");
    }

    #[test]
    fn test_focus_cards_grid() {
        let cards = vec![
            FocusCard::new("Card 1", "First card"),
            FocusCard::new("Card 2", "Second card"),
            FocusCard::new("Card 3", "Third card"),
        ];
        let grid = FocusCards::new(cards);
        assert_eq!(grid.cards.len(), 3);
        assert_eq!(grid.blur_animations.len(), 3);
    }

    #[test]
    fn test_focus_cards_config() {
        let cards = vec![FocusCard::new("Test", "Card")];
        let grid = FocusCards::new(cards)
            .card_size(250.0, 350.0)
            .spacing(15.0)
            .columns(2);

        assert_eq!(grid.card_width, 250.0);
        assert_eq!(grid.card_height, 350.0);
        assert_eq!(grid.spacing, 15.0);
        assert_eq!(grid.columns, 2);
    }
}
