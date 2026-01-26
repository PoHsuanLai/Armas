# Bento Grid

Modern grid layout with variable-sized tiles inspired by macOS.

```demo
BentoGrid::new().columns(3).cell_size(100.0).show(ui, |grid| {
    grid.item(GridSpan::Single, |ui| { ui.label("Single cell"); });
    grid.item(GridSpan::Wide, |ui| { ui.heading("Wide cell"); });
    grid.item(GridSpan::Tall, |ui| { ui.label("Tall cell"); });
    grid.item(GridSpan::Single, |ui| { ui.label("Another single"); });
});
```

## Grid Spans

```demo
BentoGrid::new().columns(3).cell_size(100.0).gap(12.0).show(ui, |grid| {
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
    grid.item(GridSpan::Single, |ui| { ui.label("Single"); });
    grid.item(GridSpan::Large, |ui| {
        ui.vertical_centered(|ui| {
            ui.label("Large");
            ui.label("(2x2)");
        });
    });
});
```

## Custom Styling

```demo
use egui::Color32;
BentoGrid::new().columns(4).cell_size(120.0).gap(16.0).corner_radius(16.0).padding(20.0).show(ui, |grid| {
    grid.item_with_background(GridSpan::Large, Color32::from_rgb(60, 100, 200), |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Featured");
            ui.label("Large item with custom color");
        });
    });
    grid.item(GridSpan::Single, |ui| { ui.label("Item 1"); });
    grid.item(GridSpan::Wide, |ui| { ui.label("Wide item"); });
    grid.item(GridSpan::Single, |ui| { ui.label("Item 2"); });
});
```

## Responsive Layout

```demo
let available_width = ui.available_width();
let columns = if available_width < 600.0 { 2 } else if available_width < 900.0 { 3 } else { 4 };
BentoGrid::new().columns(columns).cell_size(100.0).show(ui, |grid| {
    for i in 0..6 {
        let span = match i % 4 {
            0 => GridSpan::Wide,
            1 => GridSpan::Tall,
            2 => GridSpan::Large,
            _ => GridSpan::Single,
        };
        grid.item(span, |ui| { ui.label(format!("Item {}", i + 1)); });
    }
});
```
