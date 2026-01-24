# Three Value Slider

Horizontal slider with three thumbs: min bound, current value, and max bound. The center value is constrained between the two bounds.

## Basic Usage

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
ThreeValueSlider::new(0.0, 100.0)
    .id("three_1")
    .show(ui, &mut min, &mut value, &mut max);
```

## With Label

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
ThreeValueSlider::new(0.0, 100.0)
    .id("three_2")
    .label("Dynamics")
    .suffix("dB")
    .show(ui, &mut min, &mut value, &mut max);
```

## Audio: Compressor Threshold

```demo
let mut threshold_low = -40.0;
let mut threshold = -20.0;
let mut threshold_high = -6.0;
ThreeValueSlider::new(-60.0, 0.0)
    .id("three_3")
    .label("Compressor")
    .suffix("dB")
    .show(ui, &mut threshold_low, &mut threshold, &mut threshold_high);
```

## Audio: Dynamic Range

```demo
let mut floor = -48.0;
let mut level = -12.0;
let mut ceiling = -3.0;
ThreeValueSlider::new(-60.0, 0.0)
    .id("three_4")
    .label("Dynamic Range")
    .suffix("dB")
    .show(ui, &mut floor, &mut level, &mut ceiling);
```

## Value Thumb Styles

### Diamond (Default)

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
ThreeValueSlider::new(0.0, 100.0)
    .id("three_5")
    .label("Diamond Style")
    .value_thumb_style(ValueThumbStyle::Diamond)
    .show(ui, &mut min, &mut value, &mut max);
```

### Circle

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
ThreeValueSlider::new(0.0, 100.0)
    .id("three_6")
    .label("Circle Style")
    .value_thumb_style(ValueThumbStyle::Circle)
    .show(ui, &mut min, &mut value, &mut max);
```

### Line

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
ThreeValueSlider::new(0.0, 100.0)
    .id("three_7")
    .label("Line Style")
    .value_thumb_style(ValueThumbStyle::Line)
    .show(ui, &mut min, &mut value, &mut max);
```

## With Step

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
ThreeValueSlider::new(0.0, 100.0)
    .id("three_8")
    .label("Stepped")
    .step(10.0)
    .show(ui, &mut min, &mut value, &mut max);
```

## With Minimum Gap

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
ThreeValueSlider::new(0.0, 100.0)
    .id("three_9")
    .label("Min Gap")
    .min_gap(10.0)
    .show(ui, &mut min, &mut value, &mut max);
```

## Temperature Range with Current

```demo
let mut min_temp = 18.0;
let mut current = 22.0;
let mut max_temp = 26.0;
ThreeValueSlider::new(10.0, 35.0)
    .id("three_10")
    .label("Thermostat")
    .suffix("°C")
    .show(ui, &mut min_temp, &mut current, &mut max_temp);
```

## Custom Width

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
ThreeValueSlider::new(0.0, 100.0)
    .id("three_11")
    .width(400.0)
    .show(ui, &mut min, &mut value, &mut max);
```

## Handling Changes

```demo
let mut min = 20.0;
let mut value = 50.0;
let mut max = 80.0;
let response = ThreeValueSlider::new(0.0, 100.0)
    .id("three_12")
    .label("Control")
    .show(ui, &mut min, &mut value, &mut max);

if response.changed {
    // Handle value change
}
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.width()` | `f32` | `200.0` | Slider width |
| `.height()` | `f32` | `20.0` | Slider height |
| `.show_value()` | `bool` | `true` | Show value labels |
| `.label()` | `&str` | `None` | Slider label |
| `.suffix()` | `&str` | `None` | Value suffix |
| `.step()` | `f32` | `None` | Snap to step value |
| `.min_gap()` | `f32` | `0.0` | Minimum gap between adjacent thumbs |
| `.value_thumb_style()` | `ValueThumbStyle` | `Diamond` | Style of center thumb |

## ValueThumbStyle

| Variant | Description |
|---------|-------------|
| `Circle` | Same circular style as bound thumbs |
| `Diamond` | Diamond shape to differentiate from bounds (default) |
| `Line` | Vertical line indicator |

## Response

| Field | Type | Description |
|-------|------|-------------|
| `min_bound` | `f32` | Current minimum bound |
| `value` | `f32` | Current center value |
| `max_bound` | `f32` | Current maximum bound |
| `changed` | `bool` | Whether any value changed this frame |
