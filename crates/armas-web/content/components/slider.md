# Slider

Horizontal slider for value selection with customizable range and step values.

## Basic Usage

```demo
let mut value = 50.0;
Slider::new(0.0, 100.0)
    .id("slider_1")
    .show(ui, &mut value);
```

## With Label

```demo
let mut value = 75.0;
Slider::new(0.0, 100.0)
    .id("slider_2")
    .label("Volume")
    .show(ui, &mut value);
```

## With Suffix

```demo
let mut value = 50.0;
Slider::new(0.0, 100.0)
    .id("slider_3")
    .label("Opacity")
    .suffix("%")
    .show(ui, &mut value);
```

```demo
let mut value = 250.0;
Slider::new(0.0, 1000.0)
    .id("slider_4")
    .label("Delay")
    .suffix("ms")
    .show(ui, &mut value);
```

## With Step

```demo
let mut value = 5.0;
Slider::new(0.0, 10.0)
    .id("slider_5")
    .label("Rating")
    .step(1.0)
    .show(ui, &mut value);
```

```demo
let mut value = 2.5;
Slider::new(0.0, 10.0)
    .id("slider_6")
    .label("Value")
    .step(0.5)
    .show(ui, &mut value);
```

## Custom Width

```demo
let mut value = 50.0;
Slider::new(0.0, 100.0)
    .id("slider_7")
    .width(300.0)
    .show(ui, &mut value);
```

## Custom Height

```demo
let mut value = 50.0;
Slider::new(0.0, 100.0)
    .id("slider_8")
    .height(30.0)
    .show(ui, &mut value);
```

## Without Value Display

```demo
let mut value = 50.0;
Slider::new(0.0, 100.0)
    .id("slider_9")
    .label("Setting")
    .show_value(false)
    .show(ui, &mut value);
```

## Different Ranges

### Percentage (0-100)

```demo
let mut value = 75.0;
Slider::new(0.0, 100.0)
    .id("slider_10")
    .label("Progress")
    .suffix("%")
    .show(ui, &mut value);
```

### Temperature (-20 to 40)

```demo
let mut value = 20.0;
Slider::new(-20.0, 40.0)
    .id("slider_11")
    .label("Temperature")
    .suffix("°C")
    .show(ui, &mut value);
```

### Decimal (0.0 to 1.0)

```demo
let mut value = 0.5;
Slider::new(0.0, 1.0)
    .id("slider_12")
    .label("Alpha")
    .step(0.1)
    .show(ui, &mut value);
```

## Handling Changes

```demo
let mut value = 50.0;
let response = Slider::new(0.0, 100.0)
    .id("slider_13")
    .label("Volume")
    .suffix("%")
    .show(ui, &mut value);

if response.changed {
    // Handle value change
}
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.width()` | `f32` | `200.0` | Slider width |
| `.height()` | `f32` | `20.0` | Slider height |
| `.show_value()` | `bool` | `true` | Show value label |
| `.label()` | `&str` | `None` | Slider label |
| `.suffix()` | `&str` | `None` | Value suffix |
| `.step()` | `f32` | `None` | Snap to step value |

## Response

| Field | Type | Description |
|-------|------|-------------|
| `value` | `f32` | Current slider value |
| `changed` | `bool` | Whether value changed this frame |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface_variant`, `on_surface`
