use crate::layout::HStack;
use crate::Theme;
use egui::{Color32, Pos2, Response, Ui, Vec2};

/// A testimonial item with quote, author, and optional avatar
#[derive(Clone)]
pub struct TestimonialItem {
    /// The testimonial quote
    pub quote: String,
    /// Author name
    pub author: String,
    /// Author role/title
    pub role: String,
    /// Optional avatar text (emoji or initials)
    pub avatar: Option<String>,
    /// Optional avatar color
    pub avatar_color: Option<Color32>,
    /// Optional star rating (0-5)
    pub rating: Option<u8>,
}

impl TestimonialItem {
    /// Create a new testimonial
    pub fn new(
        quote: impl Into<String>,
        author: impl Into<String>,
        role: impl Into<String>,
    ) -> Self {
        Self {
            quote: quote.into(),
            author: author.into(),
            role: role.into(),
            avatar: None,
            avatar_color: None,
            rating: None,
        }
    }

    /// Set avatar text (emoji or initials)
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// Set avatar color
    pub fn avatar_color(mut self, color: Color32) -> Self {
        self.avatar_color = Some(color);
        self
    }

    /// Set star rating (0-5)
    pub fn rating(mut self, stars: u8) -> Self {
        self.rating = Some(stars.min(5));
        self
    }
}

/// Testimonial card component
///
/// Displays customer testimonials with quotes, author info, and optional ratings.
/// Perfect for social proof sections.
///
/// # Example
///
/// ```rust,no_run
/// use armas::{Theme, components::{TestimonialCard, TestimonialItem}};
///
/// fn ui(ui: &mut egui::Ui) {
///     let theme = Theme::dark();
///     let item = TestimonialItem::new(
///         "This product changed my life!",
///         "Jane Doe",
///         "CEO, Company"
///     ).avatar("JD").rating(5);
///
///     TestimonialCard::new(item).show(ui, &theme);
/// }
/// ```
pub struct TestimonialCard {
    /// The testimonial item
    item: TestimonialItem,
    /// Card width (None = fill available)
    width: Option<f32>,
    /// Card height (None = auto-size)
    height: Option<f32>,
    /// Show quote marks
    show_quotes: bool,
    /// Hover effect
    hover_effect: bool,
    /// Show border
    show_border: bool,
}

impl TestimonialCard {
    /// Create a new testimonial card
    pub fn new(item: TestimonialItem) -> Self {
        Self {
            item,
            width: None,
            height: None,
            show_quotes: true,
            hover_effect: true,
            show_border: true,
        }
    }

    /// Set card width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set card height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Enable or disable quote marks
    pub fn show_quotes(mut self, show: bool) -> Self {
        self.show_quotes = show;
        self
    }

    /// Enable or disable hover effect
    pub fn hover_effect(mut self, hover: bool) -> Self {
        self.hover_effect = hover;
        self
    }

    /// Enable or disable border
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Show the testimonial card
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        let available = ui.available_size();
        let desired_width = self.width.unwrap_or(available.x);

        // Allocate space
        let (rect, response) = if self.height.is_some() {
            ui.allocate_exact_size(
                Vec2::new(desired_width, self.height.unwrap()),
                egui::Sense::hover(),
            )
        } else {
            ui.allocate_at_least(Vec2::new(desired_width, 100.0), egui::Sense::hover())
        };

        let is_hovered = response.hovered();

        // Background
        let bg_color = if self.hover_effect && is_hovered {
            theme.hover()
        } else {
            theme.surface()
        };

        ui.painter().rect_filled(rect, 8.0, bg_color);

        // Border
        if self.show_border {
            ui.painter().rect_stroke(
                rect,
                8.0,
                egui::Stroke::new(1.0, theme.outline_variant()),
                egui::StrokeKind::Middle,
            );
        }

        // Content area with padding
        let content_rect = rect.shrink(20.0);
        let mut y = content_rect.min.y;

        // Rating stars - use theme warning color (gold/yellow)
        if let Some(rating) = self.item.rating {
            let star_size = 16.0;
            let spacing = 4.0;
            for i in 0..5 {
                let x = content_rect.min.x + i as f32 * (star_size + spacing);
                let color = if i < rating {
                    theme.warning() // Use theme warning/gold color for stars
                } else {
                    theme.outline_variant()
                };
                ui.painter().text(
                    Pos2::new(x, y),
                    egui::Align2::LEFT_TOP,
                    "★",
                    egui::FontId::proportional(star_size),
                    color,
                );
            }
            y += 20.0;
        }

