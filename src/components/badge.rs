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
/// use armas::{Theme, components::{Badge, BadgeVariant, BadgeColor}};
///
/// fn ui(ui: &mut egui::Ui) {
///     let theme = Theme::dark();
///
///     Badge::new("New")
///         .variant(BadgeVariant::Filled)
///         .color(BadgeColor::Success)
///         .show(ui, &theme);
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
    pub fn with_dot(mut self) -> Self {
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

    /// Show the badge, returns true if remove button was clicked
    pub fn show(self, ui: &mut Ui, theme: &Theme) -> bool {
        let (bg_color, text_color, border_color) = self.get_colors(theme);

        // Calculate size
        let text_galley = ui.painter().layout_no_wrap(
            self.text.clone(),
            egui::FontId::proportional(self.size),
            text_color,
        );
        let text_width = text_galley.rect.width();

        let dot_space = if self.show_dot { 16.0 } else { 0.0 };
        let remove_space = if self.removable { 20.0 } else { 0.0 };
        let padding = 12.0;

        let width = text_width + dot_space + remove_space + padding;
        let height = self.size + 12.0;

        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());

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

        let mut x = rect.min.x + 8.0;

        // Dot indicator
        if self.show_dot {
            let dot_center = Pos2::new(x + 4.0, rect.center().y);
            ui.painter().circle_filled(dot_center, 3.0, text_color);
            x += 12.0;
        }

        // Text
        ui.painter().text(
            Pos2::new(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            &self.text,
            egui::FontId::proportional(self.size),
            text_color,
        );

        x += text_width + 4.0;

        // Remove button
        let mut was_clicked = false;
        if self.removable {
            let remove_rect = egui::Rect::from_center_size(
                Pos2::new(x + 8.0, rect.center().y),
                Vec2::splat(16.0),
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

        was_clicked
    }

    /// Get colors based on variant and color theme
    fn get_colors(&self, theme: &Theme) -> (Color32, Color32, Color32) {
        let base_color = match self.color {
            BadgeColor::Primary => theme.primary(),
            BadgeColor::Success => Color32::from_rgb(34, 197, 94),
            BadgeColor::Warning => Color32::from_rgb(251, 191, 36),
            BadgeColor::Error => Color32::from_rgb(239, 68, 68),
            BadgeColor::Info => Color32::from_rgb(59, 130, 246),
            BadgeColor::Neutral => theme.outline(),
        };

        match self.variant {
            BadgeVariant::Filled => (base_color, Color32::WHITE, base_color),
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

/// Notification badge (typically shows a count)
pub struct NotificationBadge {
    /// Count to display
    count: usize,
    /// Maximum count to show (e.g., 99+ for counts > max)
    max_count: Option<usize>,
    /// Badge color
    color: Color32,
    /// Size
    size: f32,
}

impl NotificationBadge {
    /// Create a new notification badge
    pub fn new(count: usize) -> Self {
        Self {
            count,
            max_count: Some(99),
            color: Color32::from_rgb(239, 68, 68),
            size: 18.0,
        }
    }

    /// Set maximum count display
    pub fn max_count(mut self, max: usize) -> Self {
        self.max_count = Some(max);
        self
    }

    /// Set badge color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Set badge size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Show the notification badge
    pub fn show(&self, ui: &mut Ui) -> Response {
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
            .circle_filled(rect.center(), self.size / 2.0, self.color);

        // Count text
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &text,
            egui::FontId::proportional(self.size * 0.6),
            Color32::WHITE,
        );

        response
    }
}
