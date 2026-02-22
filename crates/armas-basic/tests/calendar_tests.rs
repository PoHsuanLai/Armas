//! Tests for Calendar component using `egui_kittest`

use armas_basic::prelude::*;
use armas_basic::Date;
use egui_kittest::Harness;

/// Test Calendar renders without panicking
#[test]
fn test_calendar_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let mut selected = None;
        let mut calendar = Calendar::new("test_cal");
        calendar.show(ui, &mut selected);
    });

    harness.run();
}

/// Test Calendar with pre-selected date
#[test]
fn test_calendar_with_selected() {
    let mut harness = Harness::new_ui(|ui| {
        let mut selected = Date::new(2024, 6, 15);
        let mut calendar = Calendar::new("test_cal_sel");
        calendar.show(ui, &mut selected);
    });

    harness.run();
}

/// Test Calendar with footer
#[test]
fn test_calendar_with_footer() {
    let mut harness = Harness::new_ui(|ui| {
        let mut selected = None;
        let mut calendar = Calendar::new("test_cal_foot").show_footer(true);
        calendar.show(ui, &mut selected);
    });

    harness.run();
}

/// Test Calendar without outside days
#[test]
fn test_calendar_no_outside_days() {
    let mut harness = Harness::new_ui(|ui| {
        let mut selected = None;
        let mut calendar = Calendar::new("test_cal_no_outside").show_outside_days(false);
        calendar.show(ui, &mut selected);
    });

    harness.run();
}
