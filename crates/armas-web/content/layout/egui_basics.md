# egui Basics

This page covers essential egui patterns.

## Vertical Layout

Stack elements vertically using `ui.vertical()`.

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 16.0;
    ui.label("Item 1");
    ui.label("Item 2");
    ui.label("Item 3");
});
```

## Horizontal Layout

Arrange elements horizontally using `ui.horizontal()`.

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    Button::new("Save").variant(ButtonVariant::Filled).show(ui);
    Button::new("Cancel").variant(ButtonVariant::Outlined).show(ui);
    Button::new("Delete").variant(ButtonVariant::Text).show(ui);
});
```

## Grid Layouts

Use `egui::Grid` for tables and grid layouts.

```demo
egui::Grid::new("demo_grid")
    .spacing([16.0, 16.0])
    .show(ui, |ui| {
        for i in 1..=9 {
            ui.label(format!("Item {}", i));
            if i % 3 == 0 {
                ui.end_row();
            }
        }
    });
```

### Form Layouts with Grid

```demo
egui::Grid::new("form_grid")
    .spacing([12.0, 8.0])
    .show(ui, |ui| {
        ui.label("Name:");
        Input::new("Enter name").show(ui, &mut String::new());
        ui.end_row();

        ui.label("Email:");
        Input::new("Enter email").show(ui, &mut String::new());
        ui.end_row();

        ui.label("Role:");
        ui.label("Administrator");
        ui.end_row();
    });
```

## Spacing

Add vertical or horizontal spacing.

```demo
ui.label("Before spacer");
ui.add_space(24.0);
ui.label("After 24px spacer");
ui.add_space(48.0);
ui.label("After 48px spacer");
```

## Max Width Containers

Constrain content width using `ui.set_max_width()`.

```demo
ui.vertical(|ui| {
    ui.set_max_width(400.0);
    ui.label("This content is constrained to 400px width");
    ui.separator();
    ui.label("Even if the container is wider, this text won't exceed the max width");
});
```

### Common Breakpoints

Use these standard widths for responsive layouts:

```demo
ui.vertical(|ui| {
    // Small
    ui.set_max_width(600.0);
    ui.label("📱 Small: 600px");
});

ui.add_space(8.0);

ui.vertical(|ui| {
    // Medium
    ui.set_max_width(960.0);
    ui.label("💻 Medium: 960px");
});

ui.add_space(8.0);

ui.vertical(|ui| {
    // Large
    ui.set_max_width(1280.0);
    ui.label("🖥 Large: 1280px");
});
```

## Centered Layouts

Center content horizontally and vertically.

```demo
ui.centered_and_justified(|ui| {
    ui.heading("Centered Content");
});
```

```demo
ui.vertical_centered(|ui| {
    ui.label("Horizontally centered");
    Button::new("Click Me").show(ui);
});
```

## Allocate Space

Reserve space for custom rendering.

```demo
let (rect, _response) = ui.allocate_exact_size(
    egui::vec2(200.0, 100.0),
    egui::Sense::hover()
);

ui.painter().rect_filled(
    rect,
    4.0,
    egui::Color32::from_rgb(60, 100, 200)
);

ui.painter().text(
    rect.center(),
    egui::Align2::CENTER_CENTER,
    "Custom rendering",
    egui::FontId::default(),
    egui::Color32::WHITE,
);
```

## API Reference

### Layouts

| Pattern | Usage |
|---------|-------|
| Vertical layout | `ui.vertical(\|ui\| { ... })` |
| Horizontal layout | `ui.horizontal(\|ui\| { ... })` |
| Grid | `egui::Grid::new("id").show(ui, \|ui\| { ... })` |
| Centered | `ui.centered_and_justified(\|ui\| { ... })` |
| Vertical centered | `ui.vertical_centered(\|ui\| { ... })` |
| Horizontal centered | `ui.horizontal_centered(\|ui\| { ... })` |

### Spacing & Sizing

| Method | Description |
|--------|-------------|
| `ui.add_space(pixels)` | Add vertical/horizontal spacing |
| `ui.set_max_width(width)` | Constrain width (wrap in closure) |
| `ui.set_max_height(height)` | Constrain height (wrap in closure) |
| `ui.set_width(width)` | Set exact width |
| `ui.set_height(height)` | Set exact height |
| `ui.allocate_exact_size()` | Reserve exact space for custom rendering |

### Grid Options

| Method | Default | Description |
|--------|---------|-------------|
| `.spacing([x, y])` | `[4.0, 4.0]` | Cell spacing |
| `.striped(bool)` | `false` | Alternate row backgrounds |
| `.min_col_width(width)` | Auto | Minimum column width |
| `.max_col_width(width)` | Auto | Maximum column width |

## Need More?

For advanced layouts (Flexbox, CSS Grid), consider using [egui_taffy](https://github.com/PPakalns/egui_taffy/).
