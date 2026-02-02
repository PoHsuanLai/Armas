//! Runtime SVG parsing.
//!
//! Parse SVG strings into [`OwnedIconData`](crate::OwnedIconData) at runtime,
//! without needing a build script.
//!
//! # Example
//!
//! ```rust,no_run
//! use armas_icon::runtime::parse_svg;
//!
//! let svg = r#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
//!     <circle cx="12" cy="12" r="10" fill="black"/>
//! </svg>"#;
//!
//! let icon = parse_svg(svg).unwrap();
//! // Use `Icon::from_owned(&icon)` to render it
//! ```

use crate::tessellate;
use crate::OwnedIconData;

/// Error type for runtime SVG parsing.
#[derive(Debug)]
pub enum IconError {
    /// SVG parsing failed.
    SvgParse(String),
    /// Tessellation failed.
    Tessellation(String),
}

impl std::fmt::Display for IconError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvgParse(e) => write!(f, "SVG parse error: {e}"),
            Self::Tessellation(e) => write!(f, "tessellation error: {e}"),
        }
    }
}

impl std::error::Error for IconError {}

impl From<tessellate::TessError> for IconError {
    fn from(e: tessellate::TessError) -> Self {
        match e {
            tessellate::TessError::SvgParse(msg) => Self::SvgParse(msg),
            tessellate::TessError::Tessellation(msg) => Self::Tessellation(msg),
        }
    }
}

/// Parse an SVG string into [`OwnedIconData`].
///
/// The icon name defaults to `"unnamed"`.
///
/// # Errors
///
/// Returns an error if the SVG cannot be parsed or tessellated.
pub fn parse_svg(svg_str: &str) -> Result<OwnedIconData, IconError> {
    parse_svg_named(svg_str, "unnamed")
}

/// Parse an SVG string into [`OwnedIconData`] with a given name.
///
/// # Errors
///
/// Returns an error if the SVG cannot be parsed or tessellated.
pub fn parse_svg_named(svg_str: &str, name: impl Into<String>) -> Result<OwnedIconData, IconError> {
    let icon = tessellate::tessellate_svg_data(svg_str)?;

    Ok(OwnedIconData {
        name: name.into(),
        vertices: icon.vertices,
        indices: icon.indices,
        viewbox_width: icon.viewbox_width,
        viewbox_height: icon.viewbox_height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_SVG: &str = r#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <rect x="2" y="2" width="20" height="20" fill="black"/>
    </svg>"#;

    const CIRCLE_SVG: &str = r#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <circle cx="12" cy="12" r="10" fill="black"/>
    </svg>"#;

    #[test]
    fn parse_simple_rect() {
        let icon = parse_svg(SIMPLE_SVG).unwrap();
        assert!(!icon.vertices.is_empty());
        assert!(!icon.indices.is_empty());
        assert_eq!(icon.viewbox_width, 24.0);
        assert_eq!(icon.viewbox_height, 24.0);
        assert_eq!(icon.name, "unnamed");
    }

    #[test]
    fn parse_circle() {
        let icon = parse_svg(CIRCLE_SVG).unwrap();
        assert!(!icon.vertices.is_empty());
        assert!(!icon.indices.is_empty());
    }

    #[test]
    fn parse_named() {
        let icon = parse_svg_named(SIMPLE_SVG, "my_rect").unwrap();
        assert_eq!(icon.name, "my_rect");
    }

    #[test]
    fn parse_invalid_svg() {
        let result = parse_svg("not an svg");
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_svg() {
        let svg = r#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        let icon = parse_svg(svg).unwrap();
        assert!(icon.vertices.is_empty());
        assert!(icon.indices.is_empty());
    }

    #[test]
    fn parse_stroke_svg() {
        let svg = r#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <line x1="2" y1="2" x2="22" y2="22" stroke="black" stroke-width="2"/>
        </svg>"#;
        let icon = parse_svg(svg).unwrap();
        assert!(!icon.vertices.is_empty());
    }

    #[test]
    fn viewbox_dimensions_custom() {
        let svg = r#"<svg viewBox="0 0 48 32" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="48" height="32" fill="black"/>
        </svg>"#;
        let icon = parse_svg(svg).unwrap();
        assert_eq!(icon.viewbox_width, 48.0);
        assert_eq!(icon.viewbox_height, 32.0);
    }
}
