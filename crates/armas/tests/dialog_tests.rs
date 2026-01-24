//! Tests for Dialog component using egui_kittest

use armas::prelude::*;
use armas::components::overlays::{Dialog, DialogSize};
use egui_kittest::Harness;

/// Test that Dialog renders when open
#[test]
fn test_dialog_renders_open() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("test_dialog")
            .title("Test Dialog")
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Dialog content");
        });
    });

    harness.step();
}

/// Test Dialog does not render when closed
#[test]
fn test_dialog_closed() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("test_dialog")
            .title("Test Dialog")
            .open(false);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Dialog content");
        });
    });

    harness.run();
}

/// Test Dialog with Small size
#[test]
fn test_dialog_size_small() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("small_dialog")
            .title("Small Dialog")
            .size(DialogSize::Small)
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Small content");
        });
    });

    harness.step();
}

/// Test Dialog with Medium size
#[test]
fn test_dialog_size_medium() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("medium_dialog")
            .title("Medium Dialog")
            .size(DialogSize::Medium)
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Medium content");
        });
    });

    harness.step();
}

/// Test Dialog with Large size
#[test]
fn test_dialog_size_large() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("large_dialog")
            .title("Large Dialog")
            .size(DialogSize::Large)
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Large content");
        });
    });

    harness.step();
}

/// Test Dialog with Custom size
#[test]
fn test_dialog_size_custom() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("custom_dialog")
            .title("Custom Dialog")
            .size(DialogSize::Custom(500.0))
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Custom size content");
        });
    });

    harness.step();
}

/// Test Dialog without title
#[test]
fn test_dialog_no_title() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("no_title_dialog")
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Content without title bar");
        });
    });

    harness.step();
}

/// Test Dialog with closable=false (AlertDialog behavior)
#[test]
fn test_dialog_not_closable() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("not_closable")
            .title("Cannot Close")
            .closable(false)
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("This dialog cannot be closed with ESC or backdrop click");
        });
    });

    harness.step();
}

/// Test Dialog with description
#[test]
fn test_dialog_with_description() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("desc_dialog")
            .title("Dialog Title")
            .description("This is a description that explains the dialog purpose.")
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Content goes here");
        });
    });

    harness.step();
}

/// Test Dialog with complex content
#[test]
fn test_dialog_complex_content() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("complex_dialog")
            .title("Form Dialog")
            .size(DialogSize::Medium)
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.vertical(|ui| {
                ui.label("Please fill out the form:");
                ui.add_space(8.0);

                let mut name = String::new();
                Input::new("Name").label("Your Name").show(ui, &mut name);

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    Button::new("Cancel").variant(ButtonVariant::Outlined).show(ui);
                    Button::new("Submit").variant(ButtonVariant::Filled).show(ui);
                });
            });
        });
    });

    harness.step();
}

/// Test Dialog with light theme
#[test]
fn test_dialog_light_theme() {
    let theme = Theme::light();

    let mut harness = Harness::new(|ctx| {
        let mut dialog = Dialog::new("light_dialog")
            .title("Light Theme Dialog")
            .open(true);

        dialog.show(ctx, &theme, |ui| {
            ui.label("Light theme content");
        });
    });

    harness.step();
}

/// Test multiple dialogs (stacking)
#[test]
fn test_dialog_stacking() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut dialog1 = Dialog::new("dialog1")
            .title("First Dialog")
            .open(true);

        dialog1.show(ctx, &theme, |ui| {
            ui.label("First dialog content");
        });

        let mut dialog2 = Dialog::new("dialog2")
            .title("Second Dialog")
            .open(true);

        dialog2.show(ctx, &theme, |ui| {
            ui.label("Second dialog content (on top)");
        });
    });

    harness.step();
}

