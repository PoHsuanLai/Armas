# Icon

SVG-based icons that are parsed at compile time and rendered with theme colors.

## Features

- **Compile-time parsing**: Zero runtime overhead for SVG parsing
- **Themeable**: Dynamic color tinting with theme colors
- **Scalable**: Render at any size without quality loss
- **Type-safe**: Enum-based icon selection

## Single Icon Test

```demo
use armas::icon::{Icon, TransportIcon};

ui.label("Single Play icon (64px):");
Icon::new(TransportIcon::Play)
    .size(64.0)
    .color(theme.primary())
    .show(ui);
```

## Basic Usage

```demo
use armas::icon::{Icon, TransportIcon};

ui.label("Three icons in horizontal layout:");
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::new(TransportIcon::Play)
        .size(24.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Pause)
        .size(24.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Stop)
        .size(24.0)
        .color(theme.on_surface())
        .show(ui);
});
```

## All Transport Icons

```demo
use armas::icon::{Icon, TransportIcon};

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    if Icon::new(TransportIcon::Play)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui)
        .clicked()
    {
        ui.label("Play clicked");
    }

    if Icon::new(TransportIcon::Pause)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui)
        .clicked()
    {
        ui.label("Pause clicked");
    }

    if Icon::new(TransportIcon::Stop)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui)
        .clicked()
    {
        ui.label("Stop clicked");
    }

    if Icon::new(TransportIcon::Record)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui)
        .clicked()
    {
        ui.label("Record clicked");
    }
});

ui.add_space(8.0);

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::new(TransportIcon::Rewind)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Forward)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Loop)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Metronome)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui);
});
```

## Different Sizes

```demo
use armas::icon::{Icon, TransportIcon};

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::new(TransportIcon::Play)
        .size(16.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Play)
        .size(24.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Play)
        .size(32.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Play)
        .size(48.0)
        .color(theme.on_surface())
        .show(ui);

    Icon::new(TransportIcon::Play)
        .size(64.0)
        .color(theme.on_surface())
        .show(ui);
});
```

## Different Colors

```demo
use armas::icon::{Icon, TransportIcon};

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::new(TransportIcon::Play)
        .size(32.0)
        .color(theme.primary())
        .show(ui);

    Icon::new(TransportIcon::Play)
        .size(32.0)
        .color(theme.secondary())
        .show(ui);

    Icon::new(TransportIcon::Play)
        .size(32.0)
        .color(theme.error())
        .show(ui);

    Icon::new(TransportIcon::Play)
        .size(32.0)
        .color(egui::Color32::from_rgb(255, 165, 0))
        .show(ui);
});
```

## With Labels

```demo
use armas::icon::{Icon, TransportIcon};

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        Icon::new(TransportIcon::Play)
            .size(32.0)
            .color(theme.on_surface())
            .show(ui);
        ui.label("Play");
    });

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        Icon::new(TransportIcon::Pause)
            .size(32.0)
            .color(theme.on_surface())
            .show(ui);
        ui.label("Pause");
    });

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        Icon::new(TransportIcon::Stop)
            .size(32.0)
            .color(theme.on_surface())
            .show(ui);
        ui.label("Stop");
    });

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        Icon::new(TransportIcon::Record)
            .size(32.0)
            .color(theme.error())
            .show(ui);
        ui.label("Record");
    });
});
```

## API Reference

### Icon

```rust
Icon::new(icon: TransportIcon)
    .size(size: f32)           // Default: 24.0
    .color(color: Color32)     // Default: Color32::WHITE
    .show(ui: &mut Ui) -> Response
```

### TransportIcon Enum

```rust
pub enum TransportIcon {
    Play,        // Play button (play.svg)
    Pause,       // Pause button (pause.svg)
    Stop,        // Stop button (stop.svg)
    Record,      // Record button (fad-record.svg)
    Rewind,      // Rewind/back button (back.svg)
    Forward,     // Fast forward button (forward.svg)
    Loop,        // Loop button (loop.svg)
    Metronome,   // Metronome button (fad-metronome.svg)
}
```

## Implementation Details

Icons are:
1. **Parsed at compile time** by the build script (`build.rs`)
2. **Converted to path data** - Bezier curves sampled into point arrays
3. **Rendered with egui::Painter** - No external dependencies at runtime
4. **Stored as static data** - Zero allocation during rendering

The build script processes SVG files in `icons/transport/` and generates Rust code with pre-computed path data.
