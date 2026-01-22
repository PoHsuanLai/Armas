//! Avatar Component
//!
//! User profile images and initials

use crate::ext::ArmasContextExt;
use egui::{vec2, Color32, Rect, Response, Sense, Stroke, Ui};

/// Avatar size presets
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AvatarSize {
    /// Extra small (24px)
    XSmall,
    /// Small (32px)
    Small,
    /// Medium (48px)
    Medium,
    /// Large (64px)
    Large,
    /// Extra large (96px)
    XLarge,
    /// Custom size
    Custom(f32),
}

impl AvatarSize {
    fn size(&self) -> f32 {
        match self {
            AvatarSize::XSmall => 24.0,
            AvatarSize::Small => 32.0,
            AvatarSize::Medium => 48.0,
            AvatarSize::Large => 64.0,
            AvatarSize::XLarge => 96.0,
            AvatarSize::Custom(size) => *size,
        }
    }
}

/// Avatar shape
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarShape {
    /// Circular avatar
    Circle,
    /// Rounded square
    RoundedSquare,
    /// Square
    Square,
}

/// Avatar component for displaying user images or initials
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::{Avatar, AvatarSize, ext::ArmasContextExt};
///
/// // Avatar with initials
/// Avatar::new("JD")
///     .size(AvatarSize::Large)
///     .show(ui);
///
/// // Avatar with custom color
/// let theme = ui.ctx().armas_theme();
/// Avatar::new("AM")
///     .size(AvatarSize::Medium)
///     .color(theme.primary())
///     .show(ui);
///
/// // Avatar with status indicator
/// Avatar::new("JD")
///     .status(egui::Color32::GREEN)
///     .show(ui);
/// # }
/// ```
pub struct Avatar {
    text: String,
    size: AvatarSize,
    shape: AvatarShape,
    background_color: Option<Color32>,
    text_color: Option<Color32>,
    show_border: bool,
    clickable: bool,
    status_color: Option<Color32>,
}

impl Avatar {
    /// Create a new avatar with initials or text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
            background_color: None,
            text_color: None,
            show_border: false,
            clickable: false,
            status_color: None,
        }
    }

    /// Set the avatar size
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Set the avatar shape
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set custom background color
    pub fn color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set custom text color
    pub fn text_color(mut self, color: Color32) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Show a border around the avatar
    pub fn border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Make the avatar clickable
    pub fn clickable(mut self) -> Self {
        self.clickable = true;
        self
    }

    /// Show a status badge with custom color (e.g., online indicator)
    pub fn status(mut self, color: Color32) -> Self {
        self.status_color = Some(color);
        self
    }

    /// Show the avatar
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let size = self.size.size();
        let sense = if self.clickable {
            Sense::click()
        } else {
            Sense::hover()
        };

        let (rect, response) = ui.allocate_exact_size(vec2(size, size), sense);

        if ui.is_rect_visible(rect) {
            let _visuals = ui.style().interact(&response);

            // Background color
            let bg_color = self.background_color.unwrap_or_else(|| {
                // Generate color from text hash for consistency
                let hash = self
                    .text
                    .bytes()
                    .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                let hue = (hash % 360) as f32;
                Color32::from_rgb(
                    (hue.to_radians().sin() * 127.0 + 128.0) as u8,
                    ((hue + 120.0).to_radians().sin() * 127.0 + 128.0) as u8,
                    ((hue + 240.0).to_radians().sin() * 127.0 + 128.0) as u8,
                )
            });

            let text_color = self.text_color.unwrap_or(Color32::WHITE);

            // Draw avatar shape
            let rounding = match self.shape {
                AvatarShape::Circle => size / 2.0,
                AvatarShape::RoundedSquare => size / 8.0,
                AvatarShape::Square => 0.0,
            };

            // Background
            ui.painter().rect_filled(rect, rounding, bg_color);

            // Border
            if self.show_border {
                ui.painter().rect_stroke(
                    rect,
                    rounding,
                    Stroke::new(2.0, theme.border()),
                    egui::StrokeKind::Outside,
                );
            }

            // Hover effect
            if response.hovered() && self.clickable {
                ui.painter()
                    .rect_filled(rect, rounding, Color32::from_black_alpha(30));
            }

            // Text (initials)
            let font_size = size * 0.4;
            let font_id = egui::FontId::proportional(font_size);
            let galley = ui
                .painter()
                .layout_no_wrap(self.text.clone(), font_id, text_color);

            let text_pos = rect.center() - galley.size() / 2.0;
            ui.painter().galley(text_pos, galley, text_color);

            // Status badge
            if let Some(status_color) = self.status_color {
                let badge_size = size * 0.25;
                let badge_pos = rect.right_bottom() - vec2(badge_size * 0.3, badge_size * 0.3);
                let badge_rect = Rect::from_center_size(badge_pos, vec2(badge_size, badge_size));

                // White border around status
                ui.painter().circle_filled(
                    badge_rect.center(),
                    badge_size / 2.0 + 1.5,
                    theme.card(),
                );

                // Status color
                ui.painter().circle_filled(
                    badge_rect.center(),
                    badge_size / 2.0,
                    status_color,
                );
            }
        }

        response
    }
}
