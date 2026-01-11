# Bento Grid

Responsive grid layout inspired by bento box design with varying cell sizes.

## Basic Usage

```demo
let items = vec![
    BentoItem::new(|ui| {
        ui.label("Single cell");
    }),
    BentoItem::new(|ui| {
        ui.heading("Wide cell");
    }).span(GridSpan::Wide),
    BentoItem::new(|ui| {
        ui.label("Tall cell");
    }).span(GridSpan::Tall),
];

BentoGrid::new(3, 100.0).show(ui, &theme, items);
```

## Custom Styling

```demo
let items = vec![
    BentoItem::new(|ui| {
        ui.heading("Featured");
    })
    .span(GridSpan::Large)
    .background(theme.primary())
    .corner_radius(16.0)
    .padding(20.0),

    BentoItem::new(|ui| {
        ui.label("Regular item");
    })
    .border(theme.outline()),
];

BentoGrid::new(4, 150.0)
    .gap(16.0)
    .default_background(theme.surface())
    .show(ui, &theme, items);
```

## Grid Spans

```demo
// Available span options:
// - GridSpan::Single (1x1)
// - GridSpan::Wide (2x1)
// - GridSpan::Tall (1x2)
// - GridSpan::Large (2x2)

let items = vec![
    BentoItem::new(|ui| { ui.label("1x1"); }).span(GridSpan::Single),
    BentoItem::new(|ui| { ui.label("2x1"); }).span(GridSpan::Wide),
    BentoItem::new(|ui| { ui.label("1x2"); }).span(GridSpan::Tall),
    BentoItem::new(|ui| { ui.label("2x2"); }).span(GridSpan::Large),
];

BentoGrid::new(3, 120.0).show(ui, &theme, items);
```

## API Reference

### BentoGrid

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(columns, cell_size)` | `(usize, f32)` | - | Create grid with columns and base cell size |
| `.gap()` | `f32` | `12.0` | Gap between cells |
| `.default_background()` | `Color32` | `from_gray(30)` | Default cell background |
| `.default_border()` | `Option<Color32>` | `Some(from_gray(60))` | Default cell border |
| `.show()` | `(&mut Ui, &Theme, items)` | - | Show grid with items |

### BentoItem

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(content)` | `impl FnOnce(&mut Ui)` | - | Create item with content closure |
| `.span()` | `GridSpan` | `Single` | Set grid span |
| `.background()` | `Color32` | - | Set background color |
| `.border()` | `Color32` | - | Set border color |
| `.corner_radius()` | `f32` | `12.0` | Set corner radius |
| `.padding()` | `f32` | `16.0` | Set padding |

### GridSpan

| Variant | Columns | Rows | Description |
|---------|---------|------|-------------|
| `Single` | 1 | 1 | Single cell |
| `Wide` | 2 | 1 | Wide cell |
| `Tall` | 1 | 2 | Tall cell |
| `Large` | 2 | 2 | Large cell |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline`
