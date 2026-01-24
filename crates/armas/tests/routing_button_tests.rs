//! Tests for RoutingButton component using egui_kittest

use armas::components::navigation::RoutingButton;
use egui_kittest::Harness;

/// Test that RoutingButton input renders without panicking
#[test]
fn test_routing_button_input_renders() {
    let mut harness = Harness::new_ui(|ui| {
        RoutingButton::input("Input L+R").show(ui);
    });

    harness.run();
}

/// Test that RoutingButton output renders without panicking
#[test]
fn test_routing_button_output_renders() {
    let mut harness = Harness::new_ui(|ui| {
        RoutingButton::output("Main").show(ui);
    });

    harness.run();
}

/// Test RoutingButton with custom size
#[test]
fn test_routing_button_custom_size() {
    let mut harness = Harness::new_ui(|ui| {
        RoutingButton::input("Stereo")
            .size(120.0, 36.0)
            .show(ui);
    });

    harness.run();
}

/// Test RoutingButton with custom width
#[test]
fn test_routing_button_custom_width() {
    let mut harness = Harness::new_ui(|ui| {
        RoutingButton::input("Wide")
            .width(150.0)
            .show(ui);
    });

    harness.run();
}

/// Test RoutingButton with custom height
#[test]
fn test_routing_button_custom_height() {
    let mut harness = Harness::new_ui(|ui| {
        RoutingButton::output("Tall")
            .height(48.0)
            .show(ui);
    });

    harness.run();
}

/// Test RoutingButton input with various labels
#[test]
fn test_routing_button_input_labels() {
    let labels = ["Input L", "Input R", "Input L+R", "Mono", "Stereo", "1-2"];

    let mut harness = Harness::new_ui(|ui| {
        ui.vertical(|ui| {
            for label in &labels {
                RoutingButton::input(label).show(ui);
            }
        });
    });

    harness.run();
}

/// Test RoutingButton output with various labels
#[test]
fn test_routing_button_output_labels() {
    let labels = ["Main", "Bus 1", "Bus 2", "Send A", "Send B", "Out 1-2"];

    let mut harness = Harness::new_ui(|ui| {
        ui.vertical(|ui| {
            for label in &labels {
                RoutingButton::output(label).show(ui);
            }
        });
    });

    harness.run();
}

/// Test RoutingButton input and output side by side
#[test]
fn test_routing_button_input_output_pair() {
    let mut harness = Harness::new_ui(|ui| {
        ui.horizontal(|ui| {
            RoutingButton::input("Input L+R").show(ui);
            RoutingButton::output("Main").show(ui);
        });
    });

    harness.run();
}

/// Test RoutingButton in a mixer-like layout
#[test]
fn test_routing_button_mixer_layout() {
    let mut harness = Harness::new_ui(|ui| {
        ui.vertical(|ui| {
            // Channel 1
            ui.horizontal(|ui| {
                ui.label("Ch 1");
                RoutingButton::input("Mic 1").width(80.0).show(ui);
                RoutingButton::output("Main").width(80.0).show(ui);
            });

            // Channel 2
            ui.horizontal(|ui| {
                ui.label("Ch 2");
                RoutingButton::input("Line 1-2").width(80.0).show(ui);
                RoutingButton::output("Bus 1").width(80.0).show(ui);
            });

            // Channel 3
            ui.horizontal(|ui| {
                ui.label("Ch 3");
                RoutingButton::input("Virtual").width(80.0).show(ui);
                RoutingButton::output("Bus 2").width(80.0).show(ui);
            });
        });
    });

    harness.run();
}

/// Test RoutingButton response is clickable
#[test]
fn test_routing_button_response() {
    let mut harness = Harness::new_ui(|ui| {
        let response = RoutingButton::input("Input").show(ui);

        // Response should be a valid egui Response
        let _ = response.clicked();
        let _ = response.hovered();
    });

    harness.run();
}

/// Test RoutingButton with very long label
#[test]
fn test_routing_button_long_label() {
    let mut harness = Harness::new_ui(|ui| {
        RoutingButton::input("Very Long Input Channel Name")
            .width(200.0)
            .show(ui);
    });

    harness.run();
}

/// Test RoutingButton with short label
#[test]
fn test_routing_button_short_label() {
    let mut harness = Harness::new_ui(|ui| {
        RoutingButton::input("L")
            .width(40.0)
            .show(ui);
    });

    harness.run();
}
