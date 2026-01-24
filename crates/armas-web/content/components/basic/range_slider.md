# Range Slider

Horizontal slider with two thumbs for selecting a range (min/max values).

## Basic Usage

```demo
let mut min = 20.0;
let mut max = 80.0;
RangeSlider::new(0.0, 100.0)
    .id("range_1")
    .show(ui, &mut min, &mut max);
```

## With Label

```demo
let mut min = 200.0;
let mut max = 800.0;
RangeSlider::new(0.0, 1000.0)
    .id("range_2")
    .label("Frequency Range")
    .suffix("Hz")
    .show(ui, &mut min, &mut max);
```

## Price Range Filter

```demo
let mut min = 25.0;
let mut max = 75.0;
RangeSlider::new(0.0, 100.0)
    .id("range_3")
    .label("Price")
    .suffix("$")
    .show(ui, &mut min, &mut max);
```

## With Step

```demo
let mut min = 20.0;
let mut max = 80.0;
RangeSlider::new(0.0, 100.0)
    .id("range_4")
    .label("Volume Range")
    .suffix("%")
    .step(10.0)
    .show(ui, &mut min, &mut max);
```

## With Minimum Gap

Enforce a minimum distance between thumbs.

```demo
let mut min = 30.0;
let mut max = 70.0;
RangeSlider::new(0.0, 100.0)
    .id("range_5")
    .label("Selection")
    .min_gap(20.0)
    .show(ui, &mut min, &mut max);
```

## Range Drag Disabled

By default, you can drag the filled region to move both thumbs together. This can be disabled.

```demo
let mut min = 25.0;
let mut max = 75.0;
RangeSlider::new(0.0, 100.0)
    .id("range_6")
    .label("Fixed Range")
    .allow_range_drag(false)
    .show(ui, &mut min, &mut max);
```

## Audio: EQ Band Selection

```demo
let mut low = 80.0;
let mut high = 2000.0;
RangeSlider::new(20.0, 20000.0)
    .id("range_7")
    .label("EQ Band")
    .suffix("Hz")
    .show(ui, &mut low, &mut high);
```

## Audio: Loop Region

```demo
let mut start = 4.0;
let mut end = 12.0;
RangeSlider::new(0.0, 16.0)
    .id("range_8")
    .label("Loop Region")
    .suffix(" bars")
    .step(1.0)
    .show(ui, &mut start, &mut end);
```

## Custom Width

```demo
let mut min = 20.0;
let mut max = 80.0;
RangeSlider::new(0.0, 100.0)
    .id("range_9")
    .width(400.0)
    .show(ui, &mut min, &mut max);
```

## Without Value Display

```demo
let mut min = 30.0;
let mut max = 70.0;
RangeSlider::new(0.0, 100.0)
    .id("range_10")
    .label("Range")
    .show_value(false)
    .show(ui, &mut min, &mut max);
```

## Handling Changes

```demo
let mut min = 20.0;
let mut max = 80.0;
let response = RangeSlider::new(0.0, 100.0)
    .id("range_11")
    .label("Selection")
    .show(ui, &mut min, &mut max);

if response.changed {
    // Handle range change
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
| `.min_gap()` | `f32` | `0.0` | Minimum gap between thumbs |
| `.allow_range_drag()` | `bool` | `true` | Allow dragging filled region |

## Response

| Field | Type | Description |
|-------|------|-------------|
| `min_value` | `f32` | Current minimum value |
| `max_value` | `f32` | Current maximum value |
| `changed` | `bool` | Whether either value changed this frame |
