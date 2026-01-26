//! Avatar Component
//!
//! User profile images and initials styled like shadcn/ui Avatar.

use egui::{vec2, Response, Sense, Ui};

// shadcn Avatar default size
const DEFAULT_SIZE: f32 = 32.0; // size-8 (2rem)

/// Avatar size presets
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AvatarSize {
    /// Extra small (24px)
    XSmall,
    /// Small (32px) - shadcn default
    Small,
    /// Medium (40px)
    Medium,
    /// Large (48px)
    Large,
    /// Extra large (64px)
    XLarge,
    /// Custom size
    Custom(f32),
}

impl AvatarSize {
    fn to_pixels(&self) -> f32 {
        match self {
            AvatarSize::XSmall => 24.0,
            AvatarSize::Small => 32.0,
            AvatarSize::Medium => 40.0,
            AvatarSize::Large => 48.0,
            AvatarSize::XLarge => 64.0,
            AvatarSize::Custom(size) => *size,
        }
    }
}

impl Default for AvatarSize {
    fn default() -> Self {
        AvatarSize::Small // shadcn default is size-8 (32px)
    }
}

/// Avatar shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    /// Circular avatar (shadcn default)
    #[default]
    Circle,
    /// Rounded square
    Rounded,
}

/// Avatar component for displaying user images or initials
///
/// Styled like shadcn/ui Avatar with a simple fallback showing initials.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::Avatar;
///
/// // Simple avatar with initials
/// Avatar::new("JD").show(ui);
///
/// // Larger avatar
/// Avatar::new("AM").size(48.0).show(ui);
/// # }
/// ```
pub struct Avatar {
    text: String,
    size: f32,
    shape: AvatarShape,
}

impl Avatar {
    /// Create a new avatar with initials or text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: DEFAULT_SIZE,
            shape: AvatarShape::Circle,
        }
    }

    /// Set the avatar size in pixels
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the avatar size using a preset
    pub fn size_preset(mut self, size: AvatarSize) -> Self {
        self.size = size.to_pixels();
        self
    }

    /// Set the avatar shape
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    /// Show the avatar
    pub fn show(self, ui: &mut Ui, theme: &crate::Theme) -> Response {

        let (rect, response) = ui.allocate_exact_size(vec2(self.size, self.size), Sense::hover());

        if ui.is_rect_visible(rect) {
            // shadcn uses rounded-full for circle
            let rounding = match self.shape {
                AvatarShape::Circle => self.size / 2.0,
                AvatarShape::Rounded => 6.0, // rounded-md
            };

            // Background: bg-muted (shadcn fallback style)
            ui.painter().rect_filled(rect, rounding, theme.muted());

            // Text (initials) - centered
            let font_size = self.size * 0.4;
            let font_id = egui::FontId::proportional(font_size);

            // Get just initials (first 2 chars, uppercase)
            let initials: String = self
                .text
                .split_whitespace()
                .filter_map(|word| word.chars().next())
                .take(2)
                .collect::<String>()
                .to_uppercase();

            let display_text = if initials.is_empty() {
                self.text.chars().take(2).collect::<String>().to_uppercase()
            } else {
                initials
            };

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                display_text,
                font_id,
                theme.muted_foreground(),
            );
        }

        response
    }
}
