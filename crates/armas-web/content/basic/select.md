# Select

Searchable dropdown menus with keyboard navigation and customizable options.

## Basic Usage (Closure-based API)

```demo
let mut select = Select::build(|s| {
    s.option("1", "Option 1");
    s.option("2", "Option 2");
    s.option("3", "Option 3");
})
.id("select_1");
select.show(ui);
```

## Basic Usage (Traditional API)

```demo
let options = vec![
    SelectOption::new("1", "Option 1"),
    SelectOption::new("2", "Option 2"),
    SelectOption::new("3", "Option 3"),
];

let mut select = Select::new(options)
    .id("select_1b");
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
    .label("Country");
select.show(ui);
```

## With Icons

```demo
let options = vec![
    SelectOption::new("home", "Home"),
    SelectOption::new("work", "Work"),
    SelectOption::new("other", "Other"),
];

let mut select = Select::new(options)
    .id("select_3");
select.show(ui);
```

## With Descriptions

```demo
let options = vec![
    SelectOption::new("free", "Free Plan")
        .description("Perfect for getting started"),
    SelectOption::new("pro", "Pro Plan")
        .description("For professionals and teams"),
    SelectOption::new("enterprise", "Enterprise")
        .description("Advanced features and support"),
];

let mut select = Select::new(options)
    .id("select_4");
select.show(ui);
```

## Closure API with Icons and Descriptions

```demo
let mut select = Select::build(|s| {
    s.option("apple", "Apple")
        .description("Red and crispy");
    s.option("banana", "Banana")
        .description("Yellow and sweet");
    s.option("cherry", "Cherry")
        .description("Small and tart");
})
.id("select_closure")
.label("Choose a Fruit");
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
    .selected("green");
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
    .width(300.0);
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

**Creating a Select:**
- `Select::new(options: Vec<SelectOption>)` - Create with pre-built options
- `Select::build(|builder| { ... })` - Create with closure-based API

**Configuration Methods:**

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `egui::Id` | `None` | ID for state persistence |
| `.selected()` | `&str` | `None` | Pre-selected value |
| `.label()` | `&str` | `None` | Label text |
| `.placeholder()` | `&str` | "Select..." | Placeholder text |
| `.width()` | `f32` | `200.0` | Dropdown width |
| `.max_height()` | `f32` | `300.0` | Max dropdown height |
| `.searchable()` | `bool` | `true` | Enable search |

### SelectOption (Traditional API)

| Method | Type | Description |
|--------|------|-------------|
| `.icon()` | `&str` | Add icon (emoji) |
| `.description()` | `&str` | Add description |
| `.disabled()` | `bool` | Disable option |

### SelectBuilder (Closure API)

| Method | Returns | Description |
|--------|---------|-------------|
| `.option(value, label)` | `SelectOptionBuilder` | Add new option |

**SelectOptionBuilder Methods:**
- `.icon(&str)` - Add icon
- `.description(&str)` - Add description
- `.disabled(bool)` - Mark as disabled

### SelectResponse

| Field | Type | Description |
|-------|------|-------------|
| `response` | `egui::Response` | The trigger button response |
| `changed` | `bool` | Whether the selected value changed |
| `selected_value` | `Option<String>` | The newly selected value |
| `is_open` | `bool` | Whether the dropdown is currently open |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `surface_variant`, `hover`
- Keyboard navigation support (Arrow keys, Enter, Escape)
