use crate::layout::{calc_flex_widths, Alignment, FlexItem};
use egui::{Response, Ui, Vec2};

/// Horizontal stack with spacing between items
///
/// Arranges children horizontally with consistent spacing.
/// Inspired by SwiftUI's HStack.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::{HStack, Alignment};
///
/// HStack::new(8.0)
///     .alignment(Alignment::Center)
///     .show(ui, |ui| {
///         ui.label("Left");
///         ui.label("Center");
///         ui.label("Right");
///     });
/// ```
///
/// Flex mode example:
/// ```rust,no_run
/// use armas::layout::HStack;
///
/// HStack::new(8.0)
///     .flex()
///     .show(ui, |stack| {
///         stack.item_fixed(240.0, |ui| {
///             ui.label("Sidebar");
///         });
///         stack.item_flex(1.0, |ui| {
///             ui.label("Main content");
///         });
///     });
/// ```
pub struct HStack {
    spacing: f32,
    alignment: Alignment,
    flex_mode: bool,
}

impl HStack {
    /// Create a new horizontal stack with the given spacing between items
    pub fn new(spacing: f32) -> Self {
        Self {
            spacing,
            alignment: Alignment::Center,
            flex_mode: false,
        }
    }

    /// Set the vertical alignment of items
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Enable flex mode for proportional sizing
    pub fn flex(mut self) -> Self {
        self.flex_mode = true;
        self
    }

    /// Show the horizontal stack with the given content (non-flex mode)
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> Response {
        // Use ui.horizontal for proper sizing behavior
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = self.spacing;
            content(ui)
        })
        .response
    }

    /// Show the horizontal stack in flex mode with builder
    pub fn show_flex<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut HStackBuilder) -> R,
    ) -> Response {
        let mut builder = HStackBuilder {
            ui,
            spacing: self.spacing,
            alignment: self.alignment,
            items: Vec::new(),
        };
        content(&mut builder);
        builder.layout()
    }
}

/// Builder for HStack items (supports both normal and flex modes)
pub struct HStackBuilder<'a> {
    ui: &'a mut Ui,
    spacing: f32,
    alignment: Alignment,
    items: Vec<(FlexItem, Box<dyn FnOnce(&mut Ui) + 'a>)>,
}

impl<'a> HStackBuilder<'a> {
    /// Add a fixed-width item
    pub fn item_fixed<R>(&mut self, width: f32, content: impl FnOnce(&mut Ui) -> R + 'a) {
        self.items.push((
            FlexItem::Fixed(width),
            Box::new(move |ui| {
                content(ui);
            }),
        ));
    }

    /// Add a flex item with the given ratio
    pub fn item_flex<R>(&mut self, ratio: f32, content: impl FnOnce(&mut Ui) -> R + 'a) {
        self.items.push((
            FlexItem::Flex(ratio),
            Box::new(move |ui| {
                content(ui);
            }),
        ));
    }

    fn layout(mut self) -> Response {
        let available_width = self.ui.available_width();
        let available_height = self.ui.available_height();

        // Extract specs for width calculation
        let specs: Vec<FlexItem> = self.items.iter().map(|(spec, _)| *spec).collect();
        let widths = calc_flex_widths(available_width, self.spacing, &specs);

        self.ui
            .horizontal(|ui| {
                for (i, ((_spec, content), width)) in
                    self.items.into_iter().zip(widths.iter()).enumerate()
                {
                    if i > 0 {
                        ui.add_space(self.spacing);
                    }
                    ui.allocate_ui(Vec2::new(*width, available_height), |ui| {
                        content(ui);
                    });
                }
            })
            .response
    }
}

impl Default for HStack {
    fn default() -> Self {
        Self::new(8.0)
    }
}
