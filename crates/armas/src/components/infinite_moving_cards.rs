//! Infinite Moving Cards
//!
//! Continuous scrolling card carousel with infinite loop

use egui::{Color32, Direction, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Speed of the scroll animation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollSpeed {
    /// Slow scroll (80 pixels per second)
    Slow,
    /// Normal scroll (120 pixels per second)
    Normal,
    /// Fast scroll (200 pixels per second)
    Fast,
}

impl ScrollSpeed {
    fn pixels_per_second(&self) -> f32 {
        match self {
            ScrollSpeed::Slow => 80.0,
            ScrollSpeed::Normal => 120.0,
            ScrollSpeed::Fast => 200.0,
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
            background_color: Color32::from_rgb(40, 40, 50),
            text_color: Color32::from_gray(230),
        }
    }

    /// Set the author/source
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = color;
        self
    }

    /// Set the text color
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = color;
        self
    }
}

/// Infinite moving cards component
///
/// Creates a continuous horizontal scrolling carousel of cards.
/// Uses automatic animation loop - no LoopingAnimation needed since
/// we're doing direct offset calculation.
pub struct InfiniteMovingCards {
    cards: Vec<MovingCard>,
    card_width: f32,
    card_height: f32,
    spacing: f32,
    direction: Direction,
    speed: ScrollSpeed,
    pause_on_hover: bool,

    // Animation state
    scroll_offset: f32,
    is_paused: bool,
}

impl InfiniteMovingCards {
    /// Create a new infinite moving cards component
    pub fn new(cards: Vec<MovingCard>) -> Self {
        Self {
            cards,
            card_width: 350.0,
            card_height: 200.0,
            spacing: 20.0,
            direction: Direction::LeftToRight,
            speed: ScrollSpeed::Normal,
            pause_on_hover: true,
            scroll_offset: 0.0,
            is_paused: false,
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

    /// Set the scroll direction
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Set the scroll speed
    pub fn speed(mut self, speed: ScrollSpeed) -> Self {
        self.speed = speed;
        self
    }

    /// Set whether to pause on hover
    pub fn pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;
        self
    }

    /// Show the infinite moving cards
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let dt = ui.input(|i| i.stable_dt);

        // Allocate space for the component (full width, fixed height)
        let available_width = ui.available_width();
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(available_width, self.card_height), Sense::hover());

        // Check if hovered for pause
        if self.pause_on_hover {
            self.is_paused = response.hovered();
        }

        // Update scroll offset
        if !self.is_paused {
            let direction_multiplier = match self.direction {
                Direction::LeftToRight => 1.0,
                Direction::RightToLeft => -1.0,
                // Horizontal scrolling only, but we need to handle all enum variants
                Direction::TopDown | Direction::BottomUp => 0.0,
            };

            self.scroll_offset += self.speed.pixels_per_second() * dt * direction_multiplier;
        }

        // Calculate total width of all cards
        let single_set_width = self.cards.len() as f32 * (self.card_width + self.spacing);

        // Wrap the offset to create infinite loop
        self.scroll_offset %= single_set_width;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // We need to render enough duplicates to fill the viewport
            // Calculate how many full sets we need
            let sets_needed = (available_width / single_set_width).ceil() as usize + 2;

            // Draw multiple sets of cards to create seamless loop
            for set_index in 0..sets_needed {
                let set_base_x = set_index as f32 * single_set_width - self.scroll_offset;

                for (card_index, card) in self.cards.iter().enumerate() {
                    let card_x = set_base_x + card_index as f32 * (self.card_width + self.spacing);
                    let card_pos = Pos2::new(rect.left() + card_x, rect.top());

                    let card_rect =
                        Rect::from_min_size(card_pos, Vec2::new(self.card_width, self.card_height));

                    // Only render if the card is visible in the viewport
                    if card_rect.right() >= rect.left() && card_rect.left() <= rect.right() {
                        // Clip to viewport
                        let visible_rect = card_rect.intersect(rect);

                        if visible_rect.is_positive() {
                            // Draw card background
                            painter.rect(
                                card_rect,
                                8.0,
                                card.background_color,
                                egui::Stroke::new(
                                    1.0,
                                    Color32::from_rgba_unmultiplied(255, 255, 255, 30),
                                ),
                                egui::StrokeKind::Outside,
                            );

                            // Draw card content
                            let content_rect = card_rect.shrink(20.0);

                            // Title
                            painter.text(
                                Pos2::new(content_rect.left(), content_rect.top() + 20.0),
                                egui::Align2::LEFT_TOP,
                                &card.title,
                                egui::FontId::proportional(20.0),
                                card.text_color,
                            );

                            // Subtitle
                            painter.text(
                                Pos2::new(content_rect.left(), content_rect.top() + 50.0),
                                egui::Align2::LEFT_TOP,
                                &card.subtitle,
                                egui::FontId::proportional(14.0),
                                Color32::from_gray(170),
                            );

                            // Author (if present)
                            if let Some(author) = &card.author {
                                painter.text(
                                    Pos2::new(content_rect.left(), content_rect.bottom() - 25.0),
                                    egui::Align2::LEFT_TOP,
                                    author,
                                    egui::FontId::proportional(12.0),
                                    Color32::from_gray(140),
                                );
                            }
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
        let cards = vec![
            MovingCard::new("Card 1", "First"),
            MovingCard::new("Card 2", "Second"),
        ];
        let carousel = InfiniteMovingCards::new(cards);
        assert_eq!(carousel.cards.len(), 2);
        assert_eq!(carousel.direction, Direction::LeftToRight);
        assert_eq!(carousel.speed, ScrollSpeed::Normal);
    }

    #[test]
    fn test_infinite_moving_cards_config() {
        let cards = vec![MovingCard::new("Test", "Card")];
        let carousel = InfiniteMovingCards::new(cards)
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
