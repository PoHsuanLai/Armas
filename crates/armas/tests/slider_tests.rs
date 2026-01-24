//! Tests for Slider component using egui_kittest

use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Slider renders without panicking
#[test]
fn test_slider_renders() {
    let mut value = 50.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 100.0).show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider with label
#[test]
fn test_slider_with_label() {
    let mut value = 75.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 100.0)
            .label("Volume")
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider with suffix
#[test]
fn test_slider_with_suffix() {
    let mut value = 50.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 100.0)
            .label("Progress")
            .suffix("%")
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider with dB suffix (common in audio)
#[test]
fn test_slider_db_suffix() {
    let mut value = -6.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(-60.0, 6.0)
            .label("Gain")
            .suffix(" dB")
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider with step snapping
#[test]
fn test_slider_with_step() {
    let mut value = 50.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 100.0)
            .step(10.0)
            .label("Step by 10")
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider without value display
#[test]
fn test_slider_hide_value() {
    let mut value = 0.5;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 1.0)
            .show_value(false)
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider with custom width
#[test]
fn test_slider_custom_width() {
    let mut value = 50.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 100.0)
            .width(300.0)
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider with custom height
#[test]
fn test_slider_custom_height() {
    let mut value = 50.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 100.0)
            .height(32.0)
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider at minimum value
#[test]
fn test_slider_at_minimum() {
    let mut value = 0.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 100.0)
            .label("At minimum")
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider at maximum value
#[test]
fn test_slider_at_maximum() {
    let mut value = 100.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 100.0)
            .label("At maximum")
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider with negative range
#[test]
fn test_slider_negative_range() {
    let mut value = 0.0;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(-100.0, 100.0)
            .label("Pan")
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test Slider with fractional values
#[test]
fn test_slider_fractional_values() {
    let mut value = 0.5;

    let mut harness = Harness::new_ui(|ui| {
        Slider::new(0.0, 1.0)
            .label("Opacity")
            .step(0.1)
            .show(ui, &mut value);
    });

    harness.run();
}

/// Test multiple sliders
#[test]
fn test_multiple_sliders() {
    let mut volume = 80.0;
    let mut bass = 50.0;
    let mut treble = 50.0;

    let mut harness = Harness::new_ui(|ui| {
        ui.vertical(|ui| {
            Slider::new(0.0, 100.0)
                .label("Volume")
                .suffix("%")
                .show(ui, &mut volume);

            Slider::new(0.0, 100.0)
                .label("Bass")
                .suffix("%")
                .show(ui, &mut bass);

            Slider::new(0.0, 100.0)
                .label("Treble")
                .suffix("%")
                .show(ui, &mut treble);
        });
    });

    harness.run();
}
