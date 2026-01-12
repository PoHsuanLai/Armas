# Textarea

Multi-line text input field with validation states and character count.

## Basic Usage

```demo
let mut text = String::new();
Textarea::new("Enter your message")
    .show(ui, &mut text);
```

## With Label

```demo
let mut text = String::new();
Textarea::new("Type here...")
    .label("Description")
    .show(ui, &mut text);
```

## Variants

### Default

```demo
let mut text = String::new();
Textarea::new("Default textarea")
    .variant(InputVariant::Default)
    .show(ui, &mut text);
```

### Outlined

```demo
let mut text = String::new();
Textarea::new("Outlined textarea")
    .variant(InputVariant::Outlined)
    .show(ui, &mut text);
```

### Filled

```demo
let mut text = String::new();
Textarea::new("Filled textarea")
    .variant(InputVariant::Filled)
    .show(ui, &mut text);
```

## Custom Rows

```demo
let mut text = String::new();
Textarea::new("Enter text")
    .rows(6)
    .show(ui, &mut text);
```

```demo
let mut text = String::new();
Textarea::new("Short textarea")
    .rows(2)
    .show(ui, &mut text);
```

## With Character Limit

```demo
let mut text = String::new();
Textarea::new("Enter bio")
    .label("Bio")
    .max_chars(200)
    .show(ui, &mut text);
```

## Validation States

### Success

```demo
let mut text = String::from("Great feedback!");
Textarea::new("Enter feedback")
    .state(InputState::Success)
    .helper_text("Thank you for your feedback")
    .show(ui, &mut text);
```

### Error

```demo
let mut text = String::new();
Textarea::new("Enter message")
    .state(InputState::Error)
    .helper_text("Message is required")
    .show(ui, &mut text);
```

### Warning

```demo
let mut text = String::from("Short");
Textarea::new("Enter description")
    .state(InputState::Warning)
    .helper_text("Consider adding more details")
    .show(ui, &mut text);
```

## Non-Resizable

```demo
let mut text = String::new();
Textarea::new("Fixed size")
    .resizable(false)
    .rows(4)
    .show(ui, &mut text);
```

## Custom Width

```demo
let mut text = String::new();
Textarea::new("Wide textarea")
    .width(500.0)
    .show(ui, &mut text);
```

## With Helper Text

```demo
let mut text = String::new();
Textarea::new("Enter comment")
    .label("Comment")
    .helper_text("Your comment will be visible to everyone")
    .show(ui, &mut text);
```

## Complete Form Example

```demo
let mut text = String::new();
Textarea::new("Describe your issue in detail")
    .label("Issue Description")
    .variant(InputVariant::Outlined)
    .rows(5)
    .max_chars(500)
    .helper_text("Provide as much detail as possible")
    .show(ui, &mut text);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.variant()` | `InputVariant` | `Outlined` | Textarea style |
| `.state()` | `InputState` | `Normal` | Validation state |
| `.label()` | `&str` | `None` | Label text |
| `.helper_text()` | `&str` | `None` | Helper/error text |
| `.width()` | `f32` | `full` | Textarea width |
| `.rows()` | `usize` | `4` | Visible rows |
| `.max_chars()` | `usize` | `None` | Character limit |
| `.resizable()` | `bool` | `true` | Allow resize |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline`, `success`, `error`, `warning`
- Uses InputVariant and InputState from Input component
