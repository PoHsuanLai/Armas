//! Tests for DatePicker component using egui_kittest

use armas::components::basic::{Date, DatePicker};
use armas::prelude::*;
use egui_kittest::Harness;

/// Test Date creation with valid values
#[test]
fn test_date_creation_valid() {
    let date = Date::new(2024, 6, 15);
    assert!(date.is_some());

    let date = date.unwrap();
    assert_eq!(date.year, 2024);
    assert_eq!(date.month, 6);
    assert_eq!(date.day, 15);
}

/// Test Date creation with invalid month
#[test]
fn test_date_creation_invalid_month() {
    assert!(Date::new(2024, 0, 15).is_none());
    assert!(Date::new(2024, 13, 15).is_none());
}

/// Test Date creation with invalid day
#[test]
fn test_date_creation_invalid_day() {
    assert!(Date::new(2024, 1, 0).is_none());
    assert!(Date::new(2024, 1, 32).is_none());
    assert!(Date::new(2024, 4, 31).is_none()); // April has 30 days
}

/// Test Date leap year detection
#[test]
fn test_date_leap_year() {
    assert!(Date::is_leap_year(2024)); // Divisible by 4
    assert!(!Date::is_leap_year(2023)); // Not divisible by 4
    assert!(!Date::is_leap_year(1900)); // Divisible by 100 but not 400
    assert!(Date::is_leap_year(2000)); // Divisible by 400
}

/// Test Date days in month
#[test]
fn test_date_days_in_month() {
    // 31-day months
    assert_eq!(Date::days_in_month(2024, 1), 31);
    assert_eq!(Date::days_in_month(2024, 3), 31);
    assert_eq!(Date::days_in_month(2024, 5), 31);
    assert_eq!(Date::days_in_month(2024, 7), 31);
    assert_eq!(Date::days_in_month(2024, 8), 31);
    assert_eq!(Date::days_in_month(2024, 10), 31);
    assert_eq!(Date::days_in_month(2024, 12), 31);

    // 30-day months
    assert_eq!(Date::days_in_month(2024, 4), 30);
    assert_eq!(Date::days_in_month(2024, 6), 30);
    assert_eq!(Date::days_in_month(2024, 9), 30);
    assert_eq!(Date::days_in_month(2024, 11), 30);

    // February
    assert_eq!(Date::days_in_month(2024, 2), 29); // Leap year
    assert_eq!(Date::days_in_month(2023, 2), 28); // Not leap year
}

/// Test Date formatting
#[test]
fn test_date_format() {
    let date = Date::new(2024, 6, 15).unwrap();
    assert_eq!(date.format(), "2024-06-15");

    let date = Date::new(2024, 1, 5).unwrap();
    assert_eq!(date.format(), "2024-01-05");
}

/// Test Date parsing
#[test]
fn test_date_parse() {
    let date = Date::parse("2024-06-15");
    assert!(date.is_some());

    let date = date.unwrap();
    assert_eq!(date.year, 2024);
    assert_eq!(date.month, 6);
    assert_eq!(date.day, 15);
}

/// Test Date parsing invalid formats
#[test]
fn test_date_parse_invalid() {
    assert!(Date::parse("2024/06/15").is_none());
    assert!(Date::parse("06-15-2024").is_none());
    assert!(Date::parse("invalid").is_none());
    assert!(Date::parse("2024-13-15").is_none()); // Invalid month
}

/// Test Date month name
#[test]
fn test_date_month_name() {
    assert_eq!(Date::new(2024, 1, 1).unwrap().month_name(), "January");
    assert_eq!(Date::new(2024, 6, 1).unwrap().month_name(), "June");
    assert_eq!(Date::new(2024, 12, 1).unwrap().month_name(), "December");
}

