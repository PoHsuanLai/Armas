# Separator

Visual divider between UI sections using egui's built-in separator.

## Basic Usage

```demo
ui.label("Content above");
ui.separator();
ui.label("Content below");
```

## Horizontal Separator

Default separator creates a horizontal line.

```demo
ui.vertical(|ui| {
    ui.label("Section 1");
    ui.separator();
    ui.label("Section 2");
    ui.separator();
    ui.label("Section 3");
});
```

## Separator with Spacing

```demo
ui.label("Content");
ui.add_space(16.0);
ui.separator();
ui.add_space(16.0);
ui.label("More content");
```

## In Horizontal Layouts

Separators work differently in horizontal contexts.

```demo
ui.horizontal(|ui| {
    ui.label("Left");
    ui.separator();
    ui.label("Center");
    ui.separator();
    ui.label("Right");
});
```

## With Custom Spacing

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 24.0;

    ui.label("Item 1");
    ui.separator();
    ui.label("Item 2");
    ui.separator();
    ui.label("Item 3");
});
```

## In Grids

```demo
egui::Grid::new("separator_grid")
    .spacing([16.0, 8.0])
    .show(ui, |ui| {
        ui.label("Name");
        ui.label("Value");
        ui.end_row();

        ui.separator();
        ui.separator();
        ui.end_row();

        ui.label("Setting 1");
        ui.label("Enabled");
        ui.end_row();

        ui.separator();
        ui.separator();
        ui.end_row();

        ui.label("Setting 2");
        ui.label("Disabled");
        ui.end_row();
    });
```

## Alternative: Custom Dividers

For more control, draw custom dividers.

```demo
ui.label("Above custom divider");

ui.add_space(8.0);

let rect = ui.available_rect_before_wrap();
let divider_rect = egui::Rect::from_min_size(
    rect.min,
    egui::vec2(rect.width(), 2.0)
);

ui.painter().rect_filled(
    divider_rect,
    0.0,
    egui::Color32::from_rgb(100, 100, 100)
);

ui.add_space(10.0);
ui.label("Below custom divider");
```

## With Different Styles

```demo
ui.vertical(|ui| {
    ui.label("Default separator");
    ui.separator();

    ui.add_space(12.0);

    ui.label("Thicker custom divider");
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(
        egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), 3.0)),
        1.5,
        egui::Color32::from_rgb(80, 120, 200)
    );
    ui.add_space(3.0);

    ui.add_space(12.0);

    ui.label("Dashed style (custom)");
    let rect = ui.available_rect_before_wrap();
    for i in 0..20 {
        let x = rect.min.x + i as f32 * 20.0;
        if x < rect.max.x {
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(x, rect.min.y),
                    egui::vec2(10.0, 1.0)
                ),
                0.0,
                egui::Color32::GRAY
            );
        }
    }
    ui.add_space(1.0);
});
```

## API Reference

### egui::Separator

| Method | Description |
|--------|-------------|
| `ui.separator()` | Add horizontal or vertical separator (context-dependent) |
| `ui.add(egui::Separator::default().horizontal())` | Explicitly horizontal |
| `ui.add(egui::Separator::default().vertical())` | Explicitly vertical |
| `ui.add(egui::Separator::default().spacing(pixels))` | Custom spacing |

### Common Patterns

```rust
// Basic separator
ui.separator();

// Separator with spacing
ui.add_space(16.0);
ui.separator();
ui.add_space(16.0);

// In horizontal layout (becomes vertical)
ui.horizontal(|ui| {
    ui.label("Left");
    ui.separator();
    ui.label("Right");
});
```

## Notes

- Separators automatically adapt to layout direction
- In `ui.vertical()`: creates horizontal line
- In `ui.horizontal()`: creates vertical line
- Respects theme colors automatically
- For custom styling, draw directly with `ui.painter()`
