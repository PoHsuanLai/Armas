//! Card examples

pub mod glass_panel;
pub mod gradient;

use super::Example;

pub fn all_examples() -> Vec<Example> {
    vec![
        Example {
            name: "Glass Panel",
            description: "Card with glassmorphism effect",
            code: include_str!("glass_panel.rs"),
            github_path: "crates/armas-web/src/examples/card/glass_panel.rs",
        },
        Example {
            name: "Gradient Card",
            description: "Card with gradient background",
            code: include_str!("gradient.rs"),
            github_path: "crates/armas-web/src/examples/card/gradient.rs",
        },
    ]
}
