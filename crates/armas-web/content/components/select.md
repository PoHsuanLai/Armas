# Select

Searchable dropdown menus with keyboard navigation and customizable options.

## Basic Usage

```demo
let options = vec![
    SelectOption::new("1", "Option 1"),
    SelectOption::new("2", "Option 2"),
    SelectOption::new("3", "Option 3"),
];

let mut select = Select::new(options)
    .id("select_1");
select.show(ui);
```

## With Label

```demo
let options = vec![
    SelectOption::new("us", "United States"),
    SelectOption::new("uk", "United Kingdom"),
    SelectOption::new("ca", "Canada"),
];

let mut select = Select::new(options)
    .id("select_2")
    .with_label("Country");
select.show(ui);
```

## With Icons

```demo
let options = vec![
    SelectOption::new("home", "Home").with_icon("🏠"),
    SelectOption::new("work", "Work").with_icon("💼"),
    SelectOption::new("other", "Other").with_icon("📍"),
];

let mut select = Select::new(options)
    .id("select_3");
select.show(ui);
```

## With Descriptions

```demo
let options = vec![
    SelectOption::new("free", "Free Plan")
        .with_description("Perfect for getting started"),
    SelectOption::new("pro", "Pro Plan")
        .with_description("For professionals and teams"),
    SelectOption::new("enterprise", "Enterprise")
        .with_description("Advanced features and support"),
];

let mut select = Select::new(options)
    .id("select_4");
select.show(ui);
```

## With Disabled Options

```demo
let options = vec![
    SelectOption::new("enabled", "Available Option"),
    SelectOption::new("disabled", "Unavailable Option")
        .disabled(true),
    SelectOption::new("another", "Another Option"),
];

let mut select = Select::new(options)
    .id("select_5");
select.show(ui);
```

## With Pre-Selection

```demo
let options = vec![
    SelectOption::new("red", "Red"),
    SelectOption::new("green", "Green"),
    SelectOption::new("blue", "Blue"),
];

let mut select = Select::new(options)
    .id("select_6")
    .with_selected("green");
select.show(ui);
```

## Non-Searchable

```demo
let options = vec![
    SelectOption::new("sm", "Small"),
    SelectOption::new("md", "Medium"),
    SelectOption::new("lg", "Large"),
];

let mut select = Select::new(options)
    .id("select_7")
    .searchable(false);
select.show(ui);
```

## Custom Width

```demo
let options = vec![
    SelectOption::new("1", "Option 1"),
    SelectOption::new("2", "Option 2"),
];

let mut select = Select::new(options)
    .id("select_8")
    .with_width(300.0);
select.show(ui);
```

## Handling Selection

```demo
let options = vec![
    SelectOption::new("1", "Option 1"),
    SelectOption::new("2", "Option 2"),
];

let mut select = Select::new(options)
    .id("select_9");
let response = select.show(ui);

if response.changed {
    if let Some(value) = response.selected_value {
        // Handle selection change
    }
}
```

## API Reference

### Select

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `egui::Id` | `None` | ID for state persistence |
| `.with_selected()` | `&str` | `None` | Pre-selected value |
| `.with_label()` | `&str` | `None` | Label text |
| `.with_placeholder()` | `&str` | "Select..." | Placeholder text |
| `.with_width()` | `f32` | `200.0` | Dropdown width |
| `.with_max_height()` | `f32` | `300.0` | Max dropdown height |
| `.searchable()` | `bool` | `true` | Enable search |

### SelectOption

| Method | Type | Description |
|--------|------|-------------|
| `.with_icon()` | `&str` | Add icon (emoji) |
| `.with_description()` | `&str` | Add description |
| `.disabled()` | `bool` | Disable option |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `surface_variant`, `hover`
- Keyboard navigation support (Arrow keys, Enter, Escape)
