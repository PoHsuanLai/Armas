//! Tests for Input component using egui_kittest

use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Input renders without panicking
#[test]
fn test_input_renders() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Enter text").show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with Default variant
#[test]
fn test_input_default_variant() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Placeholder")
            .variant(InputVariant::Default)
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with Outlined variant
#[test]
fn test_input_outlined_variant() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Placeholder")
            .variant(InputVariant::Outlined)
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with Filled variant
#[test]
fn test_input_filled_variant() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Placeholder")
            .variant(InputVariant::Filled)
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with Inline variant
#[test]
fn test_input_inline_variant() {
    let mut text = "Inline text".to_string();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Edit me")
            .variant(InputVariant::Inline)
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with label
#[test]
fn test_input_with_label() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Enter username")
            .label("Username")
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with helper text
#[test]
fn test_input_with_helper_text() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Enter email")
            .label("Email")
            .helper_text("We'll never share your email")
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with icons
#[test]
fn test_input_with_icons() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Search...")
            .left_icon("🔍")
            .right_icon("✕")
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with Normal state
#[test]
fn test_input_state_normal() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Text")
            .state(InputState::Normal)
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with Success state
#[test]
fn test_input_state_success() {
    let mut text = "valid@email.com".to_string();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Email")
            .state(InputState::Success)
            .helper_text("Email is valid")
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with Error state
#[test]
fn test_input_state_error() {
    let mut text = "invalid".to_string();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Email")
            .state(InputState::Error)
            .helper_text("Please enter a valid email")
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with Warning state
#[test]
fn test_input_state_warning() {
    let mut text = "weak".to_string();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Password")
            .state(InputState::Warning)
            .helper_text("Password is weak")
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input as password field
#[test]
fn test_input_password() {
    let mut text = "secret123".to_string();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Enter password")
            .password(true)
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test Input with custom width
#[test]
fn test_input_custom_width() {
    let mut text = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Input::new("Wide input")
            .width(400.0)
            .show(ui, &mut text, &theme);
    });

    harness.run();
}

/// Test multiple inputs in a form layout
#[test]
fn test_multiple_inputs() {
    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        ui.vertical(|ui| {
            Input::new("Username")
                .label("Username")
                .show(ui, &mut username, &theme);
            Input::new("Email")
                .label("Email")
                .show(ui, &mut email, &theme);
            Input::new("Password")
                .label("Password")
                .password(true)
                .show(ui, &mut password, &theme);
        });
    });

    harness.run();
}
