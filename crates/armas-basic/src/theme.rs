//! ARMAS Theme System (shadcn/ui style)
//!
//! Serializable theme system for egui applications.
//! Uses shadcn/ui naming conventions for simplicity and maintainability.

use egui::Color32;
use serde::{Deserialize, Serialize};

/// Complete theme with colors and spacing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    /// Color palette
    pub colors: ColorPalette,

    /// Spacing configuration
    pub spacing: Spacing,
}

/// Color palette using shadcn/ui naming conventions
/// All colors stored as [R, G, B] for serializability
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Default background color
    pub background: [u8; 3],
    /// Default foreground (text) color
    pub foreground: [u8; 3],

    /// Card background color for elevated surfaces
    pub card: [u8; 3],
    /// Card foreground (text) color
    pub card_foreground: [u8; 3],

    /// Popover background color
    pub popover: [u8; 3],
    /// Popover foreground (text) color
    pub popover_foreground: [u8; 3],

    /// Primary brand color
    pub primary: [u8; 3],
    /// Primary foreground (text) color
    pub primary_foreground: [u8; 3],

    /// Secondary color
    pub secondary: [u8; 3],
    /// Secondary foreground (text) color
    pub secondary_foreground: [u8; 3],

    /// Muted/subtle background color
    pub muted: [u8; 3],
    /// Muted foreground (text) color
    pub muted_foreground: [u8; 3],

    /// Accent color
    pub accent: [u8; 3],
    /// Accent foreground (text) color
    pub accent_foreground: [u8; 3],

    /// Destructive/error color
    pub destructive: [u8; 3],
    /// Destructive foreground (text) color
    pub destructive_foreground: [u8; 3],

    /// Border color
    pub border: [u8; 3],
    /// Input border color
    pub input: [u8; 3],
    /// Focus ring color
    pub ring: [u8; 3],

    /// Chart color 1 for data visualization
    pub chart_1: [u8; 3],
    /// Chart color 2 for data visualization
    pub chart_2: [u8; 3],
    /// Chart color 3 for data visualization
    pub chart_3: [u8; 3],
    /// Chart color 4 for data visualization
    pub chart_4: [u8; 3],
    /// Chart color 5 for data visualization
    pub chart_5: [u8; 3],

    /// Hover state color
    pub hover: [u8; 3],
    /// Focus state color
    pub focus: [u8; 3],

    /// Sidebar background color
    pub sidebar: [u8; 3],
    /// Sidebar foreground (text) color
    pub sidebar_foreground: [u8; 3],
    /// Sidebar primary color
    pub sidebar_primary: [u8; 3],
    /// Sidebar primary foreground (text) color
    pub sidebar_primary_foreground: [u8; 3],
    /// Sidebar accent color
    pub sidebar_accent: [u8; 3],
    /// Sidebar accent foreground (text) color
    pub sidebar_accent_foreground: [u8; 3],
    /// Sidebar border color
    pub sidebar_border: [u8; 3],
    /// Sidebar focus ring color
    pub sidebar_ring: [u8; 3],
}

