//! Tests for Stepper component using egui_kittest

use armas::components::navigation::{Stepper, StepperOrientation};
use egui_kittest::Harness;

/// Test that Stepper renders without panicking
#[test]
fn test_stepper_renders() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 0, |stepper| {
            stepper.step("Step 1");
            stepper.step("Step 2");
            stepper.step("Step 3");
        });
    });

    harness.run();
}

/// Test Stepper at first step
#[test]
fn test_stepper_first_step() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 0, |stepper| {
            stepper.step("Account");
            stepper.step("Profile");
            stepper.step("Complete");
        });
    });

    harness.run();
}

/// Test Stepper at middle step
#[test]
fn test_stepper_middle_step() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 1, |stepper| {
            stepper.step("Account");
            stepper.step("Profile");
            stepper.step("Complete");
        });
    });

    harness.run();
}

/// Test Stepper at last step
#[test]
fn test_stepper_last_step() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 2, |stepper| {
            stepper.step("Account");
            stepper.step("Profile");
            stepper.step("Complete");
        });
    });

    harness.run();
}

/// Test Stepper all completed (beyond last step)
#[test]
fn test_stepper_all_completed() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 3, |stepper| {
            stepper.step("Account");
            stepper.step("Profile");
            stepper.step("Complete");
        });
    });

    harness.run();
}

/// Test Stepper with descriptions
#[test]
fn test_stepper_with_descriptions() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 1, |stepper| {
            stepper.step("Account").description("Create your account");
            stepper.step("Profile").description("Set up your profile");
            stepper.step("Complete").description("Finish setup");
        });
    });

    harness.run();
}

/// Test Stepper with icons
#[test]
fn test_stepper_with_icons() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 1, |stepper| {
            stepper.step("User").icon("👤");
            stepper.step("Settings").icon("⚙️");
            stepper.step("Done").icon("✓");
        });
    });

    harness.run();
}

/// Test Stepper with icons and descriptions
#[test]
fn test_stepper_icons_and_descriptions() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 0, |stepper| {
            stepper.step("Account").icon("👤").description("Create account");
            stepper.step("Payment").icon("💳").description("Add payment");
            stepper.step("Review").icon("📋").description("Review order");
            stepper.step("Done").icon("✓").description("Complete");
        });
    });

    harness.run();
}

/// Test Stepper horizontal orientation (default)
#[test]
fn test_stepper_horizontal() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new()
            .orientation(StepperOrientation::Horizontal)
            .show(ui, 1, |stepper| {
                stepper.step("Step 1");
                stepper.step("Step 2");
                stepper.step("Step 3");
            });
    });

    harness.run();
}

/// Test Stepper vertical orientation
#[test]
fn test_stepper_vertical() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new()
            .orientation(StepperOrientation::Vertical)
            .show(ui, 1, |stepper| {
                stepper.step("Step 1");
                stepper.step("Step 2");
                stepper.step("Step 3");
            });
    });

    harness.run();
}

/// Test Stepper vertical with descriptions
#[test]
fn test_stepper_vertical_with_descriptions() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new()
            .orientation(StepperOrientation::Vertical)
            .show(ui, 1, |stepper| {
                stepper.step("Setup").description("Configure your environment");
                stepper.step("Install").description("Install dependencies");
                stepper.step("Configure").description("Set preferences");
                stepper.step("Done").description("Ready to use");
            });
    });

    harness.run();
}

/// Test Stepper clickable mode
#[test]
fn test_stepper_clickable() {
    let mut harness = Harness::new_ui(|ui| {
        let response = Stepper::new()
            .clickable(true)
            .show(ui, 1, |stepper| {
                stepper.step("Step 1");
                stepper.step("Step 2");
                stepper.step("Step 3");
            });

        // Response should have clicked_step field (even if None)
        let _ = response.clicked_step;
    });

    harness.run();
}

/// Test Stepper non-clickable mode
#[test]
fn test_stepper_not_clickable() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new()
            .clickable(false)
            .show(ui, 1, |stepper| {
                stepper.step("Step 1");
                stepper.step("Step 2");
                stepper.step("Step 3");
            });
    });

    harness.run();
}

/// Test Stepper with numbers shown
#[test]
fn test_stepper_show_numbers() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new()
            .show_numbers(true)
            .show(ui, 1, |stepper| {
                stepper.step("Step 1");
                stepper.step("Step 2");
                stepper.step("Step 3");
            });
    });

    harness.run();
}

/// Test Stepper with numbers hidden (checkmarks for completed)
#[test]
fn test_stepper_hide_numbers() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new()
            .show_numbers(false)
            .show(ui, 2, |stepper| {
                stepper.step("Step 1");
                stepper.step("Step 2");
                stepper.step("Step 3");
            });
    });

    harness.run();
}

/// Test Stepper with many steps
#[test]
fn test_stepper_many_steps() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 3, |stepper| {
            stepper.step("Start");
            stepper.step("Step 2");
            stepper.step("Step 3");
            stepper.step("Step 4");
            stepper.step("Step 5");
            stepper.step("Finish");
        });
    });

    harness.run();
}

/// Test Stepper with single step
#[test]
fn test_stepper_single_step() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new().show(ui, 0, |stepper| {
            stepper.step("Only Step");
        });
    });

    harness.run();
}

/// Test Stepper checkout flow
#[test]
fn test_stepper_checkout_flow() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new()
            .orientation(StepperOrientation::Horizontal)
            .show(ui, 2, |stepper| {
                stepper.step("Cart").icon("🛒").description("Review items");
                stepper.step("Shipping").icon("📦").description("Enter address");
                stepper.step("Payment").icon("💳").description("Add payment");
                stepper.step("Confirm").icon("✓").description("Place order");
            });
    });

    harness.run();
}

/// Test Stepper onboarding flow vertical
#[test]
fn test_stepper_onboarding_vertical() {
    let mut harness = Harness::new_ui(|ui| {
        Stepper::new()
            .orientation(StepperOrientation::Vertical)
            .clickable(true)
            .show(ui, 1, |stepper| {
                stepper.step("Welcome").description("Introduction to the app");
                stepper.step("Account").description("Create your account");
                stepper.step("Preferences").description("Set your preferences");
                stepper.step("Tutorial").description("Quick tour");
                stepper.step("Complete").description("You're all set!");
            });
    });

    harness.run();
}
