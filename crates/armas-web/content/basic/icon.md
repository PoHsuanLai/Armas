# Icon

SVG-based icon rendering for egui with runtime parsing and theme colors.

## Features

- **Bring your own icons**: Use any SVG icon set (Lucide, Phosphor, Material, etc.)
- **Runtime parsing**: Parse SVGs with `armas_icon::runtime::parse_svg()`
- **Static data**: Define icon geometry at compile time with `IconData`
- **Themeable**: Dynamic color tinting via `Color32`
- **Scalable**: Render at any size without quality loss

## Icon Widget

Render icons at different sizes using the `Icon` widget.

```demo
use armas::icon::Icon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::from_owned(web_icons::dark()).size(16.0).color(theme.foreground()).show(ui);
    Icon::from_owned(web_icons::dark()).size(24.0).color(theme.foreground()).show(ui);
    Icon::from_owned(web_icons::dark()).size(32.0).color(theme.foreground()).show(ui);
    Icon::from_owned(web_icons::dark()).size(48.0).color(theme.foreground()).show(ui);
});
```

## Theme Colors

Icons pick up any `Color32`, making them easy to tint with theme colors.

```demo
use armas::icon::Icon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;

    Icon::from_owned(web_icons::light()).size(32.0).color(theme.primary()).show(ui);
    Icon::from_owned(web_icons::light()).size(32.0).color(theme.secondary()).show(ui);
    Icon::from_owned(web_icons::light()).size(32.0).color(theme.destructive()).show(ui);
    Icon::from_owned(web_icons::light()).size(32.0).color(theme.foreground()).show(ui);
});
```

## Built-in Component Icons

Components that need icons (alerts, calendars, sheets) draw them procedurally — no bundled SVGs required.

```demo
use armas::icon;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;

    let size = egui::vec2(24.0, 24.0);

    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    icon::draw_close(ui.painter(), rect, theme.foreground());

    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    icon::draw_chevron_left(ui.painter(), rect, theme.foreground());

    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    icon::draw_chevron_right(ui.painter(), rect, theme.foreground());

    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    icon::draw_chevron_down(ui.painter(), rect, theme.foreground());

    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    icon::draw_info(ui.painter(), rect, theme.primary());

    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    icon::draw_error(ui.painter(), rect, theme.destructive());
});
```

## API Reference

### Icon Widget

```rust
Icon::new(icon: &IconData)            // From static data
Icon::from_owned(icon: &OwnedIconData) // From runtime-parsed data
    .size(size: f32)           // Default: 24.0
    .color(color: Color32)     // Default: Color32::WHITE
    .show(ui: &mut Ui) -> Response
```

### Runtime Parsing

Enable the `runtime` feature on `armas-icon` to parse SVGs at runtime:

```rust
use armas_icon::runtime::parse_svg;

let icon_data = parse_svg(include_str!("icons/my_icon.svg"))
    .expect("valid SVG");
```

### Procedural Icons

Built-in drawing functions for component chrome:

```rust
icon::draw_close(painter, rect, color);
icon::draw_chevron_left(painter, rect, color);
icon::draw_chevron_right(painter, rect, color);
icon::draw_chevron_down(painter, rect, color);
icon::draw_info(painter, rect, color);
icon::draw_error(painter, rect, color);
```
