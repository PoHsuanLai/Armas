//! Infinite Moving Cards
//!
//! Continuous scrolling card carousel with infinite loop

use crate::ext::ArmasContextExt;
use egui::{Color32, Direction, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Speed of the scroll animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollSpeed {
    /// Slow scroll (80 pixels per second)
    Slow,
    /// Normal scroll (120 pixels per second)
    Normal,
    /// Fast scroll (200 pixels per second)
    Fast,
}

impl ScrollSpeed {
    const fn pixels_per_second(self) -> f32 {
        match self {
            Self::Slow => 80.0,
            Self::Normal => 120.0,
            Self::Fast => 200.0,
        }
    }
}

/// Individual card in the infinite scroll
#[derive(Clone)]
pub struct MovingCard {
    /// Card title
    pub title: String,
    /// Card subtitle/description
    pub subtitle: String,
    /// Optional author/source
    pub author: Option<String>,
    /// Background color
    pub background_color: Color32,
    /// Text color
    pub text_color: Color32,
}

impl MovingCard {
    /// Create a new moving card
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: subtitle.into(),
            author: None,
            background_color: Color32::PLACEHOLDER, // Use theme.card()
            text_color: Color32::PLACEHOLDER,       // Use theme.foreground()
        }
    }

    /// Set the author/source
    #[must_use]
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set the background color
    #[must_use] 
    pub const fn background_color(mut self, color: Color32) -> Self {
        self.background_color = color;
        self
    }

    /// Set the text color
    #[must_use] 
    pub const fn text_color(mut self, color: Color32) -> Self {
        self.text_color = color;
        self
    }
}

/// Infinite moving cards component
///
/// Creates a continuous horizontal scrolling carousel of cards.
/// Uses automatic animation loop - no `LoopingAnimation` needed since
/// we're doing direct offset calculation.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::{InfiniteMovingCards, ScrollSpeed};
///
/// InfiniteMovingCards::new()
///     .speed(ScrollSpeed::Fast)
///     .show(ui, |carousel| {
///         carousel.card("Title 1", "Subtitle 1").author("John Doe");
///         carousel.card("Title 2", "Subtitle 2").author("Jane Smith");
///     });
/// # }
/// ```
pub struct InfiniteMovingCards {
    card_width: f32,
    card_height: f32,
    spacing: f32,
    direction: Direction,
    speed: ScrollSpeed,
    pause_on_hover: bool,
}

impl InfiniteMovingCards {
    /// Create a new infinite moving cards component
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            card_width: 350.0,
            card_height: 200.0,
            spacing: 20.0,
            direction: Direction::LeftToRight,
            speed: ScrollSpeed::Normal,
            pause_on_hover: true,
        }
    }

    /// Set the card dimensions
    #[must_use] 
    pub const fn card_size(mut self, width: f32, height: f32) -> Self {
        self.card_width = width;
        self.card_height = height;
        self
    }

    /// Set the spacing between cards
    #[must_use] 
    pub const fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the scroll direction
    #[must_use] 
    pub const fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Set the scroll speed
    #[must_use] 
    pub const fn speed(mut self, speed: ScrollSpeed) -> Self {
        self.speed = speed;
        self
    }

    /// Set whether to pause on hover
    #[must_use] 
    pub const fn pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;
        self
    }

    /// Show the infinite moving cards with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut InfiniteMovingCardsBuilder) -> R,
    ) -> Response {
        let theme = ui.ctx().armas_theme();
        let time = ui.input(|i| i.time) as f32;

        // Build cards using closure
        let mut builder = InfiniteMovingCardsBuilder {
            cards: Vec::new(),
            card_index: 0,
        };
        content(&mut builder);
        let cards = builder.cards;

        // Allocate space for the component (full width, fixed height)
        let available_width = ui.available_width();
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(available_width, self.card_height), Sense::hover());

        // Check if hovered for pause
        let is_paused = self.pause_on_hover && response.hovered();

        // Calculate scroll offset from absolute time
        let scroll_offset = if is_paused {
            0.0
        } else {
            let direction_multiplier = match self.direction {
                Direction::LeftToRight => 1.0,
                Direction::RightToLeft => -1.0,
                // Horizontal scrolling only, but we need to handle all enum variants
                Direction::TopDown | Direction::BottomUp => 0.0,
            };

            time * self.speed.pixels_per_second() * direction_multiplier
        };

        // Calculate total width of all cards
        let single_set_width = cards.len() as f32 * (self.card_width + self.spacing);

        // Wrap the offset to create infinite loop
        let scroll_offset = scroll_offset % single_set_width;

        if ui.is_rect_visible(rect) {
            // Set clip rect to prevent cards from drawing outside the allocated area
            ui.set_clip_rect(rect);

            let painter = ui.painter();

            // We need to render enough duplicates to fill the viewport
            // Calculate how many full sets we need
            let sets_needed = (available_width / single_set_width).ceil() as usize + 2;

            // Draw multiple sets of cards to create seamless loop
            for set_index in 0..sets_needed {
                let set_base_x = set_index as f32 * single_set_width - scroll_offset;

                for (card_index, card) in cards.iter().enumerate() {
                    let card_x = set_base_x + card_index as f32 * (self.card_width + self.spacing);
                    let card_pos = Pos2::new(rect.left() + card_x, rect.top());

                    let card_rect =
                        Rect::from_min_size(card_pos, Vec2::new(self.card_width, self.card_height));

                    // Only render if the card is visible in the viewport
                    if card_rect.right() >= rect.left() && card_rect.left() <= rect.right() {
                        // Use theme colors if not explicitly set
                        let bg_color = if card.background_color == Color32::PLACEHOLDER {
                            theme.card()
                        } else {
                            card.background_color
                        };
                        let text_color = if card.text_color == Color32::PLACEHOLDER {
                            theme.foreground()
                        } else {
                            card.text_color
                        };

                        // Draw card background
                        let outline = theme.border();
                        painter.rect(
                            card_rect,
                            theme.spacing.corner_radius,
                            bg_color,
                            egui::Stroke::new(
                                1.0,
                                Color32::from_rgba_unmultiplied(
                                    outline.r(),
                                    outline.g(),
                                    outline.b(),
                                    30,
                                ),
                            ),
                            egui::StrokeKind::Outside,
                        );

                        // Draw card content
                        let content_rect = card_rect.shrink(theme.spacing.md);

                        // Title
                        painter.text(
                            Pos2::new(content_rect.left(), content_rect.top() + theme.spacing.md),
                            egui::Align2::LEFT_TOP,
                            &card.title,
                            egui::FontId::proportional(20.0),
                            text_color,
                        );

                        // Subtitle
                        let subtitle_color = theme.muted_foreground();
                        painter.text(
                            Pos2::new(
                                content_rect.left(),
                                content_rect.top() + theme.spacing.md * 2.5,
                            ),
                            egui::Align2::LEFT_TOP,
                            &card.subtitle,
                            egui::FontId::proportional(14.0),
                            subtitle_color,
                        );

                        // Author (if present)
                        if let Some(author) = &card.author {
                            painter.text(
                                Pos2::new(
                                    content_rect.left(),
                                    content_rect.bottom() - theme.spacing.lg,
                                ),
                                egui::Align2::LEFT_TOP,
                                author,
                                egui::FontId::proportional(12.0),
                                subtitle_color,
                            );
                        }
                    }
                }
            }
        }

        // Always request repaint for continuous animation
        ui.ctx().request_repaint();

        response
    }
}

