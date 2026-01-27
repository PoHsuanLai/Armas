//! Tests for Sheet component (side panel)

use armas::components::overlays::{Sheet, SheetSide, SheetSize};
use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Sheet renders when open
#[test]
fn test_sheet_renders_open() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("test_sheet").title("Test Sheet").open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Sheet content");
        });
    });

    harness.step();
}

/// Test Sheet does not render when closed
#[test]
fn test_sheet_closed() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("test_sheet").title("Test Sheet").open(false);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Sheet content");
        });
    });

    harness.run();
}

/// Test Sheet from Left side
#[test]
fn test_sheet_side_left() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("left_sheet")
            .title("Left Sheet")
            .side(SheetSide::Left)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Left side content");
        });
    });

    harness.step();
}

/// Test Sheet from Right side
#[test]
fn test_sheet_side_right() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("right_sheet")
            .title("Right Sheet")
            .side(SheetSide::Right)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Right side content");
        });
    });

    harness.step();
}

/// Test Sheet from Top side
#[test]
fn test_sheet_side_top() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("top_sheet")
            .title("Top Sheet")
            .side(SheetSide::Top)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Top content");
        });
    });

    harness.step();
}

/// Test Sheet from Bottom side
#[test]
fn test_sheet_side_bottom() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("bottom_sheet")
            .title("Bottom Sheet")
            .side(SheetSide::Bottom)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Bottom content");
        });
    });

    harness.step();
}

/// Test Sheet with Small size
#[test]
fn test_sheet_size_small() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("small_sheet")
            .title("Small")
            .size(SheetSize::Small)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Small sheet");
        });
    });

    harness.step();
}

/// Test Sheet with Medium size
#[test]
fn test_sheet_size_medium() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("medium_sheet")
            .title("Medium")
            .size(SheetSize::Medium)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Medium sheet");
        });
    });

    harness.step();
}

/// Test Sheet with Large size
#[test]
fn test_sheet_size_large() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("large_sheet")
            .title("Large")
            .size(SheetSize::Large)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Large sheet");
        });
    });

    harness.step();
}

/// Test Sheet with Custom size
#[test]
fn test_sheet_size_custom() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("custom_sheet")
            .title("Custom Size")
            .size(SheetSize::Custom(450.0))
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Custom 450px sheet");
        });
    });

    harness.step();
}

/// Test Sheet without title
#[test]
fn test_sheet_no_title() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("no_title_sheet").open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Content without title");
        });
    });

    harness.step();
}

/// Test Sheet with description
#[test]
fn test_sheet_with_description() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("desc_sheet")
            .title("Edit Profile")
            .description("Make changes to your profile here.")
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("Profile content");
        });
    });

    harness.step();
}

/// Test Sheet without close button
#[test]
fn test_sheet_no_close_button() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("no_close")
            .title("Cannot Close")
            .show_close_button(false)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("This sheet has no close button");
        });
    });

    harness.step();
}

/// Test Sheet without backdrop
#[test]
fn test_sheet_no_backdrop() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("no_backdrop")
            .title("No Backdrop")
            .show_backdrop(false)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.label("No backdrop behind");
        });
    });

    harness.step();
}

/// Test Sheet with navigation content
#[test]
fn test_sheet_navigation_content() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("nav_sheet")
            .title("Navigation")
            .side(SheetSide::Left)
            .size(SheetSize::Small)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.vertical(|ui| {
                ui.heading("Menu");
                ui.separator();
                Button::new("Home")
                    .variant(ButtonVariant::Ghost)
                    .show(ui, &theme);
                Button::new("Settings")
                    .variant(ButtonVariant::Ghost)
                    .show(ui, &theme);
                Button::new("Profile")
                    .variant(ButtonVariant::Ghost)
                    .show(ui, &theme);
                ui.separator();
                Button::new("Logout")
                    .variant(ButtonVariant::Ghost)
                    .show(ui, &theme);
            });
        });
    });

    harness.step();
}

/// Test Sheet with settings form content
#[test]
fn test_sheet_settings_content() {
    let theme = Theme::dark();

    let mut harness = Harness::new(|ctx| {
        let mut sheet = Sheet::new("settings_sheet")
            .title("Settings")
            .description("Configure your preferences")
            .side(SheetSide::Right)
            .size(SheetSize::Medium)
            .open(true);

        sheet.show(ctx, &theme, |ui| {
            ui.vertical(|ui| {
                let mut enabled = true;
                Toggle::new()
                    .label("Dark Mode")
                    .show(ui, &mut enabled, &theme);

                ui.add_space(8.0);

                let mut volume = 75.0;
                Slider::new(0.0, 100.0)
                    .label("Volume")
                    .suffix("%")
                    .show(ui, &mut volume, &theme);
            });
        });
    });

    harness.step();
}
