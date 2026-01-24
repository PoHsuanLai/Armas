//! Tests for AnimatedTabs component using egui_kittest

use armas::components::navigation::{AnimatedTabs, TabStyle};
use egui_kittest::Harness;

/// Test that Tabs renders without panicking
#[test]
fn test_tabs_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2", "Tab 3"]);
        tabs.show(ui);
    });

    // Use step() because tabs have animation
    harness.step();
}

/// Test Tabs with first tab active
#[test]
fn test_tabs_first_active() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Home", "Profile", "Settings"])
            .active(0);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with middle tab active
#[test]
fn test_tabs_middle_active() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Home", "Profile", "Settings"])
            .active(1);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with last tab active
#[test]
fn test_tabs_last_active() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Home", "Profile", "Settings"])
            .active(2);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with Underline style (default)
#[test]
fn test_tabs_underline_style() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2"])
            .style(TabStyle::Underline);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with Pill style
#[test]
fn test_tabs_pill_style() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2"])
            .style(TabStyle::Pill);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with Segment style
#[test]
fn test_tabs_segment_style() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2"])
            .style(TabStyle::Segment);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with animation disabled
#[test]
fn test_tabs_no_animation() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2", "Tab 3"])
            .animate(false)
            .active(1);
        tabs.show(ui);
    });

    // Can use run() when animation is disabled
    harness.run();
}

/// Test Tabs with many tabs
#[test]
fn test_tabs_many() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec![
            "Overview", "Analytics", "Reports", "Settings", "Help"
        ]).active(2);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with two tabs
#[test]
fn test_tabs_two() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["On", "Off"]);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with single tab
#[test]
fn test_tabs_single() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Only Tab"]);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs with empty labels (edge case)
#[test]
fn test_tabs_empty() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(Vec::<String>::new());
        tabs.show(ui);
    });

    harness.run();
}

/// Test Tabs Underline style with active middle
#[test]
fn test_tabs_underline_middle_active() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["First", "Second", "Third", "Fourth"])
            .style(TabStyle::Underline)
            .active(2);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs Pill style with active
#[test]
fn test_tabs_pill_active() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Photos", "Videos", "Documents"])
            .style(TabStyle::Pill)
            .active(1);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs Segment style with active
#[test]
fn test_tabs_segment_active() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Day", "Week", "Month", "Year"])
            .style(TabStyle::Segment)
            .active(2);
        tabs.show(ui);
    });

    harness.step();
}

/// Test Tabs active index clamping (out of bounds)
#[test]
fn test_tabs_active_clamping() {
    let mut harness = Harness::new_ui(|ui| {
        // Index 10 should clamp to 2 (last index)
        let mut tabs = AnimatedTabs::new(vec!["A", "B", "C"])
            .active(10);
        tabs.show(ui);
    });

    harness.step();
}

/// Test all tab styles in sequence
#[test]
fn test_tabs_all_styles() {
    // Underline
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2"])
            .style(TabStyle::Underline)
            .animate(false);
        tabs.show(ui);
    });
    harness.run();

    // Pill
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2"])
            .style(TabStyle::Pill)
            .animate(false);
        tabs.show(ui);
    });
    harness.run();

    // Segment
    let mut harness = Harness::new_ui(|ui| {
        let mut tabs = AnimatedTabs::new(vec!["Tab 1", "Tab 2"])
            .style(TabStyle::Segment)
            .animate(false);
        tabs.show(ui);
    });
    harness.run();
}
