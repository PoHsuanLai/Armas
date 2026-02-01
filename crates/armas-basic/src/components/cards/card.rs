//! Card Component
//!
//! Card container styled like shadcn/ui Card.
//! A surface for displaying grouped content with consistent styling.
//!
//! # Variants
//!
//! - **Filled**: Muted background, no border
//! - **Outlined**: Card background with border (shadcn default)
//! - **Elevated**: Card background with shadow effect
//!
//! # Example
//!
//! ```rust,no_run
//! use armas_basic::{Card, CardVariant, Theme};
//!
//! fn show_cards(ui: &mut egui::Ui, theme: &Theme) {
//!     // Outlined card (shadcn default)
//!     Card::new()
//!         .variant(CardVariant::Outlined)
//!         .title("Card Title")
//!         .show(ui, theme, |ui| {
//!             ui.label("Content goes here");
//!         });
//! }
//! ```

use crate::theme::Theme;
use egui::{self, Color32, CornerRadius};

// shadcn Card constants
const CORNER_RADIUS: f32 = 8.0; // rounded-lg
const PADDING: f32 = 24.0; // p-6
const BORDER_WIDTH: f32 = 1.0;

/// Card variant (shadcn/ui style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    /// Filled card - muted background, no border
    Filled,
    /// Outlined card - card background with border (shadcn default)
    #[default]
    Outlined,
    /// Elevated card - card background with shadow effect
    Elevated,
}

/// Card component styled like shadcn/ui
pub struct Card<'a> {
    /// Optional title for the card
    pub title: Option<&'a str>,
    /// Card variant (Filled, Outlined, Elevated)
    pub variant: CardVariant,
    /// Whether the card is clickable (adds hover effect)
    pub clickable: bool,
    /// Custom width (None = fill available)
    pub width: Option<f32>,
    /// Custom height (None = determined by content)
    pub height: Option<f32>,
    /// Custom min height
    pub min_height: Option<f32>,
    /// Custom max height
    pub max_height: Option<f32>,
    /// Custom inner margin (None = use theme default)
    pub inner_margin: Option<f32>,
    /// Custom asymmetric margin (overrides `inner_margin` if set)
    pub margin: Option<egui::Margin>,
    /// Custom background color (None = use theme default)
    pub fill_color: Option<Color32>,
    /// Custom border color (None = use theme default)
    pub stroke_color: Option<Color32>,
    /// Custom corner radius (None = use theme default)
    pub corner_radius: Option<f32>,
}

impl<'a> Card<'a> {
    /// Create a new card with default Filled variant
    #[must_use]
    pub const fn new() -> Self {
        Self {
            title: None,
            variant: CardVariant::Filled,
            clickable: false,
            width: None,
            height: None,
            min_height: None,
            max_height: None,
            inner_margin: None,
            margin: None,
            fill_color: None,
            stroke_color: None,
            corner_radius: None,
        }
    }

