//! ALIG Theme System
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
    pub corner_radius: f32,
    /// Small corner radius (8px)
    pub corner_radius_small: f32,
    /// Large corner radius (16px)
    pub corner_radius_large: f32,

    // Legacy names for backward compatibility
    #[serde(skip)]
    pub spacing_small: f32,
    #[serde(skip)]
    pub spacing_medium: f32,
    #[serde(skip)]
    pub spacing_large: f32,
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
                corner_radius: 12.0,
                corner_radius_small: 8.0,
                corner_radius_large: 16.0,
                spacing_small: 8.0,
                spacing_medium: 16.0,
                spacing_large: 24.0,
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
                corner_radius: 12.0,
                corner_radius_small: 8.0,
                corner_radius_large: 16.0,
                spacing_small: 8.0,
                spacing_medium: 16.0,
                spacing_large: 24.0,
            },
        }
    }

    /// Nord theme - cool Nordic palette
    pub fn nord() -> Self {
        Self {
            colors: ColorPalette {
                primary: [136, 192, 208],            // Frost blue
                secondary: [163, 190, 140],          // Aurora green
                background: [46, 52, 64],            // Polar night
                surface: [59, 66, 82],               // Lighter polar
                surface_variant: [67, 76, 94],       // Even lighter
                on_background: [236, 239, 244],      // Snow storm white
                on_surface: [236, 239, 244],         // Snow storm white
                on_surface_variant: [216, 222, 233], // Dimmed white
                outline: [76, 86, 106],              // Borders
                outline_variant: [76, 86, 106],      // Subtle borders
                hover: [76, 86, 106],                // Hover state
                focus: [136, 192, 208],              // Focus (frost)
                error: [191, 97, 106],               // Aurora red
                warning: [235, 203, 139],            // Aurora yellow
                success: [163, 190, 140],            // Aurora green
                info: [136, 192, 208],               // Frost blue
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
                corner_radius: 12.0,
                corner_radius_small: 8.0,
                corner_radius_large: 16.0,
                spacing_small: 8.0,
                spacing_medium: 16.0,
                spacing_large: 24.0,
            },
        }
    }

    /// Dracula theme - popular dark theme
    pub fn dracula() -> Self {
        Self {
            colors: ColorPalette {
                primary: [189, 147, 249],            // Purple
                secondary: [255, 121, 198],          // Pink
                background: [40, 42, 54],            // Dark background
                surface: [68, 71, 90],               // Current line
                surface_variant: [98, 114, 164],     // Selection
                on_background: [248, 248, 242],      // Foreground
                on_surface: [248, 248, 242],         // Foreground
                on_surface_variant: [189, 147, 249], // Purple text
                outline: [98, 114, 164],             // Comment
                outline_variant: [68, 71, 90],       // Subtle
                hover: [68, 71, 90],                 // Current line
                focus: [189, 147, 249],              // Purple
                error: [255, 85, 85],                // Red
                warning: [241, 250, 140],            // Yellow
                success: [80, 250, 123],             // Green
                info: [139, 233, 253],               // Cyan
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
                corner_radius: 12.0,
                corner_radius_small: 8.0,
                corner_radius_large: 16.0,
                spacing_small: 8.0,
                spacing_medium: 16.0,
                spacing_large: 24.0,
            },
        }
    }

    /// Studio theme - professional neutral dark theme (DAW-inspired)
    pub fn studio() -> Self {
        Self {
            colors: ColorPalette {
                primary: [100, 181, 246],            // Light blue
                secondary: [129, 199, 132],          // Light green
                background: [30, 30, 30],            // Very dark gray
                surface: [45, 45, 45],               // Dark gray
                surface_variant: [60, 60, 60],       // Medium gray
                on_background: [224, 224, 224],      // Light gray text
                on_surface: [224, 224, 224],         // Light gray text
                on_surface_variant: [158, 158, 158], // Dimmed text
                outline: [97, 97, 97],               // Borders
                outline_variant: [66, 66, 66],       // Subtle borders
                hover: [55, 55, 55],                 // Hover state
                focus: [100, 181, 246],              // Light blue focus
                error: [244, 67, 54],                // Red
                warning: [255, 152, 0],              // Orange
                success: [76, 175, 80],              // Green
                info: [33, 150, 243],                // Blue
            },
            spacing: Spacing {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
                corner_radius: 12.0,
                corner_radius_small: 8.0,
                corner_radius_large: 16.0,
                spacing_small: 8.0,
                spacing_medium: 16.0,
                spacing_large: 24.0,
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

    /// Save theme to JSON file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load theme from JSON file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let theme = serde_json::from_str(&json)?;
        Ok(theme)
    }
}
