//! Tests for `HoverCard` component using `egui_kittest`

use armas_basic::prelude::*;
use egui_kittest::Harness;

/// Test `HoverCard` renders without panicking (not hovered = closed)
#[test]
fn test_hover_card_closed() {
    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let trigger = ui.button("Hover me");
            let mut card = HoverCard::new("test_hover");
            card.show(ui.ctx(), &trigger, |ui| {
                ui.label("Card content");
            });
        });
    });

    harness.run();
}

/// Test `HoverCard` with custom delays
#[test]
fn test_hover_card_custom_delays() {
    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let trigger = ui.button("Hover");
            let mut card = HoverCard::new("delay_test")
                .open_delay(0.5)
                .close_delay(0.2);
            card.show(ui.ctx(), &trigger, |ui| {
                ui.label("Quick card");
            });
        });
    });

    harness.run();
}

/// Test `HoverCard` with custom width and position
#[test]
fn test_hover_card_custom_config() {
    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let trigger = ui.button("User");
            let mut card = HoverCard::new("config_test")
                .width(300.0)
                .position(PopoverPosition::Right);
            card.show(ui.ctx(), &trigger, |ui| {
                ui.label("User profile preview");
                ui.label("@username");
            });
        });
    });

    harness.run();
}

/// Test `HoverCard` response has `is_open` field
#[test]
fn test_hover_card_response() {
    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let trigger = ui.button("Info");
            let mut card = HoverCard::new("response_test");
            let response = card.show(ui.ctx(), &trigger, |ui| {
                ui.label("Details here");
            });
            // Not hovered, so should be closed
            assert!(!response.is_open);
        });
    });

    harness.run();
}
