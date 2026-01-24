//! Tests for Modal component using egui_kittest

use armas::prelude::*;
use armas::components::overlays::{Modal, ModalSize};
use egui_kittest::Harness;

/// Test that Modal renders when open
#[test]
fn test_modal_renders_open() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("test_modal")
            .title("Test Modal")
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("Modal content");
        });
    });

    // Use step() because Modal has fade animation
    harness.step();
}

/// Test Modal does not render when closed
#[test]
fn test_modal_closed() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("test_modal")
            .title("Test Modal")
            .open(false);

        modal.show(ctx, &theme, |ui| {
            ui.label("Modal content");
        });
    });

    harness.run();
}

/// Test Modal with Small size
#[test]
fn test_modal_size_small() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("small_modal")
            .title("Small Modal")
            .size(ModalSize::Small)
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("Small content");
        });
    });

    harness.step();
}

/// Test Modal with Medium size
#[test]
fn test_modal_size_medium() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("medium_modal")
            .title("Medium Modal")
            .size(ModalSize::Medium)
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("Medium content");
        });
    });

    harness.step();
}

/// Test Modal with Large size
#[test]
fn test_modal_size_large() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("large_modal")
            .title("Large Modal")
            .size(ModalSize::Large)
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("Large content");
        });
    });

    harness.step();
}

/// Test Modal with Custom size
#[test]
fn test_modal_size_custom() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("custom_modal")
            .title("Custom Modal")
            .size(ModalSize::Custom(500.0, 350.0))
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("Custom size content");
        });
    });

    harness.step();
}

/// Test Modal without title
#[test]
fn test_modal_no_title() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("no_title_modal")
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("Content without title bar");
        });
    });

    harness.step();
}

/// Test Modal with closable=false
#[test]
fn test_modal_not_closable() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("not_closable")
            .title("Cannot Close")
            .closable(false)
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("This modal cannot be closed with ESC or backdrop click");
        });
    });

    harness.step();
}

/// Test Modal without backdrop blur
#[test]
fn test_modal_no_backdrop_blur() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("no_blur")
            .title("No Blur")
            .backdrop_blur(false)
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("Backdrop without blur");
        });
    });

    harness.step();
}

/// Test Modal with complex content
#[test]
fn test_modal_complex_content() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("complex_modal")
            .title("Form Modal")
            .size(ModalSize::Medium)
            .open(true);

        modal.show(ctx, &theme, |ui| {
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

/// Test Modal with light theme
#[test]
fn test_modal_light_theme() {
    let theme = Theme::light();

    let mut harness = Harness::new(|ctx| {
        let mut modal = Modal::new("light_modal")
            .title("Light Theme Modal")
            .open(true);

        modal.show(ctx, &theme, |ui| {
            ui.label("Light theme content");
        });
    });

    harness.step();
}

/// Test multiple modals (stacking)
#[test]
fn test_modal_stacking() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut modal1 = Modal::new("modal1")
            .title("First Modal")
            .open(true);

        modal1.show(ctx, &theme, |ui| {
            ui.label("First modal content");
        });

        let mut modal2 = Modal::new("modal2")
            .title("Second Modal")
            .open(true);

        modal2.show(ctx, &theme, |ui| {
            ui.label("Second modal content (on top)");
        });
    });

    harness.step();
}
