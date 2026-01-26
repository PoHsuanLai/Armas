# Button Variants

Specialty button styles with unique visual effects and animations.

## Shimmer Button

Button with animated shimmer background effect.

```demo
ShimmerButton::new("Shimmer")
    .show(ui, &theme);
```

### Custom Size

```demo
ShimmerButton::new("Large Shimmer")
    .min_size(egui::vec2(150.0, 56.0))
    .show(ui, &theme);
```

### Disabled

```demo
ShimmerButton::new("Disabled")
    .enabled(false)
    .show(ui, &theme);
```

## Brutal Button

Brutalist design with stacked shadows.

```demo
BrutalButton::new("Brutal")
    .show(ui, &theme);
```

### Custom Size

```demo
BrutalButton::new("Large Brutal")
    .min_size(egui::vec2(120.0, 48.0))
    .show(ui, &theme);
```

## Simple Button

Clean, minimalist button style.

```demo
SimpleButton::new("Simple")
    .show(ui, &theme);
```

### Custom Size

```demo
SimpleButton::new("Large Simple")
    .min_size(egui::vec2(140.0, 50.0))
    .show(ui, &theme);
```

## Sketch Button

Hand-drawn sketch style with rough edges.

```demo
SketchButton::new("Sketch")
    .show(ui, &theme);
```

### Custom Size

```demo
SketchButton::new("Large Sketch")
    .min_size(egui::vec2(130.0, 52.0))
    .show(ui, &theme);
```

## Invert Button

Button with color inversion effect on hover.

```demo
InvertButton::new("Invert")
    .show(ui, &theme);
```

### Custom Size

```demo
InvertButton::new("Large Invert")
    .min_size(egui::vec2(140.0, 50.0))
    .show(ui, &theme);
```

## Figma Button

Figma-inspired button style.

```demo
FigmaButton::new("Figma")
    .show(ui, &theme);
```

### Custom Size

```demo
FigmaButton::new("Large Figma")
    .min_size(egui::vec2(140.0, 50.0))
    .show(ui, &theme);
```

## Spotify Button

Spotify-inspired button with brand styling.

```demo
SpotifyButton::new("Spotify")
    .show(ui, &theme);
```

### Custom Size

```demo
SpotifyButton::new("Large Spotify")
    .min_size(egui::vec2(140.0, 50.0))
    .show(ui, &theme);
```

## All Variants Comparison

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 12.0;

    ShimmerButton::new("Shimmer Button").show(ui, &theme);
    BrutalButton::new("Brutal Button").show(ui, &theme);
    SimpleButton::new("Simple Button").show(ui, &theme);
    SketchButton::new("Sketch Button").show(ui, &theme);
    InvertButton::new("Invert Button").show(ui, &theme);
    FigmaButton::new("Figma Button").show(ui, &theme);
    SpotifyButton::new("Spotify Button").show(ui, &theme);
});
```

## API Reference

All specialty buttons share a similar API:

### Common Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `new()` | `String` | - | Create button with text |
| `.min_size()` | `Vec2` | Varies | Set minimum size |
| `.enabled()` | `bool` | `true` | Set enabled state |
| `.show()` | `&mut Ui` | - | Display button, returns Response |

### Default Sizes

- **ShimmerButton**: 100x48
- **BrutalButton**: 80x32
- **SimpleButton**: 80x32
- **SketchButton**: 80x32
- **InvertButton**: 80x32
- **FigmaButton**: 80x32
- **SpotifyButton**: 80x32

## Usage Examples

### Multiple Buttons in Row

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;

    if ShimmerButton::new("Save").show(ui, &theme).clicked() {
        // Handle save
    }

    if BrutalButton::new("Cancel").show(ui, &theme).clicked() {
        // Handle cancel
    }
});
```

### Buttons with State

```demo
let mut enabled = true;

ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 8.0;

    ui.checkbox(&mut enabled, "Enable buttons");

    SimpleButton::new("Action")
        .enabled(enabled)
        .show(ui, &theme);
});
```

### Custom Sized Buttons

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;

    ShimmerButton::new("Small")
        .min_size(egui::vec2(60.0, 28.0))
        .show(ui, &theme);

    ShimmerButton::new("Medium")
        .min_size(egui::vec2(100.0, 36.0))
        .show(ui, &theme);

    ShimmerButton::new("Large")
        .min_size(egui::vec2(140.0, 48.0))
        .show(ui, &theme);
});
```

## Dependencies

- `egui = "0.33"`
- No additional theme dependencies
- Minimum version: `armas 0.2.0`
