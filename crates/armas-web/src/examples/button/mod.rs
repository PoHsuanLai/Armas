//! Button examples

pub mod basic;
pub mod figma;
pub mod shimmer;
pub mod variants;

use super::Example;

pub fn all_examples() -> Vec<Example> {
    vec![
        Example {
            name: "Basic Button",
            description: "Simple button with filled variant",
            code: include_str!("basic.rs"),
            github_path: "crates/armas-web/src/examples/button/basic.rs",
        },
        Example {
            name: "Button Variants",
            description: "Filled, Outlined, and Text button variants",
            code: include_str!("variants.rs"),
            github_path: "crates/armas-web/src/examples/button/variants.rs",
        },
        Example {
            name: "Shimmer Button",
            description: "Button with animated shimmer effect",
            code: include_str!("shimmer.rs"),
            github_path: "crates/armas-web/src/examples/button/shimmer.rs",
        },
        Example {
            name: "Figma Buttons",
            description: "Figma-style button components",
            code: include_str!("figma.rs"),
            github_path: "crates/armas-web/src/examples/button/figma.rs",
        },
    ]
}
