//! ARMAS Theme System
//!
//! Serializable theme system for egui applications.
//! Provides a complete color palette and spacing configuration that can be
//! saved/loaded from JSON files.

use egui::Color32;
use serde::{Deserialize, Serialize};

/// Complete ALIG theme with colors and spacing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    /// Color palette
    pub colors: ColorPalette,

    /// Spacing configuration
    pub spacing: Spacing,
}

/// Complete color palette following Material Design principles
/// All colors stored as [R, G, B] for serializability
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorPalette {
    // Primary & Secondary (brand colors)
    pub primary: [u8; 3],
    pub secondary: [u8; 3],

    // Background hierarchy (darkest to lightest)
    pub background: [u8; 3],
    pub surface: [u8; 3],
    pub surface_variant: [u8; 3],

    // Text / Foreground
    pub on_background: [u8; 3],      // Text on background
    pub on_surface: [u8; 3],         // Text on surface
    pub on_surface_variant: [u8; 3], // Dimmed text

    // Borders & Dividers
    pub outline: [u8; 3],
    pub outline_variant: [u8; 3], // Subtle borders

    // Interactive states
    pub hover: [u8; 3],
    pub focus: [u8; 3],

    // Semantic colors (Material Design standard)
    pub error: [u8; 3],
    pub warning: [u8; 3],
    pub success: [u8; 3],
    pub info: [u8; 3],
}

/// Spacing configuration for layouts
/// Following a consistent 4px/8px scale
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spacing {
    /// Extra small spacing (4px) - tight spacing
    pub xs: f32,
    /// Small spacing (8px) - compact layouts
    pub sm: f32,
    /// Medium spacing (16px) - standard spacing
    pub md: f32,
    /// Large spacing (24px) - comfortable spacing
    pub lg: f32,
    /// Extra large spacing (32px) - spacious layouts
    pub xl: f32,
    /// 2XL spacing (48px) - section separators
    pub xxl: f32,

    /// Standard corner radius (12px)
    pub corner_radius: u8,
    /// Small corner radius (8px)
    pub corner_radius_small: u8,
    /// Large corner radius (16px)
    pub corner_radius_large: u8,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Material Design 3 baseline dark theme (default)
    pub fn dark() -> Self {
        Self {
            colors: ColorPalette {
                primary: [208, 188, 255],            // Purple (M3 baseline)
                secondary: [204, 194, 220],          // Light purple
                background: [28, 27, 31],            // Dark background
                surface: [28, 27, 31],               // Surface same as bg
                surface_variant: [73, 69, 79],       // Elevated surface
                on_background: [230, 225, 229],      // Text on background
                on_surface: [230, 225, 229],         // Text on surface
                on_surface_variant: [202, 196, 208], // Dimmed text
                outline: [147, 143, 153],            // Borders
                outline_variant: [73, 69, 79],       // Subtle borders
                hover: [49, 48, 51],                 // Hover state
                focus: [208, 188, 255],              // Focus ring (primary)
                error: [242, 184, 181],              // Error red
                warning: [241, 196, 15],             // Warning yellow
                success: [166, 218, 149],            // Success green
                info: [175, 221, 255],               // Info blue
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
                corner_radius: 12,
                corner_radius_small: 8,
                corner_radius_large: 16,
            },
        }
    }

    /// Material Design 3 baseline light theme
    pub fn light() -> Self {
        Self {
            colors: ColorPalette {
                primary: [103, 80, 164],          // Purple
                secondary: [98, 91, 113],         // Gray purple
                background: [255, 251, 254],      // Off-white background
                surface: [255, 251, 254],         // Surface same as bg
                surface_variant: [231, 224, 236], // Elevated surface
                on_background: [28, 27, 31],      // Dark text
                on_surface: [28, 27, 31],         // Dark text
                on_surface_variant: [73, 69, 79], // Dimmed text
                outline: [121, 116, 126],         // Borders
                outline_variant: [202, 196, 208], // Subtle borders
                hover: [245, 237, 247],           // Hover state
                focus: [103, 80, 164],            // Focus ring
                error: [179, 38, 30],             // Error red
                warning: [204, 130, 0],           // Warning orange
                success: [56, 142, 60],           // Success green
                info: [33, 150, 243],             // Info blue
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
                corner_radius: 12,
                corner_radius_small: 8,
                corner_radius_large: 16,
            },
        }
    }

    // Helper methods to convert [u8; 3] to Color32

    /// Primary brand color
    pub fn primary(&self) -> Color32 {
        let [r, g, b] = self.colors.primary;
        Color32::from_rgb(r, g, b)
    }

    /// Secondary brand color
    pub fn secondary(&self) -> Color32 {
        let [r, g, b] = self.colors.secondary;
        Color32::from_rgb(r, g, b)
    }

    /// Background color (deepest)
    pub fn background(&self) -> Color32 {
        let [r, g, b] = self.colors.background;
        Color32::from_rgb(r, g, b)
    }

    /// Surface color (for cards, panels)
    pub fn surface(&self) -> Color32 {
        let [r, g, b] = self.colors.surface;
        Color32::from_rgb(r, g, b)
    }

    /// Surface variant color
    pub fn surface_variant(&self) -> Color32 {
        let [r, g, b] = self.colors.surface_variant;
        Color32::from_rgb(r, g, b)
    }

    /// Text color on background
    pub fn on_background(&self) -> Color32 {
        let [r, g, b] = self.colors.on_background;
        Color32::from_rgb(r, g, b)
    }

    /// Text color on surface
    pub fn on_surface(&self) -> Color32 {
        let [r, g, b] = self.colors.on_surface;
        Color32::from_rgb(r, g, b)
    }

    /// Dimmed text color on surface
    pub fn on_surface_variant(&self) -> Color32 {
        let [r, g, b] = self.colors.on_surface_variant;
        Color32::from_rgb(r, g, b)
    }

    /// Outline/border color
    pub fn outline(&self) -> Color32 {
        let [r, g, b] = self.colors.outline;
        Color32::from_rgba_unmultiplied(r, g, b, 77) // 0.3 alpha
    }

    /// Subtle outline/border color
    pub fn outline_variant(&self) -> Color32 {
        let [r, g, b] = self.colors.outline_variant;
        Color32::from_rgba_unmultiplied(r, g, b, 40) // Subtle effect
    }

    /// Hover state color
    pub fn hover(&self) -> Color32 {
        let [r, g, b] = self.colors.hover;
        Color32::from_rgb(r, g, b)
    }

    /// Focus state color
    pub fn focus(&self) -> Color32 {
        let [r, g, b] = self.colors.focus;
        Color32::from_rgb(r, g, b)
    }

    /// Error color
    pub fn error(&self) -> Color32 {
        let [r, g, b] = self.colors.error;
        Color32::from_rgb(r, g, b)
    }

    /// Warning color
    pub fn warning(&self) -> Color32 {
        let [r, g, b] = self.colors.warning;
        Color32::from_rgb(r, g, b)
    }

    /// Success color
    pub fn success(&self) -> Color32 {
        let [r, g, b] = self.colors.success;
        Color32::from_rgb(r, g, b)
    }

    /// Info color
    pub fn info(&self) -> Color32 {
        let [r, g, b] = self.colors.info;
        Color32::from_rgb(r, g, b)
    }
}
