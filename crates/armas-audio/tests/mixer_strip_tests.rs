//! Tests for MixerStrip component using egui_kittest
//!
//! Note: MixerStrip contains AudioMeter which has spring animation,
//! so we use harness.step() instead of harness.run() for rendering tests.

use armas::Theme;
use armas_audio::mixer_strip::{Insert, MixerStrip, Route, Send};
use egui::Color32;
use egui_kittest::Harness;

/// Test that MixerStrip renders without panicking
#[test]
fn test_mixer_strip_renders() {
    let theme = Theme::dark();
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1");
        strip.show(ui, &theme);
    });

    // Use step() because MixerStrip contains AudioMeter with spring animation
    harness.step();
}

/// Test MixerStrip with custom width
#[test]
fn test_mixer_strip_custom_width() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").width(90.0);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom scale
#[test]
fn test_mixer_strip_custom_scale() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").scale(1.2);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with small scale
#[test]
fn test_mixer_strip_small_scale() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").scale(0.8);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom fader level
#[test]
fn test_mixer_strip_custom_fader_level() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").fader_level(0.5);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with fader at minimum
#[test]
fn test_mixer_strip_fader_minimum() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").fader_level(0.0);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with fader at maximum
#[test]
fn test_mixer_strip_fader_maximum() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").fader_level(1.0);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom pan
#[test]
fn test_mixer_strip_custom_pan() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").pan(-0.5);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with pan hard left
#[test]
fn test_mixer_strip_pan_left() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").pan(-1.0);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with pan hard right
#[test]
fn test_mixer_strip_pan_right() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").pan(1.0);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip muted
#[test]
fn test_mixer_strip_muted() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").muted(true);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip soloed
#[test]
fn test_mixer_strip_soloed() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").soloed(true);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip record armed
#[test]
fn test_mixer_strip_record_armed() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").record_armed(true);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip input monitoring
#[test]
fn test_mixer_strip_input_monitoring() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").input_monitoring(true);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with meter level
#[test]
fn test_mixer_strip_meter_level() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").meter_level(0.8);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom card color
#[test]
fn test_mixer_strip_custom_card_color() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").card_color(Color32::from_rgb(40, 40, 50));
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom knob color
#[test]
fn test_mixer_strip_custom_knob_color() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").knob_color(Color32::from_rgb(100, 200, 255));
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom meter color
#[test]
fn test_mixer_strip_custom_meter_color() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").meter_color(Color32::from_rgb(0, 255, 100));
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom inserts
#[test]
fn test_mixer_strip_custom_inserts() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let inserts = vec![
            Insert::new("EQ"),
            Insert::new("Compressor"),
            Insert::empty(),
            Insert::empty(),
        ];
        let mut strip = MixerStrip::new("Channel 1").inserts(inserts);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom sends
#[test]
fn test_mixer_strip_custom_sends() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let sends = vec![Send::new("Reverb"), Send::new("Delay"), Send::new("Chorus")];
        let mut strip = MixerStrip::new("Channel 1").sends(sends);
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom input route
#[test]
fn test_mixer_strip_custom_input_route() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").input_route(Route::new("Mic 1"));
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip with custom output route
#[test]
fn test_mixer_strip_custom_output_route() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1").output_route(Route::new("Bus A"));
        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test MixerStrip getter methods
#[test]
fn test_mixer_strip_getters() {
    let theme = Theme::dark();
    let inserts = vec![Insert::new("EQ"), Insert::empty()];
    let sends = vec![Send::new("Reverb")];

    let strip = MixerStrip::new("Test Channel")
        .fader_level(0.8)
        .pan(0.25)
        .muted(true)
        .soloed(false)
        .record_armed(true)
        .input_monitoring(false)
        .meter_level(0.5)
        .inserts(inserts)
        .sends(sends)
        .input_route(Route::new("Input 3"))
        .output_route(Route::new("Main"));

    assert_eq!(strip.get_name(), "Test Channel");
    assert!((strip.get_fader_level() - 0.8).abs() < 0.01);
    assert!((strip.get_pan() - 0.25).abs() < 0.01);
    assert!(strip.is_muted());
    assert!(!strip.is_soloed());
    assert!(strip.is_record_armed());
    assert!(!strip.is_input_monitoring());
    assert!((strip.get_meter_level() - 0.5).abs() < 0.01);
    assert_eq!(strip.get_inserts().len(), 2);
    assert_eq!(strip.get_sends().len(), 1);
    assert_eq!(strip.get_input_route().name, "Input 3");
    assert_eq!(strip.get_output_route().name, "Main");
}

