//! Sheet Component
//!
//! Slide-out side panels styled like shadcn/ui Sheet.
//! Extends from the edge of the screen for navigation, settings, or forms.
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::Context;
//! # fn example(ctx: &Context) {
//! use armas_basic::{Sheet, SheetSide, Theme};
//!
//! let theme = Theme::dark();
//! let mut open = true;
//!
//! Sheet::new("my-sheet")
//!     .side(SheetSide::Right)
//!     .open(open)
//!     .title("Edit Profile")
//!     .description("Make changes to your profile here.")
//!     .show(ctx, &theme, |ui| {
//!         ui.label("Content goes here");
//!     });
//! # }
//! ```

use crate::icon::{render_icon, WindowIcon};
use crate::Theme;
use egui::{vec2, Color32, Key, Pos2, Rect, Sense, Stroke, Ui};

// shadcn Sheet constants
const SHEET_WIDTH_SM: f32 = 320.0; // sm:max-w-sm (default for side sheets)
const SHEET_WIDTH_MD: f32 = 420.0; // roughly max-w-md
const SHEET_WIDTH_LG: f32 = 540.0; // lg:max-w-lg
const SHEET_WIDTH_XL: f32 = 672.0; // xl:max-w-xl
const SHEET_HEIGHT_DEFAULT: f32 = 400.0; // For top/bottom sheets

const HEADER_PADDING: f32 = 24.0; // p-6
const CONTENT_PADDING_X: f32 = 24.0; // px-6
const _FOOTER_PADDING: f32 = 24.0; // p-6 (reserved for future footer support)
const GAP_Y: f32 = 8.0; // gap-2 between title and description

const CLOSE_BUTTON_SIZE: f32 = 16.0; // h-4 w-4
const CLOSE_BUTTON_OFFSET: f32 = 16.0; // top-4 right-4
const CLOSE_BUTTON_ROUNDING: f32 = 4.0; // rounded-sm

const BACKDROP_ALPHA: f32 = 0.8; // bg-black/80
const BORDER_WIDTH: f32 = 1.0;

/// Side from which the sheet slides in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SheetSide {
    /// Slide from top
    Top,
    /// Slide from right (default, like shadcn)
    #[default]
    Right,
    /// Slide from bottom
    Bottom,
    /// Slide from left
    Left,
}

/// Sheet size presets
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SheetSize {
    /// Small (320px)
    #[default]
    Small,
    /// Medium (420px)
    Medium,
    /// Large (540px)
    Large,
    /// Extra large (672px)
    XLarge,
    /// Full width/height
    Full,
    /// Custom size in pixels
    Custom(f32),
}

impl SheetSize {
    fn to_pixels(self) -> f32 {
        match self {
            SheetSize::Small => SHEET_WIDTH_SM,
            SheetSize::Medium => SHEET_WIDTH_MD,
            SheetSize::Large => SHEET_WIDTH_LG,
            SheetSize::XLarge => SHEET_WIDTH_XL,
            SheetSize::Full => 0.0, // Calculated at runtime
            SheetSize::Custom(px) => px,
        }
    }
}

/// Sheet component
///
/// A panel that slides out from the edge of the screen,
/// styled like shadcn/ui Sheet component.
pub struct Sheet {
    id: egui::Id,
    side: SheetSide,
    size: SheetSize,
    title: Option<String>,
    description: Option<String>,
    show_close_button: bool,
    show_backdrop: bool,
    is_open: bool,
}

impl Sheet {
    /// Create a new sheet
    pub fn new(id: impl Into<egui::Id>) -> Self {
        Self {
            id: id.into(),
            side: SheetSide::Right,
            size: SheetSize::Small,
            title: None,
            description: None,
            show_close_button: true,
            show_backdrop: true,
            is_open: false,
        }
    }

    /// Set which side the sheet slides from
    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    /// Set the sheet size
    pub fn size(mut self, size: SheetSize) -> Self {
        self.size = size;
        self
    }

