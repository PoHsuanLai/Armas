# Alert

Inline alert messages with icons and multiple severity levels.

## Basic Usage

```demo
Alert::new("This is an informational alert", AlertVariant::Info)
    .show(ui);
```

## Variants

### Info

```demo
Alert::new("This is informational", AlertVariant::Info)
    .show(ui);
```

### Success

```demo
Alert::new("Operation completed successfully", AlertVariant::Success)
    .show(ui);
```

### Warning

```demo
Alert::new("Please review before continuing", AlertVariant::Warning)
    .show(ui);
```

### Error

```demo
Alert::new("Something went wrong", AlertVariant::Error)
    .show(ui);
```

## With Title

```demo
Alert::new("Your changes have been saved to the server", AlertVariant::Success)
    .title("Success")
    .show(ui);
```

## Dismissible

```demo
Alert::new("Click the X to dismiss this alert", AlertVariant::Info)
    .dismissible(true)
    .show(ui);
```

## Without Icon

```demo
Alert::new("Alert without an icon", AlertVariant::Warning)
    .show_icon(false)
    .show(ui);
```

## Custom Width

```demo
Alert::new("This alert has a custom width", AlertVariant::Info)
    .width(400.0)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.variant()` | `AlertVariant` | Required | Alert severity level |
| `.title()` | `&str` | `None` | Optional alert title |
| `.dismissible()` | `bool` | `false` | Show dismiss button |
| `.show_icon()` | `bool` | `true` | Show variant icon |
| `.width()` | `f32` | `full` | Custom width |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `success`, `warning`, `error`
