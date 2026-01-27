//! Icon system for Armas
//!
//! Re-exports the generic icon infrastructure from [`armas_icon`] and provides
//! window control icons for use with [`FloatingWindow`](crate::components::cards::FloatingWindow).
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas::icon::{WindowIcon, WindowIconWidget};
//! use armas::ext::ArmasContextExt;
//! use egui::Color32;
//!
//! let theme = ui.ctx().armas_theme();
//! WindowIconWidget::new(WindowIcon::Close)
//!     .size(12.0)
//!     .color(Color32::WHITE)
//!     .show(ui, &theme);
//! # }
//! ```

// Re-export the generic icon infrastructure
pub use armas_icon::{render_icon, Icon, IconData};

// Include the generated window icon data
include!(concat!(env!("OUT_DIR"), "/window_icons.rs"));

use egui::{Color32, Response, Sense, Ui, Vec2};

/// Window and UI icons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowIcon {
    /// Close button (X)
    Close,
    /// Enter full screen
    IntoFullScreen,
    /// Exit full screen
    ExitFullScreen,
    /// Info icon (circle with i)
    Info,
    /// Error icon (circle with !)
    Error,
    /// Chevron left (<)
    ChevronLeft,
    /// Chevron right (>)
    ChevronRight,
    /// Chevron down (v)
    ChevronDown,
}

impl WindowIcon {
    /// Get the icon data for this window icon
    pub fn data(self) -> &'static IconData {
        match self {
            Self::Close => &CLOSE,
            Self::IntoFullScreen => &INTO_FULL_SCREEN,
            Self::ExitFullScreen => &EXIT_FULL_SCREEN,
            Self::Info => &INFO,
            Self::Error => &ERROR,
            Self::ChevronLeft => &CHEVRON_LEFT,
            Self::ChevronRight => &CHEVRON_RIGHT,
            Self::ChevronDown => &CHEVRON_DOWN,
        }
    }

    /// Create a WindowIconWidget from this WindowIcon
    pub fn widget(self) -> WindowIconWidget {
        WindowIconWidget::new(self)
    }
}

/// Window icon widget component
///
/// Renders window control icons with theme colors and custom sizing.
pub struct WindowIconWidget {
    icon: WindowIcon,
    size: f32,
    color: Color32,
}

impl WindowIconWidget {
    /// Create a new window icon widget
    pub fn new(icon: WindowIcon) -> Self {
        Self {
            icon,
            size: 12.0,
            color: Color32::WHITE,
        }
    }

    /// Set the icon size (width and height will be equal)
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the icon color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// Show the icon
    pub fn show(self, ui: &mut Ui, _theme: &crate::Theme) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::click());

        if ui.is_rect_visible(rect) {
            let icon_data = self.icon.data();
            if icon_data.vertices.is_empty() {
                // Fallback: draw a placeholder if icon data is missing
                ui.painter().rect_filled(rect, 2.0, Color32::from_gray(100));
            } else {
                render_icon(ui.painter(), rect, icon_data, self.color);
            }
        }

        response
    }
}
