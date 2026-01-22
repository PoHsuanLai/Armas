//! Icon system for Armas
//!
//! Provides SVG-based icons that are:
//! - Parsed at compile time for zero runtime overhead
//! - Themeable with dynamic colors
//! - Scalable to any size
//! - Type-safe with enum variants
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # use armas::Theme;
//! # fn example(ui: &mut Ui, theme: &Theme) {
//! use armas::icon::{Icon, TransportIcon};
//!
//! Icon::new(TransportIcon::Play)
//!     .size(24.0)
//!     .color(theme.foreground())
//!     .show(ui);
//! # }
//! ```

use egui::{Response, Sense, Ui, Vec2};

// Include the generated icon data (brings in Color32, Pos2, Rect, etc.)
include!(concat!(env!("OUT_DIR"), "/icon_data.rs"));
use std::collections::HashMap;
use std::sync::OnceLock;

/// Transport control icons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransportIcon {
    /// Play button
    Play,
    /// Pause button
    Pause,
    /// Stop button
    Stop,
    /// Record button
    Record,
    /// Rewind button
    Rewind,
    /// Fast forward button
    Forward,
    /// Loop button
    Loop,
    /// Metronome button
    Metronome,
}

impl TransportIcon {
    /// Get the icon file name
    fn file_name(self) -> &'static str {
        match self {
            Self::Play => "play",
            Self::Pause => "pause",
            Self::Stop => "stop",
            Self::Record => "fad-record",
            Self::Rewind => "back",
            Self::Forward => "forward",
            Self::Loop => "loop",
            Self::Metronome => "fad-metronome",
        }
    }

    /// Get the icon data
    pub fn data(self) -> Option<&'static IconData> {
        static ICON_MAP: OnceLock<HashMap<&'static str, &'static IconData>> = OnceLock::new();

        let map = ICON_MAP.get_or_init(|| {
            TRANSPORT_ICONS
                .iter()
                .map(|(name, data)| (*name, data))
                .collect()
        });

        map.get(self.file_name()).copied()
    }

    /// Create an Icon from this TransportIcon
    pub fn icon(self) -> Icon {
        Icon::new(self)
    }
}

/// Icon component
///
/// Renders SVG-based icons with theme colors and custom sizing.
pub struct Icon {
    icon: TransportIcon,
    size: f32,
    color: Color32,
}

impl Icon {
    /// Create a new icon
    pub fn new(icon: TransportIcon) -> Self {
        Self {
            icon,
            size: 24.0,
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
    pub fn show(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::click());

        if ui.is_rect_visible(rect) {
            if let Some(icon_data) = self.icon.data() {
                render_icon(ui.painter(), rect, icon_data, self.color);
            } else {
                // Fallback: draw a placeholder if icon data is missing
                ui.painter().rect_filled(rect, 2.0, Color32::from_gray(100));
            }
        }

        response
    }
}
