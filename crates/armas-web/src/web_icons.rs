//! Web-specific icons (generated from assets/icons/ SVGs at compile time)

use armas::icon::IconData;

include!(concat!(env!("OUT_DIR"), "/web_icons.rs"));
