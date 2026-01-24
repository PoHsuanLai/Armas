//! Tests for Browser component using egui_kittest

use armas::components::navigation::{Browser, BrowserItem};
use egui_kittest::Harness;
use std::path::PathBuf;

/// Test that Browser renders without panicking
#[test]
fn test_browser_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new();
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser with fixed dimensions
#[test]
fn test_browser_fixed_dimensions() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(300.0)
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser with items
#[test]
fn test_browser_with_items() {
    let mut harness = Harness::new_ui(|ui| {
        let items = vec![
            BrowserItem::folder("src", "/src"),
            BrowserItem::file("main.rs", "/src/main.rs"),
            BrowserItem::file("lib.rs", "/src/lib.rs"),
        ];

        let mut browser = Browser::new()
            .items(items)
            .width(300.0)
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser with nested folders
#[test]
fn test_browser_nested_folders() {
    let mut harness = Harness::new_ui(|ui| {
        let items = vec![
            BrowserItem::folder("src", "/src"),
            BrowserItem::folder("components", "/src/components"),
            BrowserItem::file("button.rs", "/src/components/button.rs"),
            BrowserItem::file("input.rs", "/src/components/input.rs"),
        ];

        let mut browser = Browser::new()
            .items(items)
            .width(300.0)
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test BrowserItem file creation
#[test]
fn test_browser_item_file() {
    let item = BrowserItem::file("test.txt", "/path/to/test.txt");
    assert_eq!(item.name, "test.txt");
    assert_eq!(item.path, PathBuf::from("/path/to/test.txt"));
    assert!(!item.is_directory);
}

/// Test BrowserItem folder creation
#[test]
fn test_browser_item_folder() {
    let item = BrowserItem::folder("docs", "/path/to/docs");
    assert_eq!(item.name, "docs");
    assert_eq!(item.path, PathBuf::from("/path/to/docs"));
    assert!(item.is_directory);
}

/// Test Browser getters
#[test]
fn test_browser_getters() {
    let browser = Browser::new();
    assert!(browser.selected().is_none());
}

/// Test Browser with empty items
#[test]
fn test_browser_empty_items() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .items(vec![])
            .width(300.0)
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser with custom root path
#[test]
fn test_browser_custom_root() {
    let mut harness = Harness::new_ui(|ui| {
        let items = vec![
            BrowserItem::folder("components", "/app/src/components"),
            BrowserItem::file("button.rs", "/app/src/components/button.rs"),
        ];

        let mut browser = Browser::new()
            .root_path("/app/src")
            .items(items)
            .width(300.0)
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser response
#[test]
fn test_browser_response() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(300.0)
            .height(400.0);
        let response = browser.show(ui);

        assert!(response.selected.is_none());
        assert!(response.toggled.is_none());
    });

    harness.run();
}
