//! Tests for Resizable component using `egui_kittest`

use armas_basic::prelude::*;
use egui_kittest::Harness;

/// Test Resizable renders without panicking
#[test]
fn test_resizable_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let panels = vec![ResizablePanel::new(0.5), ResizablePanel::new(0.5)];
        let mut resizable = Resizable::new("test_resize", ResizableDirection::Horizontal);
        resizable.show(ui, &panels, |ui, index| {
            ui.label(format!("Panel {}", index + 1));
        });
    });

    harness.run();
}

/// Test Resizable with empty panels
#[test]
fn test_resizable_empty() {
    let mut harness = Harness::new_ui(|ui| {
        let panels: Vec<ResizablePanel> = vec![];
        let mut resizable = Resizable::new("empty_resize", ResizableDirection::Horizontal);
        let response = resizable.show(ui, &panels, |_ui, _index| {});
        assert!(response.sizes.is_empty());
        assert!(!response.changed);
    });

    harness.run();
}

/// Test Resizable with single panel
#[test]
fn test_resizable_single_panel() {
    let mut harness = Harness::new_ui(|ui| {
        let panels = vec![ResizablePanel::new(1.0)];
        let mut resizable = Resizable::new("single_resize", ResizableDirection::Horizontal);
        resizable.show(ui, &panels, |ui, _index| {
            ui.label("Only panel");
        });
    });

    harness.run();
}

/// Test Resizable vertical
#[test]
fn test_resizable_vertical() {
    let mut harness = Harness::new_ui(|ui| {
        let panels = vec![
            ResizablePanel::new(0.3),
            ResizablePanel::new(0.4),
            ResizablePanel::new(0.3),
        ];
        let mut resizable = Resizable::new("vert_resize", ResizableDirection::Vertical);
        resizable.show(ui, &panels, |ui, index| {
            ui.label(format!("Row {}", index + 1));
        });
    });

    harness.run();
}

/// Test Resizable with min/max constraints
#[test]
fn test_resizable_constraints() {
    let mut harness = Harness::new_ui(|ui| {
        let panels = vec![
            ResizablePanel::new(0.3).min_size(0.2).max_size(0.5),
            ResizablePanel::new(0.7).min_size(0.3),
        ];
        let mut resizable = Resizable::new("constrained", ResizableDirection::Horizontal);
        resizable.show(ui, &panels, |ui, index| {
            ui.label(format!("Panel {}", index + 1));
        });
    });

    harness.run();
}

/// Test Resizable three panels
#[test]
fn test_resizable_three_panels() {
    let mut harness = Harness::new_ui(|ui| {
        let panels = vec![
            ResizablePanel::new(0.25),
            ResizablePanel::new(0.5),
            ResizablePanel::new(0.25),
        ];
        let mut resizable = Resizable::new("three_panels", ResizableDirection::Horizontal);
        let response = resizable.show(ui, &panels, |ui, index| {
            match index {
                0 => ui.label("Sidebar"),
                1 => ui.label("Main Content"),
                2 => ui.label("Inspector"),
                _ => ui.label("?"),
            };
        });
        assert_eq!(response.sizes.len(), 3);
    });

    harness.run();
}
