use crate::layout::Alignment;
use egui::{Response, Ui, Vec2};

/// Stack items on top of each other (Z-axis)
///
/// Overlays multiple UI elements in the same space.
/// Inspired by SwiftUI's ZStack.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::{ZStack, Alignment};
///
/// ZStack::new()
///     .alignment(Alignment::Center)
///     .show(ui, |z| {
///         z.layer(|ui| {
///             // Background layer
///             ui.label("Background");
///         });
///         z.layer(|ui| {
///             // Foreground layer
///             ui.label("Foreground");
///         });
///     });
/// ```
pub struct ZStack {
    alignment: Alignment,
    width: Option<f32>,
    height: Option<f32>,
}

impl ZStack {
    /// Create a new ZStack
    pub fn new() -> Self {
        Self {
            alignment: Alignment::Center,
            width: None,
            height: None,
        }
    }

    /// Set the alignment of layers
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set fixed height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Show the ZStack with the given content builder
    pub fn show<R>(self, ui: &mut Ui, builder: impl FnOnce(&mut ZStackBuilder) -> R) -> Response {
        let available = ui.available_size();
        let width = self.width.unwrap_or(available.x);
        let height = self.height.unwrap_or(available.y);

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());

        let painter = ui.painter().clone();

        let mut z_builder = ZStackBuilder {
            ui,
            rect,
            alignment: self.alignment,
            painter,
        };

        builder(&mut z_builder);

        response
    }
}

impl Default for ZStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding layers to the ZStack
pub struct ZStackBuilder<'a> {
    ui: &'a mut Ui,
    rect: egui::Rect,
    alignment: Alignment,
    painter: egui::Painter,
}

impl<'a> ZStackBuilder<'a> {
    /// Add a layer to the stack
    pub fn layer<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> R {
        let layout = match self.alignment {
            Alignment::Leading => egui::Layout::left_to_right(egui::Align::Min),
            Alignment::Trailing => egui::Layout::right_to_left(egui::Align::Min),
            Alignment::Top => egui::Layout::top_down(egui::Align::Min),
            Alignment::Bottom => egui::Layout::bottom_up(egui::Align::Min),
            Alignment::Center => egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
        };

        self.ui
            .allocate_ui_at_rect(self.rect, |ui| {
                ui.with_layout(layout, |ui| content(ui)).inner
            })
            .inner
    }
}
