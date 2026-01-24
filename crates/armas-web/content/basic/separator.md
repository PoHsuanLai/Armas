# Separator

Simple divider line. Styled to match shadcn/ui separator.

## Basic Usage

```demo
ui.label("Above");
Separator::new().show(ui);
ui.label("Below");
```

## Vertical

```demo
ui.horizontal(|ui| {
    ui.label("Left");
    Separator::new().vertical().length(20.0).show(ui);
    ui.label("Right");
});
```

## In Lists

```demo
ui.vertical(|ui| {
    ui.label("Item 1");
    Separator::new().show(ui);
    ui.label("Item 2");
    Separator::new().show(ui);
    ui.label("Item 3");
});
```

## Custom Length

```demo
Separator::new().length(200.0).show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create horizontal separator |
| `.horizontal()` | - | - | Set horizontal orientation |
| `.vertical()` | - | - | Set vertical orientation |
| `.length()` | `f32` | auto | Custom length |
| `.show()` | `&mut Ui` | - | Display the separator |

## Styling

- Color: `border`
- Thickness: 1px
- Width: Full available width (horizontal)
- Height: Full available height (vertical)