impl Default for InfiniteMovingCards {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding moving cards
pub struct InfiniteMovingCardsBuilder {
    cards: Vec<MovingCard>,
    card_index: usize,
}

impl InfiniteMovingCardsBuilder {
    /// Add a card
    pub fn card(&mut self, title: &str, subtitle: &str) -> MovingCardBuilder<'_> {
        let card = MovingCard {
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            author: None,
            background_color: Color32::PLACEHOLDER,
            text_color: Color32::PLACEHOLDER,
        };

        self.cards.push(card);
        let current_index = self.card_index;
        self.card_index += 1;

        MovingCardBuilder {
            cards: &mut self.cards,
            card_index: current_index,
        }
    }
}

/// Builder for chaining moving card modifiers
pub struct MovingCardBuilder<'a> {
    cards: &'a mut Vec<MovingCard>,
    card_index: usize,
}

impl MovingCardBuilder<'_> {
    /// Set the author/source
    #[must_use] 
    pub fn author(self, author: &str) -> Self {
        if let Some(card) = self.cards.get_mut(self.card_index) {
            card.author = Some(author.to_string());
        }
        self
    }

    /// Set the background color
    #[must_use] 
    pub fn background_color(self, color: Color32) -> Self {
        if let Some(card) = self.cards.get_mut(self.card_index) {
            card.background_color = color;
        }
        self
    }

    /// Set the text color
    #[must_use] 
    pub fn text_color(self, color: Color32) -> Self {
        if let Some(card) = self.cards.get_mut(self.card_index) {
            card.text_color = color;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_card_creation() {
        let card = MovingCard::new("Title", "Subtitle");
        assert_eq!(card.title, "Title");
        assert_eq!(card.subtitle, "Subtitle");
        assert!(card.author.is_none());
    }

    #[test]
    fn test_moving_card_author() {
        let card = MovingCard::new("Title", "Subtitle").author("John Doe");
        assert_eq!(card.author, Some("John Doe".to_string()));
    }

    #[test]
    fn test_infinite_moving_cards_creation() {
        let carousel = InfiniteMovingCards::new();
        assert_eq!(carousel.direction, Direction::LeftToRight);
        assert_eq!(carousel.speed, ScrollSpeed::Normal);
    }

    #[test]
    fn test_infinite_moving_cards_config() {
        let carousel = InfiniteMovingCards::new()
            .card_size(300.0, 180.0)
            .spacing(15.0)
            .direction(Direction::RightToLeft)
            .speed(ScrollSpeed::Fast)
            .pause_on_hover(false);

        assert_eq!(carousel.card_width, 300.0);
        assert_eq!(carousel.card_height, 180.0);
        assert_eq!(carousel.spacing, 15.0);
        assert_eq!(carousel.direction, Direction::RightToLeft);
        assert_eq!(carousel.speed, ScrollSpeed::Fast);
        assert!(!carousel.pause_on_hover);
    }

    #[test]
    fn test_scroll_speed_values() {
        assert_eq!(ScrollSpeed::Slow.pixels_per_second(), 80.0);
        assert_eq!(ScrollSpeed::Normal.pixels_per_second(), 120.0);
        assert_eq!(ScrollSpeed::Fast.pixels_per_second(), 200.0);
    }
}