/// Test Date day of week
#[test]
fn test_date_day_of_week() {
    // 2024-01-01 was a Monday (1)
    let date = Date::new(2024, 1, 1).unwrap();
    assert_eq!(date.day_of_week(), 1);

    // 2024-06-15 was a Saturday (6)
    let date = Date::new(2024, 6, 15).unwrap();
    assert_eq!(date.day_of_week(), 6);
}

/// Test DatePicker renders
#[test]
fn test_datepicker_renders() {
    let theme = Theme::dark();
    let mut selected_date = None;

    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut picker = DatePicker::new("test_picker");
            picker.show(ctx, &theme, ui, &mut selected_date);
        });
    });

    harness.run();
}

/// Test DatePicker with pre-selected date
#[test]
fn test_datepicker_with_selection() {
    let theme = Theme::dark();
    let mut selected_date = Date::new(2024, 6, 15);

    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut picker = DatePicker::new("test_picker");
            picker.show(ctx, &theme, ui, &mut selected_date);
        });
    });

    harness.run();
}

/// Test DatePicker with label
#[test]
fn test_datepicker_with_label() {
    let theme = Theme::dark();
    let mut selected_date = None;

    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut picker = DatePicker::new("labeled_picker").label("Birth Date");
            picker.show(ctx, &theme, ui, &mut selected_date);
        });
    });

    harness.run();
}

/// Test DatePicker with custom placeholder
#[test]
fn test_datepicker_with_placeholder() {
    let theme = Theme::dark();
    let mut selected_date = None;

    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut picker = DatePicker::new("placeholder_picker").placeholder("Choose a date...");
            picker.show(ctx, &theme, ui, &mut selected_date);
        });
    });

    harness.run();
}

/// Test DatePicker with light theme
#[test]
fn test_datepicker_light_theme() {
    let theme = Theme::light();
    let mut selected_date = None;

    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut picker = DatePicker::new("light_picker");
            picker.show(ctx, &theme, ui, &mut selected_date);
        });
    });

    harness.run();
}

/// Test DatePicker full configuration
#[test]
fn test_datepicker_full_config() {
    let theme = Theme::dark();
    let mut selected_date = Date::new(2024, 12, 25);

    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut picker = DatePicker::new("full_picker")
                .label("Event Date")
                .placeholder("Select event date...");
            picker.show(ctx, &theme, ui, &mut selected_date);
        });
    });

    harness.run();
}

/// Test multiple DatePickers
#[test]
fn test_multiple_datepickers() {
    let theme = Theme::dark();
    let mut start_date = None;
    let mut end_date = None;

    let mut harness = Harness::new(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let mut start_picker = DatePicker::new("start_picker").label("Start Date");
                start_picker.show(ctx, &theme, ui, &mut start_date);

                ui.add_space(8.0);

                let mut end_picker = DatePicker::new("end_picker").label("End Date");
                end_picker.show(ctx, &theme, ui, &mut end_date);
            });
        });
    });

    harness.run();
}

/// Test Date February edge cases
#[test]
fn test_date_february_edge_cases() {
    // Feb 29 valid in leap year
    assert!(Date::new(2024, 2, 29).is_some());

    // Feb 29 invalid in non-leap year
    assert!(Date::new(2023, 2, 29).is_none());

    // Feb 28 always valid
    assert!(Date::new(2023, 2, 28).is_some());
    assert!(Date::new(2024, 2, 28).is_some());
}

/// Test Date today (just ensure it doesn't panic)
#[test]
fn test_date_today() {
    let today = Date::today();
    assert!(today.year >= 2024);
    assert!((1..=12).contains(&today.month));
    assert!((1..=31).contains(&today.day));
}

/// Test Date comparison
#[test]
fn test_date_comparison() {
    let date1 = Date::new(2024, 6, 15).unwrap();
    let date2 = Date::new(2024, 6, 15).unwrap();
    let date3 = Date::new(2024, 6, 16).unwrap();

    assert_eq!(date1, date2);
    assert_ne!(date1, date3);
}
