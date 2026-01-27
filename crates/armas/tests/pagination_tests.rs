//! Tests for Pagination component using egui_kittest

use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Pagination renders without panicking
#[test]
fn test_pagination_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Pagination::new(1, 10).show(ui, &theme);
    });

    harness.run();
}

/// Test Pagination on first page
#[test]
fn test_pagination_first_page() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let (_, page) = Pagination::new(1, 10).show(ui, &theme);
        assert_eq!(page, 1);
    });

    harness.run();
}

/// Test Pagination on last page
#[test]
fn test_pagination_last_page() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let (_, page) = Pagination::new(10, 10).show(ui, &theme);
        assert_eq!(page, 10);
    });

    harness.run();
}

/// Test Pagination in the middle
#[test]
fn test_pagination_middle_page() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let (_, page) = Pagination::new(5, 10).show(ui, &theme);
        assert_eq!(page, 5);
    });

    harness.run();
}

/// Test Pagination with single page
#[test]
fn test_pagination_single_page() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Pagination::new(1, 1).show(ui, &theme);
    });

    harness.run();
}

/// Test Pagination with few pages (no ellipsis needed)
#[test]
fn test_pagination_few_pages() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Pagination::new(2, 5).show(ui, &theme);
    });

    harness.run();
}

/// Test Pagination with many pages (ellipsis shown)
#[test]
fn test_pagination_many_pages() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Pagination::new(50, 100).show(ui, &theme);
    });

    harness.run();
}

/// Test Pagination without prev/next buttons
#[test]
fn test_pagination_no_prev_next() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Pagination::new(3, 10)
            .show_prev_next(false)
            .show(ui, &theme);
    });

    harness.run();
}

/// Test Pagination with custom sibling count
#[test]
fn test_pagination_sibling_count() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Pagination::new(10, 20).sibling_count(2).show(ui, &theme);
    });

    harness.run();
}

/// Test Pagination clamping initial page to valid range
#[test]
fn test_pagination_clamps_initial_page() {
    // Page 0 should clamp to 1
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let (_, page) = Pagination::new(0, 10).show(ui, &theme);
        assert_eq!(page, 1);
    });
    harness.run();

    // Page > total should clamp to total
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        let (_, page) = Pagination::new(100, 10).show(ui, &theme);
        assert_eq!(page, 10);
    });
    harness.run();
}

/// Test edge case: page near start with ellipsis
#[test]
fn test_pagination_near_start() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Pagination::new(2, 20).show(ui, &theme);
    });

    harness.run();
}

/// Test edge case: page near end with ellipsis
#[test]
fn test_pagination_near_end() {
    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        Pagination::new(19, 20).show(ui, &theme);
    });

    harness.run();
}
