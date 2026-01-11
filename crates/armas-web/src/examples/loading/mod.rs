//! Loading indicator examples

pub mod circular;
pub mod dots;
pub mod spinner;

use super::Example;

pub fn all_examples() -> Vec<Example> {
    vec![
        Example {
            name: "Spinner",
            description: "Rotating spinner loading indicator",
            code: include_str!("spinner.rs"),
            github_path: "crates/armas-web/src/examples/loading/spinner.rs",
        },
        Example {
            name: "Loading Dots",
            description: "Animated bouncing dots",
            code: include_str!("dots.rs"),
            github_path: "crates/armas-web/src/examples/loading/dots.rs",
        },
        Example {
            name: "Circular Progress",
            description: "Circular progress indicator",
            code: include_str!("circular.rs"),
            github_path: "crates/armas-web/src/examples/loading/circular.rs",
        },
    ]
}
