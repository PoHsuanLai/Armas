//! Tests for AudioMeter component using egui_kittest

use armas::Theme;
use armas_audio::meter::{AudioMeter, MeterStyle, ScalePosition};
use egui::Color32;
use egui_kittest::Harness;

/// Test that AudioMeter renders without panicking
#[test]
fn test_meter_renders() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).show(ui);
    });

    // Use step() because meter has spring animation
    harness.step();
}

/// Test AudioMeter with custom width
#[test]
fn test_meter_custom_width() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).width(30.0).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with custom height
#[test]
fn test_meter_custom_height() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).height(300.0).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with smooth style (default)
#[test]
fn test_meter_smooth_style() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.75).style(MeterStyle::Smooth).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with segmented style
#[test]
fn test_meter_segmented_style() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.75).style(MeterStyle::Segmented(16)).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with 24 segments
#[test]
fn test_meter_segmented_24() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.6).style(MeterStyle::Segmented(24)).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with VU colors preset
#[test]
fn test_meter_vu_colors() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.8).vu_colors(&theme).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with monochrome preset
#[test]
fn test_meter_monochrome() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.7)
            .monochrome(Color32::from_rgb(100, 200, 255))
            .show(ui);
    });

    harness.step();
}

/// Test AudioMeter with custom color range
#[test]
fn test_meter_custom_color_range() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.6)
            .color_range(Color32::BLUE, Color32::RED)
            .show(ui);
    });

    harness.step();
}

/// Test AudioMeter with scale on right
#[test]
fn test_meter_scale_right() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).scale_right().show(ui);
    });

    harness.step();
}

/// Test AudioMeter with scale on left
#[test]
fn test_meter_scale_left() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).scale_left().show(ui);
    });

    harness.step();
}

/// Test AudioMeter with show_scale convenience method
#[test]
fn test_meter_show_scale() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).show_scale().show(ui);
    });

    harness.step();
}

/// Test AudioMeter with scale position enum
#[test]
fn test_meter_scale_position_enum() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5)
            .scale_position(ScalePosition::Right)
            .show(ui);
    });

    harness.step();
}

/// Test AudioMeter with custom peak color
#[test]
fn test_meter_custom_peak_color() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.9)
            .peak_color(Color32::from_rgb(255, 200, 0))
            .show(ui);
    });

    harness.step();
}

/// Test AudioMeter with custom corner radius
#[test]
fn test_meter_custom_corner_radius() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).corner_radius(8.0).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with custom background opacity
#[test]
fn test_meter_custom_background_opacity() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).background_opacity(0.5).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with glassmorphic disabled
#[test]
fn test_meter_glassmorphic_disabled() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).glassmorphic(false).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with custom animation speed
#[test]
fn test_meter_animation_speed() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).animation_speed(400.0).show(ui);
    });

    harness.step();
}

/// Test AudioMeter with custom animation damping
#[test]
fn test_meter_animation_damping() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.5).animation_damping(25.0).show(ui);
    });

    harness.step();
}

/// Test AudioMeter at minimum level
#[test]
fn test_meter_minimum_level() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.0).show(ui);
    });

    harness.step();
}

/// Test AudioMeter at maximum level
#[test]
fn test_meter_maximum_level() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(1.0).show(ui);
    });

    harness.step();
}

/// Test AudioMeter response fields
#[test]
fn test_meter_response() {
    let mut harness = Harness::new_ui(|ui| {
        let response = AudioMeter::new(0.5).show(ui);

        // Check response fields exist
        let _ = response.response;
        // Level may differ slightly due to animation
        assert!(response.level >= 0.0 && response.level <= 1.0);
        assert!(response.peak >= 0.0 && response.peak <= 1.0);
    });

    harness.step();
}

/// Test AudioMeter with full configuration
#[test]
fn test_meter_full_config() {
    let mut harness = Harness::new_ui(|ui| {
        AudioMeter::new(0.75)
            .width(25.0)
            .height(250.0)
            .style(MeterStyle::Segmented(20))
            .color_range(Color32::from_rgb(0, 100, 0), Color32::from_rgb(255, 50, 50))
            .peak_color(Color32::WHITE)
            .scale_position(ScalePosition::Right)
            .corner_radius(12.0)
            .background_opacity(0.4)
            .glassmorphic(true)
            .animation_speed(300.0)
            .animation_damping(20.0)
            .show(ui);
    });

    harness.step();
}

/// Test multiple AudioMeters (stereo pair)
#[test]
fn test_stereo_meters() {
    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            AudioMeter::new(0.7).width(15.0).height(200.0).show(ui);
            AudioMeter::new(0.65).width(15.0).height(200.0).show(ui);
        });
    });

    harness.step();
}

/// Test ScalePosition enum
#[test]
fn test_scale_position_enum() {
    assert_eq!(ScalePosition::Left, ScalePosition::Left);
    assert_eq!(ScalePosition::Right, ScalePosition::Right);
    assert_eq!(ScalePosition::None, ScalePosition::None);
    assert_ne!(ScalePosition::Left, ScalePosition::Right);
}

/// Test MeterStyle enum
#[test]
fn test_meter_style_enum() {
    assert_eq!(MeterStyle::Smooth, MeterStyle::Smooth);
    assert_eq!(MeterStyle::Segmented(16), MeterStyle::Segmented(16));
    assert_ne!(MeterStyle::Smooth, MeterStyle::Segmented(16));
    assert_ne!(MeterStyle::Segmented(16), MeterStyle::Segmented(24));
}

/// Test AudioMeter default
#[test]
fn test_meter_default() {
    let mut harness = Harness::new_ui(|ui| {
        let meter = AudioMeter::default();
        meter.show(ui);
    });

    harness.step();
}
