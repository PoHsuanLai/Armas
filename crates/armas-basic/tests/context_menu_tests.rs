//! Tests for `ContextMenu` component using `egui_kittest`

use armas_basic::ContextMenu;
use egui_kittest::Harness;

/// Test that `ContextMenu` renders without panicking when closed
#[test]
fn test_context_menu_closed() {
    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.allocate_response(egui::vec2(200.0, 100.0), egui::Sense::click());
            let mut menu = ContextMenu::new("test_ctx_menu");
            menu.show(ui.ctx(), &response, |menu| {
                menu.item("Cut");
                menu.item("Copy");
                menu.item("Paste");
            });
        });
    });

    harness.run();
}

/// Test `ContextMenu` with custom width
#[test]
fn test_context_menu_custom_width() {
    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.allocate_response(egui::vec2(200.0, 100.0), egui::Sense::click());
            let mut menu = ContextMenu::new("wide_ctx_menu").width(300.0);
            menu.show(ui.ctx(), &response, |menu| {
                menu.item("Option 1");
                menu.item("Option 2");
            });
        });
    });

    harness.run();
}

/// Test `ContextMenu` with separators and destructive items
#[test]
fn test_context_menu_with_separators() {
    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.allocate_response(egui::vec2(200.0, 100.0), egui::Sense::click());
            let mut menu = ContextMenu::new("sep_ctx_menu");
            menu.show(ui.ctx(), &response, |menu| {
                menu.item("Edit");
                menu.item("Duplicate");
                menu.separator();
                let _ = menu.item("Delete").destructive();
            });
        });
    });

    harness.run();
}

/// Test `ContextMenu` with shortcuts and icons
#[test]
fn test_context_menu_full() {
    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.allocate_response(egui::vec2(200.0, 100.0), egui::Sense::click());
            let mut menu = ContextMenu::new("full_ctx_menu");
            menu.show(ui.ctx(), &response, |menu| {
                let _ = menu.item("Cut").icon("✂️").shortcut("⌘X");
                let _ = menu.item("Copy").icon("📋").shortcut("⌘C");
                let _ = menu.item("Paste").icon("📄").shortcut("⌘V");
                menu.separator();
                menu.checkbox("Show Hidden", false);
                menu.separator();
                let _ = menu.item("Delete").destructive().shortcut("⌫");
            });
        });
    });

    harness.run();
}
