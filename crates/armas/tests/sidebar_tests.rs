//! Tests for Sidebar component using egui_kittest

use armas::components::navigation::{CollapsibleMode, Sidebar};
use egui_kittest::Harness;

/// Test that Sidebar renders without panicking
#[test]
fn test_sidebar_renders() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home");
            sidebar.item("📧", "Messages");
            sidebar.item("⚙️", "Settings");
        });
    });

    // Use step() because sidebar has animations
    harness.step();
}

/// Test Sidebar expanded state (default)
#[test]
fn test_sidebar_expanded() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().collapsed(false).show(ui, |sidebar| {
            sidebar.item("🏠", "Home");
            sidebar.item("📧", "Messages");
        });
    });

    harness.step();
}

/// Test Sidebar collapsed state
#[test]
fn test_sidebar_collapsed() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().collapsed(true).show(ui, |sidebar| {
            sidebar.item("🏠", "Home");
            sidebar.item("📧", "Messages");
        });
    });

    harness.step();
}

/// Test Sidebar with active item
#[test]
fn test_sidebar_active_item() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home").active(true);
            sidebar.item("📧", "Messages");
            sidebar.item("⚙️", "Settings");
        });
    });

    harness.step();
}

/// Test Sidebar with badge
#[test]
fn test_sidebar_with_badge() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home");
            sidebar.item("📧", "Messages").badge("5");
            sidebar.item("🔔", "Notifications").badge("12");
        });
    });

    harness.step();
}

/// Test Sidebar with active item and badge
#[test]
fn test_sidebar_active_with_badge() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home");
            sidebar.item("📧", "Messages").active(true).badge("3");
            sidebar.item("⚙️", "Settings");
        });
    });

    harness.step();
}

/// Test Sidebar with groups
#[test]
fn test_sidebar_with_groups() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home").active(true);

            sidebar.group("⚙️", "Settings", |group| {
                group.item("👤", "Profile");
                group.item("🔒", "Privacy");
                group.item("🎨", "Appearance");
            });
        });
    });

    harness.step();
}

/// Test Sidebar with nested groups
#[test]
fn test_sidebar_nested_groups() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home");

            sidebar.group("📁", "Projects", |projects| {
                projects.item("📄", "Project A");
                projects.item("📄", "Project B");
            });
        });
    });

    harness.step();
}

/// Test Sidebar custom collapsed width
#[test]
fn test_sidebar_custom_collapsed_width() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new()
            .collapsed_width(50.0)
            .collapsed(true)
            .show(ui, |sidebar| {
                sidebar.item("🏠", "Home");
                sidebar.item("📧", "Messages");
            });
    });

    harness.step();
}

/// Test Sidebar custom expanded width
#[test]
fn test_sidebar_custom_expanded_width() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().expanded_width(300.0).show(ui, |sidebar| {
            sidebar.item("🏠", "Home");
            sidebar.item("📧", "Messages");
        });
    });

    harness.step();
}

/// Test Sidebar non-collapsible
#[test]
fn test_sidebar_not_collapsible() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new()
            .collapsible(CollapsibleMode::None)
            .show(ui, |sidebar| {
                sidebar.item("🏠", "Home");
                sidebar.item("📧", "Messages");
            });
    });

    harness.step();
}

/// Test Sidebar without icons
#[test]
fn test_sidebar_no_icons() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show_icons(false).show(ui, |sidebar| {
            sidebar.item("", "Home");
            sidebar.item("", "Messages");
            sidebar.item("", "Settings");
        });
    });

    harness.step();
}

/// Test Sidebar with many items
#[test]
fn test_sidebar_many_items() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home").active(true);
            sidebar.item("📧", "Messages").badge("5");
            sidebar.item("📅", "Calendar");
            sidebar.item("📊", "Analytics");
            sidebar.item("📁", "Files");
            sidebar.item("👥", "Team");
            sidebar.item("⚙️", "Settings");
            sidebar.item("❓", "Help");
        });
    });

    harness.step();
}

/// Test Sidebar single item
#[test]
fn test_sidebar_single_item() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home");
        });
    });

    harness.step();
}

/// Test Sidebar response fields
#[test]
fn test_sidebar_response() {
    let mut harness = Harness::new_ui(|ui| {
        let response = Sidebar::new().show(ui, |sidebar| {
            sidebar.item("🏠", "Home");
            sidebar.item("📧", "Messages");
        });

        // Check response fields exist
        let _ = response.clicked;
        let _ = response.hovered;
        let _ = response.is_expanded;
    });

    harness.step();
}

/// Test Sidebar full configuration
#[test]
fn test_sidebar_full_config() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new()
            .collapsed(false)
            .collapsed_width(60.0)
            .expanded_width(250.0)
            .collapsible(CollapsibleMode::Icon)
            .show_icons(true)
            .show(ui, |sidebar| {
                sidebar.item("🏠", "Dashboard").active(true);
                sidebar.item("📧", "Inbox").badge("12");

                sidebar.group("📁", "Projects", |group| {
                    group.item("📄", "Active");
                    group.item("📄", "Archived");
                });

                sidebar.item("⚙️", "Settings");
            });
    });

    harness.step();
}

/// Test Sidebar app-like layout
#[test]
fn test_sidebar_app_layout() {
    let mut harness = Harness::new_ui(|ui| {
        Sidebar::new()
            .collapsed_width(70.0)
            .expanded_width(240.0)
            .show(ui, |sidebar| {
                // Main navigation
                sidebar.item("🏠", "Home").active(true);
                sidebar.item("🔍", "Search");
                sidebar.item("📊", "Dashboard");

                // Notifications
                sidebar.item("🔔", "Notifications").badge("3");
                sidebar.item("📧", "Messages").badge("7");

                // Groups
                sidebar.group("⚙️", "Settings", |settings| {
                    settings.item("👤", "Account");
                    settings.item("🔐", "Security");
                    settings.item("🎨", "Theme");
                });
            });
    });

    harness.step();
}
