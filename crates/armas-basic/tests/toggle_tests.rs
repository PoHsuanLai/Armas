//! Tests for Switch and Checkbox components using `egui_kittest`

use armas_basic::prelude::*;
use egui_kittest::Harness;

/// Test that Switch renders without panicking
#[test]
fn test_switch_renders() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Switch::new().label("Enable feature").show(ui, &mut checked);
    });

    harness.run();
}

/// Test Switch Small size
#[test]
fn test_switch_size_small() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Switch::new()
            .size(SwitchSize::Small)
            .label("Small switch")
            .show(ui, &mut checked);
    });
    harness.run();
}

/// Test Switch Medium size
#[test]
fn test_switch_size_medium() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Switch::new()
            .size(SwitchSize::Medium)
            .label("Medium switch")
            .show(ui, &mut checked);
    });
    harness.run();
}

/// Test Switch Large size
#[test]
fn test_switch_size_large() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Switch::new()
            .size(SwitchSize::Large)
            .label("Large switch")
            .show(ui, &mut checked);
    });
    harness.run();
}

/// Test disabled Switch
#[test]
fn test_switch_disabled() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Switch::new()
            .disabled(true)
            .label("Disabled switch")
            .show(ui, &mut checked);
    });

    harness.run();
}

/// Test Switch with description
/// Note: Uses `step()` instead of `run()` because the `checked=true` state triggers
/// the spring animation which continuously requests repaints.
#[test]
fn test_switch_with_description() {
    let mut checked = true;

    let mut harness = Harness::new_ui(|ui| {
        Switch::new()
            .label("Dark mode")
            .description("Enable dark theme for better visibility at night")
            .show(ui, &mut checked);
    });

    // Use step() for animated components - run() expects the UI to settle
    harness.step();
}

/// Test Checkbox renders without panicking
#[test]
fn test_checkbox_renders() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Checkbox::new().label("Accept terms").show(ui, &mut checked);
    });

    harness.run();
}

/// Test Checkbox with description
#[test]
fn test_checkbox_with_description() {
    let mut checked = true;

    let mut harness = Harness::new_ui(|ui| {
        Checkbox::new()
            .label("Subscribe")
            .description("Receive email notifications")
            .show(ui, &mut checked);
    });

    harness.step();
}

/// Test disabled Checkbox
#[test]
fn test_checkbox_disabled() {
    let mut checked = false;

    let mut harness = Harness::new_ui(|ui| {
        Checkbox::new()
            .disabled(true)
            .label("Disabled checkbox")
            .show(ui, &mut checked);
    });

    harness.run();
}
