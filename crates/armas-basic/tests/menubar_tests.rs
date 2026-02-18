//! Tests for Menubar component using `egui_kittest`

use armas_basic::prelude::*;
use egui_kittest::Harness;

/// Test Menubar renders without panicking
#[test]
fn test_menubar_renders() {
    let mut harness = Harness::new_ui(|ui| {
        Menubar::new("test_bar").show(ui, |bar| {
            bar.menu("File", |menu| {
                menu.item("New");
                menu.item("Open");
                menu.separator();
                menu.item("Exit");
            });
            bar.menu("Edit", |menu| {
                let _ = menu.item("Undo").shortcut("⌘Z");
                let _ = menu.item("Redo").shortcut("⇧⌘Z");
            });
        });
    });

    harness.run();
}

/// Test Menubar with single menu
#[test]
fn test_menubar_single_menu() {
    let mut harness = Harness::new_ui(|ui| {
        Menubar::new("single_bar").show(ui, |bar| {
            bar.menu("File", |menu| {
                menu.item("New");
            });
        });
    });

    harness.run();
}

/// Test Menubar with many menus
#[test]
fn test_menubar_many_menus() {
    let mut harness = Harness::new_ui(|ui| {
        Menubar::new("many_bar").show(ui, |bar| {
            bar.menu("File", |menu| {
                menu.item("New");
                menu.item("Open");
            });
            bar.menu("Edit", |menu| {
                let _ = menu.item("Undo").shortcut("⌘Z");
                let _ = menu.item("Cut").shortcut("⌘X");
                let _ = menu.item("Copy").shortcut("⌘C");
                let _ = menu.item("Paste").shortcut("⌘V");
            });
            bar.menu("View", |menu| {
                menu.checkbox("Show Toolbar", true);
                menu.checkbox("Show Sidebar", false);
            });
            bar.menu("Help", |menu| {
                menu.item("About");
            });
        });
    });

    harness.run();
}

/// Test Menubar with submenus
#[test]
fn test_menubar_with_submenus() {
    let mut harness = Harness::new_ui(|ui| {
        Menubar::new("sub_bar").show(ui, |bar| {
            bar.menu("File", |menu| {
                menu.item("New");
                menu.submenu("Recent Files", |sub| {
                    sub.item("doc.txt");
                    sub.item("image.png");
                });
                menu.separator();
                menu.item("Exit");
            });
        });
    });

    harness.run();
}
