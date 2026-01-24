//! Tests for Toggle component using egui_kittest

use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Toggle renders without panicking
#[test]
fn test_toggle_renders() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Toggle::new().label("Enable feature").show(ui, &mut checked);
    });

    harness.run();
}

/// Test Toggle Switch variant
#[test]
fn test_toggle_switch_variant() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Toggle::new()
            .variant(ToggleVariant::Switch)
            .label("Switch toggle")
            .show(ui, &mut checked);
    });
    harness.run();
}

/// Test Toggle Checkbox variant
#[test]
fn test_toggle_checkbox_variant() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Toggle::new()
            .variant(ToggleVariant::Checkbox)
            .label("Checkbox toggle")
            .show(ui, &mut checked);
    });
    harness.run();
}

/// Test Toggle Small size
#[test]
fn test_toggle_size_small() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Toggle::new()
            .size(ToggleSize::Small)
            .label("Small toggle")
            .show(ui, &mut checked);
    });
    harness.run();
}

/// Test Toggle Medium size
#[test]
fn test_toggle_size_medium() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Toggle::new()
            .size(ToggleSize::Medium)
            .label("Medium toggle")
            .show(ui, &mut checked);
    });
    harness.run();
}

/// Test Toggle Large size
#[test]
fn test_toggle_size_large() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Toggle::new()
            .size(ToggleSize::Large)
            .label("Large toggle")
            .show(ui, &mut checked);
    });
    harness.run();
}

/// Test disabled Toggle
#[test]
fn test_toggle_disabled() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Toggle::new()
            .disabled(true)
            .label("Disabled toggle")
            .show(ui, &mut checked);
    });

    harness.run();
}

/// Test Toggle with description
/// Note: Uses step() instead of run() because the checked=true state triggers
/// the spring animation which continuously requests repaints.
#[test]
fn test_toggle_with_description() {
    let mut checked = true;

    let mut harness = Harness::new_ui(|ui| {
        Toggle::new()
            .label("Dark mode")
            .description("Enable dark theme for better visibility at night")
            .show(ui, &mut checked);
    });

    // Use step() for animated components - run() expects the UI to settle
    harness.step();
}

// Snapshot tests - uncomment when ready to generate baseline images
// #[test]
// fn test_toggle_snapshot_unchecked() {
//     let mut checked = false;
//
//     let mut harness = Harness::new_ui(|ui| {
//         Toggle::new()
//             .label("Feature toggle")
//             .show(ui, &mut checked);
//     });
//
//     harness.fit_contents();
//     harness.snapshot("toggle_unchecked");
// }
//
// #[test]
// fn test_toggle_snapshot_checked() {
//     let mut checked = true;
//
//     let mut harness = Harness::new_ui(|ui| {
//         Toggle::new()
//             .label("Feature toggle")
//             .show(ui, &mut checked);
//     });
//
//     harness.fit_contents();
//     harness.snapshot("toggle_checked");
// }
