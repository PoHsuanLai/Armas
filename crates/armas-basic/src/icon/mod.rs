//! Icon system for Armas
//!
//! Re-exports the generic icon infrastructure from [`armas_icon`] and provides
//! window/UI icons parsed from embedded SVGs at runtime.

// Re-export the generic icon infrastructure
pub use armas_icon::{render_icon, render_icon_data, Icon, IconData, OwnedIconData};

use armas_icon::OwnedIconData as OID;
use std::sync::OnceLock;

fn parse(svg: &str, name: &str) -> OID {
    armas_icon::runtime::parse_svg_named(svg, name).unwrap()
}

static CLOSE: OnceLock<OID> = OnceLock::new();
static INFO: OnceLock<OID> = OnceLock::new();
static ERROR: OnceLock<OID> = OnceLock::new();
static CHEVRON_LEFT: OnceLock<OID> = OnceLock::new();
static CHEVRON_RIGHT: OnceLock<OID> = OnceLock::new();
static CHEVRON_DOWN: OnceLock<OID> = OnceLock::new();

/// Close icon (X)
pub fn close() -> &'static OwnedIconData {
    CLOSE.get_or_init(|| parse(include_str!("../../icons/window/close.svg"), "close"))
}

/// Info icon (circle with i)
pub fn info() -> &'static OwnedIconData {
    INFO.get_or_init(|| parse(include_str!("../../icons/window/info.svg"), "info"))
}

/// Error icon (circle with !)
pub fn error() -> &'static OwnedIconData {
    ERROR.get_or_init(|| parse(include_str!("../../icons/window/error.svg"), "error"))
}

/// Chevron left icon (<)
pub fn chevron_left() -> &'static OwnedIconData {
    CHEVRON_LEFT.get_or_init(|| {
        parse(
            include_str!("../../icons/window/chevron_left.svg"),
            "chevron_left",
        )
    })
}

/// Chevron right icon (>)
pub fn chevron_right() -> &'static OwnedIconData {
    CHEVRON_RIGHT.get_or_init(|| {
        parse(
            include_str!("../../icons/window/chevron_right.svg"),
            "chevron_right",
        )
    })
}

/// Chevron down icon (v)
pub fn chevron_down() -> &'static OwnedIconData {
    CHEVRON_DOWN.get_or_init(|| {
        parse(
            include_str!("../../icons/window/chevron_down.svg"),
            "chevron_down",
        )
    })
}
