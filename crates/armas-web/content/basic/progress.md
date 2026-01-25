# Progress

Displays an indicator showing the completion progress of a task, styled like shadcn/ui.

## Basic Usage

```demo
Progress::new(65.0).show(ui);
```

## Custom Width

```demo
Progress::new(33.0).width(200.0).show(ui);
```

## Custom Height

```demo
Progress::new(50.0).height(12.0).show(ui);
```

## Multiple Progress Bars

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 8.0;

    Progress::new(25.0).show(ui);
    Progress::new(50.0).show(ui);
    Progress::new(75.0).show(ui);
    Progress::new(100.0).show(ui);
});
```

## Circular Progress

For circular progress indicators, use `CircularProgressBar`:

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    CircularProgressBar::new(25.0).show(ui);
    CircularProgressBar::new(50.0).show(ui);
    CircularProgressBar::new(75.0).show(ui);
});
```

## Circular with Percentage

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    CircularProgressBar::new(33.0)
        .show_percentage(true)
        .show(ui);

    CircularProgressBar::new(66.0)
        .size(64.0)
        .show_percentage(true)
        .show(ui);

    CircularProgressBar::new(100.0)
        .size(80.0)
        .show_percentage(true)
        .show(ui);
});
```

## Indeterminate Circular

For loading states without a known progress:

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    CircularProgressBar::indeterminate()
        .size(32.0)
        .show(ui);

    CircularProgressBar::indeterminate()
        .size(48.0)
        .show(ui);

    CircularProgressBar::indeterminate()
        .size(64.0)
        .stroke_width(6.0)
        .show(ui);
});
```

## Ring Progress

For larger ring-style progress with labels:

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 24.0;

    RingProgress::new(75.0)
        .size(100.0)
        .show(ui);

    RingProgress::new(42.0)
        .size(100.0)
        .label("Complete")
        .show(ui);
});
```

## Ring Progress Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 24.0;

    RingProgress::new(60.0)
        .size(80.0)
        .thickness(8.0)
        .show(ui);

    RingProgress::new(60.0)
        .size(120.0)
        .thickness(12.0)
        .label("Storage")
        .show(ui);

    RingProgress::new(60.0)
        .size(160.0)
        .thickness(16.0)
        .label("Upload")
        .show(ui);
});
```

## API Reference

### Progress

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `Progress::new(value)` | `impl Into<f32>` | - | Create with value 0-100 |
| `.width()` | `f32` | fill available | Bar width |
| `.height()` | `f32` | `8.0` | Bar height |

### CircularProgressBar

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `CircularProgressBar::new(value)` | `impl Into<f32>` | - | Create with value 0-100 |
| `CircularProgressBar::indeterminate()` | - | - | Create loading spinner |
| `.size()` | `f32` | `48.0` | Circle diameter |
| `.stroke_width()` | `f32` | `4.0` | Stroke width |
| `.show_percentage()` | `bool` | `false` | Show % in center |

### RingProgress

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `RingProgress::new(value)` | `impl Into<f32>` | - | Create with value 0-100 |
| `.size()` | `f32` | `120.0` | Ring diameter |
| `.thickness()` | `f32` | `12.0` | Ring thickness |
| `.label()` | `impl Into<String>` | `None` | Label below percentage |

## shadcn/ui Styling

The Progress component follows shadcn/ui conventions:

- **Track**: `bg-primary/20` (primary color at 20% opacity)
- **Indicator**: `bg-primary` (solid primary color)
- **Height**: `h-2` (8px)
- **Border radius**: `rounded-full` (fully rounded)
