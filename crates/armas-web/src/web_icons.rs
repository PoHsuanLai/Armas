//! Web-specific icons (parsed from embedded SVGs at runtime)

use armas::icon::OwnedIconData;
use std::sync::OnceLock;

const DARK_SVG: &str = include_str!("../assets/icons/dark.svg");
const LIGHT_SVG: &str = include_str!("../assets/icons/light.svg");

static DARK: OnceLock<OwnedIconData> = OnceLock::new();
static LIGHT: OnceLock<OwnedIconData> = OnceLock::new();

pub fn dark() -> &'static OwnedIconData {
    DARK.get_or_init(|| armas_icon::runtime::parse_svg_named(DARK_SVG, "dark").unwrap())
}

pub fn light() -> &'static OwnedIconData {
    LIGHT.get_or_init(|| armas_icon::runtime::parse_svg_named(LIGHT_SVG, "light").unwrap())
}
