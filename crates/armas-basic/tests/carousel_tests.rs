//! Tests for Carousel component using `egui_kittest`

use armas_basic::prelude::*;
use egui_kittest::Harness;

/// Test Carousel renders without panicking
#[test]
fn test_carousel_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let mut carousel = Carousel::new("test_carousel");
        carousel.show(ui, 3, |ui, index| {
            ui.label(format!("Slide {}", index + 1));
        });
    });

    harness.run();
}

/// Test Carousel with zero items
#[test]
fn test_carousel_empty() {
    let mut harness = Harness::new_ui(|ui| {
        let mut carousel = Carousel::new("empty_carousel");
        let response = carousel.show(ui, 0, |_ui, _index| {});
        assert_eq!(response.active_index, 0);
        assert!(!response.changed);
    });

    harness.run();
}

/// Test Carousel with single item
#[test]
fn test_carousel_single() {
    let mut harness = Harness::new_ui(|ui| {
        let mut carousel = Carousel::new("single_carousel");
        let response = carousel.show(ui, 1, |ui, _index| {
            ui.label("Only slide");
        });
        assert_eq!(response.active_index, 0);
    });

    harness.run();
}

/// Test Carousel with custom config
#[test]
fn test_carousel_custom_config() {
    let mut harness = Harness::new_ui(|ui| {
        let mut carousel = Carousel::new("custom_carousel")
            .item_basis(0.33)
            .gap(8.0)
            .height(300.0)
            .show_buttons(false);
        carousel.show(ui, 9, |ui, index| {
            ui.label(format!("Item {}", index));
        });
    });

    harness.run();
}

/// Test Carousel vertical orientation
#[test]
fn test_carousel_vertical() {
    let mut harness = Harness::new_ui(|ui| {
        let mut carousel =
            Carousel::new("vertical_carousel").orientation(CarouselOrientation::Vertical);
        carousel.show(ui, 5, |ui, index| {
            ui.label(format!("Row {}", index));
        });
    });

    harness.run();
}

/// Test Carousel loop mode
#[test]
fn test_carousel_loop() {
    let mut harness = Harness::new_ui(|ui| {
        let mut carousel = Carousel::new("loop_carousel").loop_mode(true);
        carousel.show(ui, 3, |ui, index| {
            ui.label(format!("Slide {}", index));
        });
    });

    harness.run();
}
