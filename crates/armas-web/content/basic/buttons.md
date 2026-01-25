# Button

Button component with shadcn/ui styling.

## Basic Usage

```demo
Button::new("Click me").show(ui);
```

## Variants

### Default

Primary button with high emphasis.

```demo
Button::new("Default Button")
    .variant(ButtonVariant::Default)
    .show(ui);
```

### Secondary

Secondary button with medium emphasis.

```demo
Button::new("Secondary")
    .variant(ButtonVariant::Secondary)
    .show(ui);
```

### Outline

Border with transparent background.

```demo
Button::new("Outline")
    .variant(ButtonVariant::Outline)
    .show(ui);
```

### Ghost

No background, hover shows accent.

```demo
Button::new("Ghost")
    .variant(ButtonVariant::Ghost)
    .show(ui);
```

### Link

Text style with underline on hover.

```demo
Button::new("Link")
    .variant(ButtonVariant::Link)
    .show(ui);
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Button::new("Small")
        .size(ButtonSize::Small)
        .show(ui);
    Button::new("Default")
        .size(ButtonSize::Default)
        .show(ui);
    Button::new("Large")
        .size(ButtonSize::Large)
        .show(ui);
});
```

## Disabled State

```demo
Button::new("Disabled")
    .enabled(false)
    .show(ui);
```

## Full Width

```demo
Button::new("Full Width Button")
    .full_width(true)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.variant()` | `ButtonVariant` | `Default` | Sets the button style |
| `.size()` | `ButtonSize` | `Default` | Sets the button size |
| `.enabled()` | `bool` | `true` | Enables/disables interaction |
| `.full_width()` | `bool` | `false` | Makes button take full width |
| `.min_width()` | `f32` | auto | Sets minimum width |

### ButtonVariant

| Variant | Description |
|---------|-------------|
| `Default` | Primary background, high emphasis |
| `Secondary` | Secondary background, medium emphasis |
| `Outline` | Border with transparent background |
| `Ghost` | No background, hover shows accent |
| `Link` | Text style with underline on hover |

### ButtonSize

| Size | Height |
|------|--------|
| `Small` | 32px |
| `Default` | 36px |
| `Large` | 40px |

## Composition Examples

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    Button::new("Save").show(ui);
    Button::new("Cancel")
        .variant(ButtonVariant::Outline)
        .show(ui);
});
```
