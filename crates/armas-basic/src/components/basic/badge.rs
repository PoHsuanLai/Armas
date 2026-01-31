//! Badge Component
//!
//! Small status indicator styled like shadcn/ui Badge.
//! Provides variants for different contexts:
//! - Default (primary colored)
//! - Secondary (muted)
//! - Destructive (red)
//! - Outline (border only)

use crate::Theme;
use egui::{Color32, Pos2, Response, Ui, Vec2};

// shadcn Badge constants
const CORNER_RADIUS: f32 = 9999.0; // rounded-full (pill shape)
const PADDING_X: f32 = 10.0; // px-2.5
const PADDING_Y: f32 = 2.0; // py-0.5
const FONT_SIZE: f32 = 12.0; // text-xs

/// Badge variant styles (shadcn/ui)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    /// Primary background (default)
    #[default]
    Default,
    /// Secondary/muted background
    Secondary,
    /// Destructive/error style
    Destructive,
    /// Outline only
    Outline,
}

// Backwards compatibility aliases
#[allow(non_upper_case_globals)]
impl BadgeVariant {
    /// Alias for Default (backwards compatibility)
    pub const Filled: BadgeVariant = BadgeVariant::Default;
    /// Alias for Outline (backwards compatibility)
    pub const Outlined: BadgeVariant = BadgeVariant::Outline;
    /// Alias for Secondary (backwards compatibility)
    pub const Soft: BadgeVariant = BadgeVariant::Secondary;
}

/// Small status indicator badge styled like shadcn/ui
///
/// # Example
///
/// ```rust,no_run
/// use armas_basic::components::{Badge, BadgeVariant};
/// use armas_basic::ext::ArmasContextExt;
///
/// fn ui(ui: &mut egui::Ui) {
///     let theme = ui.ctx().armas_theme();
///     // Default badge
///     Badge::new("New").show(ui, &theme);
///
///     // Secondary badge
///     Badge::new("Draft").variant(BadgeVariant::Secondary).show(ui, &theme);
///
///     // Destructive badge
///     Badge::new("Error").variant(BadgeVariant::Destructive).show(ui, &theme);
///
///     // Outline badge
///     Badge::new("Outline").variant(BadgeVariant::Outline).show(ui, &theme);
/// }
/// ```
pub struct Badge {
    text: String,
    variant: BadgeVariant,
    custom_color: Option<Color32>,
    show_dot: bool,
    removable: bool,
    is_selected: bool,
    custom_font_size: Option<f32>,
    custom_corner_radius: Option<f32>,
    custom_vertical_padding: Option<f32>,
    custom_height: Option<f32>,
    min_width: Option<f32>,
}

