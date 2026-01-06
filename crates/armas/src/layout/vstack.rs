use crate::layout::{calc_flex_heights, Alignment, FlexItem};
use egui::{Response, Ui, Vec2};

/// Vertical stack with spacing between items
///
/// Arranges children vertically with consistent spacing.
/// Inspired by SwiftUI's VStack.
///
/// # Example
///
/// ```rust,no_run
/// use armas::layout::{VStack, Alignment};
///
/// VStack::new(16.0)
///     .alignment(Alignment::Center)
///     .show(ui, |ui| {
///         ui.label("Item 1");
///         ui.label("Item 2");
///         ui.label("Item 3");
///     });
/// ```
///
/// Flex mode example:
/// ```rust,no_run
/// use armas::layout::VStack;
///
/// VStack::new(8.0)
///     .flex()
///     .show(ui, |stack| {
///         stack.item_fixed(60.0, |ui| {
///             ui.label("Header");
///         });
///         stack.item_flex(1.0, |ui| {
///             ui.label("Main content");
///         });
///         stack.item_fixed(40.0, |ui| {
///             ui.label("Footer");
///         });
///     });
/// ```
pub struct VStack {
    spacing: f32,
    alignment: Alignment,
    flex_mode: bool,
}

impl VStack {
    /// Create a new vertical stack with the given spacing between items
    pub fn new(spacing: f32) -> Self {
        Self {
            spacing,
            alignment: Alignment::Leading,
            flex_mode: false,
        }
    }

    /// Set the horizontal alignment of items
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Enable flex mode for proportional sizing
    pub fn flex(mut self) -> Self {
        self.flex_mode = true;
        self
    }

    /// Show the vertical stack with the given content (non-flex mode)
    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> Response {
        let align = match self.alignment {
            Alignment::Leading => egui::Align::Min,
            Alignment::Center => egui::Align::Center,
            Alignment::Trailing => egui::Align::Max,
            _ => egui::Align::Min,
        };

        ui.with_layout(egui::Layout::top_down(align).with_main_wrap(false), |ui| {
            ui.spacing_mut().item_spacing.y = self.spacing;
            content(ui)
        })
        .response
    }

    /// Show the vertical stack and return both the response and the inner value
    pub fn show_with_inner<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> egui::InnerResponse<R> {
        let align = match self.alignment {
            Alignment::Leading => egui::Align::Min,
            Alignment::Center => egui::Align::Center,
            Alignment::Trailing => egui::Align::Max,
            _ => egui::Align::Min,
        };

        ui.with_layout(egui::Layout::top_down(align).with_main_wrap(false), |ui| {
            ui.spacing_mut().item_spacing.y = self.spacing;
            content(ui)
        })
    }

    /// Show the vertical stack in flex mode with builder
    pub fn show_flex<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut VStackBuilder) -> R,
    ) -> Response {
        let mut builder = VStackBuilder {
            ui,
            spacing: self.spacing,
            alignment: self.alignment,
            items: Vec::new(),
        };
        content(&mut builder);
        builder.layout()
    }
}

/// Builder for VStack items (supports both normal and flex modes)
pub struct VStackBuilder<'a> {
    ui: &'a mut Ui,
    spacing: f32,
    alignment: Alignment,
    items: Vec<(FlexItem, Box<dyn FnOnce(&mut Ui) + 'a>)>,
}

impl<'a> VStackBuilder<'a> {
    /// Add a fixed-height item
    pub fn item_fixed<R>(&mut self, height: f32, content: impl FnOnce(&mut Ui) -> R + 'a) {
        self.items.push((
            FlexItem::Fixed(height),
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

        // Extract specs for height calculation
        let specs: Vec<FlexItem> = self.items.iter().map(|(spec, _)| *spec).collect();
        let heights = calc_flex_heights(available_height, self.spacing, &specs);

        self.ui
            .vertical(|ui| {
                for (i, ((_spec, content), height)) in
                    self.items.into_iter().zip(heights.iter()).enumerate()
                {
                    if i > 0 {
                        ui.add_space(self.spacing);
                    }
                    ui.allocate_ui(Vec2::new(available_width, *height), |ui| {
                        content(ui);
                    });
                }
            })
            .response
    }
}

impl Default for VStack {
    fn default() -> Self {
        Self::new(8.0)
    }
}
