//! Tests for Tabs component using `egui_kittest`

use armas_basic::components::navigation::Tabs;
use armas_basic::ArmasContextExt;
use egui_kittest::Harness;

/// Test that Tabs renders without panicking
#[test]
fn test_tabs_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs = Tabs::new(vec!["Tab 1", "Tab 2", "Tab 3"]);
        tabs.show(ui, &theme);
    });

    harness.step();
}

/// Test Tabs with first tab active
#[test]
fn test_tabs_first_active() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs = Tabs::new(vec!["Home", "Profile", "Settings"]).active(0);
        tabs.show(ui, &theme);
    });

    harness.step();
}

/// Test Tabs with middle tab active
#[test]
fn test_tabs_middle_active() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs = Tabs::new(vec!["Home", "Profile", "Settings"]).active(1);
        tabs.show(ui, &theme);
    });

    harness.step();
}

/// Test Tabs with last tab active
#[test]
fn test_tabs_last_active() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs = Tabs::new(vec!["Home", "Profile", "Settings"]).active(2);
        tabs.show(ui, &theme);
    });

    harness.step();
}

/// Test Tabs with animation disabled
#[test]
fn test_tabs_no_animation() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs = Tabs::new(vec!["Tab 1", "Tab 2", "Tab 3"])
            .animate(false)
            .active(1);
        tabs.show(ui, &theme);
    });

    harness.run();
}

/// Test Tabs with many tabs
#[test]
fn test_tabs_many() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs =
            Tabs::new(vec!["Overview", "Analytics", "Reports", "Settings", "Help"]).active(2);
        tabs.show(ui, &theme);
    });

    harness.step();
}

/// Test Tabs with two tabs
#[test]
fn test_tabs_two() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs = Tabs::new(vec!["On", "Off"]);
        tabs.show(ui, &theme);
    });

    harness.step();
}

/// Test Tabs with single tab
#[test]
fn test_tabs_single() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs = Tabs::new(vec!["Only Tab"]);
        tabs.show(ui, &theme);
    });

    harness.step();
}

/// Test Tabs with empty labels (edge case)
#[test]
fn test_tabs_empty() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let mut tabs = Tabs::new(Vec::<String>::new());
        tabs.show(ui, &theme);
    });

    harness.run();
}

/// Test Tabs active index clamping (out of bounds)
#[test]
fn test_tabs_active_clamping() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        // Index 10 should clamp to 2 (last index)
        let mut tabs = Tabs::new(vec!["A", "B", "C"]).active(10);
        tabs.show(ui, &theme);
    });

    harness.step();
}
