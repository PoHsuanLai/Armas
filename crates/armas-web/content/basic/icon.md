# Icon

SVG-based icons parsed at runtime and rendered with theme colors.

## Features

- **Runtime parsing**: SVGs embedded via `include_str!` and parsed once with `OnceLock`
- **Themeable**: Dynamic color tinting with theme colors
- **Scalable**: Render at any size without quality loss

## Single Icon Test

```demo
use armas::icon::Icon;
use armas_audio::icons;

ui.label("Single Play icon (64px):");
Icon::from_owned(icons::play())
    .size(64.0)
    .color(theme.primary())
    .show(ui);
```

## Basic Usage

```demo
use armas::icon::Icon;
use armas_audio::icons;

ui.label("Three icons in horizontal layout:");
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::from_owned(icons::play()).size(24.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::pause()).size(24.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::stop()).size(24.0).color(theme.foreground()).show(ui);
});
```

## All Transport Icons

```demo
use armas::icon::Icon;
use armas_audio::icons;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    if Icon::from_owned(icons::play()).size(32.0).color(theme.foreground()).show(ui).clicked() {
        ui.label("Play clicked");
    }

    if Icon::from_owned(icons::pause()).size(32.0).color(theme.foreground()).show(ui).clicked() {
        ui.label("Pause clicked");
    }

    if Icon::from_owned(icons::stop()).size(32.0).color(theme.foreground()).show(ui).clicked() {
        ui.label("Stop clicked");
    }

    if Icon::from_owned(icons::record()).size(32.0).color(theme.foreground()).show(ui).clicked() {
        ui.label("Record clicked");
    }
});

ui.add_space(8.0);

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::from_owned(icons::rewind()).size(32.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::forward()).size(32.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::loop_icon()).size(32.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::metronome()).size(32.0).color(theme.foreground()).show(ui);
});
```

## Different Sizes

```demo
use armas::icon::Icon;
use armas_audio::icons;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::from_owned(icons::play()).size(16.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::play()).size(24.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::play()).size(32.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::play()).size(48.0).color(theme.foreground()).show(ui);
    Icon::from_owned(icons::play()).size(64.0).color(theme.foreground()).show(ui);
});
```

## Different Colors

```demo
use armas::icon::Icon;
use armas_audio::icons;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::from_owned(icons::play()).size(32.0).color(theme.primary()).show(ui);
    Icon::from_owned(icons::play()).size(32.0).color(theme.secondary()).show(ui);
    Icon::from_owned(icons::play()).size(32.0).color(theme.destructive()).show(ui);
    Icon::from_owned(icons::play()).size(32.0).color(egui::Color32::from_rgb(255, 165, 0)).show(ui);
});
```

## With Labels

```demo
use armas::icon::Icon;
use armas_audio::icons;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 16.0;

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        Icon::from_owned(icons::play()).size(32.0).color(theme.foreground()).show(ui);
        ui.label("Play");
    });

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        Icon::from_owned(icons::pause()).size(32.0).color(theme.foreground()).show(ui);
        ui.label("Pause");
    });

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        Icon::from_owned(icons::stop()).size(32.0).color(theme.foreground()).show(ui);
        ui.label("Stop");
    });

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        Icon::from_owned(icons::record()).size(32.0).color(theme.destructive()).show(ui);
        ui.label("Record");
    });
});
```

## API Reference

### Icon

```rust
Icon::from_owned(icon: &OwnedIconData)
    .size(size: f32)           // Default: 24.0
    .color(color: Color32)     // Default: Color32::WHITE
    .show(ui: &mut Ui) -> Response
```

### Transport Icons (from armas_audio::icons)

```rust
icons::play()       -> &'static OwnedIconData
icons::pause()      -> &'static OwnedIconData
icons::stop()       -> &'static OwnedIconData
icons::record()     -> &'static OwnedIconData
icons::rewind()     -> &'static OwnedIconData
icons::forward()    -> &'static OwnedIconData
icons::loop_icon()  -> &'static OwnedIconData
icons::metronome()  -> &'static OwnedIconData
```

## Implementation Details

Icons are:
1. **Embedded at compile time** via `include_str!()` macros
2. **Parsed once at runtime** using `OnceLock` for lazy initialization
3. **Tessellated into triangles** using Lyon tessellation
4. **Rendered with egui::Painter** - No external dependencies at render time
