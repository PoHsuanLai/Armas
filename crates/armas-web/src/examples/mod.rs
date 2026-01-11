//! Component examples with embedded source code
//!
//! This module provides a unified system for component examples.
//! Each example includes both the demo function and its source code.

pub mod button;
pub mod card;
pub mod loading;

use eframe::egui;

/// A component example with code and demo
pub struct Example {
    pub name: &'static str,
    pub description: &'static str,
    pub code: &'static str,
    pub github_path: &'static str,
}

impl Example {
    /// Show the example demo
    pub fn demo(&self, ui: &mut egui::Ui) {
        // This will be implemented per-example
        // For now, just show placeholder
        ui.label("Demo goes here");
    }
}

/// Helper macro to define examples with automatic code capture
#[macro_export]
macro_rules! define_example {
    (
        name: $name:expr,
        description: $desc:expr,
        file: $file:expr,
        demo: $demo:expr
    ) => {
        Example {
            name: $name,
            description: $desc,
            code: include_str!($file),
            github_path: concat!("crates/armas-web/src/examples/", $file),
        }
    };
}
