//! Font utilities for loading common fonts with multiple weights
//!
//! This module provides helper functions to easily load popular fonts
//! with multiple weights (regular, medium, semibold, bold) for use in
//! Armas components.
//!
//! Since egui doesn't natively support font weights, this module works
//! around the limitation by loading separate font files for each weight
//! and registering them as separate font families.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use armas_basic::fonts::{FontWeight, FontFamilyBuilder};
//! use eframe::egui;
//!
//! fn setup_fonts(ctx: &egui::Context) {
//!     // Build font definitions
//!     let mut builder = FontFamilyBuilder::new();
//!
//!     builder.add_family(
//!         "Inter",
//!         include_bytes!("../../fonts/Inter-Regular.ttf"),
//!         Some(include_bytes!("../../fonts/Inter-Medium.ttf")),
//!         Some(include_bytes!("../../fonts/Inter-SemiBold.ttf")),
//!         Some(include_bytes!("../../fonts/Inter-Bold.ttf")),
//!     );
//!
//!     // Apply to context (must be done before first frame or in CreationContext)
//!     builder.install(ctx, true); // true = set as default
//! }
//!
//! fn my_button(ui: &mut egui::Ui) {
//!     // Use the medium weight variant
//!     let font = FontWeight::medium("Inter", 14.0);
//!     ui.label(egui::RichText::new("Click me").font(font));
//! }
//! ```

use egui::{Context, FontData, FontDefinitions, FontFamily, FontId};
use std::collections::HashMap;
use std::sync::Arc;

/// Font weight variants
///
/// Since egui doesn't natively support font weights, we simulate them
/// by using separate font families for each weight.
pub struct FontWeight;

impl FontWeight {
    /// Get `FontId` for regular weight (400)
    #[must_use]
    pub fn regular(family_name: &str, size: f32) -> FontId {
        FontId::new(size, FontFamily::Name(family_name.into()))
    }

    /// Get `FontId` for medium weight (500)
    #[must_use]
    pub fn medium(family_name: &str, size: f32) -> FontId {
        FontId::new(
            size,
            FontFamily::Name(format!("{family_name}Medium").into()),
        )
    }

    /// Get `FontId` for semibold weight (600)
    #[must_use]
    pub fn semibold(family_name: &str, size: f32) -> FontId {
        FontId::new(
            size,
            FontFamily::Name(format!("{family_name}SemiBold").into()),
        )
    }

    /// Get `FontId` for bold weight (700)
    #[must_use]
    pub fn bold(family_name: &str, size: f32) -> FontId {
        FontId::new(size, FontFamily::Name(format!("{family_name}Bold").into()))
    }
}

/// Builder for composing multiple font families
///
/// This builder allows you to add multiple font families and their weights,
/// then install them all at once into the egui context. This is more efficient
/// and safer than calling `load_font_family` multiple times.
///
/// # Example
///
/// ```rust,ignore
/// use armas_basic::fonts::FontFamilyBuilder;
/// use eframe::egui;
///
/// fn setup(ctx: &egui::Context) {
///     let mut builder = FontFamilyBuilder::new();
///
///     builder.add_family(
///         "Inter",
///         include_bytes!("../../fonts/Inter-Regular.ttf"),
///         Some(include_bytes!("../../fonts/Inter-Medium.ttf")),
///         Some(include_bytes!("../../fonts/Inter-SemiBold.ttf")),
///         Some(include_bytes!("../../fonts/Inter-Bold.ttf")),
///     );
///
///     // Install and set Inter as default
///     builder.install(ctx, true);
/// }
/// ```
pub struct FontFamilyBuilder {
    font_data: HashMap<String, Arc<FontData>>,
    families: HashMap<FontFamily, Vec<String>>,
    default_family: Option<String>,
}