    /// Set the card title
    #[must_use]
    pub const fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Set custom height (forces exact height regardless of content)
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set minimum height
    #[must_use]
    pub const fn min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }

    /// Set maximum height
    #[must_use]
    pub const fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set the Material Design 3 card variant
    #[must_use]
    pub const fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Make the card clickable (adds hover effect)
    #[must_use]
    pub const fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set custom width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set custom inner margin (overrides theme default)
    #[must_use]
    pub const fn inner_margin(mut self, margin: f32) -> Self {
        self.inner_margin = Some(margin);
        self
    }

    /// Set custom asymmetric margin (overrides `inner_margin`)
    /// Use this for different padding on each side
    #[must_use]
    pub const fn margin(mut self, margin: egui::Margin) -> Self {
        self.margin = Some(margin);
        self
    }

    /// Set custom fill/background color (overrides theme default)
    #[must_use]
    pub const fn fill(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set custom stroke/border color (overrides theme default)
    #[must_use]
    pub const fn stroke(mut self, color: Color32) -> Self {
        self.stroke_color = Some(color);
        self
    }

    /// Set custom corner radius (overrides theme default)
    #[must_use]
    pub const fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Alias for `corner_radius` for backwards compatibility
    #[must_use]
    pub const fn rounding(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Enable hover effect (same as clickable)
    #[must_use]
    pub const fn hover_effect(mut self, enable: bool) -> Self {
        self.clickable = enable;
        self
    }

    /// Show the card with content
    ///
    /// # Panics
    ///
    /// Panics if the content closure is not invoked during frame rendering.
    pub fn show<R>(
        self,
        ui: &mut egui::Ui,
        theme: &Theme,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> CardResponse<R> {
        // shadcn/ui variant styling
        let (fill_color, border_width, border_color) = match self.variant {
            CardVariant::Filled => {
                // Filled: muted background, no border
                (
                    self.fill_color.unwrap_or_else(|| theme.muted()),
                    0.0,
                    Color32::TRANSPARENT,
                )
            }
            CardVariant::Outlined => {
                // Outlined: card background with border (shadcn default)
                (
                    self.fill_color.unwrap_or_else(|| theme.card()),
                    BORDER_WIDTH,
                    self.stroke_color.unwrap_or_else(|| theme.border()),
                )
            }
            CardVariant::Elevated => {
                // Elevated: card background with shadow effect (simulated via border)
                (
                    self.fill_color.unwrap_or_else(|| theme.card()),
                    BORDER_WIDTH,
                    self.stroke_color
                        .unwrap_or_else(|| theme.border().gamma_multiply(0.5)),
                )
            }
        };

        let corner_rad = self.corner_radius.unwrap_or(CORNER_RADIUS) as u8;

        let sense = if self.clickable {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        };

        // Use asymmetric margin if provided, otherwise uniform margin (using shadcn PADDING)
        let frame_margin = self.margin.unwrap_or_else(|| {
            let margin_val = self.inner_margin.unwrap_or(PADDING) as i8;
            egui::Margin::same(margin_val)
        });
        let mut content_result = None;

        // If both width and height are specified, use exact size allocation
        let outer_response = if let (Some(width), Some(height)) = (self.width, self.height) {
            let desired_size = egui::Vec2::new(width, height);
            let (rect, _) = ui.allocate_exact_size(desired_size, sense);

            // Create a child UI at the exact allocated rect
            // Use top-down layout
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );

            let frame_response = egui::Frame::new()
                .fill(fill_color)
                .corner_radius(CornerRadius::same(corner_rad))
                .stroke(egui::Stroke::new(border_width, border_color))
                .inner_margin(frame_margin)
                .outer_margin(0.0) // No outer margin to prevent spacing issues
                .show(&mut child_ui, |ui| {
                    // Title if provided
                    if let Some(title) = self.title {
                        ui.label(
                            egui::RichText::new(title)
                                .size(ui.spacing().interact_size.y * 0.7)
                                .color(theme.foreground())
                                .strong(),
                        );
                        ui.add_space(theme.spacing.sm);
                    }

                    // User content (no wrapping - components handle their own layout)
                    content_result = Some(content(ui));
                });

            frame_response
        } else {
            // Fallback to flexible sizing for cases where exact size is not specified
            ui.vertical(|ui| {
                // Apply width constraint
                if let Some(width) = self.width {
                    ui.set_max_width(width);
                }

                // Apply height constraints
                if let Some(height) = self.height {
                    ui.set_height(height);
                }
                if let Some(min_height) = self.min_height {
                    ui.set_min_height(min_height);
                }
                if let Some(max_height) = self.max_height {
                    ui.set_max_height(max_height);
                }

                let frame_response = egui::Frame::new()
                    .fill(fill_color)
                    .corner_radius(CornerRadius::same(corner_rad))
                    .stroke(egui::Stroke::new(border_width, border_color))
                    .inner_margin(frame_margin)
                    .outer_margin(0.0) // No outer margin to prevent spacing issues
                    .show(ui, |ui| {
                        // Title if provided
                        if let Some(title) = self.title {
                            ui.label(
                                egui::RichText::new(title)
                                    .size(ui.spacing().interact_size.y * 0.7)
                                    .color(theme.foreground())
                                    .strong(),
                            );
                            ui.add_space(theme.spacing.sm);
                        }

                        // User content
                        content_result = Some(content(ui));
                    });

                frame_response
            })
            .inner
        };

        // Make the entire frame interactive if clickable
        let rect = outer_response.response.rect;
        let response = if self.clickable {
            ui.interact(rect, ui.id().with("card"), sense)
        } else {
            outer_response.response
        };

        // Apply hover background if clickable and hovered
        if self.clickable && response.hovered() {
            ui.painter()
                .rect_filled(rect, CornerRadius::same(corner_rad), theme.accent());
        }

        CardResponse {
            response,
            inner: content_result.expect("content should be set during frame render"),
        }
    }
}

impl Default for Card<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from showing a card
pub struct CardResponse<R> {
    /// The interaction response for the card
    pub response: egui::Response,
    /// The result from the content closure
    pub inner: R,
}

impl<R> CardResponse<R> {
    /// Whether the card was clicked (if clickable)
    pub fn clicked(&self) -> bool {
        self.response.clicked()
    }

    /// Whether the card is hovered
    pub fn hovered(&self) -> bool {
        self.response.hovered()
    }
}
