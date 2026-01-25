//! Tests for Drawer component (vaul-style bottom sheet)

use armas::components::overlays::Drawer;
use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Drawer renders when open
#[test]
fn test_drawer_renders_open() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("test_drawer")
            .title("Test Drawer")
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Drawer content");
        });
    });

    harness.step();
}

/// Test Drawer does not render when closed
#[test]
fn test_drawer_closed() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("test_drawer")
            .title("Test Drawer")
            .open(false);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Drawer content");
        });
    });

    harness.run();
}

/// Test Drawer without title
#[test]
fn test_drawer_no_title() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("no_title_drawer").open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Content without title");
        });
    });

    harness.step();
}

/// Test Drawer with description
#[test]
fn test_drawer_with_description() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("desc_drawer")
            .title("Settings")
            .description("Configure your preferences")
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Settings content");
        });
    });

    harness.step();
}

/// Test Drawer without handle
#[test]
fn test_drawer_no_handle() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("no_handle")
            .title("No Handle")
            .show_handle(false)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("This drawer has no drag handle");
        });
    });

    harness.step();
}

/// Test Drawer without backdrop
#[test]
fn test_drawer_no_backdrop() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("no_backdrop")
            .title("No Backdrop")
            .show_backdrop(false)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("No backdrop behind");
        });
    });

    harness.step();
}

/// Test Drawer with custom height
#[test]
fn test_drawer_custom_height() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("custom_height")
            .title("Custom Height")
            .height(300.0)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Custom 300px height");
        });
    });

    harness.step();
}

/// Test Drawer with form content
#[test]
fn test_drawer_form_content() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("form_drawer")
            .title("Edit Profile")
            .description("Make changes to your profile")
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.vertical(|ui| {
                let mut enabled = true;
                Toggle::new().label("Dark Mode").show(ui, &mut enabled);

                ui.add_space(8.0);

                let mut volume = 75.0;
                Slider::new(0.0, 100.0)
                    .label("Volume")
                    .suffix("%")
                    .show(ui, &mut volume);

                ui.add_space(16.0);

                Button::new("Save Changes")
                    .variant(ButtonVariant::Default)
                    .show(ui);
            });
        });
    });

    harness.step();
}