impl FontFamilyBuilder {
    /// Create a new font family builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            font_data: HashMap::new(),
            families: HashMap::new(),
            default_family: None,
        }
    }

    /// Add a font family with multiple weights
    ///
    /// # Naming Convention
    ///
    /// Fonts are registered with the following names:
    /// - Regular: `{family_name}` (e.g., "Inter")
    /// - Medium: `{family_name}Medium` (e.g., "`InterMedium`")
    /// - `SemiBold`: `{family_name}SemiBold` (e.g., "`InterSemiBold`")
    /// - Bold: `{family_name}Bold` (e.g., "`InterBold`")
    pub fn add_family(
        &mut self,
        family_name: &str,
        regular: &'static [u8],
        medium: Option<&'static [u8]>,
        semibold: Option<&'static [u8]>,
        bold: Option<&'static [u8]>,
    ) -> &mut Self {
        // Load regular weight (required)
        let regular_key = format!("{}_regular", family_name.to_lowercase());
        self.font_data.insert(
            regular_key.clone(),
            Arc::new(FontData::from_static(regular)),
        );

        // Create regular family
        self.families
            .insert(FontFamily::Name(family_name.into()), vec![regular_key]);

        // Load medium weight if provided
        if let Some(medium_data) = medium {
            let medium_key = format!("{}_medium", family_name.to_lowercase());
            self.font_data.insert(
                medium_key.clone(),
                Arc::new(FontData::from_static(medium_data)),
            );

            self.families.insert(
                FontFamily::Name(format!("{family_name}Medium").into()),
                vec![medium_key],
            );
        }

        // Load semibold weight if provided
        if let Some(semibold_data) = semibold {
            let semibold_key = format!("{}_semibold", family_name.to_lowercase());
            self.font_data.insert(
                semibold_key.clone(),
                Arc::new(FontData::from_static(semibold_data)),
            );

            self.families.insert(
                FontFamily::Name(format!("{family_name}SemiBold").into()),
                vec![semibold_key],
            );
        }

        // Load bold weight if provided
        if let Some(bold_data) = bold {
            let bold_key = format!("{}_bold", family_name.to_lowercase());
            self.font_data
                .insert(bold_key.clone(), Arc::new(FontData::from_static(bold_data)));

            self.families.insert(
                FontFamily::Name(format!("{family_name}Bold").into()),
                vec![bold_key],
            );
        }

        self
    }

    /// Set which family should be the default proportional font
    pub fn set_default(&mut self, family_name: &str) -> &mut Self {
        self.default_family = Some(family_name.to_string());
        self
    }

    /// Install the fonts into the egui context
    ///
    /// This merges with existing font definitions, preserving egui's default fonts.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The egui context (use from `CreationContext` or before first frame)
    /// * `set_as_default` - If true, sets the first added family as default proportional font
    pub fn install(self, ctx: &Context, set_as_default: bool) {
        let mut fonts = FontDefinitions::default();

        // Merge our font data
        for (key, data) in self.font_data {
            fonts.font_data.insert(key, data);
        }

        // Merge our families
        for (family, keys) in self.families {
            fonts.families.insert(family, keys);
        }

        // Set default if requested
        if set_as_default {
            if let Some(default_family) = self.default_family.as_ref() {
                let default_key = format!("{}_regular", default_family.to_lowercase());
                if let Some(proportional) = fonts.families.get_mut(&FontFamily::Proportional) {
                    proportional.insert(0, default_key);
                }
            }
        }

        ctx.set_fonts(fonts);
    }

    /// Build `FontDefinitions` without installing to context
    ///
    /// Useful if you want to customize further before applying.
    #[must_use]
    pub fn build(self) -> FontDefinitions {
        let mut fonts = FontDefinitions::default();

        for (key, data) in self.font_data {
            fonts.font_data.insert(key, data);
        }

        for (family, keys) in self.families {
            fonts.families.insert(family, keys);
        }

        fonts
    }
}

impl Default for FontFamilyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Common font recommendations for Aceternity-style UIs
///
/// These fonts are commonly used in modern UI design and pair well
/// with the Armas component library.
///
/// ## Inter
/// - **Use case**: General UI, buttons, labels
/// - **Download**: <https://fonts.google.com/specimen/Inter>
/// - **Weights**: 100-900 (use 400, 500, 600, 700)
///
/// ## Geist
/// - **Use case**: Modern tech UIs
/// - **Download**: <https://vercel.com/font>
/// - **Weights**: Regular, Medium, `SemiBold`, Bold
///
/// ## `JetBrains` Mono
/// - **Use case**: Code, monospace needs
/// - **Download**: <https://www.jetbrains.com/lp/mono/>
/// - **Weights**: 100-800
///
/// ## Example: Loading Inter
///
/// ```rust,ignore
/// # let ctx = &egui::Context::default();
/// let inter_regular = include_bytes!("../fonts/Inter-Regular.ttf");
/// let inter_medium = include_bytes!("../fonts/Inter-Medium.ttf");
/// let inter_semibold = include_bytes!("../fonts/Inter-SemiBold.ttf");
/// let inter_bold = include_bytes!("../fonts/Inter-Bold.ttf");
///
/// armas_basic::fonts::load_font_family(
///     ctx,
///     "Inter",
///     inter_regular,
///     Some(inter_medium),
///     Some(inter_semibold),
///     Some(inter_bold),
/// );
///
/// // Set as default
/// armas_basic::fonts::set_default_font(ctx, "Inter");
/// ```
pub mod recommended {}
