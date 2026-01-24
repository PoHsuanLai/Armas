# Button

Material Design button component with multiple variants.

## Basic Usage

```demo
Button::new("Click me")
    .variant(ButtonVariant::Filled)
    .show(ui);
```

## Variants

### Filled

```demo
Button::new("Filled Button")
    .variant(ButtonVariant::Filled)
    .show(ui);
```

### Filled Tonal

```demo
Button::new("Filled Tonal")
    .variant(ButtonVariant::FilledTonal)
    .show(ui);
```

### Elevated

```demo
Button::new("Elevated")
    .variant(ButtonVariant::Elevated)
    .show(ui);
```

### Outlined

```demo
Button::new("Outlined")
    .variant(ButtonVariant::Outlined)
    .show(ui);
```

### Text

```demo
Button::new("Text")
    .variant(ButtonVariant::Text)
    .show(ui);
```

## Sizes

```demo
Button::new("Small")
    .variant(ButtonVariant::Filled)
    .min_size(egui::vec2(80.0, 32.0))
    .show(ui);
ui.add_space(8.0);
Button::new("Medium")
    .variant(ButtonVariant::Filled)
    .min_size(egui::vec2(100.0, 40.0))
    .show(ui);
ui.add_space(8.0);
Button::new("Large")
    .variant(ButtonVariant::Filled)
    .min_size(egui::vec2(120.0, 48.0))
    .show(ui);
```

## Disabled State

```demo
Button::new("Disabled")
    .variant(ButtonVariant::Filled)
    .enabled(false)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.variant()` | `ButtonVariant` | `Filled` | Sets the button style |
| `.min_size()` | `Vec2` | `(100, 40)` | Sets minimum size |
| `.enabled()` | `bool` | `true` | Enables/disables interaction |
| `.text_align()` | `Align2` | `CENTER` | Sets text alignment |
| `.text_color()` | `Color32` | theme | Custom text color |

## Composition Examples

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    Button::new("Save")
        .variant(ButtonVariant::Filled)
        .show(ui);
    Button::new("Cancel")
        .variant(ButtonVariant::Outlined)
        .show(ui);
});
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `on_surface`
