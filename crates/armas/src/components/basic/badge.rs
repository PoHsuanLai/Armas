use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{Color32, Pos2, Response, Ui, Vec2};

/// Badge variant styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    /// Solid fill background
    Filled,
    /// Outline style
    Outlined,
    /// Subtle background
    Soft,
}

/// Badge color themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeColor {
    /// Primary theme color
    Primary,
    /// Success/positive (green)
    Success,
    /// Warning/caution (yellow)
    Warning,
    /// Error/danger (red)
    Error,
    /// Informational (blue)
    Info,
    /// Neutral/default (gray)
    Neutral,
}

/// Small status indicator badge
///
/// Compact component for displaying status, counts, labels, or categories.
/// Perfect for tags, notifications, and status indicators.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::{Badge, BadgeVariant, BadgeColor};
///
/// fn ui(ui: &mut egui::Ui) {
///     Badge::new("New")
///         .variant(BadgeVariant::Filled)
///         .color(BadgeColor::Success)
///         .show(ui);
/// }
/// ```
pub struct Badge {
    /// Badge text
    text: String,
    /// Visual variant
    variant: BadgeVariant,
    /// Color theme
    color: BadgeColor,
    /// Show dot indicator
    show_dot: bool,
    /// Custom size
    size: f32,
    /// Removable (shows × button)
    removable: bool,
}

impl Badge {
    /// Create a new badge
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: BadgeVariant::Soft,
            color: BadgeColor::Primary,
            show_dot: false,
            size: 13.0,
            removable: false,
        }
    }

    /// Set badge variant
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set badge color
    pub fn color(mut self, color: BadgeColor) -> Self {
        self.color = color;
        self
    }

    /// Show dot indicator
    pub fn dot(mut self) -> Self {
        self.show_dot = true;
        self
    }

    /// Set text size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Make badge removable
    pub fn removable(mut self) -> Self {
        self.removable = true;
        self
    }

    /// Show the badge
    pub fn show(self, ui: &mut Ui) -> BadgeResponse {
        let theme = ui.ctx().armas_theme();
        let (bg_color, text_color, border_color) = self.get_colors(&theme);

        // Calculate size
        let font_id = egui::FontId::proportional(self.size);
        let text_galley =
            ui.painter()
                .layout_no_wrap(self.text.clone(), font_id.clone(), text_color);
        let text_width = text_galley.rect.width();

        let dot_space = if self.show_dot { theme.spacing.md } else { 0.0 };
        let remove_space = if self.removable {
            theme.spacing.lg
        } else {
            0.0
        };
        let padding = theme.spacing.md;

        let width = text_width + dot_space + remove_space + padding;
        let height = self.size + theme.spacing.md;

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());

        // Background
        match self.variant {
            BadgeVariant::Filled => {
                ui.painter().rect_filled(rect, height / 2.0, bg_color);
            }
            BadgeVariant::Outlined => {
                ui.painter().rect_stroke(
                    rect,
                    height / 2.0,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Middle,
                );
            }
            BadgeVariant::Soft => {
                ui.painter().rect_filled(rect, height / 2.0, bg_color);
            }
        }

        let mut x = rect.min.x + theme.spacing.sm;

        // Dot indicator
        if self.show_dot {
            let dot_center = Pos2::new(x + theme.spacing.xs, rect.center().y);
            ui.painter().circle_filled(dot_center, 3.0, text_color);
            x += theme.spacing.md;
        }

        // Text
        ui.painter().text(
            Pos2::new(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            &self.text,
            font_id,
            text_color,
        );

        x += text_width + theme.spacing.xs;

        // Remove button
        let mut was_clicked = false;
        if self.removable {
            let remove_rect = egui::Rect::from_center_size(
                Pos2::new(x + theme.spacing.sm, rect.center().y),
                Vec2::splat(theme.spacing.md),
            );

            let is_hovered = ui.rect_contains_pointer(remove_rect);

            if is_hovered {
                ui.painter().circle_filled(
                    remove_rect.center(),
                    8.0,
                    text_color.gamma_multiply(0.2),
                );
            }

            // Draw X
            let cross_size = 4.0;
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
            removed: was_clicked,
            response,
        }
    }

    /// Get colors based on variant and color theme
    fn get_colors(&self, theme: &Theme) -> (Color32, Color32, Color32) {
        let base_color = match self.color {
            BadgeColor::Primary => theme.primary(),
            BadgeColor::Success => theme.success(),
            BadgeColor::Warning => theme.warning(),
            BadgeColor::Error => theme.error(),
            BadgeColor::Info => theme.info(),
            BadgeColor::Neutral => theme.outline(),
        };

        match self.variant {
            BadgeVariant::Filled => {
                // Use theme-aware text color for filled badges
                // For filled badges, we need high contrast text
                let text_color = if self.color == BadgeColor::Warning {
                    theme.on_surface() // Dark text on yellow
                } else {
                    theme.on_background() // Light text on dark colors
                };
                (base_color, text_color, base_color)
            }
            BadgeVariant::Outlined => (Color32::TRANSPARENT, base_color, base_color),
            BadgeVariant::Soft => {
                let soft_bg = Color32::from_rgba_unmultiplied(
                    base_color.r(),
                    base_color.g(),
                    base_color.b(),
                    40,
                );
                (soft_bg, base_color, base_color)
            }
        }
    }
}

/// Response from a badge
#[derive(Debug, Clone)]
pub struct BadgeResponse {
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
    /// Badge color (None = use theme error color)
    color: Option<Color32>,
    /// Size
    size: f32,
}

impl NotificationBadge {
    /// Create a new notification badge with count
    /// Color defaults to theme error color
    pub fn new(count: usize) -> Self {
        Self {
            count,
            max_count: Some(99),
            color: None, // Will use theme.error()
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
    pub fn show(&self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();

        // Use custom color or theme error
        let color = self.color.unwrap_or_else(|| theme.error());

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

        // Count text - use theme color for better contrast
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &text,
            egui::FontId::proportional(self.size * 0.6),
            theme.on_background(),
        );

        response
    }
}
