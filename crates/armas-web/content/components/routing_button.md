# Routing Button

Input/Output routing buttons (Studio One style).

## Basic Usage

```demo
// Input button
if RoutingButton::input("Input L+R", 80.0, 24.0)
    .show(ui)
    .clicked()
{
    // Handle click
}

// Output button
if RoutingButton::output("Main", 80.0, 24.0)
    .show(ui)
    .clicked()
{
    // Handle click
}
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::input(label, width, height)` | `(&str, f32, f32)` | - | Create input routing button |
| `::output(label, width, height)` | `(&str, f32, f32)` | - | Create output routing button |
| `.show(&mut Ui)` | - | - | Show the button |

## Note

This component is designed to match Studio One's I/O display style. Output buttons are slightly lighter than input buttons.

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline_variant`, `on_surface`