    /// Set the sheet open state
    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    /// Set the title (SheetTitle equivalent)
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the description (SheetDescription equivalent)
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Show or hide the close button (default: true)
    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Show or hide the backdrop overlay (default: true)
    pub fn show_backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    /// Show the sheet and render content
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        content: impl FnOnce(&mut Ui),
    ) -> SheetResponse {
        let mut response = SheetResponse { closed: false };

        if !self.is_open {
            return response;
        }

        #[allow(deprecated)]
        let screen_rect = ctx.screen_rect(); // Full viewport for overlay

        // Calculate sheet dimensions
        let is_horizontal = matches!(self.side, SheetSide::Left | SheetSide::Right);
        let sheet_size = if self.size == SheetSize::Full {
            if is_horizontal {
                screen_rect.width()
            } else {
                screen_rect.height()
            }
        } else {
            self.size.to_pixels()
        };

        // For top/bottom, use a reasonable default if not full
        let sheet_size = if !is_horizontal && self.size != SheetSize::Full {
            sheet_size.min(SHEET_HEIGHT_DEFAULT)
        } else {
            sheet_size
        };

        // Draw backdrop
        if self.show_backdrop {
            let backdrop_color = Color32::from_black_alpha((255.0 * BACKDROP_ALPHA) as u8);

            egui::Area::new(self.id.with("backdrop"))
                .order(egui::Order::Middle)
                .interactable(true)
                .fixed_pos(screen_rect.min)
                .show(ctx, |ui| {
                    let backdrop_response =
                        ui.allocate_response(screen_rect.size(), Sense::click());
                    ui.painter().rect_filled(screen_rect, 0.0, backdrop_color);

                    if backdrop_response.clicked() {
                        response.closed = true;
                    }
                });
        }

        // Calculate sheet rect based on side
        let sheet_rect = match self.side {
            SheetSide::Right => Rect::from_min_size(
                Pos2::new(screen_rect.right() - sheet_size, screen_rect.top()),
                vec2(sheet_size, screen_rect.height()),
            ),
            SheetSide::Left => Rect::from_min_size(
                screen_rect.left_top(),
                vec2(sheet_size, screen_rect.height()),
            ),
            SheetSide::Top => Rect::from_min_size(
                screen_rect.left_top(),
                vec2(screen_rect.width(), sheet_size),
            ),
            SheetSide::Bottom => Rect::from_min_size(
                Pos2::new(screen_rect.left(), screen_rect.bottom() - sheet_size),
                vec2(screen_rect.width(), sheet_size),
            ),
        };

        // Draw the sheet panel
        egui::Area::new(self.id)
            .order(egui::Order::Foreground)
            .fixed_pos(sheet_rect.min)
            .show(ctx, |ui| {
                ui.set_clip_rect(sheet_rect);

                // Background with border
                ui.painter()
                    .rect_filled(sheet_rect, 0.0, theme.background());

                // Border on the edge facing the content
                let border_stroke = Stroke::new(BORDER_WIDTH, theme.border());
                match self.side {
                    SheetSide::Right => {
                        ui.painter()
                            .vline(sheet_rect.left(), sheet_rect.y_range(), border_stroke);
                    }
                    SheetSide::Left => {
                        ui.painter()
                            .vline(sheet_rect.right(), sheet_rect.y_range(), border_stroke);
                    }
                    SheetSide::Top => {
                        ui.painter().hline(
                            sheet_rect.x_range(),
                            sheet_rect.bottom(),
                            border_stroke,
                        );
                    }
                    SheetSide::Bottom => {
                        ui.painter()
                            .hline(sheet_rect.x_range(), sheet_rect.top(), border_stroke);
                    }
                }

                // Close button (top-right corner)
                if self.show_close_button {
                    let close_rect = Rect::from_min_size(
                        Pos2::new(
                            sheet_rect.right() - CLOSE_BUTTON_OFFSET - CLOSE_BUTTON_SIZE,
                            sheet_rect.top() + CLOSE_BUTTON_OFFSET,
                        ),
                        vec2(CLOSE_BUTTON_SIZE, CLOSE_BUTTON_SIZE),
                    );

                    let close_response = ui.interact(
                        close_rect.expand(4.0), // Larger hit area
                        self.id.with("close"),
                        Sense::click(),
                    );

                    // Hover background
                    if close_response.hovered() {
                        ui.painter().rect_filled(
                            close_rect.expand(4.0),
                            CLOSE_BUTTON_ROUNDING,
                            theme.accent(),
                        );
                    }

                    // Draw close icon
                    let icon_color = if close_response.hovered() {
                        theme.foreground()
                    } else {
                        theme.muted_foreground()
                    };
                    render_icon(
                        ui.painter(),
                        close_rect,
                        WindowIcon::Close.data(),
                        icon_color,
                    );

                    if close_response.clicked() {
                        response.closed = true;
                    }
                }

                // Content area
                let content_rect =
                    Rect::from_min_max(sheet_rect.min + vec2(0.0, 0.0), sheet_rect.max);

                let mut content_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content_rect)
                        .layout(egui::Layout::top_down(egui::Align::Min)),
                );

                content_ui.set_clip_rect(content_rect);

                // Header section (title + description)
                if self.title.is_some() || self.description.is_some() {
                    content_ui.add_space(HEADER_PADDING);
                    content_ui.horizontal(|ui| {
                        ui.add_space(HEADER_PADDING);
                        ui.vertical(|ui| {
                            ui.set_width(
                                sheet_rect.width()
                                    - HEADER_PADDING * 2.0
                                    - CLOSE_BUTTON_SIZE
                                    - CLOSE_BUTTON_OFFSET,
                            );

                            if let Some(title) = &self.title {
                                ui.label(
                                    egui::RichText::new(title)
                                        .size(18.0)
                                        .strong()
                                        .color(theme.foreground()),
                                );
                            }

                            if let Some(desc) = &self.description {
                                ui.add_space(GAP_Y);
                                ui.label(
                                    egui::RichText::new(desc)
                                        .size(14.0)
                                        .color(theme.muted_foreground()),
                                );
                            }
                        });
                    });
                    content_ui.add_space(HEADER_PADDING);
                }

                // Main content with horizontal padding
                content_ui.horizontal(|ui| {
                    ui.add_space(CONTENT_PADDING_X);
                    ui.vertical(|ui| {
                        ui.set_width(sheet_rect.width() - CONTENT_PADDING_X * 2.0);
                        content(ui);
                    });
                    ui.add_space(CONTENT_PADDING_X);
                });
            });

        // Handle ESC key
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            response.closed = true;
        }

        response
    }
}

/// Response from showing a sheet
#[derive(Debug, Clone, Copy)]
pub struct SheetResponse {
    /// Whether the sheet was closed this frame (via close button, backdrop, or ESC)
    pub closed: bool,
}

impl SheetResponse {
    /// Check if the sheet was closed
    pub fn closed(&self) -> bool {
        self.closed
    }
}
