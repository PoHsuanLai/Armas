use crate::layout::Alignment;
use crate::Theme;
use egui::{Response, Ui, Vec2};

/// Predefined container sizes following Material Design breakpoints
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerSize {
    /// Small - 600px max width
    Small,
    /// Medium - 960px max width
    Medium,
    /// Large - 1280px max width
    Large,
    /// Extra Large - 1920px max width
    ExtraLarge,
    /// Full width - no max width constraint
    Full,
    /// Percentage of available width (0.0 to 1.0)
    Percentage(f32),
    /// Fixed pixel width
    Fixed(f32),
}

impl ContainerSize {
    /// Get the max width in pixels (None for percentage-based sizing)
    pub fn max_width(self) -> Option<f32> {
        match self {
            ContainerSize::Small => Some(600.0),
            ContainerSize::Medium => Some(960.0),
            ContainerSize::Large => Some(1280.0),
            ContainerSize::ExtraLarge => Some(1920.0),
            ContainerSize::Full => None,
            ContainerSize::Percentage(_) => None, // Calculated at runtime
            ContainerSize::Fixed(width) => Some(width),
        }
    }

    /// Calculate actual width given available width
    pub fn calculate_width(self, available_width: f32) -> f32 {
        match self {
            ContainerSize::Small => available_width.min(600.0),
            ContainerSize::Medium => available_width.min(960.0),
            ContainerSize::Large => available_width.min(1280.0),
            ContainerSize::ExtraLarge => available_width.min(1920.0),
            ContainerSize::Full => available_width,
            ContainerSize::Percentage(pct) => available_width * pct.clamp(0.0, 1.0),
            ContainerSize::Fixed(width) => width.min(available_width),
        }
    }
}

/// Container with max-width and padding
///
/// Centers content and constrains width, inspired by SwiftUI's frame modifier
/// and Material Design containers.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::{Container, ContainerSize, Alignment};
/// use armas::Theme;
///
/// let theme = Theme::dark();
///
/// // Fixed breakpoint
/// Container::new(ContainerSize::Medium)
///     .padding(16.0)
///     .alignment(Alignment::Center)
///     .show(ui, &theme, |ui| {
///         ui.label("Centered content with max-width");
///     });
///
/// // Percentage-based (responsive)
/// Container::new(ContainerSize::Percentage(0.8))
///     .show(ui, &theme, |ui| {
///         ui.label("80% of available width");
///     });
/// ```
pub struct Container {
    size: ContainerSize,
    padding: Option<f32>,
    alignment: Alignment,
}

impl Container {
    /// Create a new container with the given size
    pub fn new(size: ContainerSize) -> Self {
        Self {
            size,
            padding: None,
            alignment: Alignment::Center,
        }
    }

    /// Set custom padding (overrides theme padding)
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Set horizontal alignment
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Show the container with the given content
    pub fn show<R>(
        self,
        ui: &mut Ui,
        theme: &Theme,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Response {
        let padding = self.padding.unwrap_or(16.0);
        let available_width = ui.available_width();

        // Calculate container width using new method
        let container_width = self.size.calculate_width(available_width);

        // Calculate horizontal offset for alignment
        let offset_x = match self.alignment {
            Alignment::Leading => 0.0,
            Alignment::Center => (available_width - container_width) / 2.0,
            Alignment::Trailing => available_width - container_width,
            _ => (available_width - container_width) / 2.0,
        };

        ui.horizontal(|ui| {
            if offset_x > 0.0 {
                ui.add_space(offset_x);
            }

            ui.allocate_ui(Vec2::new(container_width, ui.available_height()), |ui| {
                ui.vertical(|ui| {
                    ui.add_space(padding); // Top padding
                    ui.horizontal(|ui| {
                        ui.add_space(padding); // Left padding
                        ui.vertical(|ui| {
                            ui.set_max_width(container_width - padding * 2.0);
                            content(ui)
                        });
                        ui.add_space(padding); // Right padding
                    });
                    ui.add_space(padding); // Bottom padding
                })
            })
            .response
        })
        .response
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new(ContainerSize::Medium)
    }
}
