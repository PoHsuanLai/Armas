# Drawer

Slide-out side panels for navigation, settings, or additional content with smooth animations.

## Interactive Demo

```demo
use egui::Id;

let id = Id::new("drawer_demo_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if Button::new("Open Drawer")
    .variant(ButtonVariant::Filled)
    .show(ui)
    .clicked()
{
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}

let mut drawer = Drawer::new("drawer_demo")
    .open(is_open)
    .position(DrawerPosition::Right)
    .size(DrawerSize::Medium)
    .title("Settings");

let theme = ui.ctx().armas_theme();
let response = drawer.show(ui.ctx(), &theme, |ui| {
    if is_open {
        SectionHeader::new("General Settings", false).show(ui);
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let mut dark_mode = ui.data_mut(|d| d.get_temp::<bool>(Id::new("dark_mode")).unwrap_or(false));
            Toggle::new()
                .id("dark_mode")
                .size(ToggleSize::Medium)
                .show(ui, &mut dark_mode);
            ui.label("Dark Mode");
            ui.data_mut(|d| d.insert_temp(Id::new("dark_mode"), dark_mode));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let mut notifications = ui.data_mut(|d| d.get_temp::<bool>(Id::new("notifications")).unwrap_or(true));
            Toggle::new()
                .id("notifications")
                .size(ToggleSize::Medium)
                .show(ui, &mut notifications);
            ui.label("Enable Notifications");
            ui.data_mut(|d| d.insert_temp(Id::new("notifications"), notifications));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let mut autosave = ui.data_mut(|d| d.get_temp::<bool>(Id::new("autosave")).unwrap_or(false));
            Toggle::new()
                .id("autosave")
                .size(ToggleSize::Medium)
                .show(ui, &mut autosave);
            ui.label("Auto-save changes");
            ui.data_mut(|d| d.insert_temp(Id::new("autosave"), autosave));
        });

        ui.add_space(16.0);

        Button::new("Save Changes")
            .variant(ButtonVariant::Filled)
            .show(ui);
    }
});

if response.closed {
    is_open = false;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
```

## Left Position

```demo
use egui::Id;

let id = Id::new("drawer_left_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if Button::new("Open Left Drawer")
    .variant(ButtonVariant::Outlined)
    .show(ui)
    .clicked()
{
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}

let mut drawer = Drawer::new("drawer_left")
    .open(is_open)
    .position(DrawerPosition::Left)
    .title("Navigation");

let theme = ui.ctx().armas_theme();
let response = drawer.show(ui.ctx(), &theme, |ui| {
    if is_open {
        Button::new("🏠 Home")
            .variant(ButtonVariant::Text)
            .text_align(egui::Align2::LEFT_CENTER)
            .min_size(egui::vec2(200.0, 40.0))
            .show(ui);

        Button::new("📊 Dashboard")
            .variant(ButtonVariant::Text)
            .text_align(egui::Align2::LEFT_CENTER)
            .min_size(egui::vec2(200.0, 40.0))
            .show(ui);

        Button::new("⚙️ Settings")
            .variant(ButtonVariant::Text)
            .text_align(egui::Align2::LEFT_CENTER)
            .min_size(egui::vec2(200.0, 40.0))
            .show(ui);

        Button::new("❓ Help")
            .variant(ButtonVariant::Text)
            .text_align(egui::Align2::LEFT_CENTER)
            .min_size(egui::vec2(200.0, 40.0))
            .show(ui);
    }
});

if response.closed {
    is_open = false;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
```

## Different Sizes

```demo
use egui::Id;

ui.horizontal(|ui| {
    let small_id = Id::new("drawer_small_open");
    let mut small_open = ui.data_mut(|d| d.get_temp::<bool>(small_id).unwrap_or(false));

    if Button::new("Small")
        .variant(ButtonVariant::Text)
        .show(ui)
        .clicked()
    {
        small_open = true;
        ui.data_mut(|d| d.insert_temp(small_id, small_open));
    }

    let mut drawer_small = Drawer::new("drawer_small")
        .open(small_open)
        .size(DrawerSize::Small)
        .title("Small (256px)");

    let theme = ui.ctx().armas_theme();
    let response = drawer_small.show(ui.ctx(), &theme, |ui| {
        if small_open {
            Badge::new("Small")
                .variant(BadgeVariant::Filled)
                .color(BadgeColor::Primary)
                .show(ui);
            ui.label("256px wide");
        }
    });

    if response.closed {
        small_open = false;
        ui.data_mut(|d| d.insert_temp(small_id, small_open));
    }

    let large_id = Id::new("drawer_large_open");
    let mut large_open = ui.data_mut(|d| d.get_temp::<bool>(large_id).unwrap_or(false));

    if Button::new("Large")
        .variant(ButtonVariant::Text)
        .show(ui)
        .clicked()
    {
        large_open = true;
        ui.data_mut(|d| d.insert_temp(large_id, large_open));
    }

    let mut drawer_large = Drawer::new("drawer_large")
        .open(large_open)
        .size(DrawerSize::Large)
        .title("Large (512px)");

    let theme = ui.ctx().armas_theme();
    let response = drawer_large.show(ui.ctx(), &theme, |ui| {
        if large_open {
            Badge::new("Large")
                .variant(BadgeVariant::Filled)
                .color(BadgeColor::Success)
                .show(ui);
            ui.label("512px wide");
        }
    });

    if response.closed {
        large_open = false;
        ui.data_mut(|d| d.insert_temp(large_id, large_open));
    }
});
```

## API Reference

### Drawer

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(id)` | `impl Into<Id>` | - | Create drawer with unique ID |
| `.open()` | `bool` | `false` | Control drawer open state |
| `.position()` | `DrawerPosition` | `Right` | Sets slide-in direction |
| `.size()` | `DrawerSize` | `Medium` | Sets drawer size |
| `.title()` | `&str` | `None` | Sets drawer title |
| `.closable()` | `bool` | `true` | Allow closing with X/ESC/backdrop |
| `.show_backdrop()` | `bool` | `true` | Show dimmed backdrop |
| `.show(&egui::Context, &Theme, impl FnOnce(&mut Ui))` | - | - | Show drawer with content |

### DrawerPosition

| Variant | Description |
|---------|-------------|
| `Left` | Slide from left edge |
| `Right` | Slide from right edge |
| `Top` | Slide from top edge |
| `Bottom` | Slide from bottom edge |

### DrawerSize

| Variant | Width/Height |
|---------|--------------|
| `Small` | 256px |
| `Medium` | 384px |
| `Large` | 512px |
| `Full` | Full screen |
| `Custom(f32)` | Custom size |

### DrawerResponse

| Field | Type | Description |
|-------|------|-------------|
| `closed` | `bool` | Whether drawer was closed this frame |

## Features

- **Smooth Slide Animation**: Cubic easing for natural motion
- **Multiple Close Methods**: X button, backdrop click, or ESC key
- **Backdrop Overlay**: Darkened background that fades with animation
- **Flexible Positioning**: Slide from any edge
- **Customizable Sizes**: Preset sizes or custom dimensions
- **Auto State Management**: Handles open/close state internally

## Dependencies

- `egui = "0.33"`
- Animation system with cubic easing
- Theme colors: `surface`, `on_surface`
