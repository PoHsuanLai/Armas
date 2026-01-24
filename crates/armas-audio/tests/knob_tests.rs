//! Tests for Knob component using egui_kittest

use armas::Theme;
use armas_audio::knob::{Knob, KnobCurve};
use egui_kittest::Harness;

/// Test that Knob renders without panicking
#[test]
fn test_knob_renders() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value).show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with custom diameter
#[test]
fn test_knob_custom_diameter() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value).diameter(80.0).show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with small diameter
#[test]
fn test_knob_small_diameter() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value).diameter(30.0).show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with label
#[test]
fn test_knob_with_label() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .label("Volume")
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with value display hidden
#[test]
fn test_knob_hide_value() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.75;
        Knob::new(value)
            .show_value(false)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with custom glow color
#[test]
fn test_knob_custom_glow_color() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .glow_color(egui::Color32::from_rgb(255, 100, 50))
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with custom color
#[test]
fn test_knob_custom_color() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .color(egui::Color32::from_rgb(100, 100, 120))
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with linear response curve
#[test]
fn test_knob_linear_curve() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .response_curve(KnobCurve::Linear)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with logarithmic response curve
#[test]
fn test_knob_logarithmic_curve() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .response_curve(KnobCurve::Logarithmic)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with exponential response curve
#[test]
fn test_knob_exponential_curve() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .response_curve(KnobCurve::Exponential)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with custom angle range
#[test]
fn test_knob_custom_angle_range() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .angle_range(-3.0, 3.0)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with custom sensitivity
#[test]
fn test_knob_custom_sensitivity() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .sensitivity(0.005)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with velocity mode disabled
#[test]
fn test_knob_velocity_mode_disabled() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .velocity_mode(false)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with custom value range
#[test]
fn test_knob_custom_value_range() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .value_range(0.0, 100.0)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob with default value for double-click reset
#[test]
fn test_knob_default_value() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.8;
        Knob::new(value)
            .default_value(0.5)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob at minimum value
#[test]
fn test_knob_minimum_value() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.0;
        Knob::new(value).show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob at maximum value
#[test]
fn test_knob_maximum_value() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 1.0;
        Knob::new(value).show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test Knob response fields
#[test]
fn test_knob_response() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        let response = Knob::new(value).show(ui, &mut value, &theme);

        // Check response fields exist
        let _ = response.response;
        assert_eq!(response.value, 0.5);
        assert!(!response.changed);
    });

    harness.run();
}

/// Test Knob with full configuration
#[test]
fn test_knob_full_config() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .diameter(70.0)
            .label("Frequency")
            .show_value(true)
            .color(egui::Color32::from_rgb(200, 200, 210))
            .glow_color(egui::Color32::from_rgb(100, 200, 255))
            .angle_range(-2.5, 2.5)
            .sensitivity(0.01)
            .velocity_sensitivity(1.5)
            .response_curve(KnobCurve::Logarithmic)
            .value_range(20.0, 20000.0)
            .default_value(440.0)
            .velocity_mode(true)
            .show(ui, &mut value, &theme);
    });

    harness.run();
}

/// Test multiple Knobs in a row
#[test]
fn test_multiple_knobs() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            let mut volume = 0.75;
            let mut pan = 0.5;
            let mut frequency = 0.3;

            Knob::new(volume)
                .diameter(50.0)
                .label("Vol")
                .show(ui, &mut volume, &theme);

            Knob::new(pan)
                .diameter(50.0)
                .label("Pan")
                .show(ui, &mut pan, &theme);

            Knob::new(frequency)
                .diameter(50.0)
                .label("Freq")
                .show(ui, &mut frequency, &theme);
        });
    });

    harness.run();
}

/// Test Knob with light theme
#[test]
fn test_knob_light_theme() {
    let theme = Theme::light();
    let mut harness = Harness::new_ui(|ui| {
        let mut value = 0.5;
        Knob::new(value)
            .diameter(60.0)
            .label("Level")
            .show(ui, &mut value, &theme);
    });

    harness.run();
}
