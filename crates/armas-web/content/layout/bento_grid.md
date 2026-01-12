# Bento Grid

Modern grid layout with variable-sized tiles, inspired by macOS and Japanese bento boxes. Now part of the layout module with a builder API similar to Table.

## Basic Usage

```demo
BentoGrid::new()
    .columns(3)
    .cell_size(100.0)
    .show(ui, |grid| {
        grid.item(GridSpan::Single, |ui| {
            ui.label("Single cell");
        });

        grid.item(GridSpan::Wide, |ui| {
            ui.heading("Wide cell");
        });

        grid.item(GridSpan::Tall, |ui| {
            ui.label("Tall cell");
        });

        grid.item(GridSpan::Single, |ui| {
            ui.label("Another single");
        });
    });
```

## Custom Styling

```demo
use egui::Color32;

BentoGrid::new()
    .columns(4)
    .cell_size(120.0)
    .gap(16.0)
    .corner_radius(16.0)
    .padding(20.0)
    .show(ui, |grid| {
        // Featured item with custom background
        grid.item_with_background(GridSpan::Large, Color32::from_rgb(60, 100, 200), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Featured");
                ui.label("Large item with custom color");
            });
        });

        // Regular items
        grid.item(GridSpan::Single, |ui| {
            ui.label("Item 1");
        });

        grid.item(GridSpan::Wide, |ui| {
            ui.label("Wide item");
        });

        grid.item(GridSpan::Single, |ui| {
            ui.label("Item 2");
        });
    });
```

## Grid Spans

All available span options:

```demo
BentoGrid::new()
    .columns(3)
    .cell_size(100.0)
    .gap(12.0)
    .show(ui, |grid| {
        grid.item(GridSpan::Single, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Single");
                ui.label("(1x1)");
            });
        });

        grid.item(GridSpan::Wide, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Wide");
                ui.label("(2x1)");
            });
        });

        grid.item(GridSpan::Tall, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Tall");
                ui.label("(1x2)");
            });
        });

        grid.item(GridSpan::Single, |ui| {
            ui.label("Single");
        });

        grid.item(GridSpan::Large, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Large");
                ui.label("(2x2)");
            });
        });
    });
```

## Responsive Layout

```demo
// Adjust columns based on available space
let available_width = ui.available_width();
let columns = if available_width < 600.0 {
    2
} else if available_width < 900.0 {
    3
} else {
    4
};

BentoGrid::new()
    .columns(columns)
    .cell_size(100.0)
    .show(ui, |grid| {
        for i in 0..6 {
            let span = match i % 4 {
                0 => GridSpan::Wide,
                1 => GridSpan::Tall,
                2 => GridSpan::Large,
                _ => GridSpan::Single,
            };

            grid.item(span, |ui| {
                ui.label(format!("Item {}", i + 1));
            });
        }
    });
```

## API Reference

### BentoGrid

Builder pattern for creating bento grids:

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create grid with default settings |
| `.columns()` | `usize` | `3` | Set number of columns |
| `.cell_size()` | `f32` | `120.0` | Set base cell size (min 50.0) |
| `.gap()` | `f32` | `12.0` | Set gap between cells |
| `.corner_radius()` | `f32` | `12.0` | Set corner radius for cells |
| `.padding()` | `f32` | `16.0` | Set padding inside cells |
| `.show()` | `(&mut Ui, closure)` | - | Show grid with builder closure |

### GridBuilder

Builder methods inside the `show` closure:

| Method | Description |
|--------|-------------|
| `.item(span, content)` | Add item with default styling |
| `.item_with_background(span, color, content)` | Add item with custom background |
| `.item_with_style(span, bg, border, content)` | Add item with full custom styling |

### GridSpan

| Variant | Columns | Rows | Description |
|---------|---------|------|-------------|
| `Single` | 1 | 1 | Single cell (1x1) |
| `Wide` | 2 | 1 | Wide cell (2x1) |
| `Tall` | 1 | 2 | Tall cell (1x2) |
| `Large` | 2 | 2 | Large cell (2x2) |

## Notes

- Uses theme colors by default (`surface` for background, `outline` for border)
- Items automatically wrap to next row when they don't fit
- Minimum cell size is 50.0 to ensure usability