/// Test MixerStrip response fields
#[test]
fn test_mixer_strip_response() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::new("Channel 1");
        let response = strip.show(ui, &theme);

        // Check response fields exist
        let _ = response.response;
        assert!(!response.fader_changed);
        assert!(!response.pan_changed);
        assert!(!response.mute_toggled);
        assert!(!response.solo_toggled);
        assert!(!response.record_toggled);
        assert!(!response.monitor_toggled);
        assert!(!response.sends_clicked);
        assert!(!response.input_routing_clicked);
        assert!(!response.output_routing_clicked);
    });

    harness.step();
}

/// Test MixerStrip with full configuration
#[test]
fn test_mixer_strip_full_config() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let inserts = vec![
            Insert::new("EQ"),
            Insert::new("Compressor"),
            Insert::new("Limiter"),
            Insert::empty(),
        ];
        let sends = vec![Send::new("Reverb"), Send::new("Delay")];

        let mut strip = MixerStrip::new("Vocal")
            .width(80.0)
            .scale(1.0)
            .fader_level(0.75)
            .pan(0.0)
            .muted(false)
            .soloed(false)
            .record_armed(true)
            .input_monitoring(true)
            .meter_level(0.6)
            .card_color(Color32::from_rgb(35, 35, 40))
            .knob_color(Color32::from_rgb(255, 150, 50))
            .meter_color(Color32::from_rgb(100, 255, 100))
            .inserts(inserts)
            .sends(sends)
            .input_route(Route::new("Mic 1"))
            .output_route(Route::new("Main"));

        strip.show(ui, &theme);
    });

    harness.step();
}

/// Test multiple MixerStrips (mixer view)
#[test]
fn test_multiple_mixer_strips() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            let mut ch1 = MixerStrip::new("Ch 1").fader_level(0.75);
            let mut ch2 = MixerStrip::new("Ch 2").fader_level(0.6).pan(-0.3);
            let mut ch3 = MixerStrip::new("Ch 3").fader_level(0.8).pan(0.3);
            let mut master = MixerStrip::new("Master").fader_level(0.75);

            ch1.show(ui, &theme);
            ch2.show(ui, &theme);
            ch3.show(ui, &theme);
            master.show(ui, &theme);
        });
    });

    harness.step();
}

/// Test Send struct
#[test]
fn test_send_struct() {
    let send = Send::new("Reverb");

    assert_eq!(send.name, "Reverb");
    assert!((send.level - 0.5).abs() < 0.01);
    assert!(!send.pre_fader);
    assert!(!send.muted);
}

/// Test Insert struct - empty
#[test]
fn test_insert_empty() {
    let insert = Insert::empty();

    assert!(insert.name.is_none());
    assert!(!insert.bypassed);
}

/// Test Insert struct - with name
#[test]
fn test_insert_with_name() {
    let insert = Insert::new("Compressor");

    assert_eq!(insert.name, Some("Compressor".to_string()));
    assert!(!insert.bypassed);
}

/// Test Route struct
#[test]
fn test_route_struct() {
    let route = Route::new("Main");

    assert_eq!(route.name, "Main");
}

/// Test MixerStrip default
#[test]
fn test_mixer_strip_default() {
    let theme = Theme::dark();
    let mut harness = Harness::new_ui(|ui| {
        let mut strip = MixerStrip::default();
        strip.show(ui, &theme);
    });

    harness.step();
}
