//! Tests for `ButtonGroup` component using `egui_kittest`

use armas_basic::prelude::*;
use egui_kittest::Harness;

/// Test `ButtonGroup` renders without panicking
#[test]
fn test_button_group_renders() {
    let mut harness = Harness::new_ui(|ui| {
        ButtonGroup::new("test_bg").show(ui, |ui| {
            Button::new("A").variant(ButtonVariant::Outline).show(ui);
            Button::new("B").variant(ButtonVariant::Outline).show(ui);
            Button::new("C").variant(ButtonVariant::Outline).show(ui);
        });
    });

    harness.run();
}

/// Test `ButtonGroup` vertical orientation
#[test]
fn test_button_group_vertical() {
    let mut harness = Harness::new_ui(|ui| {
        ButtonGroup::new("test_bg_vert")
            .orientation(ButtonGroupOrientation::Vertical)
            .show(ui, |ui| {
                Button::new("Top").variant(ButtonVariant::Outline).show(ui);
                Button::new("Bottom")
                    .variant(ButtonVariant::Outline)
                    .show(ui);
            });
    });

    harness.run();
}

/// Test `ButtonGroup` with single button
#[test]
fn test_button_group_single() {
    let mut harness = Harness::new_ui(|ui| {
        ButtonGroup::new("test_bg_single").show(ui, |ui| {
            Button::new("Only").variant(ButtonVariant::Outline).show(ui);
        });
    });

    harness.run();
}

/// Test `ButtonGroup` empty
#[test]
fn test_button_group_empty() {
    let mut harness = Harness::new_ui(|ui| {
        ButtonGroup::new("test_bg_empty").show(ui, |_ui| {});
    });

    harness.run();
}
