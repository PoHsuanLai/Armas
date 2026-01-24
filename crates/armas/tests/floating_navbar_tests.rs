//! Tests for FloatingNavbar component using egui_kittest

use armas::components::navigation::{FloatingNavbar, NavbarPosition};
use egui_kittest::Harness;

/// Test that FloatingNavbar renders without panicking
#[test]
fn test_floating_navbar_renders() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new().show(ctx, |navbar| {
            navbar.item("Home", Some("🏠"));
            navbar.item("About", Some("ℹ️"));
            navbar.item("Contact", Some("📧"));
        });
    });

    // Use step() because navbar has animations
    harness.step();
}

/// Test FloatingNavbar with active item
#[test]
fn test_floating_navbar_active_item() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new().show(ctx, |navbar| {
            navbar.item("Home", Some("🏠")).active(true);
            navbar.item("About", Some("ℹ️"));
            navbar.item("Contact", Some("📧"));
        });
    });

    harness.step();
}

/// Test FloatingNavbar at top position (default)
#[test]
fn test_floating_navbar_position_top() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .position(NavbarPosition::Top)
            .show(ctx, |navbar| {
                navbar.item("Home", Some("🏠"));
                navbar.item("About", Some("ℹ️"));
            });
    });

    harness.step();
}

/// Test FloatingNavbar at bottom position
#[test]
fn test_floating_navbar_position_bottom() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .position(NavbarPosition::Bottom)
            .show(ctx, |navbar| {
                navbar.item("Home", Some("🏠"));
                navbar.item("About", Some("ℹ️"));
            });
    });

    harness.step();
}

/// Test FloatingNavbar with backdrop
#[test]
fn test_floating_navbar_with_backdrop() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .backdrop(true)
            .show(ctx, |navbar| {
                navbar.item("Home", Some("🏠"));
                navbar.item("About", Some("ℹ️"));
            });
    });

    harness.step();
}

/// Test FloatingNavbar without backdrop
#[test]
fn test_floating_navbar_no_backdrop() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .backdrop(false)
            .show(ctx, |navbar| {
                navbar.item("Home", Some("🏠"));
                navbar.item("About", Some("ℹ️"));
            });
    });

    harness.step();
}

/// Test FloatingNavbar with custom width
#[test]
fn test_floating_navbar_custom_width() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .width(600.0)
            .show(ctx, |navbar| {
                navbar.item("Home", Some("🏠"));
                navbar.item("About", Some("ℹ️"));
            });
    });

    harness.step();
}

/// Test FloatingNavbar with custom ID
#[test]
fn test_floating_navbar_custom_id() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .id("my_navbar")
            .show(ctx, |navbar| {
                navbar.item("Home", Some("🏠"));
                navbar.item("About", Some("ℹ️"));
            });
    });

    harness.step();
}

/// Test FloatingNavbar without icons
#[test]
fn test_floating_navbar_no_icons() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new().show(ctx, |navbar| {
            navbar.item("Home", None);
            navbar.item("About", None);
            navbar.item("Contact", None);
        });
    });

    harness.step();
}

/// Test FloatingNavbar with many items
#[test]
fn test_floating_navbar_many_items() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .width(900.0)
            .show(ctx, |navbar| {
                navbar.item("Home", Some("🏠")).active(true);
                navbar.item("Products", Some("📦"));
                navbar.item("Services", Some("🛠️"));
                navbar.item("About", Some("ℹ️"));
                navbar.item("Contact", Some("📧"));
            });
    });

    harness.step();
}

/// Test FloatingNavbar response
#[test]
fn test_floating_navbar_response() {
    let mut harness = Harness::new(|ctx| {
        let response = FloatingNavbar::new().show(ctx, |navbar| {
            navbar.item("Home", Some("🏠"));
            navbar.item("About", Some("ℹ️"));
        });

        // Check response fields exist
        let _ = response.clicked;
        let _ = response.hovered;
        let _ = response.close_clicked;
        let _ = response.backdrop_clicked;
    });

    harness.step();
}

/// Test FloatingNavbar full configuration
#[test]
fn test_floating_navbar_full_config() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .id("main_nav")
            .position(NavbarPosition::Top)
            .width(700.0)
            .backdrop(true)
            .show(ctx, |navbar| {
                navbar.item("Dashboard", Some("📊")).active(true);
                navbar.item("Analytics", Some("📈"));
                navbar.item("Reports", Some("📋"));
                navbar.item("Settings", Some("⚙️"));
            });
    });

    harness.step();
}

/// Test multiple FloatingNavbars with different IDs
#[test]
fn test_multiple_floating_navbars() {
    let mut harness = Harness::new(|ctx| {
        FloatingNavbar::new()
            .id("navbar_top")
            .position(NavbarPosition::Top)
            .show(ctx, |navbar| {
                navbar.item("Top 1", None);
                navbar.item("Top 2", None);
            });

        FloatingNavbar::new()
            .id("navbar_bottom")
            .position(NavbarPosition::Bottom)
            .show(ctx, |navbar| {
                navbar.item("Bottom 1", None);
                navbar.item("Bottom 2", None);
            });
    });

    harness.step();
}
