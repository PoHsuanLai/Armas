# Sparkles

Twinkling sparkle particle effect overlay.

## Basic Usage

```demo
let mut sparkles = Sparkles::new(800.0, 600.0);
let _ = sparkles.show(ui);
```

## Custom Configuration

```demo
use egui::Color32;

let mut sparkles = Sparkles::new(800.0, 600.0)
    .particle_count(50)
    .colors(vec![
        Color32::from_rgb(255, 215, 0),   // Gold
        Color32::from_rgb(255, 255, 255), // White
        Color32::from_rgb(135, 206, 250), // Sky blue
    ])
    .size_range(3.0, 6.0);

let _response = sparkles.show(ui);
```

## With Content Overlay

```demo
let mut sparkles = Sparkles::new(400.0, 300.0);
sparkles.show_with_content(ui, &theme, |ui| {
    ui.heading("Sparkling Content");
    ui.label("Content with sparkles overlay");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(f32, f32)` | - | Create with width and height |
| `.particle_count()` | `usize` | `30` | Number of sparkles |
| `.colors()` | `Vec<Color32>` | `gold/white/blue` | Sparkle colors |
| `.size_range()` | `(f32, f32)` | `(2.0, 4.0)` | Min and max sparkle size |

## Dependencies

- `egui = "0.33"`
- Theme colors: None (uses built-in colors)
