# Input

Modern text input fields with icons, validation states, and multiple style variants.

## Basic Usage

```demo
let mut text = String::new();
Input::new("Enter your name").id("input_1")
    .show(ui, &mut text);
```

## Variants

### Default

```demo
let mut text = String::new();
Input::new("Default input").id("input_2")
    .variant(InputVariant::Default)
    .show(ui, &mut text);
```

### Outlined

```demo
let mut text = String::new();
Input::new("Outlined input").id("input_3")
    .variant(InputVariant::Outlined)
    .show(ui, &mut text);
```

### Filled

```demo
let mut text = String::new();
Input::new("Filled input").id("input_4")
    .variant(InputVariant::Filled)
    .show(ui, &mut text);
```

## With Label

```demo
let mut text = String::new();
Input::new("Enter email").id("input_5")
    .label("Email Address")
    .show(ui, &mut text);
```

## With Icons

```demo
let mut text = String::new();
Input::new("Search...")
    .left_icon("🔍")
    .show(ui, &mut text);
```

```demo
let mut text = String::new();
Input::new("Username")
    .left_icon("👤")
    .right_icon("✓")
    .show(ui, &mut text);
```

## Validation States

### Success

```demo
let mut text = String::from("valid@email.com");
Input::new("Enter email").id("input_5")
    .state(InputState::Success)
    .helper_text("Email is valid")
    .show(ui, &mut text);
```

### Error

```demo
let mut text = String::from("invalid");
Input::new("Enter email").id("input_5")
    .state(InputState::Error)
    .helper_text("Please enter a valid email")
    .show(ui, &mut text);
```

### Warning

```demo
let mut text = String::from("example");
Input::new("Username")
    .state(InputState::Warning)
    .helper_text("Username is short")
    .show(ui, &mut text);
```

## Password Field

```demo
let mut password = String::new();
Input::new("Enter password")
    .label("Password")
    .password(true)
    .left_icon("🔒")
    .show(ui, &mut password);
```

## Custom Width

```demo
let mut text = String::new();
Input::new("Wide input").id("input_12")
    .width(400.0)
    .show(ui, &mut text);
```

## Search Input

```demo
let mut search = String::new();
SearchInput::new()
    .placeholder("Search files...")
    .show(ui, &mut search);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.variant()` | `InputVariant` | `Default` | Input style variant |
| `.state()` | `InputState` | `Normal` | Validation state |
| `.label()` | `&str` | `None` | Label text |
| `.helper_text()` | `&str` | `None` | Helper/error text |
| `.left_icon()` | `&str` | `None` | Left icon (emoji) |
| `.right_icon()` | `&str` | `None` | Right icon (emoji) |
| `.width()` | `f32` | `200.0` | Input width |
| `.password()` | `bool` | `false` | Password masking |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface_variant`, `success`, `error`, `warning`
