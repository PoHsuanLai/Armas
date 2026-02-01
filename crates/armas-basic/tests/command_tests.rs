//! Tests for Command component using `egui_kittest`

use armas_basic::components::navigation::Command;
use armas_basic::ArmasContextExt;
use egui_kittest::Harness;

/// Test that Command renders without panicking (closed by default)
#[test]
fn test_command_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new();
        menu.show(ui, |cmd| {
            cmd.item("cmd1", "Command 1");
            cmd.item("cmd2", "Command 2");
        });
    });

    harness.run();
}

/// Test Command with custom placeholder
#[test]
fn test_command_placeholder() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new().placeholder("Search commands...");
        menu.show(ui, |cmd| {
            cmd.item("test", "Test Command");
        });
    });

    harness.run();
}

/// Test Command with groups
#[test]
fn test_command_groups() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new();
        menu.show(ui, |cmd| {
            cmd.group("File");
            cmd.item("new", "New File");
            cmd.item("open", "Open File");
            cmd.group("Edit");
            cmd.item("copy", "Copy");
            cmd.item("paste", "Paste");
        });
    });

    harness.run();
}

/// Test Command with icons
#[test]
fn test_command_icons() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new();
        menu.show(ui, |cmd| {
            let _ = cmd.item("new", "New File").icon("📄");
            let _ = cmd.item("save", "Save").icon("💾");
            let _ = cmd.item("settings", "Settings").icon("⚙️");
        });
    });

    harness.run();
}

/// Test Command with shortcuts
#[test]
fn test_command_shortcuts() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new();
        menu.show(ui, |cmd| {
            let _ = cmd.item("copy", "Copy").shortcut("⌘C");
            let _ = cmd.item("paste", "Paste").shortcut("⌘V");
            let _ = cmd.item("undo", "Undo").shortcut("⌘Z");
        });
    });

    harness.run();
}

/// Test Command with full configuration
#[test]
fn test_command_full_config() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new();
        menu.show(ui, |cmd| {
            cmd.group("File Operations");
            let _ = cmd.item("new", "New File").icon("📄").shortcut("⌘N");
            let _ = cmd.item("open", "Open File").icon("📂").shortcut("⌘O");
            let _ = cmd.item("save", "Save").icon("💾").shortcut("⌘S");

            cmd.separator();

            cmd.group("Edit");
            let _ = cmd.item("undo", "Undo").icon("↩️").shortcut("⌘Z");
            let _ = cmd.item("redo", "Redo").icon("↪️").shortcut("⌘⇧Z");
        });
    });

    harness.run();
}

/// Test Command with custom trigger key
#[test]
fn test_command_custom_trigger() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new().trigger(egui::Key::P, egui::Modifiers::COMMAND);
        menu.show(ui, |cmd| {
            cmd.item("cmd", "Command");
        });
    });

    harness.run();
}

/// Test Command response
#[test]
fn test_command_response() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new();
        let response = menu.show(ui, |cmd| {
            cmd.item("test", "Test");
        });

        // Response should have executed field (None since menu is closed)
        assert!(response.executed.is_none());
    });

    harness.run();
}

/// Test Command with many items
#[test]
fn test_command_many_items() {
    let mut harness = Harness::new_ui(|ui| {
        let _theme = ui.ctx().armas_theme();
        let mut menu = Command::new();
        menu.show(ui, |cmd| {
            for i in 1..=20 {
                cmd.item(&format!("cmd_{i}"), &format!("Command {i}"));
            }
        });
    });

    harness.run();
}
