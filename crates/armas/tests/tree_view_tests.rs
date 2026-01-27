//! Tests for TreeView component using egui_kittest

use armas::components::navigation::{TreeItem, TreeView};
use egui_kittest::Harness;
use std::path::PathBuf;

/// Test that TreeView renders without panicking
#[test]
fn test_tree_view_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tree = TreeView::new();
        tree.show(ui);
    });

    harness.run();
}

/// Test TreeView with fixed dimensions
#[test]
fn test_tree_view_fixed_dimensions() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tree = TreeView::new().width(300.0).height(400.0);
        tree.show(ui);
    });

    harness.run();
}

/// Test TreeView with items
#[test]
fn test_tree_view_with_items() {
    let mut harness = Harness::new_ui(|ui| {
        let items = vec![
            TreeItem::folder("src", "/src"),
            TreeItem::file("main.rs", "/src/main.rs"),
            TreeItem::file("lib.rs", "/src/lib.rs"),
        ];

        let mut tree = TreeView::new().items(items).width(300.0).height(400.0);
        tree.show(ui);
    });

    harness.run();
}

/// Test TreeView with nested folders
#[test]
fn test_tree_view_nested_folders() {
    let mut harness = Harness::new_ui(|ui| {
        let items = vec![
            TreeItem::folder("src", "/src"),
            TreeItem::folder("components", "/src/components"),
            TreeItem::file("button.rs", "/src/components/button.rs"),
            TreeItem::file("input.rs", "/src/components/input.rs"),
        ];

        let mut tree = TreeView::new().items(items).width(300.0).height(400.0);
        tree.show(ui);
    });

    harness.run();
}

/// Test TreeItem leaf creation
#[test]
fn test_tree_item_leaf() {
    let item = TreeItem::leaf("test.txt", "/path/to/test.txt");
    assert_eq!(item.name, "test.txt");
    assert_eq!(item.path, PathBuf::from("/path/to/test.txt"));
    assert!(!item.is_directory);
}

/// Test TreeItem branch creation
#[test]
fn test_tree_item_branch() {
    let item = TreeItem::branch("docs", "/path/to/docs");
    assert_eq!(item.name, "docs");
    assert_eq!(item.path, PathBuf::from("/path/to/docs"));
    assert!(item.is_directory);
}

/// Test TreeItem file alias
#[test]
fn test_tree_item_file() {
    let item = TreeItem::file("test.txt", "/path/to/test.txt");
    assert_eq!(item.name, "test.txt");
    assert!(!item.is_directory);
}

/// Test TreeItem folder alias
#[test]
fn test_tree_item_folder() {
    let item = TreeItem::folder("docs", "/path/to/docs");
    assert_eq!(item.name, "docs");
    assert!(item.is_directory);
}

/// Test TreeView getters
#[test]
fn test_tree_view_getters() {
    let tree = TreeView::new();
    assert!(tree.selected().is_none());
}

/// Test TreeView with empty items
#[test]
fn test_tree_view_empty_items() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tree = TreeView::new().items(vec![]).width(300.0).height(400.0);
        tree.show(ui);
    });

    harness.run();
}

/// Test TreeView with custom root path
#[test]
fn test_tree_view_custom_root() {
    let mut harness = Harness::new_ui(|ui| {
        let items = vec![
            TreeItem::folder("components", "/app/src/components"),
            TreeItem::file("button.rs", "/app/src/components/button.rs"),
        ];

        let mut tree = TreeView::new()
            .root_path("/app/src")
            .items(items)
            .width(300.0)
            .height(400.0);
        tree.show(ui);
    });

    harness.run();
}

/// Test TreeView response
#[test]
fn test_tree_view_response() {
    let mut harness = Harness::new_ui(|ui| {
        let mut tree = TreeView::new().width(300.0).height(400.0);
        let response = tree.show(ui);

        assert!(response.selected.is_none());
        assert!(response.toggled.is_none());
    });

    harness.run();
}
