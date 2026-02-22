//! Tests for `InputGroup` component using `egui_kittest`

use armas_basic::prelude::*;
use egui_kittest::Harness;

/// Test `InputGroup` renders without panicking
#[test]
fn test_input_group_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let mut text = String::new();
        InputGroup::new("test_ig")
            .placeholder("Search...")
            .show(ui, &mut text);
    });

    harness.run();
}

/// Test `InputGroup` with leading addon
#[test]
fn test_input_group_leading() {
    let mut harness = Harness::new_ui(|ui| {
        let mut text = String::new();
        InputGroup::new("test_ig_lead")
            .leading(|ui| {
                ui.label("$");
            })
            .show(ui, &mut text);
    });

    harness.run();
}

/// Test `InputGroup` with trailing addon
#[test]
fn test_input_group_trailing() {
    let mut harness = Harness::new_ui(|ui| {
        let mut text = String::new();
        InputGroup::new("test_ig_trail")
            .trailing(|ui| {
                ui.label(".com");
            })
            .show(ui, &mut text);
    });

    harness.run();
}

/// Test `InputGroup` with both addons
#[test]
fn test_input_group_both() {
    let mut harness = Harness::new_ui(|ui| {
        let mut text = String::new();
        InputGroup::new("test_ig_both")
            .leading(|ui| {
                ui.label("https://");
            })
            .trailing(|ui| {
                ui.label(".com");
            })
            .placeholder("example")
            .width(400.0)
            .show(ui, &mut text);
    });

    harness.run();
}
