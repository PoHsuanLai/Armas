# Progress

Progress indicators for showing task completion and loading states with semantic color variants.

## Basic Linear Progress

```demo
LinearProgress::new(0.7).show(ui);
```

## Linear Progress with Label

```demo
LinearProgress::new(0.65)
    .show_label()
    .show(ui);
```

## Indeterminate Progress

```demo
LinearProgress::indeterminate().show(ui);
```

## Color Variants

### Success Progress

```demo
LinearProgress::new(0.85)
    .color_variant(ProgressColor::Success)
    .show_label()
    .show(ui);
```

### Warning Progress

```demo
LinearProgress::new(0.45)
    .color_variant(ProgressColor::Warning)
    .show_label()
    .show(ui);
```

### Error Progress

```demo
LinearProgress::new(0.25)
    .color_variant(ProgressColor::Error)
    .show_label()
    .show(ui);
```

### Info Progress

```demo
LinearProgress::new(0.60)
    .color_variant(ProgressColor::Info)
    .show_label()
    .show(ui);
```

## All Variants Comparison

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 12.0;

    ui.label("Primary");
    LinearProgress::new(0.75).show(ui);

    ui.label("Success");
    LinearProgress::new(0.75)
        .color_variant(ProgressColor::Success)
        .show(ui);

    ui.label("Warning");
    LinearProgress::new(0.75)
        .color_variant(ProgressColor::Warning)
        .show(ui);

    ui.label("Error");
    LinearProgress::new(0.75)
        .color_variant(ProgressColor::Error)
        .show(ui);

    ui.label("Info");
    LinearProgress::new(0.75)
        .color_variant(ProgressColor::Info)
        .show(ui);
});
```

## Circular Progress Bar

```demo
CircularProgressBar::new(0.65)
    .show_percentage(true)
    .show(ui);
```

## Circular with Color Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    ui.vertical(|ui| {
        CircularProgressBar::new(0.85)
            .color_variant(ProgressColor::Success)
            .show_percentage(true)
            .show(ui);
        ui.label("Success");
    });

    ui.vertical(|ui| {
        CircularProgressBar::new(0.45)
            .color_variant(ProgressColor::Warning)
            .show_percentage(true)
            .show(ui);
        ui.label("Warning");
    });

    ui.vertical(|ui| {
        CircularProgressBar::new(0.25)
            .color_variant(ProgressColor::Error)
            .show_percentage(true)
            .show(ui);
        ui.label("Error");
    });
});
```

## Indeterminate Circular

```demo
CircularProgressBar::indeterminate()
    .size(60.0)
    .show(ui);
```

## Ring Progress

```demo
RingProgress::new(0.75)
    .label("Upload")
    .show(ui);
```

## Ring with Color Variants

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 24.0;

    RingProgress::new(0.85)
        .color_variant(ProgressColor::Success)
        .label("Complete")
        .size(100.0)
        .show(ui);

    RingProgress::new(0.45)
        .color_variant(ProgressColor::Warning)
        .label("In Progress")
        .size(100.0)
        .show(ui);

    RingProgress::new(0.15)
        .color_variant(ProgressColor::Error)
        .label("Low")
        .size(100.0)
        .show(ui);
});
```

## Sizes

### Linear Sizes

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 12.0;

    LinearProgress::new(0.7)
        .height(2.0)
        .show(ui);

    LinearProgress::new(0.7)
        .height(4.0)
        .show(ui);

    LinearProgress::new(0.7)
        .height(8.0)
        .show(ui);
});
```

### Circular Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    CircularProgressBar::new(0.7)
        .size(32.0)
        .show(ui);

    CircularProgressBar::new(0.7)
        .size(48.0)
        .show(ui);

    CircularProgressBar::new(0.7)
        .size(64.0)
        .show(ui);
});
```

## API Reference

### ProgressColor Enum

```rust
pub enum ProgressColor {
    Primary,
    Success,
    Warning,
    Error,
    Info,
}
```

### LinearProgress

#### Constructor

```rust
LinearProgress::new(progress: f32) -> Self
LinearProgress::indeterminate() -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.height()` | `f32` | `4.0` | Bar height in pixels |
| `.width()` | `f32` | Fill available | Bar width in pixels |
| `.color_variant()` | `ProgressColor` | `Primary` | Semantic color variant |
| `.color()` | `Color32` | Theme color | Custom color (overrides variant) |
| `.show_label()` | - | `false` | Show percentage label |

### CircularProgressBar

#### Constructor

```rust
CircularProgressBar::new(progress: f32) -> Self
CircularProgressBar::indeterminate() -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.size()` | `f32` | `48.0` | Circle diameter |
| `.stroke_width()` | `f32` | `4.0` | Ring thickness |
| `.color_variant()` | `ProgressColor` | `Primary` | Semantic color variant |
| `.color()` | `Color32` | Theme color | Custom color (overrides variant) |
| `.show_percentage()` | `bool` | `false` | Show percentage in center |

### RingProgress

#### Constructor

```rust
RingProgress::new(progress: f32) -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.size()` | `f32` | `120.0` | Ring diameter |
| `.thickness()` | `f32` | `12.0` | Ring thickness |
| `.label()` | `String` | `None` | Center label text |
| `.color_variant()` | `ProgressColor` | `Primary` | Semantic color variant |
| `.color()` | `Color32` | Theme color | Custom color (overrides variant) |

## Composition Examples

### Progress in Card

```demo
let theme = ui.ctx().armas_theme();

Card::new()
    .variant(CardVariant::Outlined)
    .title("Upload Progress")
    .width(300.0)
    .show(ui, &theme, |ui| {
        ui.label("Uploading file.pdf");
        ui.add_space(8.0);
        LinearProgress::new(0.65)
            .color_variant(ProgressColor::Info)
            .show_label()
            .show(ui);
    });
```

### Multiple Progress States

```demo
let theme = ui.ctx().armas_theme();

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    Card::new()
        .variant(CardVariant::Filled)
        .width(150.0)
        .show(ui, &theme, |ui| {
            ui.vertical_centered(|ui| {
                CircularProgressBar::new(0.85)
                    .color_variant(ProgressColor::Success)
                    .show_percentage(true)
                    .show(ui);
                ui.label("Tests Passed");
            });
        });

    Card::new()
        .variant(CardVariant::Filled)
        .width(150.0)
        .show(ui, &theme, |ui| {
            ui.vertical_centered(|ui| {
                CircularProgressBar::new(0.45)
                    .color_variant(ProgressColor::Warning)
                    .show_percentage(true)
                    .show(ui);
                ui.label("Code Coverage");
            });
        });
});
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `success`, `warning`, `error`, `info`, `surface_variant`
- Minimum version: `armas 0.2.0`