/// Spacing configuration for layouts
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spacing {
    /// 2XS spacing (2px)
    pub xxs: f32,
    /// Extra small spacing (4px)
    pub xs: f32,
    /// Small spacing (8px)
    pub sm: f32,
    /// Medium spacing (16px)
    pub md: f32,
    /// Large spacing (24px)
    pub lg: f32,
    /// Extra large spacing (32px)
    pub xl: f32,
    /// 2XL spacing (48px)
    pub xxl: f32,

    /// Micro corner radius (2px)
    pub corner_radius_micro: u8,
    /// Tiny corner radius (4px)
    pub corner_radius_tiny: u8,
    /// Small corner radius (8px)
    pub corner_radius_small: u8,
    /// Standard corner radius (12px)
    pub corner_radius: u8,
    /// Large corner radius (16px)
    pub corner_radius_large: u8,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Dark theme using Zinc color palette (shadcn default)
    #[must_use] 
    pub const fn dark() -> Self {
        Self {
            colors: ColorPalette {
                background: [9, 9, 11],      // zinc-950
                foreground: [250, 250, 250], // zinc-50

                card: [9, 9, 11],                 // zinc-950
                card_foreground: [250, 250, 250], // zinc-50

                popover: [9, 9, 11],                 // zinc-950
                popover_foreground: [250, 250, 250], // zinc-50

                primary: [250, 250, 250],         // zinc-50
                primary_foreground: [24, 24, 27], // zinc-900

                secondary: [39, 39, 42],               // zinc-800
                secondary_foreground: [250, 250, 250], // zinc-50

                muted: [39, 39, 42],               // zinc-800
                muted_foreground: [161, 161, 170], // zinc-400

                accent: [39, 39, 42],               // zinc-800
                accent_foreground: [250, 250, 250], // zinc-50

                destructive: [127, 29, 29],              // red-900
                destructive_foreground: [250, 250, 250], // zinc-50

                border: [39, 39, 42],  // zinc-800
                input: [39, 39, 42],   // zinc-800
                ring: [212, 212, 216], // zinc-300

                chart_1: [59, 130, 246], // blue-500
                chart_2: [34, 197, 94],  // green-500
                chart_3: [234, 179, 8],  // yellow-500
                chart_4: [168, 85, 247], // purple-500
                chart_5: [249, 115, 22], // orange-500

                hover: [39, 39, 42],    // zinc-800
                focus: [250, 250, 250], // zinc-50

                // Sidebar (slightly lighter than background for distinction)
                sidebar: [9, 9, 11],                      // zinc-950 (same as bg)
                sidebar_foreground: [250, 250, 250],      // zinc-50
                sidebar_primary: [250, 250, 250],         // zinc-50
                sidebar_primary_foreground: [24, 24, 27], // zinc-900
                sidebar_accent: [39, 39, 42],             // zinc-800
                sidebar_accent_foreground: [250, 250, 250], // zinc-50
                sidebar_border: [39, 39, 42],             // zinc-800
                sidebar_ring: [212, 212, 216],            // zinc-300
            },
            spacing: Spacing {
                xxs: 2.0,
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
                corner_radius_micro: 2,
                corner_radius_tiny: 4,
                corner_radius_small: 8,
                corner_radius: 12,
                corner_radius_large: 16,
            },
        }
    }

    /// Light theme using Zinc color palette
    #[must_use] 
    pub const fn light() -> Self {
        Self {
            colors: ColorPalette {
                background: [255, 255, 255], // white
                foreground: [9, 9, 11],      // zinc-950

                card: [255, 255, 255],       // white
                card_foreground: [9, 9, 11], // zinc-950

                popover: [255, 255, 255],       // white
                popover_foreground: [9, 9, 11], // zinc-950

                primary: [24, 24, 27],               // zinc-900
                primary_foreground: [250, 250, 250], // zinc-50

                secondary: [244, 244, 245],         // zinc-100
                secondary_foreground: [24, 24, 27], // zinc-900

                muted: [244, 244, 245],            // zinc-100
                muted_foreground: [113, 113, 122], // zinc-500

                accent: [244, 244, 245],         // zinc-100
                accent_foreground: [24, 24, 27], // zinc-900

                destructive: [239, 68, 68],              // red-500
                destructive_foreground: [250, 250, 250], // zinc-50

                border: [228, 228, 231], // zinc-200
                input: [228, 228, 231],  // zinc-200
                ring: [24, 24, 27],      // zinc-900

                chart_1: [59, 130, 246], // blue-500
                chart_2: [34, 197, 94],  // green-500
                chart_3: [234, 179, 8],  // yellow-500
                chart_4: [168, 85, 247], // purple-500
                chart_5: [249, 115, 22], // orange-500

                hover: [244, 244, 245], // zinc-100
                focus: [24, 24, 27],    // zinc-900

                // Sidebar (slightly darker than background for distinction)
                sidebar: [250, 250, 250],                    // zinc-50
                sidebar_foreground: [9, 9, 11],              // zinc-950
                sidebar_primary: [24, 24, 27],               // zinc-900
                sidebar_primary_foreground: [250, 250, 250], // zinc-50
                sidebar_accent: [244, 244, 245],             // zinc-100
                sidebar_accent_foreground: [24, 24, 27],     // zinc-900
                sidebar_border: [228, 228, 231],             // zinc-200
                sidebar_ring: [24, 24, 27],                  // zinc-900
            },
            spacing: Spacing {
                xxs: 2.0,
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
                xxl: 48.0,
                corner_radius_micro: 2,
                corner_radius_tiny: 4,
                corner_radius_small: 8,
                corner_radius: 12,
                corner_radius_large: 16,
            },
        }
    }

    // =========================================================================
    // Color accessor methods (shadcn naming)
    // =========================================================================

    /// Background color
    #[must_use] 
    pub const fn background(&self) -> Color32 {
        let [r, g, b] = self.colors.background;
        Color32::from_rgb(r, g, b)
    }

    /// Foreground/text color
    #[must_use] 
    pub const fn foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Card background color
    #[must_use] 
    pub const fn card(&self) -> Color32 {
        let [r, g, b] = self.colors.card;
        Color32::from_rgb(r, g, b)
    }

    /// Card foreground color
    #[must_use] 
    pub const fn card_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.card_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Popover background color
    #[must_use] 
    pub const fn popover(&self) -> Color32 {
        let [r, g, b] = self.colors.popover;
        Color32::from_rgb(r, g, b)
    }

    /// Popover foreground color
    #[must_use] 
    pub const fn popover_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.popover_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Primary brand color
    #[must_use] 
    pub const fn primary(&self) -> Color32 {
        let [r, g, b] = self.colors.primary;
        Color32::from_rgb(r, g, b)
    }

    /// Primary foreground color
    #[must_use] 
    pub const fn primary_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.primary_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Secondary color
    #[must_use] 
    pub const fn secondary(&self) -> Color32 {
        let [r, g, b] = self.colors.secondary;
        Color32::from_rgb(r, g, b)
    }

    /// Secondary foreground color
    #[must_use] 
    pub const fn secondary_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.secondary_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Muted color (subtle backgrounds)
    #[must_use] 
    pub const fn muted(&self) -> Color32 {
        let [r, g, b] = self.colors.muted;
        Color32::from_rgb(r, g, b)
    }

    /// Muted foreground color
    #[must_use] 
    pub const fn muted_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.muted_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Accent color
    #[must_use] 
    pub const fn accent(&self) -> Color32 {
        let [r, g, b] = self.colors.accent;
        Color32::from_rgb(r, g, b)
    }

    /// Accent foreground color
    #[must_use] 
    pub const fn accent_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.accent_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Destructive/error color
    #[must_use] 
    pub const fn destructive(&self) -> Color32 {
        let [r, g, b] = self.colors.destructive;
        Color32::from_rgb(r, g, b)
    }

    /// Destructive foreground color
    #[must_use] 
    pub const fn destructive_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.destructive_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Border color
    #[must_use] 
    pub const fn border(&self) -> Color32 {
        let [r, g, b] = self.colors.border;
        Color32::from_rgb(r, g, b)
    }

    /// Input border color
    #[must_use] 
    pub const fn input(&self) -> Color32 {
        let [r, g, b] = self.colors.input;
        Color32::from_rgb(r, g, b)
    }

    /// Focus ring color
    #[must_use] 
    pub const fn ring(&self) -> Color32 {
        let [r, g, b] = self.colors.ring;
        Color32::from_rgb(r, g, b)
    }

    /// Chart color 1 (blue)
    #[must_use] 
    pub const fn chart_1(&self) -> Color32 {
        let [r, g, b] = self.colors.chart_1;
        Color32::from_rgb(r, g, b)
    }

    /// Chart color 2 (green) - also used for success
    #[must_use] 
    pub const fn chart_2(&self) -> Color32 {
        let [r, g, b] = self.colors.chart_2;
        Color32::from_rgb(r, g, b)
    }

    /// Chart color 3 (yellow) - also used for warning
    #[must_use] 
    pub const fn chart_3(&self) -> Color32 {
        let [r, g, b] = self.colors.chart_3;
        Color32::from_rgb(r, g, b)
    }

    /// Chart color 4 (purple) - also used for info
    #[must_use] 
    pub const fn chart_4(&self) -> Color32 {
        let [r, g, b] = self.colors.chart_4;
        Color32::from_rgb(r, g, b)
    }

    /// Chart color 5 (orange)
    #[must_use] 
    pub const fn chart_5(&self) -> Color32 {
        let [r, g, b] = self.colors.chart_5;
        Color32::from_rgb(r, g, b)
    }

    /// Hover state color
    #[must_use] 
    pub const fn hover(&self) -> Color32 {
        let [r, g, b] = self.colors.hover;
        Color32::from_rgb(r, g, b)
    }

    /// Focus state color
    #[must_use] 
    pub const fn focus(&self) -> Color32 {
        let [r, g, b] = self.colors.focus;
        Color32::from_rgb(r, g, b)
    }

    // =========================================================================
    // Sidebar color accessors
    // =========================================================================

    /// Sidebar background color
    #[must_use] 
    pub const fn sidebar(&self) -> Color32 {
        let [r, g, b] = self.colors.sidebar;
        Color32::from_rgb(r, g, b)
    }

    /// Sidebar foreground/text color
    #[must_use] 
    pub const fn sidebar_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.sidebar_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Sidebar primary color
    #[must_use] 
    pub const fn sidebar_primary(&self) -> Color32 {
        let [r, g, b] = self.colors.sidebar_primary;
        Color32::from_rgb(r, g, b)
    }

    /// Sidebar primary foreground color
    #[must_use] 
    pub const fn sidebar_primary_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.sidebar_primary_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Sidebar accent color (hover/active background)
    #[must_use] 
    pub const fn sidebar_accent(&self) -> Color32 {
        let [r, g, b] = self.colors.sidebar_accent;
        Color32::from_rgb(r, g, b)
    }

    /// Sidebar accent foreground color
    #[must_use] 
    pub const fn sidebar_accent_foreground(&self) -> Color32 {
        let [r, g, b] = self.colors.sidebar_accent_foreground;
        Color32::from_rgb(r, g, b)
    }

    /// Sidebar border color
    #[must_use] 
    pub const fn sidebar_border(&self) -> Color32 {
        let [r, g, b] = self.colors.sidebar_border;
        Color32::from_rgb(r, g, b)
    }

    /// Sidebar focus ring color
    #[must_use] 
    pub const fn sidebar_ring(&self) -> Color32 {
        let [r, g, b] = self.colors.sidebar_ring;
        Color32::from_rgb(r, g, b)
    }
}
