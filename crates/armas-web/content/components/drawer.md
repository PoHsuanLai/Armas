# Drawer

Slide-out side panels for navigation, settings, or additional content with smooth animations.

## Basic Usage

```demo
let mut drawer = Drawer::new("drawer_1").open(true)
    .position(DrawerPosition::Right)
    .title("Settings");

let theme = ui.ctx().armas_theme();
drawer.show(ui.ctx(), &theme, |ui| {
    ui.label("Drawer content here");
});
```

## Positions

### Left

```demo
let mut drawer = Drawer::new("drawer_2").open(true)
    .position(DrawerPosition::Left)
    .title("Left Panel");

let theme = ui.ctx().armas_theme();
drawer.show(ui.ctx(), &theme, |ui| {
    ui.label("Slides from the left");
});
```

### Right

```demo
let mut drawer = Drawer::new("drawer_3").open(true)
    .position(DrawerPosition::Right)
    .title("Right Panel");

let theme = ui.ctx().armas_theme();
drawer.show(ui.ctx(), &theme, |ui| {
    ui.label("Slides from the right");
});
```

### Top

```demo
let mut drawer = Drawer::new("drawer_4").open(true)
    .position(DrawerPosition::Top)
    .title("Top Panel");

let theme = ui.ctx().armas_theme();
drawer.show(ui.ctx(), &theme, |ui| {
    ui.label("Slides from the top");
});
```

### Bottom

```demo
let mut drawer = Drawer::new("drawer_5").open(true)
    .position(DrawerPosition::Bottom)
    .title("Bottom Panel");

let theme = ui.ctx().armas_theme();
drawer.show(ui.ctx(), &theme, |ui| {
    ui.label("Slides from the bottom");
});
```

## Sizes

```demo
let mut drawer = Drawer::new("drawer_6").open(true)
    .size(DrawerSize::Small)
    .title("Small Drawer");

let theme = ui.ctx().armas_theme();
drawer.show(ui.ctx(), &theme, |ui| {
    ui.label("256px wide drawer");
});
```

### Size Options

- `DrawerSize::Small` - 256px
- `DrawerSize::Medium` - 384px
- `DrawerSize::Large` - 512px
- `DrawerSize::Full` - Full screen
- `DrawerSize::Custom(size)` - Custom size

## Without Backdrop

```demo
let mut drawer = Drawer::new("drawer_7").open(true)
    .show_backdrop(false)
    .title("No Backdrop");

let theme = ui.ctx().armas_theme();
drawer.show(ui.ctx(), &theme, |ui| {
    ui.label("Drawer without backdrop overlay");
});
```

## Non-Closable

```demo
let mut drawer = Drawer::new("drawer_8").open(true)
    .closable(false)
    .title("Modal Drawer");

let theme = ui.ctx().armas_theme();
drawer.show(ui.ctx(), &theme, |ui| {
    ui.label("Cannot close with ESC or backdrop");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.position()` | `DrawerPosition` | `Right` | Sets slide-in direction |
| `.size()` | `DrawerSize` | `Medium` | Sets drawer size |
| `.title()` | `&str` | `None` | Sets drawer title |
| `.closable()` | `bool` | `true` | Allow closing with ESC/backdrop |
| `.show_backdrop()` | `bool` | `true` | Show dimmed backdrop |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `on_surface`
- Animation system with cubic easing
