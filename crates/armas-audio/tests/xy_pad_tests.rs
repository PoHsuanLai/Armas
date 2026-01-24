//! Tests for XYPad component using egui_kittest

use armas_audio::xy_pad::{XYPad, XYPadVariant};
use egui_kittest::Harness;

/// Test that XYPad renders without panicking
#[test]
fn test_xy_pad_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).show(ui);
    });

    harness.run();
}

/// Test XYPad with custom size
#[test]
fn test_xy_pad_custom_size() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).size(300.0).show(ui);
    });

    harness.run();
}

/// Test XYPad with small size
#[test]
fn test_xy_pad_small_size() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).size(100.0).show(ui);
    });

    harness.run();
}

/// Test XYPad with Filled variant
#[test]
fn test_xy_pad_filled_variant() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y)
            .variant(XYPadVariant::Filled)
            .show(ui);
    });

    harness.run();
}

/// Test XYPad with Outlined variant
#[test]
fn test_xy_pad_outlined_variant() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y)
            .variant(XYPadVariant::Outlined)
            .show(ui);
    });

    harness.run();
}

/// Test XYPad with Elevated variant
#[test]
fn test_xy_pad_elevated_variant() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y)
            .variant(XYPadVariant::Elevated)
            .show(ui);
    });

    harness.run();
}

/// Test XYPad with X label
#[test]
fn test_xy_pad_x_label() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).x_label("Cutoff").show(ui);
    });

    harness.run();
}

/// Test XYPad with Y label
#[test]
fn test_xy_pad_y_label() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).y_label("Resonance").show(ui);
    });

    harness.run();
}

/// Test XYPad with both labels
#[test]
fn test_xy_pad_both_labels() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y)
            .x_label("Cutoff")
            .y_label("Resonance")
            .show(ui);
    });

    harness.run();
}

/// Test XYPad with crosshair hidden
#[test]
fn test_xy_pad_no_crosshair() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).show_crosshair(false).show(ui);
    });

    harness.run();
}

/// Test XYPad with values displayed
#[test]
fn test_xy_pad_show_values() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).show_values(true).show(ui);
    });

    harness.run();
}

/// Test XYPad with custom handle size
#[test]
fn test_xy_pad_custom_handle_size() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).handle_size(24.0).show(ui);
    });

    harness.run();
}

/// Test XYPad with small handle size
#[test]
fn test_xy_pad_small_handle_size() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).handle_size(8.0).show(ui);
    });

    harness.run();
}

/// Test XYPad with custom glow intensity
#[test]
fn test_xy_pad_custom_glow_intensity() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).glow_intensity(1.0).show(ui);
    });

    harness.run();
}

/// Test XYPad with no glow
#[test]
fn test_xy_pad_no_glow() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).glow_intensity(0.0).show(ui);
    });

    harness.run();
}

/// Test XYPad with custom ID
#[test]
fn test_xy_pad_custom_id() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        XYPad::new(&mut x, &mut y).id("filter_xy").show(ui);
    });

    harness.run();
}

/// Test XYPad at corner position (0, 0)
#[test]
fn test_xy_pad_corner_bottom_left() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.0;
        let mut y = 0.0;
        XYPad::new(&mut x, &mut y).show(ui);
    });

    harness.run();
}

/// Test XYPad at corner position (1, 1)
#[test]
fn test_xy_pad_corner_top_right() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 1.0;
        let mut y = 1.0;
        XYPad::new(&mut x, &mut y).show(ui);
    });

    harness.run();
}

/// Test XYPad at corner position (0, 1)
#[test]
fn test_xy_pad_corner_top_left() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.0;
        let mut y = 1.0;
        XYPad::new(&mut x, &mut y).show(ui);
    });

    harness.run();
}

/// Test XYPad at corner position (1, 0)
#[test]
fn test_xy_pad_corner_bottom_right() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 1.0;
        let mut y = 0.0;
        XYPad::new(&mut x, &mut y).show(ui);
    });

    harness.run();
}

/// Test XYPad response fields
#[test]
fn test_xy_pad_response() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.5;
        let mut y = 0.5;
        let response = XYPad::new(&mut x, &mut y).show(ui);

        // Check response fields exist
        let _ = response.response;
        assert!((response.x - 0.5).abs() < 0.01);
        assert!((response.y - 0.5).abs() < 0.01);
        assert!(!response.changed);
        assert!(!response.changed());
    });

    harness.run();
}

/// Test XYPad with full configuration
#[test]
fn test_xy_pad_full_config() {
    let mut harness = Harness::new_ui(|ui| {
        let mut x = 0.3;
        let mut y = 0.7;
        XYPad::new(&mut x, &mut y)
            .id("synth_filter_xy")
            .size(250.0)
            .variant(XYPadVariant::Elevated)
            .x_label("Cutoff")
            .y_label("Resonance")
            .show_crosshair(true)
            .show_values(true)
            .handle_size(20.0)
            .glow_intensity(0.8)
            .show(ui);
    });

    harness.run();
}

/// Test XYPadVariant enum
#[test]
fn test_xy_pad_variant_enum() {
    assert_eq!(XYPadVariant::Filled, XYPadVariant::Filled);
    assert_eq!(XYPadVariant::Outlined, XYPadVariant::Outlined);
    assert_eq!(XYPadVariant::Elevated, XYPadVariant::Elevated);
    assert_ne!(XYPadVariant::Filled, XYPadVariant::Outlined);
}

/// Test multiple XYPads
#[test]
fn test_multiple_xy_pads() {
    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            let mut x1 = 0.5;
            let mut y1 = 0.5;
            let mut x2 = 0.3;
            let mut y2 = 0.7;

            XYPad::new(&mut x1, &mut y1)
                .id("pad1")
                .size(150.0)
                .x_label("Filter")
                .show(ui);

            XYPad::new(&mut x2, &mut y2)
                .id("pad2")
                .size(150.0)
                .x_label("Effect")
                .show(ui);
        });
    });

    harness.run();
}
