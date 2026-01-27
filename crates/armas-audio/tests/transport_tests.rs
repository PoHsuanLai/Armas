//! Tests for TransportControl component using egui_kittest

use armas::ArmasContextExt;
use armas_audio::transport::{TransportButtons, TransportControl, TransportState};
use egui_kittest::Harness;

/// Test that TransportControl renders without panicking
#[test]
fn test_transport_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new().show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl in stopped state
#[test]
fn test_transport_stopped_state() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .state(TransportState::Stopped)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl in playing state
#[test]
fn test_transport_playing_state() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .state(TransportState::Playing)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl in paused state
#[test]
fn test_transport_paused_state() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .state(TransportState::Paused)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl in recording state
#[test]
fn test_transport_recording_state() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .state(TransportState::Recording)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with custom tempo
#[test]
fn test_transport_custom_tempo() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new().tempo(140.0).show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with slow tempo
#[test]
fn test_transport_slow_tempo() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new().tempo(60.0).show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with fast tempo
#[test]
fn test_transport_fast_tempo() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new().tempo(200.0).show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with custom time signature 4/4
#[test]
fn test_transport_time_sig_4_4() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .time_signature(4, 4)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with 3/4 time signature
#[test]
fn test_transport_time_sig_3_4() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .time_signature(3, 4)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with 6/8 time signature
#[test]
fn test_transport_time_sig_6_8() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .time_signature(6, 8)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with custom current time
#[test]
fn test_transport_custom_time() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .current_time(65.5) // 1:05.500
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with loop enabled
#[test]
fn test_transport_loop_enabled() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new().loop_enabled(true).show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with metronome enabled
#[test]
fn test_transport_metronome_enabled() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .metronome_enabled(true)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with custom width
#[test]
fn test_transport_custom_width() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new().width(600.0).show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with custom button color
#[test]
fn test_transport_custom_button_color() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .button_color(egui::Color32::from_rgb(100, 150, 200))
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with custom button configuration
#[test]
fn test_transport_custom_buttons() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let buttons = TransportButtons {
            show_rewind: true,
            show_play: true,
            show_stop: true,
            show_record: true,
            show_loop: true,
            show_metronome: true,
        };

        TransportControl::new().buttons(buttons).show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with minimal buttons
#[test]
fn test_transport_minimal_buttons() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let buttons = TransportButtons {
            show_rewind: false,
            show_play: true,
            show_stop: true,
            show_record: false,
            show_loop: false,
            show_metronome: false,
        };

        TransportControl::new().buttons(buttons).show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl response fields
#[test]
fn test_transport_response() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let response = TransportControl::new()
            .tempo(120.0)
            .time_signature(4, 4)
            .show(ui, &theme);

        // Check response fields exist
        let _ = response.response;
        assert_eq!(response.state, TransportState::Stopped);
        assert_eq!(response.tempo, 120.0);
        assert_eq!(response.time_signature, (4, 4));
        assert!(!response.play_clicked);
        assert!(!response.stop_clicked);
        assert!(!response.record_clicked);
    });

    harness.run();
}

/// Test TransportControl with full configuration
#[test]
fn test_transport_full_config() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .state(TransportState::Playing)
            .current_time(123.456)
            .tempo(128.0)
            .time_signature(4, 4)
            .loop_enabled(true)
            .metronome_enabled(true)
            .width(800.0)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test TransportState enum
#[test]
fn test_transport_state_enum() {
    assert_eq!(TransportState::Stopped, TransportState::Stopped);
    assert_eq!(TransportState::Playing, TransportState::Playing);
    assert_eq!(TransportState::Paused, TransportState::Paused);
    assert_eq!(TransportState::Recording, TransportState::Recording);
    assert_ne!(TransportState::Stopped, TransportState::Playing);
}

/// Test TransportButtons default
#[test]
fn test_transport_buttons_default() {
    let buttons = TransportButtons::default();

    assert!(buttons.show_rewind);
    assert!(buttons.show_play);
    assert!(!buttons.show_stop);
    assert!(buttons.show_record);
    assert!(buttons.show_loop);
    assert!(buttons.show_metronome);
}

/// Test TransportControl default
#[test]
fn test_transport_default() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let transport = TransportControl::default();
        transport.show(ui, &theme);
    });

    harness.run();
}

/// Test TransportControl with light theme
#[test]
fn test_transport_light_theme() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        TransportControl::new()
            .tempo(120.0)
            .state(TransportState::Playing)
            .show(ui, &theme);
    });

    harness.run();
}
