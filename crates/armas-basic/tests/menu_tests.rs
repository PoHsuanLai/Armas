//! Tests for Menu component using `egui_kittest`

use armas_basic::DropdownMenu;
use armas_basic::prelude::*;
use egui_kittest::Harness;

/// Test that Menu renders when open
#[test]
fn test_menu_renders_open() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("test_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.item("Option 1");
            menu.item("Option 2");
            menu.item("Option 3");
        });
    });

    harness.step();
}

/// Test Menu does not render when closed
#[test]
fn test_menu_closed() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("test_menu").open(false);

        menu.show(ctx, anchor_rect, |menu| {
            menu.item("Option 1");
        });
    });

    harness.run();
}

/// Test Menu with icons
#[test]
fn test_menu_with_icons() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("icon_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            let _ = menu.item("Copy").icon("📋");
            let _ = menu.item("Paste").icon("📄");
            let _ = menu.item("Cut").icon("✂️");
        });
    });

    harness.step();
}

/// Test Menu with shortcuts
#[test]
fn test_menu_with_shortcuts() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("shortcut_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            let _ = menu.item("Copy").shortcut("⌘C");
            let _ = menu.item("Paste").shortcut("⌘V");
            let _ = menu.item("Cut").shortcut("⌘X");
            let _ = menu.item("Undo").shortcut("⌘Z");
        });
    });

    harness.step();
}

/// Test Menu with icons and shortcuts
#[test]
fn test_menu_icons_and_shortcuts() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("full_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            let _ = menu.item("Copy").icon("📋").shortcut("⌘C");
            let _ = menu.item("Paste").icon("📄").shortcut("⌘V");
            let _ = menu.item("Cut").icon("✂️").shortcut("⌘X");
        });
    });

    harness.step();
}

/// Test Menu with separators
#[test]
fn test_menu_with_separators() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("sep_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.item("New File");
            menu.item("Open File");
            menu.separator();
            menu.item("Save");
            menu.item("Save As");
            menu.separator();
            menu.item("Exit");
        });
    });

    harness.step();
}

/// Test Menu with grouped items
#[test]
fn test_menu_with_groups() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("group_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.item("New");
            menu.item("Open");
            menu.separator();
            menu.item("Undo");
            menu.item("Redo");
        });
    });

    harness.step();
}

/// Test Menu with destructive item
#[test]
fn test_menu_destructive_item() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("destructive_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.item("Edit");
            menu.item("Duplicate");
            menu.separator();
            let _ = menu.item("Delete").destructive().shortcut("⌫");
        });
    });

    harness.step();
}

/// Test Menu with disabled items
#[test]
fn test_menu_disabled_items() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("disabled_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.item("Available");
            let _ = menu.item("Not Available").disabled(true);
            let _ = menu.item("Also Disabled").disabled(true);
        });
    });

    harness.step();
}

/// Test Menu with checkbox items
#[test]
fn test_menu_checkbox_items() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("checkbox_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.checkbox("Show Toolbar", true);
            menu.checkbox("Show Sidebar", false);
            menu.checkbox("Show Status Bar", true);
        });
    });

    harness.step();
}

/// Test Menu with radio items
#[test]
fn test_menu_radio_items() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("radio_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.radio("Light", "theme", "light", false);
            menu.radio("Dark", "theme", "dark", true);
            menu.radio("System", "theme", "system", false);
        });
    });

    harness.step();
}

/// Test Menu with custom width
#[test]
fn test_menu_custom_width() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("wide_menu").width(300.0).open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.item("This is a very long menu item text");
            menu.item("Another long item");
        });
    });

    harness.step();
}

/// Test Menu with different positions
#[test]
fn test_menu_position_bottom() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("bottom_menu")
            .position(PopoverPosition::Bottom)
            .open(true);

        menu.show(ctx, anchor_rect, |menu| {
            menu.item("Option 1");
            menu.item("Option 2");
        });
    });

    harness.step();
}

/// Test Menu with inset items
#[test]
fn test_menu_inset_items() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("inset_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            let _ = menu.item("With Icon").icon("📄");
            let _ = menu.item("No Icon (inset)").inset();
            let _ = menu.item("Another Icon").icon("📁");
        });
    });

    harness.step();
}

/// Test comprehensive context menu
#[test]
fn test_context_menu_comprehensive() {
    let anchor_rect = egui::Rect::from_min_size(egui::pos2(100.0, 100.0), egui::vec2(100.0, 32.0));

    let mut harness = Harness::new(|ctx| {
        let mut menu = DropdownMenu::new("context_menu").open(true);

        menu.show(ctx, anchor_rect, |menu| {
            let _ = menu.item("Open").icon("📂").shortcut("⌘O");
            let _ = menu.item("Edit").icon("✏️").shortcut("⌘E");
            let _ = menu.item("Duplicate").icon("📋").shortcut("⌘D");
            menu.separator();
            menu.checkbox("Show Preview", true);
            menu.checkbox("Show Details", false);
            menu.separator();
            menu.radio("Name", "sort", "name", true);
            menu.radio("Date", "sort", "date", false);
            menu.radio("Size", "sort", "size", false);
            menu.separator();
            let _ = menu.item("Delete").destructive().shortcut("⌫");
        });
    });

    harness.step();
}
