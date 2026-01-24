//! Tests for Fader and FaderStrip components using egui_kittest

use armas_audio::fader::{Fader, FaderCurve, FaderScalePosition, FaderStrip};
use egui_kittest::Harness;

/// Test that Fader renders without panicking
#[test]
fn test_fader_renders() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.75).show(ui);
    });

    harness.run();
}

/// Test Fader with custom size
#[test]
fn test_fader_custom_size() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).size(40.0, 300.0).show(ui);
    });

    harness.run();
}

/// Test Fader with scale on right
#[test]
fn test_fader_scale_right() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).scale_right().show(ui);
    });

    harness.run();
}

/// Test Fader with scale on left
#[test]
fn test_fader_scale_left() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).scale_left().show(ui);
    });

    harness.run();
}

/// Test Fader with show_scale convenience method
#[test]
fn test_fader_show_scale() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).show_scale().show(ui);
    });

    harness.run();
}

/// Test Fader with linear response curve
#[test]
fn test_fader_linear_curve() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).response_curve(FaderCurve::Linear).show(ui);
    });

    harness.run();
}

/// Test Fader with logarithmic response curve
#[test]
fn test_fader_logarithmic_curve() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5)
            .response_curve(FaderCurve::Logarithmic)
            .show(ui);
    });

    harness.run();
}

/// Test Fader with exponential response curve
#[test]
fn test_fader_exponential_curve() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5)
            .response_curve(FaderCurve::Exponential)
            .show(ui);
    });

    harness.run();
}

/// Test Fader with custom track color
#[test]
fn test_fader_custom_track_color() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5)
            .track_color(egui::Color32::from_rgb(50, 50, 60))
            .show(ui);
    });

    harness.run();
}

/// Test Fader with custom dB range
#[test]
fn test_fader_custom_db_range() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).db_range(-60.0, 12.0).show(ui);
    });

    harness.run();
}

/// Test Fader with default value for double-click reset
#[test]
fn test_fader_default_value() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.8).default_value(0.75).show(ui);
    });

    harness.run();
}

/// Test Fader with velocity mode disabled
#[test]
fn test_fader_velocity_mode_disabled() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).velocity_mode(false).show(ui);
    });

    harness.run();
}

/// Test Fader with custom velocity sensitivity
#[test]
fn test_fader_velocity_sensitivity() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).velocity_sensitivity(2.0).show(ui);
    });

    harness.run();
}

/// Test Fader with custom ID
#[test]
fn test_fader_custom_id() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.5).id("my_fader").show(ui);
    });

    harness.run();
}

/// Test Fader at minimum value
#[test]
fn test_fader_minimum_value() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.0).show(ui);
    });

    harness.run();
}

/// Test Fader at maximum value
#[test]
fn test_fader_maximum_value() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(1.0).show(ui);
    });

    harness.run();
}

/// Test Fader response tuple
#[test]
fn test_fader_response() {
    let mut harness = Harness::new_ui(|ui| {
        let (response, value) = Fader::new(0.5).show(ui);

        // Check response and value
        let _ = response.clicked();
        assert!((value - 0.5).abs() < 0.01);
    });

    harness.run();
}

/// Test Fader with full configuration
#[test]
fn test_fader_full_config() {
    let mut harness = Harness::new_ui(|ui| {
        Fader::new(0.75)
            .id("master_fader")
            .size(35.0, 280.0)
            .scale_right()
            .response_curve(FaderCurve::Linear)
            .track_color(egui::Color32::from_rgb(40, 40, 50))
            .db_range(-96.0, 6.0)
            .default_value(0.75)
            .velocity_mode(true)
            .velocity_sensitivity(1.5)
            .show(ui);
    });

    harness.run();
}

/// Test multiple Faders in a row
#[test]
fn test_multiple_faders() {
    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            Fader::new(0.75).id("ch1").size(30.0, 200.0).show(ui);
            Fader::new(0.6).id("ch2").size(30.0, 200.0).show(ui);
            Fader::new(0.8).id("ch3").size(30.0, 200.0).show(ui);
            Fader::new(0.75).id("master").size(30.0, 200.0).show(ui);
        });
    });

    harness.run();
}

// FaderStrip tests

/// Test that FaderStrip renders without panicking
#[test]
fn test_fader_strip_renders() {
    let mut harness = Harness::new_ui(|ui| {
        FaderStrip::new(0.75).show(ui);
    });

    harness.run();
}

/// Test FaderStrip with custom size
#[test]
fn test_fader_strip_custom_size() {
    let mut harness = Harness::new_ui(|ui| {
        FaderStrip::new(0.5).size(50.0, 300.0).show(ui);
    });

    harness.run();
}

/// Test FaderStrip at minimum value
#[test]
fn test_fader_strip_minimum_value() {
    let mut harness = Harness::new_ui(|ui| {
        FaderStrip::new(0.0).show(ui);
    });

    harness.run();
}

/// Test FaderStrip at maximum value
#[test]
fn test_fader_strip_maximum_value() {
    let mut harness = Harness::new_ui(|ui| {
        FaderStrip::new(1.0).show(ui);
    });

    harness.run();
}

/// Test FaderStrip response tuple
#[test]
fn test_fader_strip_response() {
    let mut harness = Harness::new_ui(|ui| {
        let (response, value) = FaderStrip::new(0.5).show(ui);

        let _ = response.hovered();
        assert!((value - 0.5).abs() < 0.01);
    });

    harness.run();
}

/// Test multiple FaderStrips in a row
#[test]
fn test_multiple_fader_strips() {
    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            FaderStrip::new(0.75).show(ui);
            FaderStrip::new(0.6).show(ui);
            FaderStrip::new(0.8).show(ui);
        });
    });

    harness.run();
}

/// Test FaderScalePosition enum
#[test]
fn test_fader_scale_position_enum() {
    assert_eq!(FaderScalePosition::Left, FaderScalePosition::Left);
    assert_eq!(FaderScalePosition::Right, FaderScalePosition::Right);
    assert_eq!(FaderScalePosition::None, FaderScalePosition::None);
    assert_ne!(FaderScalePosition::Left, FaderScalePosition::Right);
}

/// Test FaderCurve enum
#[test]
fn test_fader_curve_enum() {
    assert_eq!(FaderCurve::Linear, FaderCurve::Linear);
    assert_eq!(FaderCurve::Logarithmic, FaderCurve::Logarithmic);
    assert_eq!(FaderCurve::Exponential, FaderCurve::Exponential);
    assert_ne!(FaderCurve::Linear, FaderCurve::Logarithmic);
}
