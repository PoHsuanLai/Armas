# Alert

Inline alert messages with icons.

## Basic Usage

```demo
Alert::new("This is an informational alert")
    .show(ui);
```

## Variants

### Default (Info)

```demo
Alert::new("This is informational")
    .show(ui);
```

### Destructive

```demo
Alert::new("Something went wrong")
    .destructive()
    .show(ui);
```

## With Title

```demo
Alert::new("Your changes have been saved to the server")
    .title("Success")
    .show(ui);
```

## Dismissible

```demo
Alert::new("Click the X to dismiss this alert")
    .dismissible(true)
    .show(ui);
```

## Without Icon

```demo
Alert::new("Alert without an icon")
    .show_icon(false)
    .show(ui);
```

## Custom Width

```demo
Alert::new("This alert has a custom width")
    .width(400.0)
    .show(ui);
```

## Custom Color

```demo
Alert::new("Alert with custom color")
    .color(Color32::from_rgb(100, 200, 150))
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.variant()` | `AlertVariant` | `Info` | Alert variant |
| `.destructive()` | - | - | Make destructive |
| `.color()` | `Color32` | theme | Custom accent color |
| `.title()` | `&str` | `None` | Optional alert title |
| `.dismissible()` | `bool` | `false` | Show dismiss button |
| `.show_icon()` | `bool` | `true` | Show variant icon |
| `.width()` | `f32` | `full` | Custom width |

## AlertVariant

- `AlertVariant::Info` - Default informational (default)
- `AlertVariant::Destructive` - Red for errors/danger
