//! Tests for Breadcrumb component using `egui_kittest`

use armas_basic::Breadcrumb;
use armas_basic::ArmasContextExt;
use egui_kittest::Harness;

/// Test that Breadcrumb renders without panicking
#[test]
fn test_breadcrumbs_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().show(ui, |bc| {
            bc.item("Home", None);
            bc.item("Products", None);
            bc.item("Electronics", None);
        });
    });

    harness.run();
}

/// Test Breadcrumb with single item
#[test]
fn test_breadcrumbs_single_item() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().show(ui, |bc| {
            bc.item("Home", None);
        });
    });

    harness.run();
}

/// Test Breadcrumb with current (non-clickable) item
#[test]
fn test_breadcrumbs_with_current() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().show(ui, |bc| {
            bc.item("Home", None);
            bc.item("Products", None);
            let _ = bc.item("Current Page", None).current();
        });
    });

    harness.run();
}

/// Test Breadcrumb with icons
#[test]
fn test_breadcrumbs_with_icons() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().show(ui, |bc| {
            bc.item("Home", Some("🏠"));
            bc.item("Settings", Some("⚙️"));
            let _ = bc.item("Profile", Some("👤")).current();
        });
    });

    harness.run();
}

/// Test Breadcrumb with custom spacing
#[test]
fn test_breadcrumbs_custom_spacing() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().spacing(8.0).show(ui, |bc| {
            bc.item("A", None);
            bc.item("B", None);
            bc.item("C", None);
        });
    });

    harness.run();
}

/// Test Breadcrumb with zero spacing
#[test]
fn test_breadcrumbs_zero_spacing() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().spacing(0.0).show(ui, |bc| {
            bc.item("One", None);
            bc.item("Two", None);
        });
    });

    harness.run();
}

/// Test Breadcrumb with many items
#[test]
fn test_breadcrumbs_many_items() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().show(ui, |bc| {
            bc.item("Root", None);
            bc.item("Level 1", None);
            bc.item("Level 2", None);
            bc.item("Level 3", None);
            bc.item("Level 4", None);
            let _ = bc.item("Current", None).current();
        });
    });

    harness.run();
}

/// Test Breadcrumb file path style
#[test]
fn test_breadcrumbs_file_path() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().show(ui, |bc| {
            bc.item("Users", Some("📁"));
            bc.item("john", Some("📁"));
            bc.item("Documents", Some("📁"));
            let _ = bc.item("report.pdf", Some("📄")).current();
        });
    });

    harness.run();
}

/// Test Breadcrumb website navigation style
#[test]
fn test_breadcrumbs_website_nav() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        Breadcrumb::new().show(ui, |bc| {
            bc.item("Home", Some("🏠"));
            bc.item("Shop", None);
            bc.item("Electronics", None);
            bc.item("Smartphones", None);
            let _ = bc.item("iPhone 15", None).current();
        });
    });

    harness.run();
}

/// Test Breadcrumb response (clicked field exists)
#[test]
fn test_breadcrumbs_response() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let response = Breadcrumb::new().show(ui, |bc| {
            bc.item("Home", None);
            let _ = bc.item("Page", None).current();
        });

        // Response should have clicked field (None since no click)
        assert!(response.clicked.is_none());
    });

    harness.run();
}
