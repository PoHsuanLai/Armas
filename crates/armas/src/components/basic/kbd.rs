//! Kbd Component (shadcn/ui style)
//!
//! Keyboard shortcut display element.

use crate::theme::Theme;
use egui::{Response, Ui, Vec2};

/// Keyboard shortcut display component
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::Kbd;
/// use armas::ext::ArmasContextExt;
///
/// let theme = ui.ctx().armas_theme();
/// // Single key
/// Kbd::new("K").show(ui, &theme);
///
/// // Key combination (auto-splits on +)
/// Kbd::new("Ctrl+K").show(ui, &theme);
/// # }
/// ```
pub struct Kbd {
    text: String,
}

impl Kbd {
    /// Create a new Kbd with the given key text
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    /// Show the keyboard shortcut
    pub fn show(self, ui: &mut Ui, theme: &crate::Theme) -> Response {
        // Check if this is a key combination
        let parts: Vec<&str> = self.text.split('+').map(|s| s.trim()).collect();

        if parts.len() > 1 {
            // Multiple keys - render as group
            let response = ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        ui.label("+");
                    }
                    render_key(ui, part, theme);
                }
            });
            response.response
        } else {
            // Single key
            render_key(ui, &self.text, theme)
        }
    }
}

fn render_key(ui: &mut Ui, text: &str, theme: &Theme) -> Response {
    let font_size = 12.0;
    let font_id = egui::FontId::proportional(font_size);
    let text_color = theme.muted_foreground();
    let bg_color = theme.muted();

    // Calculate text size
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_string(), font_id.clone(), text_color);

    let text_size = galley.size();
    let padding_x = 8.0;
    let padding_y = 4.0;
    let min_width = 24.0;
    let height = font_size + padding_y * 2.0;

    let size = Vec2::new((text_size.x + padding_x * 2.0).max(min_width), height);

    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let rounding = 4.0;

        // Background
        ui.painter().rect_filled(rect, rounding, bg_color);

        // Border for depth
        ui.painter().rect_stroke(
            rect,
            rounding,
            egui::Stroke::new(1.0, theme.border()),
            egui::StrokeKind::Inside,
        );

        // Text centered
        ui.painter()
            .galley(rect.center() - text_size / 2.0, galley, text_color);
    }

    response
}
