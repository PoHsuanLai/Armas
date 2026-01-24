//! Tests for Drawer component using egui_kittest

use armas::prelude::*;
use armas::components::overlays::Drawer;
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

    // Use step() because Drawer has slide animation
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

/// Test Drawer from Left position
#[test]
fn test_drawer_position_left() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("left_drawer")
            .title("Left Drawer")
            .position(armas::components::overlays::DrawerPosition::Left)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Left side content");
        });
    });

    harness.step();
}

/// Test Drawer from Right position
#[test]
fn test_drawer_position_right() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("right_drawer")
            .title("Right Drawer")
            .position(armas::components::overlays::DrawerPosition::Right)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Right side content");
        });
    });

    harness.step();
}

/// Test Drawer from Top position
#[test]
fn test_drawer_position_top() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("top_drawer")
            .title("Top Drawer")
            .position(armas::components::overlays::DrawerPosition::Top)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Top content");
        });
    });

    harness.step();
}

/// Test Drawer from Bottom position
#[test]
fn test_drawer_position_bottom() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("bottom_drawer")
            .title("Bottom Drawer")
            .position(armas::components::overlays::DrawerPosition::Bottom)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Bottom content");
        });
    });

    harness.step();
}

/// Test Drawer with Small size
#[test]
fn test_drawer_size_small() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("small_drawer")
            .title("Small")
            .size(DrawerSize::Small)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Small drawer");
        });
    });

    harness.step();
}

/// Test Drawer with Medium size
#[test]
fn test_drawer_size_medium() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("medium_drawer")
            .title("Medium")
            .size(DrawerSize::Medium)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Medium drawer");
        });
    });

    harness.step();
}

/// Test Drawer with Large size
#[test]
fn test_drawer_size_large() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("large_drawer")
            .title("Large")
            .size(DrawerSize::Large)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Large drawer");
        });
    });

    harness.step();
}

/// Test Drawer with Custom size
#[test]
fn test_drawer_size_custom() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("custom_drawer")
            .title("Custom Size")
            .size(DrawerSize::Custom(450.0))
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Custom 450px drawer");
        });
    });

    harness.step();
}

/// Test Drawer without title
#[test]
fn test_drawer_no_title() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("no_title_drawer")
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("Content without title");
        });
    });

    harness.step();
}

/// Test Drawer with closable=false
#[test]
fn test_drawer_not_closable() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("not_closable")
            .title("Cannot Close")
            .closable(false)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.label("This drawer cannot be closed");
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

/// Test Drawer with complex navigation content
#[test]
fn test_drawer_navigation_content() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("nav_drawer")
            .title("Navigation")
            .position(armas::components::overlays::DrawerPosition::Left)
            .size(DrawerSize::Small)
            .open(true);

        drawer.show(ctx, &theme, |ui| {
            ui.vertical(|ui| {
                ui.heading("Menu");
                ui.separator();
                Button::new("Home").variant(ButtonVariant::Text).show(ui);
                Button::new("Settings").variant(ButtonVariant::Text).show(ui);
                Button::new("Profile").variant(ButtonVariant::Text).show(ui);
                ui.separator();
                Button::new("Logout").variant(ButtonVariant::Text).show(ui);
            });
        });
    });

    harness.step();
}

/// Test Drawer with settings form content
#[test]
fn test_drawer_settings_content() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut drawer = Drawer::new("settings_drawer")
            .title("Settings")
            .position(armas::components::overlays::DrawerPosition::Right)
            .size(DrawerSize::Medium)
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
            });
        });
    });

    harness.step();
}
