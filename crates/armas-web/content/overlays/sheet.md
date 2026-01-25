# Sheet

Slide-out side panels styled like shadcn/ui Sheet. Extends from the edge of the screen for navigation, settings, or forms. For mobile-style bottom drawers with drag gestures, use [Drawer](/components/overlays/drawer) instead.

## Interactive Demo

```demo
use egui::Id;

let id = Id::new("sheet_demo_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if Button::new("Open Sheet")
    .variant(ButtonVariant::Default)
    .show(ui)
    .clicked()
{
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}

let mut sheet = Sheet::new("sheet_demo")
    .open(is_open)
    .side(SheetSide::Right)
    .size(SheetSize::Medium)
    .title("Settings")
    .description("Make changes to your preferences here.");

let theme = ui.ctx().armas_theme();
let response = sheet.show(ui.ctx(), &theme, |ui| {
    if is_open {
        ui.horizontal(|ui| {
            let mut dark_mode = ui.data_mut(|d| d.get_temp::<bool>(Id::new("sheet_dark_mode")).unwrap_or(false));
            Toggle::new()
                .id("sheet_dark_mode")
                .size(ToggleSize::Medium)
                .show(ui, &mut dark_mode);
            ui.label("Dark Mode");
            ui.data_mut(|d| d.insert_temp(Id::new("sheet_dark_mode"), dark_mode));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            let mut notifications = ui.data_mut(|d| d.get_temp::<bool>(Id::new("sheet_notifications")).unwrap_or(true));
            Toggle::new()
                .id("sheet_notifications")
                .size(ToggleSize::Medium)
                .show(ui, &mut notifications);
            ui.label("Enable Notifications");
            ui.data_mut(|d| d.insert_temp(Id::new("sheet_notifications"), notifications));
        });

        ui.add_space(16.0);

        Button::new("Save Changes")
            .variant(ButtonVariant::Default)
            .show(ui);
    }
});

if response.closed {
    is_open = false;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
```

## Left Side

```demo
use egui::Id;

let id = Id::new("sheet_left_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if Button::new("Open Left Sheet")
    .variant(ButtonVariant::Outline)
    .show(ui)
    .clicked()
{
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}

let mut sheet = Sheet::new("sheet_left")
    .open(is_open)
    .side(SheetSide::Left)
    .title("Navigation");

let theme = ui.ctx().armas_theme();
let response = sheet.show(ui.ctx(), &theme, |ui| {
    if is_open {
        Button::new("Home")
            .variant(ButtonVariant::Ghost)
            .full_width(true)
            .show(ui);

        Button::new("Dashboard")
            .variant(ButtonVariant::Ghost)
            .full_width(true)
            .show(ui);

        Button::new("Settings")
            .variant(ButtonVariant::Ghost)
            .full_width(true)
            .show(ui);

        Button::new("Help")
            .variant(ButtonVariant::Ghost)
            .full_width(true)
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
    let small_id = Id::new("sheet_small_open");
    let mut small_open = ui.data_mut(|d| d.get_temp::<bool>(small_id).unwrap_or(false));

    if Button::new("Small (320px)")
        .variant(ButtonVariant::Ghost)
        .show(ui)
        .clicked()
    {
        small_open = true;
        ui.data_mut(|d| d.insert_temp(small_id, small_open));
    }

    let mut sheet_small = Sheet::new("sheet_small")
        .open(small_open)
        .size(SheetSize::Small)
        .title("Small Sheet");

    let theme = ui.ctx().armas_theme();
    let response = sheet_small.show(ui.ctx(), &theme, |ui| {
        if small_open {
            Badge::new("Small")
                .variant(BadgeVariant::Filled)
                .show(ui);
            ui.label("320px wide (sm:max-w-sm)");
        }
    });

    if response.closed {
        small_open = false;
        ui.data_mut(|d| d.insert_temp(small_id, small_open));
    }

    let large_id = Id::new("sheet_large_open");
    let mut large_open = ui.data_mut(|d| d.get_temp::<bool>(large_id).unwrap_or(false));

    if Button::new("Large (540px)")
        .variant(ButtonVariant::Ghost)
        .show(ui)
        .clicked()
    {
        large_open = true;
        ui.data_mut(|d| d.insert_temp(large_id, large_open));
    }

    let mut sheet_large = Sheet::new("sheet_large")
        .open(large_open)
        .size(SheetSize::Large)
        .title("Large Sheet");

    let theme = ui.ctx().armas_theme();
    let response = sheet_large.show(ui.ctx(), &theme, |ui| {
        if large_open {
            Badge::new("Large")
                .variant(BadgeVariant::Filled)
                .color(theme.chart_2())
                .show(ui);
            ui.label("540px wide (lg:max-w-lg)");
        }
    });

    if response.closed {
        large_open = false;
        ui.data_mut(|d| d.insert_temp(large_id, large_open));
    }
});
```

## API Reference

### Sheet

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(id)` | `impl Into<Id>` | - | Create sheet with unique ID |
| `.open()` | `bool` | `false` | Control sheet open state |
| `.side()` | `SheetSide` | `Right` | Sets slide-in direction |
| `.size()` | `SheetSize` | `Small` | Sets sheet size |
| `.title()` | `&str` | `None` | Sets sheet title (SheetTitle) |
| `.description()` | `&str` | `None` | Sets description text (SheetDescription) |
| `.show_close_button()` | `bool` | `true` | Show/hide close button |
| `.show_backdrop()` | `bool` | `true` | Show dimmed backdrop |
| `.show(&egui::Context, &Theme, impl FnOnce(&mut Ui))` | - | - | Show sheet with content |

### SheetSide

| Variant | Description |
|---------|-------------|
| `Left` | Slide from left edge |
| `Right` | Slide from right edge (default) |
| `Top` | Slide from top edge |
| `Bottom` | Slide from bottom edge |

### SheetSize

| Variant | Width |
|---------|-------|
| `Small` | 320px (sm:max-w-sm) |
| `Medium` | 420px (max-w-md) |
| `Large` | 540px (lg:max-w-lg) |
| `XLarge` | 672px (xl:max-w-xl) |
| `Full` | Full screen |
| `Custom(f32)` | Custom size |

### SheetResponse

| Field | Type | Description |
|-------|------|-------------|
| `closed` | `bool` | Whether sheet was closed this frame |

## Features

- **Shadcn Sheet Styling**: Matches shadcn/ui Sheet component design
- **Multiple Close Methods**: X button, backdrop click, or ESC key
- **Backdrop Overlay**: 80% opacity black backdrop (bg-black/80)
- **Flexible Positioning**: Slide from any edge
- **Title & Description**: Built-in header section like SheetHeader
- **Customizable Sizes**: Preset sizes matching Tailwind max-w classes

## Comparison with Drawer

| Feature | Sheet | Drawer |
|---------|-------|--------|
| Position | Left, Right, Top, Bottom | Bottom only |
| Drag handle | No | Yes |
| Drag to dismiss | No | Yes |
| Rounded corners | None | Top only |
| Use case | Side navigation, forms | Mobile-style actions |

## Dependencies

- `egui = "0.33"`
- Theme colors: `background`, `foreground`, `muted_foreground`, `border`, `accent`
