//! Tests for Breadcrumbs component using egui_kittest

use armas::components::navigation::Breadcrumbs;
use egui_kittest::Harness;

/// Test that Breadcrumbs renders without panicking
#[test]
fn test_breadcrumbs_renders() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new().show(ui, |bc| {
            bc.item("Home", None);
            bc.item("Products", None);
            bc.item("Electronics", None);
        });
    });

    harness.run();
}

/// Test Breadcrumbs with single item
#[test]
fn test_breadcrumbs_single_item() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new().show(ui, |bc| {
            bc.item("Home", None);
        });
    });

    harness.run();
}

/// Test Breadcrumbs with current (non-clickable) item
#[test]
fn test_breadcrumbs_with_current() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new().show(ui, |bc| {
            bc.item("Home", None);
            bc.item("Products", None);
            bc.item("Current Page", None).current();
        });
    });

    harness.run();
}

/// Test Breadcrumbs with icons
#[test]
fn test_breadcrumbs_with_icons() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new().show(ui, |bc| {
            bc.item("Home", Some("🏠"));
            bc.item("Settings", Some("⚙️"));
            bc.item("Profile", Some("👤")).current();
        });
    });

    harness.run();
}

/// Test Breadcrumbs with custom spacing
#[test]
fn test_breadcrumbs_custom_spacing() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new()
            .spacing(8.0)
            .show(ui, |bc| {
                bc.item("A", None);
                bc.item("B", None);
                bc.item("C", None);
            });
    });

    harness.run();
}

/// Test Breadcrumbs with zero spacing
#[test]
fn test_breadcrumbs_zero_spacing() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new()
            .spacing(0.0)
            .show(ui, |bc| {
                bc.item("One", None);
                bc.item("Two", None);
            });
    });

    harness.run();
}

/// Test Breadcrumbs with many items
#[test]
fn test_breadcrumbs_many_items() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new().show(ui, |bc| {
            bc.item("Root", None);
            bc.item("Level 1", None);
            bc.item("Level 2", None);
            bc.item("Level 3", None);
            bc.item("Level 4", None);
            bc.item("Current", None).current();
        });
    });

    harness.run();
}

/// Test Breadcrumbs file path style
#[test]
fn test_breadcrumbs_file_path() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new().show(ui, |bc| {
            bc.item("Users", Some("📁"));
            bc.item("john", Some("📁"));
            bc.item("Documents", Some("📁"));
            bc.item("report.pdf", Some("📄")).current();
        });
    });

    harness.run();
}

/// Test Breadcrumbs website navigation style
#[test]
fn test_breadcrumbs_website_nav() {
    let mut harness = Harness::new_ui(|ui| {
        Breadcrumbs::new().show(ui, |bc| {
            bc.item("Home", Some("🏠"));
            bc.item("Shop", None);
            bc.item("Electronics", None);
            bc.item("Smartphones", None);
            bc.item("iPhone 15", None).current();
        });
    });

    harness.run();
}

/// Test Breadcrumbs response (clicked field exists)
#[test]
fn test_breadcrumbs_response() {
    let mut harness = Harness::new_ui(|ui| {
        let response = Breadcrumbs::new().show(ui, |bc| {
            bc.item("Home", None);
            bc.item("Page", None).current();
        });

        // Response should have clicked field (None since no click)
        assert!(response.clicked.is_none());
    });

    harness.run();
}
