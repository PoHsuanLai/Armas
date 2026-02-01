use crate::ext::ArmasContextExt;
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
    #[must_use]
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// Set avatar color
    #[must_use] 
    pub const fn avatar_color(mut self, color: Color32) -> Self {
        self.avatar_color = Some(color);
        self
    }

    /// Set star rating (0-5)
    #[must_use] 
    pub fn rating(mut self, stars: u8) -> Self {
        self.rating = Some(stars.min(5));
        self
    }
}

/// Testimonial card component
///
/// Displays customer testimonials with quotes, author info, and optional ratings.
/// Useful for social proof sections.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::components::TestimonialCard;
/// use armas_basic::testimonial::TestimonialItem;
///
/// let item = TestimonialItem::new(
///     "This product changed my life!",
///     "Jane Doe",
///     "CEO, Company"
/// ).avatar("JD").rating(5);
///
/// TestimonialCard::new(item).show(ui);
/// # }
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
    #[must_use] 
    pub const fn new(item: TestimonialItem) -> Self {
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
    #[must_use] 
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set card height
    #[must_use] 
    pub const fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Enable or disable quote marks
    #[must_use] 
    pub const fn show_quotes(mut self, show: bool) -> Self {
        self.show_quotes = show;
        self
    }

    /// Enable or disable hover effect
    #[must_use] 
    pub const fn hover_effect(mut self, hover: bool) -> Self {
        self.hover_effect = hover;
        self
    }

    /// Enable or disable border
    #[must_use] 
    pub const fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Show the testimonial card
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let available = ui.available_size();
        let desired_width = self.width.unwrap_or(available.x);

        // Allocate space - larger default height for more breathing room
        let (rect, response) = if let Some(height) = self.height {
            ui.allocate_exact_size(Vec2::new(desired_width, height), egui::Sense::hover())
        } else {
            ui.allocate_at_least(Vec2::new(desired_width, 180.0), egui::Sense::hover())
        };

        let is_hovered = response.hovered();

        // Background - use standard style
        let bg_color = if self.hover_effect && is_hovered {
            theme.muted()
        } else {
            theme.card()
        };

        ui.painter().rect_filled(rect, 8.0, bg_color);

        // Border
        if self.show_border {
            ui.painter().rect_stroke(
                rect,
                8.0,
                egui::Stroke::new(1.0, theme.border()),
                egui::StrokeKind::Middle,
            );
        }

        // Content area with generous padding for breathing room
        let content_rect = rect.shrink(24.0);
        let mut y = content_rect.min.y;

        // Quote content with larger, readable text
        let quote_galley = ui.painter().layout(
            self.item.quote.clone(),
            egui::FontId::proportional(15.0),
            theme.foreground(),
            content_rect.width(),
        );
        let quote_height = quote_galley.rect.height();
        ui.painter().galley(
            Pos2::new(content_rect.min.x, y),
            quote_galley,
            theme.foreground(),
        );
        y += quote_height + 20.0;

        // Author section with avatar - spacious layout
        let avatar_size = 40.0;
        let author_x = content_rect.min.x;

        if let Some(avatar_text) = &self.item.avatar {
            // Draw avatar circle
            let avatar_center = Pos2::new(author_x + avatar_size / 2.0, y + avatar_size / 2.0);
            let avatar_color = self.item.avatar_color.unwrap_or_else(|| theme.primary());

            ui.painter()
                .circle_filled(avatar_center, avatar_size / 2.0, avatar_color);

            // Avatar text
            ui.painter().text(
                avatar_center,
                egui::Align2::CENTER_CENTER,
                avatar_text,
                egui::FontId::proportional(16.0),
                Color32::WHITE,
            );

            // Author info next to avatar
            let info_x = author_x + avatar_size + 12.0;
            ui.painter().text(
                Pos2::new(info_x, y + 8.0),
                egui::Align2::LEFT_TOP,
                &self.item.author,
                egui::FontId::proportional(14.0),
                theme.foreground(),
            );
            ui.painter().text(
                Pos2::new(info_x, y + 24.0),
                egui::Align2::LEFT_TOP,
                &self.item.role,
                egui::FontId::proportional(12.0),
                theme.muted_foreground(),
            );
        } else {
            // No avatar, just text
            ui.painter().text(
                Pos2::new(author_x, y),
                egui::Align2::LEFT_TOP,
                &self.item.author,
                egui::FontId::proportional(14.0),
                theme.foreground(),
            );
            ui.painter().text(
                Pos2::new(author_x, y + 18.0),
                egui::Align2::LEFT_TOP,
                &self.item.role,
                egui::FontId::proportional(12.0),
                theme.muted_foreground(),
            );
        }

        response
    }
}

