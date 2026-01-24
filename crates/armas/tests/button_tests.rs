//! Tests for Button component using egui_kittest

use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Button renders without panicking
#[test]
fn test_button_renders() {
    let mut harness = Harness::new_ui(|ui| {
        Button::new("Click me").show(ui);
    });

    harness.run();
}

/// Test Button with different variants
#[test]
fn test_button_variants() {
    let variants = [
        ButtonVariant::Filled,
        ButtonVariant::FilledTonal,
        ButtonVariant::Elevated,
        ButtonVariant::Outlined,
        ButtonVariant::Text,
    ];

    for variant in variants {
        let mut harness = Harness::new_ui(|ui| {
            Button::new("Test Button").variant(variant).show(ui);
        });
        harness.run();
    }
}

/// Test disabled Button
#[test]
fn test_button_disabled() {
    let mut harness = Harness::new_ui(|ui| {
        Button::new("Disabled").enabled(false).show(ui);
    });

    harness.run();
}

/// Test Button with custom min size
#[test]
fn test_button_min_size() {
    let mut harness = Harness::new_ui(|ui| {
        Button::new("Wide Button")
            .min_size(egui::vec2(200.0, 48.0))
            .show(ui);
    });

    harness.run();
}

/// Test Button with max width (text truncation)
#[test]
fn test_button_max_width() {
    let mut harness = Harness::new_ui(|ui| {
        Button::new("This is a very long button text that should be truncated")
            .max_width(100.0)
            .show(ui);
    });

    harness.run();
}

/// Test Button with different text alignments
#[test]
fn test_button_text_alignment() {
    let alignments = [
        egui::Align2::LEFT_CENTER,
        egui::Align2::CENTER_CENTER,
        egui::Align2::RIGHT_CENTER,
    ];

    for align in alignments {
        let mut harness = Harness::new_ui(|ui| {
            Button::new("Aligned")
                .min_size(egui::vec2(150.0, 32.0))
                .text_align(align)
                .show(ui);
        });
        harness.run();
    }
}

/// Test multiple buttons in a row
#[test]
fn test_multiple_buttons() {
    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            Button::new("Primary").variant(ButtonVariant::Filled).show(ui);
            Button::new("Secondary")
                .variant(ButtonVariant::Outlined)
                .show(ui);
            Button::new("Cancel").variant(ButtonVariant::Text).show(ui);
        });
    });

    harness.run();
}

// Snapshot tests - uncomment when ready to generate baseline images
// #[test]
// fn test_button_variants_snapshot() {
//     let mut harness = Harness::new_ui(|ui| {
//         ui.vertical(|ui| {
//             ui.spacing_mut().item_spacing.y = 8.0;
//             Button::new("Filled").variant(ButtonVariant::Filled).show(ui);
//             Button::new("Tonal").variant(ButtonVariant::FilledTonal).show(ui);
//             Button::new("Elevated").variant(ButtonVariant::Elevated).show(ui);
//             Button::new("Outlined").variant(ButtonVariant::Outlined).show(ui);
//             Button::new("Text").variant(ButtonVariant::Text).show(ui);
//         });
//     });
//
//     harness.fit_contents();
//     harness.snapshot("button_variants");
// }