impl Badge {
    /// Create a new badge
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: BadgeVariant::default(),
            custom_color: None,
            show_dot: false,
            removable: false,
            is_selected: false,
            custom_font_size: None,
            custom_corner_radius: None,
            custom_vertical_padding: None,
            custom_height: None,
            min_width: None,
        }
    }

    /// Set badge variant
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set custom color (overrides variant colors)
    pub fn color(mut self, color: Color32) -> Self {
        self.custom_color = Some(color);
        self
    }

    /// Make this a destructive badge (shorthand)
    pub fn destructive(mut self) -> Self {
        self.variant = BadgeVariant::Destructive;
        self
    }

    /// Show dot indicator
    pub fn dot(mut self) -> Self {
        self.show_dot = true;
        self
    }

    /// Set text size
    pub fn size(mut self, size: f32) -> Self {
        self.custom_font_size = Some(size);
        self
    }

    /// Make badge removable
    pub fn removable(mut self) -> Self {
        self.removable = true;
        self
    }

    /// Set corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.custom_corner_radius = Some(radius);
        self
    }

    /// Set vertical padding
    pub fn vertical_padding(mut self, padding: f32) -> Self {
        self.custom_vertical_padding = Some(padding);
        self
    }

    /// Set explicit height (overrides computed height)
    pub fn height(mut self, height: f32) -> Self {
        self.custom_height = Some(height);
        self
    }

    /// Set minimum width
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set selected state (for interactive badge use)
    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    /// Show the badge
    pub fn show(self, ui: &mut Ui, theme: &crate::Theme) -> BadgeResponse {
        let (bg_color, text_color, border_color) = self.get_colors(theme);

        // Resolve effective values (custom overrides or defaults)
        let font_size = self.custom_font_size.unwrap_or(FONT_SIZE);
        let corner_radius = self.custom_corner_radius.unwrap_or(CORNER_RADIUS);
        let padding_y = self.custom_vertical_padding.unwrap_or(PADDING_Y);

        // Calculate size
        let font_id = egui::FontId::proportional(font_size);
        let text_galley =
            ui.painter()
                .layout_no_wrap(self.text.clone(), font_id.clone(), text_color);
        let text_width = text_galley.rect.width();

        let dot_space = if self.show_dot { 12.0 } else { 0.0 };
        let remove_space = if self.removable { 16.0 } else { 0.0 };

        let content_width = text_width + dot_space + remove_space + PADDING_X * 2.0;
        let width = if let Some(min_w) = self.min_width {
            content_width.max(min_w)
        } else {
            content_width
        };
        let height = self
            .custom_height
            .unwrap_or(font_size + padding_y * 2.0 + 4.0);

        // Use click sense for interactive badges
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());

        // Draw background
        match self.variant {
            BadgeVariant::Outline => {
                // Outline: no fill, just border
                ui.painter().rect_stroke(
                    rect,
                    corner_radius,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Inside,
                );
            }
            _ => {
                // All other variants: filled background
                ui.painter().rect_filled(rect, corner_radius, bg_color);
            }
        }

        let mut x = rect.min.x + PADDING_X;

        // Dot indicator
        if self.show_dot {
            let dot_center = Pos2::new(x + 3.0, rect.center().y);
            ui.painter().circle_filled(dot_center, 3.0, text_color);
            x += 12.0;
        }

        // Text (centered)
        let text_pos = if self.show_dot || self.removable {
            Pos2::new(x, rect.center().y)
        } else {
            rect.center()
        };
        let text_align = if self.show_dot || self.removable {
            egui::Align2::LEFT_CENTER
        } else {
            egui::Align2::CENTER_CENTER
        };

        ui.painter()
            .text(text_pos, text_align, &self.text, font_id, text_color);

        // Remove button
        let mut was_clicked = false;
        if self.removable {
            x += text_width + 4.0;
            let remove_rect = egui::Rect::from_center_size(
                Pos2::new(x + 6.0, rect.center().y),
                Vec2::splat(12.0),
            );

            let is_hovered = ui.rect_contains_pointer(remove_rect);

            if is_hovered {
                ui.painter().circle_filled(
                    remove_rect.center(),
                    6.0,
                    text_color.gamma_multiply(0.2),
                );
            }

            // Draw X
            let cross_size = 3.0;
            let center = remove_rect.center();
            ui.painter().line_segment(
                [
                    Pos2::new(center.x - cross_size, center.y - cross_size),
                    Pos2::new(center.x + cross_size, center.y + cross_size),
                ],
                egui::Stroke::new(1.5, text_color),
            );
            ui.painter().line_segment(
                [
                    Pos2::new(center.x + cross_size, center.y - cross_size),
                    Pos2::new(center.x - cross_size, center.y + cross_size),
                ],
                egui::Stroke::new(1.5, text_color),
            );

            if is_hovered && ui.input(|i| i.pointer.primary_clicked()) {
                was_clicked = true;
            }
        }

        BadgeResponse {
            clicked: response.clicked(),
            removed: was_clicked,
            response,
        }
    }

    /// Get colors based on variant (shadcn/ui style)
    fn get_colors(&self, theme: &Theme) -> (Color32, Color32, Color32) {
        // Custom color overrides everything
        if let Some(color) = self.custom_color {
            return (color, theme.primary_foreground(), color);
        }

        // When selected, use primary filled style
        if self.is_selected {
            return (theme.primary(), theme.primary_foreground(), theme.primary());
        }

        match self.variant {
            BadgeVariant::Default => (theme.primary(), theme.primary_foreground(), theme.primary()),
            BadgeVariant::Secondary => (
                theme.secondary(),
                theme.secondary_foreground(),
                theme.secondary(),
            ),
            BadgeVariant::Destructive => (
                theme.destructive(),
                theme.destructive_foreground(),
                theme.destructive(),
            ),
            BadgeVariant::Outline => (Color32::TRANSPARENT, theme.foreground(), theme.border()),
        }
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new("")
    }
}

/// Response from a badge
#[derive(Debug, Clone)]
pub struct BadgeResponse {
    /// Whether the badge was clicked
    pub clicked: bool,
    /// Whether the remove button was clicked (only relevant if badge is removable)
    pub removed: bool,
    /// The underlying egui response
    pub response: egui::Response,
}

/// Notification badge (typically shows a count)
pub struct NotificationBadge {
    /// Count to display
    count: usize,
    /// Maximum count to show (e.g., 99+ for counts > max)
    max_count: Option<usize>,
    /// Badge color (None = use theme destructive color)
    color: Option<Color32>,
    /// Size
    size: f32,
}

impl NotificationBadge {
    /// Create a new notification badge with count
    /// Color defaults to theme destructive color
    pub fn new(count: usize) -> Self {
        Self {
            count,
            max_count: Some(99),
            color: None,
            size: 18.0,
        }
    }

    /// Set maximum count display
    pub fn max_count(mut self, max: usize) -> Self {
        self.max_count = Some(max);
        self
    }

    /// Set badge color (overrides theme)
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set badge size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Show the notification badge
    pub fn show(&self, ui: &mut Ui, theme: &crate::Theme) -> Response {
        let color = self.color.unwrap_or_else(|| theme.destructive());

        let text = if let Some(max) = self.max_count {
            if self.count > max {
                format!("{}+", max)
            } else {
                self.count.to_string()
            }
        } else {
            self.count.to_string()
        };

        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), egui::Sense::hover());

        // Circle background
        ui.painter()
            .circle_filled(rect.center(), self.size / 2.0, color);

        // Count text
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &text,
            egui::FontId::proportional(self.size * 0.6),
            theme.primary_foreground(),
        );

        response
    }
}