/// Testimonial grid for displaying multiple testimonials
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_basic::TestimonialGrid;
///
/// TestimonialGrid::new()
///     .columns(2)
///     .gap(20.0)
///     .show(ui, |grid| {
///         grid.testimonial("Great product!", "Jane Doe", "CEO, Company")
///             .avatar("JD")
///             .rating(5);
///         grid.testimonial("Highly recommended", "John Smith", "CTO, Startup")
///             .avatar("JS")
///             .rating(4);
///     });
/// # }
/// ```
pub struct TestimonialGrid {
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
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            columns: None,
            gap: 20.0,
            show_quotes: true,
            hover_effect: true,
        }
    }

    /// Set number of columns
    #[must_use] 
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = Some(columns.max(1));
        self
    }

    /// Set gap between cards
    #[must_use] 
    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Enable or disable quote marks
    #[must_use] 
    pub const fn show_quotes(mut self, show: bool) -> Self {
        self.show_quotes = show;
        self
    }

    /// Enable or disable hover effect
    #[must_use] 
    pub const fn hover_effect(mut self, hover: bool) -> Self {
        self.hover_effect = hover;
        self
    }

    /// Show the testimonial grid with closure-based API
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut TestimonialGridBuilder) -> R) {
        let _theme = ui.ctx().armas_theme();
        let available_width = ui.available_width();

        // Build testimonials using closure
        let mut builder = TestimonialGridBuilder {
            items: Vec::new(),
            item_index: 0,
        };
        content(&mut builder);
        let items = builder.items;

        // Calculate columns
        let columns = self.columns.unwrap_or_else(|| {
            let min_card_width = 300.0;
            ((available_width / min_card_width).floor() as usize).clamp(1, 3)
        });

        let card_width = (available_width - (columns - 1) as f32 * self.gap) / columns as f32;

        // Render grid
        let chunks: Vec<_> = items.chunks(columns).collect();

        for (row_idx, row) in chunks.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = self.gap;
                for item in *row {
                    TestimonialCard::new(item.clone())
                        .width(card_width)
                        .show_quotes(self.show_quotes)
                        .hover_effect(self.hover_effect)
                        .show(ui);
                }
            });

            if row_idx < chunks.len() - 1 {
                ui.add_space(self.gap);
            }
        }
    }
}

impl Default for TestimonialGrid {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding testimonials
pub struct TestimonialGridBuilder {
    items: Vec<TestimonialItem>,
    item_index: usize,
}

impl TestimonialGridBuilder {
    /// Add a testimonial
    pub fn testimonial(
        &mut self,
        quote: &str,
        author: &str,
        role: &str,
    ) -> TestimonialItemBuilder<'_> {
        let item = TestimonialItem {
            quote: quote.to_string(),
            author: author.to_string(),
            role: role.to_string(),
            avatar: None,
            avatar_color: None,
            rating: None,
        };

        self.items.push(item);
        let current_index = self.item_index;
        self.item_index += 1;

        TestimonialItemBuilder {
            items: &mut self.items,
            item_index: current_index,
        }
    }
}

/// Builder for chaining testimonial modifiers
pub struct TestimonialItemBuilder<'a> {
    items: &'a mut Vec<TestimonialItem>,
    item_index: usize,
}

impl TestimonialItemBuilder<'_> {
    /// Set avatar text (emoji or initials)
    #[must_use] 
    pub fn avatar(self, avatar: &str) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.avatar = Some(avatar.to_string());
        }
        self
    }

    /// Set avatar color
    #[must_use] 
    pub fn avatar_color(self, color: Color32) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.avatar_color = Some(color);
        }
        self
    }

    /// Set star rating (0-5)
    #[must_use] 
    pub fn rating(self, stars: u8) -> Self {
        if let Some(item) = self.items.get_mut(self.item_index) {
            item.rating = Some(stars.min(5));
        }
        self
    }
}
