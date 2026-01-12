# Routing Button

Audio/MIDI routing button for input/output channel selection.

## Basic Usage

```demo
ui.horizontal(|ui| {
    // Input button
    if RoutingButton::input("Input L+R")
        .show(ui)
        .clicked()
    {
        // Handle click
    }

    // Output button
    if RoutingButton::output("Main")
        .show(ui)
        .clicked()
    {
        // Handle click
    }
});
```

## Multiple Routing Options

```demo
ui.horizontal(|ui| {
    RoutingButton::input("Stereo").show(ui);
    RoutingButton::input("Mono").show(ui);
    RoutingButton::input("MIDI").show(ui);
});

ui.horizontal(|ui| {
    RoutingButton::output("Main").show(ui);
    RoutingButton::output("Bus 1").show(ui);
    RoutingButton::output("Bus 2").show(ui);
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::input(label)` | `&str` | `100x32` | Create input routing button |
| `::output(label)` | `&str` | `100x32` | Create output routing button |
| `.size()` | `(f32, f32)` | - | Set custom width and height |
| `.width()` | `f32` | `100.0` | Set width |
| `.height()` | `f32` | `32.0` | Set height |
| `.show(&mut Ui)` | - | - | Show the button |

## Note

Output buttons are styled slightly lighter than input buttons for visual distinction.

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline_variant`, `on_surface`