        // Quote text
        if self.show_quotes {
            let quote_mark = "\"";
            ui.painter().text(
                Pos2::new(content_rect.min.x, y),
                egui::Align2::LEFT_TOP,
                quote_mark,
                egui::FontId::proportional(32.0),
                theme.primary().gamma_multiply(0.3),
            );
            y += 10.0;
        }

        // Quote content
        let quote_galley = ui.painter().layout(
            self.item.quote.clone(),
            egui::FontId::proportional(16.0),
            theme.on_surface(),
            content_rect.width(),
        );
        let quote_height = quote_galley.rect.height();
        ui.painter().galley(
            Pos2::new(content_rect.min.x, y),
            quote_galley,
            theme.on_surface(),
        );
        y += quote_height + 20.0;

        // Author section with avatar
        let avatar_size = 48.0;
        let author_x = content_rect.min.x;

        if let Some(avatar_text) = &self.item.avatar {
            // Draw avatar circle
            let avatar_center = Pos2::new(author_x + avatar_size / 2.0, y + avatar_size / 2.0);
            let avatar_color = self.item.avatar_color.unwrap_or(theme.primary());

            ui.painter()
                .circle_filled(avatar_center, avatar_size / 2.0, avatar_color);

            // Avatar text
            ui.painter().text(
                avatar_center,
                egui::Align2::CENTER_CENTER,
                avatar_text,
                egui::FontId::proportional(20.0),
                Color32::WHITE,
            );

            // Author info next to avatar
            let info_x = author_x + avatar_size + 12.0;
            ui.painter().text(
                Pos2::new(info_x, y + 12.0),
                egui::Align2::LEFT_TOP,
                &self.item.author,
                egui::FontId::proportional(16.0),
                theme.on_surface(),
            );
            ui.painter().text(
                Pos2::new(info_x, y + 30.0),
                egui::Align2::LEFT_TOP,
                &self.item.role,
                egui::FontId::proportional(14.0),
                theme.on_surface_variant(),
            );
        } else {
            // No avatar, just text
            ui.painter().text(
                Pos2::new(author_x, y),
                egui::Align2::LEFT_TOP,
                &self.item.author,
                egui::FontId::proportional(16.0),
                theme.on_surface(),
            );
            ui.painter().text(
                Pos2::new(author_x, y + 20.0),
                egui::Align2::LEFT_TOP,
                &self.item.role,
                egui::FontId::proportional(14.0),
                theme.on_surface_variant(),
            );
        }

        response
    }
}

/// Testimonial grid for displaying multiple testimonials
pub struct TestimonialGrid {
    /// Testimonial items
    items: Vec<TestimonialItem>,
    /// Number of columns (None = auto-calculate)
    columns: Option<usize>,
    /// Gap between cards
    gap: f32,
    /// Show quotes
    show_quotes: bool,
    /// Hover effect
    hover_effect: bool,
}

impl TestimonialGrid {
    /// Create a new testimonial grid
    pub fn new(items: Vec<TestimonialItem>) -> Self {
        Self {
            items,
            columns: None,
            gap: 20.0,
            show_quotes: true,
            hover_effect: true,
        }
    }

    /// Set number of columns
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = Some(columns.max(1));
        self
    }

    /// Set gap between cards
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Enable or disable quote marks
    pub fn show_quotes(mut self, show: bool) -> Self {
        self.show_quotes = show;
        self
    }

    /// Enable or disable hover effect
    pub fn hover_effect(mut self, hover: bool) -> Self {
        self.hover_effect = hover;
        self
    }

    /// Show the testimonial grid
    pub fn show(&self, ui: &mut Ui, theme: &Theme) {
        let available_width = ui.available_width();

        // Calculate columns
        let columns = self.columns.unwrap_or_else(|| {
            let min_card_width = 300.0;
            ((available_width / min_card_width).floor() as usize).clamp(1, 3)
        });

        let card_width = (available_width - (columns - 1) as f32 * self.gap) / columns as f32;

        // Render grid
        let chunks: Vec<_> = self.items.chunks(columns).collect();

        for (row_idx, row) in chunks.iter().enumerate() {
            HStack::new(self.gap).show(ui, |ui| {
                for item in row.iter() {
                    TestimonialCard::new(item.clone())
                        .width(card_width)
                        .show_quotes(self.show_quotes)
                        .hover_effect(self.hover_effect)
                        .show(ui, theme);
                }
            });

            if row_idx < chunks.len() - 1 {
                ui.add_space(self.gap);
            }
        }
    }
}
