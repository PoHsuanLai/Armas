# Layout

Armas follows the principle: **use egui's built-in layout primitives**. This page shows you the essential egui patterns, plus Armas layout helpers: AspectRatio and Table.

## Vertical & Horizontal Layouts

Use egui's built-in `ui.vertical()` and `ui.horizontal()`.

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 16.0;
    ui.label("Item 1");
    ui.label("Item 2");
    ui.label("Item 3");
});
```

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    Button::new("Save").variant(ButtonVariant::Filled).show(ui);
    Button::new("Cancel").variant(ButtonVariant::Outlined).show(ui);
});
```

## Grid Layouts

Use `egui::Grid` for tables and grid layouts.

```demo
egui::Grid::new("my_grid")
    .spacing([16.0, 16.0])
    .show(ui, |ui| {
        for i in 1..=6 {
            ui.label(format!("Item {}", i));
            if i % 3 == 0 {
                ui.end_row();
            }
        }
    });
```

## Spacing & Separators

```demo
ui.label("Before spacer");
ui.add_space(24.0);
ui.label("After spacer");
```

```demo
ui.label("Content above");
ui.separator();
ui.label("Content below");
```

## Table (Armas)

Styled tables with multiple visual variants.

```demo
Table::new()
    .style(TableStyle::Striped)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("Name");
            row.cell("Role");
            row.cell("Status");
        });

        table.row(|row| {
            row.cell("Alice");
            row.cell("Engineer");
            row.cell("Active");
        });

        table.row(|row| {
            row.cell("Bob");
            row.cell("Designer");
            row.cell("Away");
        });
    });
```

```demo
Table::new()
    .style(TableStyle::Bordered)
    .compact(true)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("Feature");
            row.cell("Available");
        });

        table.row(|row| {
            row.cell("Striped");
            row.cell("✓");
        });

        table.row(|row| {
            row.cell("Bordered");
            row.cell("✓");
        });
    });
```

## Aspect Ratio (Armas)

Maintains aspect ratios with fit/fill modes.

```demo
AspectRatio::new(16.0 / 9.0)
    .show(ui, |ui| {
        ui.centered_and_justified(|ui| {
            ui.label("16:9 content");
        });
    });
```

```demo
AspectRatio::square()
    .show(ui, |ui| {
        ui.centered_and_justified(|ui| {
            ui.label("1:1 Square");
        });
    });
```

```demo
AspectRatio::widescreen()
    .content_mode(ContentMode::Fill)
    .show(ui, |ui| {
        ui.centered_and_justified(|ui| {
            ui.label("Fill mode");
        });
    });
```

## Max Width Containers

For max-width containers, wrap content in a closure that sets the width.

**Example:** Medium container (960px)

```rust
ui.vertical(|ui| {
    ui.set_max_width(960.0);
    ui.label("Constrained content");
});
```

**Common breakpoints:**
- Small = 600.0
- Medium = 960.0
- Large = 1280.0
- ExtraLarge = 1920.0

## Need More?

For advanced layouts (Flexbox, CSS Grid), use [egui_taffy](https://github.com/PPakalns/egui_taffy/).

## API Reference

### egui Built-ins

| Pattern | Usage |
|---------|-------|
| Vertical layout | `ui.vertical(\|ui\| { ... })` |
| Horizontal layout | `ui.horizontal(\|ui\| { ... })` |
| Grid | `egui::Grid::new("id").show(ui, \|ui\| { ... })` |
| Spacing | `ui.add_space(pixels)` |
| Separator | `ui.separator()` |
| Max width | `ui.set_max_width(width)` (wrap in closure) |

### Table

| Method | Description |
|--------|-------------|
| `Table::new()` | Create new table |
| `.style(TableStyle)` | `Default`, `Striped`, `Bordered`, `Lined` |
| `.compact(bool)` | Reduce padding for dense data |
| `.hoverable(bool)` | Highlight rows on hover |
| `.show(ui, \|table\| { ... })` | Render table content |

### AspectRatio

| Method | Description |
|--------|-------------|
| `AspectRatio::new(ratio)` | Create with width/height ratio |
| `AspectRatio::square()` | 1:1 aspect ratio |
| `AspectRatio::widescreen()` | 16:9 aspect ratio |
| `AspectRatio::standard()` | 4:3 aspect ratio |
| `.content_mode(mode)` | `ContentMode::Fit` or `Fill` |
| `.show(ui, \|ui\| { ... })` | Render with content |
