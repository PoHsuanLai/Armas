use egui::{Response, Ui, Vec2};

/// Maintain aspect ratio wrapper
///
/// Constrains content to a specific aspect ratio.
/// Inspired by SwiftUI's aspectRatio modifier.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::AspectRatio;
///
/// // 16:9 aspect ratio
/// AspectRatio::new(16.0 / 9.0)
///     .show(ui, |ui| {
///         ui.label("16:9 content");
///     });
///
/// // Square (1:1)
/// AspectRatio::square()
///     .show(ui, |ui| {
///         ui.label("Square content");
///     });
/// ```
pub struct AspectRatio {
    ratio: f32,
    content_mode: ContentMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentMode {
    /// Content fills available space (may be cropped)
    Fill,
    /// Content fits within available space (may have empty space)
    Fit,
}

impl AspectRatio {
    /// Create aspect ratio constraint (width / height)
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio,
            content_mode: ContentMode::Fit,
        }
    }

    /// Create a square aspect ratio (1:1)
    pub fn square() -> Self {
        Self::new(1.0)
    }

    /// Create 16:9 aspect ratio (widescreen)
    pub fn widescreen() -> Self {
        Self::new(16.0 / 9.0)
    }

    /// Create 4:3 aspect ratio (standard)
    pub fn standard() -> Self {
        Self::new(4.0 / 3.0)
    }

    /// Set content mode (fill or fit)
    pub fn content_mode(mut self, mode: ContentMode) -> Self {
        self.content_mode = mode;
        self
    }

    /// Show the aspect ratio container with the given content
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> Response {
        let available = ui.available_size();

        // Calculate size maintaining aspect ratio (ratio = width / height)
        let size = match self.content_mode {
            ContentMode::Fit => {
                // If available height is 0 or very small, use width to calculate both dimensions
                if available.y < 1.0 {
                    let height = available.x / self.ratio;
                    Vec2::new(available.x, height)
                } else {
                    // Calculate what height we'd get if we use full width
                    let height_from_width = available.x / self.ratio;
                    // Calculate what width we'd get if we use full height
                    let width_from_height = available.y * self.ratio;

                    if height_from_width <= available.y {
                        // Width is the limiting factor - use full width
                        Vec2::new(available.x, height_from_width)
                    } else {
                        // Height is the limiting factor - use full height
                        Vec2::new(width_from_height, available.y)
                    }
                }
            }
            ContentMode::Fill => {
                // If available height is 0 or very small, use width to calculate both dimensions
                if available.y < 1.0 {
                    let height = available.x / self.ratio;
                    Vec2::new(available.x, height)
                } else {
                    let height_from_width = available.x / self.ratio;
                    let width_from_height = available.y * self.ratio;

                    if height_from_width >= available.y {
                        // Width gives us more - use it
                        Vec2::new(available.x, height_from_width)
                    } else {
                        // Height gives us more - use it
                        Vec2::new(width_from_height, available.y)
                    }
                }
            }
        };

        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());

        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| content(ui));

        response
    }
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self::new(1.0)
    }
}
