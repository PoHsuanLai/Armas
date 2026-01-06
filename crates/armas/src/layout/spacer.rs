use crate::Theme;
use egui::{Response, Sense, Ui, Vec2};

/// Flexible or fixed spacing component
///
/// Inspired by SwiftUI's Spacer.
/// - Flexible spacer fills available space
/// - Fixed spacer creates exact spacing
/// - Theme-aware spacing helpers (xs, sm, md, lg, xl, xxl)
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::{HStack, Spacer};
/// use armas::Theme;
///
/// let theme = Theme::dark();
///
/// HStack::new(0.0).show(ui, |ui| {
///     ui.label("Left");
///     Spacer::new().show(ui);  // Flexible - pushes to edges
///     ui.label("Right");
/// });
///
/// // Fixed spacing
/// Spacer::fixed(24.0).show(ui);
///
/// // Theme spacing
/// Spacer::md(&theme).show(ui);  // 16px from theme
/// ```
pub struct Spacer {
    min_size: Option<f32>,
    is_vertical: bool,
}

impl Spacer {
    /// Create a flexible spacer that fills available space
    pub fn new() -> Self {
        Self {
            min_size: None,
            is_vertical: false,
        }
    }

    /// Create a fixed-size spacer
    pub fn fixed(size: f32) -> Self {
        Self {
            min_size: Some(size),
            is_vertical: false,
        }
    }

    /// Extra small spacing from theme (4px)
    pub fn xs(theme: &Theme) -> Self {
        Self::fixed(theme.spacing.xs)
    }

    /// Small spacing from theme (8px)
    pub fn sm(theme: &Theme) -> Self {
        Self::fixed(theme.spacing.sm)
    }

    /// Medium spacing from theme (16px)
    pub fn md(theme: &Theme) -> Self {
        Self::fixed(theme.spacing.md)
    }

    /// Large spacing from theme (24px)
    pub fn lg(theme: &Theme) -> Self {
        Self::fixed(theme.spacing.lg)
    }

    /// Extra large spacing from theme (32px)
    pub fn xl(theme: &Theme) -> Self {
        Self::fixed(theme.spacing.xl)
    }

    /// 2XL spacing from theme (48px)
    pub fn xxl(theme: &Theme) -> Self {
        Self::fixed(theme.spacing.xxl)
    }

    /// Set whether this spacer is vertical (default: horizontal in HStack)
    pub fn vertical(mut self) -> Self {
        self.is_vertical = true;
        self
    }

    /// Show the spacer
    pub fn show(self, ui: &mut Ui) -> Response {
        if let Some(size) = self.min_size {
            // Fixed spacer
            let vec_size = if self.is_vertical {
                Vec2::new(0.0, size)
            } else {
                Vec2::new(size, 0.0)
            };
            ui.allocate_response(vec_size, Sense::hover())
        } else {
            // Flexible spacer - expand to fill available space
            let available = ui.available_size();
            let size = if self.is_vertical {
                Vec2::new(0.0, available.y.max(0.0))
            } else {
                Vec2::new(available.x.max(0.0), 0.0)
            };
            ui.allocate_response(size, Sense::hover())
        }
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}
