//! Tests for Select component using egui_kittest

use armas::components::basic::SelectOption;
use armas::prelude::*;
use egui_kittest::Harness;

/// Test that Select renders without panicking
#[test]
fn test_select_renders() {
    let options = vec![
        SelectOption::new("opt1", "Option 1"),
        SelectOption::new("opt2", "Option 2"),
        SelectOption::new("opt3", "Option 3"),
    ];

    let mut select = Select::new(options);

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select with pre-selected value
#[test]
fn test_select_with_selection() {
    let options = vec![
        SelectOption::new("apple", "Apple"),
        SelectOption::new("banana", "Banana"),
        SelectOption::new("cherry", "Cherry"),
    ];

    let mut select = Select::new(options).selected("banana");

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select with label
#[test]
fn test_select_with_label() {
    let options = vec![
        SelectOption::new("sm", "Small"),
        SelectOption::new("md", "Medium"),
        SelectOption::new("lg", "Large"),
    ];

    let mut select = Select::new(options).label("Size");

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select with custom placeholder
#[test]
fn test_select_with_placeholder() {
    let options = vec![
        SelectOption::new("red", "Red"),
        SelectOption::new("green", "Green"),
        SelectOption::new("blue", "Blue"),
    ];

    let mut select = Select::new(options).placeholder("Choose a color...");

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select with icons
#[test]
fn test_select_with_icons() {
    let options = vec![
        SelectOption::new("home", "Home").icon("🏠"),
        SelectOption::new("work", "Work").icon("💼"),
        SelectOption::new("school", "School").icon("🎓"),
    ];

    let mut select = Select::new(options);

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select with descriptions
#[test]
fn test_select_with_descriptions() {
    let options = vec![
        SelectOption::new("free", "Free Plan").description("Basic features, limited usage"),
        SelectOption::new("pro", "Pro Plan").description("All features, unlimited usage"),
        SelectOption::new("enterprise", "Enterprise").description("Custom solutions for teams"),
    ];

    let mut select = Select::new(options).label("Select Plan");

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select with disabled options
#[test]
fn test_select_with_disabled_options() {
    let options = vec![
        SelectOption::new("available", "Available"),
        SelectOption::new("unavailable", "Unavailable").disabled(true),
        SelectOption::new("coming_soon", "Coming Soon").disabled(true),
    ];

    let mut select = Select::new(options);

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select with search disabled
#[test]
fn test_select_not_searchable() {
    let options = vec![
        SelectOption::new("a", "Alpha"),
        SelectOption::new("b", "Beta"),
        SelectOption::new("c", "Gamma"),
    ];

    let mut select = Select::new(options).searchable(false);

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select with custom width
#[test]
fn test_select_custom_width() {
    let options = vec![
        SelectOption::new("1", "Option 1"),
        SelectOption::new("2", "Option 2"),
    ];

    let mut select = Select::new(options).width(300.0);

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test Select using builder API
#[test]
fn test_select_builder_api() {
    let mut select = Select::build(|s| {
        s.option("apple", "Apple").icon("🍎");
        s.option("banana", "Banana").icon("🍌");
        s.option("cherry", "Cherry")
            .icon("🍒")
            .description("Sweet red fruit");
    })
    .label("Fruit")
    .selected("apple");

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        select.show(ui, &theme);
    });

    harness.run();
}

/// Test multiple selects
#[test]
fn test_multiple_selects() {
    let country_options = vec![
        SelectOption::new("us", "United States"),
        SelectOption::new("uk", "United Kingdom"),
        SelectOption::new("ca", "Canada"),
    ];

    let language_options = vec![
        SelectOption::new("en", "English"),
        SelectOption::new("es", "Spanish"),
        SelectOption::new("fr", "French"),
    ];

    let mut country_select = Select::new(country_options).label("Country");
    let mut language_select = Select::new(language_options).label("Language");

    let mut harness = Harness::new_ui(|ui| {
        let theme = ui.ctx().armas_theme();
        ui.vertical(|ui| {
            country_select.show(ui, &theme);
            language_select.show(ui, &theme);
        });
    });

    harness.run();
}
