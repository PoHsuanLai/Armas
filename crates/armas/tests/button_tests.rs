//! Tests for Button component using egui_kittest

use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Button renders without panicking
#[test]
fn test_button_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Button::new("Click me").show(ui, &theme);
    });

    harness.run();
}

/// Test Button with different variants
#[test]
fn test_button_variants() {
    let variants = [
        ButtonVariant::Default,
        ButtonVariant::Secondary,
        ButtonVariant::Outline,
        ButtonVariant::Ghost,
        ButtonVariant::Link,
    ];

    for variant in variants {
        let mut harness = Harness::new_ui(|ui| {
            let theme = ui.ctx().armas_theme();
            Button::new("Test Button").variant(variant).show(ui, &theme);
        });
        harness.run();
    }
}

/// Test disabled Button
#[test]
fn test_button_disabled() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Button::new("Disabled").enabled(false).show(ui, &theme);
    });

    harness.run();
}

/// Test Button with custom min width
#[test]
fn test_button_min_width() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Button::new("Wide Button").min_width(200.0).show(ui, &theme);
    });

    harness.run();
}

/// Test Button with different sizes
#[test]
fn test_button_sizes() {
    let sizes = [
        ButtonSize::Xs,
        ButtonSize::Small,
        ButtonSize::Default,
        ButtonSize::Large,
    ];

    for size in sizes {
        let mut harness = Harness::new_ui(|ui| {
            let theme = ui.ctx().armas_theme();
            Button::new("Sized").size(size).show(ui, &theme);
        });
        harness.run();
    }
}

/// Test Button with full width
#[test]
fn test_button_full_width() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Button::new("Full Width").full_width(true).show(ui, &theme);
    });

    harness.run();
}

/// Test multiple buttons in a row
#[test]
fn test_multiple_buttons() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        ui.horizontal(|ui| {
            Button::new("Primary")
                .variant(ButtonVariant::Default)
                .show(ui, &theme);
            Button::new("Secondary")
                .variant(ButtonVariant::Outline)
                .show(ui, &theme);
            Button::new("Cancel")
                .variant(ButtonVariant::Ghost)
                .show(ui, &theme);
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
//             Button::new("Default").variant(ButtonVariant::Default).show(ui, &theme);
//             Button::new("Secondary").variant(ButtonVariant::Secondary).show(ui, &theme);
//             Button::new("Outline").variant(ButtonVariant::Outline).show(ui, &theme);
//             Button::new("Ghost").variant(ButtonVariant::Ghost).show(ui, &theme);
//             Button::new("Link").variant(ButtonVariant::Link).show(ui, &theme);
//         });
//     });
//
//     harness.fit_contents();
//     harness.snapshot("button_variants");
// }
