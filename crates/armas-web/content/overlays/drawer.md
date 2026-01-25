# Drawer

Bottom drawer with drag handle and gesture-based dismissal, styled like shadcn/ui Drawer (which uses vaul library). For side panels, use [Sheet](/components/overlays/sheet) instead.

## Interactive Demo

```demo
use egui::Id;

let id = Id::new("drawer_demo_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if Button::new("Open Drawer")
    .variant(ButtonVariant::Default)
    .show(ui)
    .clicked()
{
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}

let mut drawer = Drawer::new("drawer_demo")
    .open(is_open)
    .title("Edit Profile")
    .description("Make changes to your profile here.");

let theme = ui.ctx().armas_theme();
let response = drawer.show(ui.ctx(), &theme, |ui| {
    if is_open {
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

## Custom Height

```demo
use egui::Id;

let id = Id::new("drawer_height_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if Button::new("Open Tall Drawer")
    .variant(ButtonVariant::Outline)
    .show(ui)
    .clicked()
{
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}

let mut drawer = Drawer::new("drawer_height")
    .open(is_open)
    .height(500.0)
    .title("Extended Content");

let theme = ui.ctx().armas_theme();
let response = drawer.show(ui.ctx(), &theme, |ui| {
    if is_open {
        ui.label("This drawer has a custom height of 500px.");
        ui.add_space(16.0);

        for i in 1..=5 {
            ui.label(format!("Item {}", i));
            ui.add_space(8.0);
        }
    }
});

if response.closed {
    is_open = false;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
```

## Without Handle

```demo
use egui::Id;

let id = Id::new("drawer_no_handle_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

if Button::new("No Handle Drawer")
    .variant(ButtonVariant::Ghost)
    .show(ui)
    .clicked()
{
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}

let mut drawer = Drawer::new("drawer_no_handle")
    .open(is_open)
    .show_handle(false)
    .title("No Drag Handle");

let theme = ui.ctx().armas_theme();
let response = drawer.show(ui.ctx(), &theme, |ui| {
    if is_open {
        ui.label("This drawer has no drag handle.");
        ui.label("Close via backdrop click or ESC key.");
    }
});

if response.closed {
    is_open = false;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
```

## API Reference

### Drawer

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(id)` | `impl Into<Id>` | - | Create drawer with unique ID |
| `.open()` | `bool` | `false` | Control drawer open state |
| `.title()` | `&str` | `None` | Sets drawer title |
| `.description()` | `&str` | `None` | Sets description text |
| `.height()` | `f32` | `400.0` | Sets drawer height |
| `.show_handle()` | `bool` | `true` | Show/hide drag handle |
| `.show_backdrop()` | `bool` | `true` | Show dimmed backdrop |
| `.show(&egui::Context, &Theme, impl FnOnce(&mut Ui))` | - | - | Show drawer with content |

### DrawerResponse

| Field | Type | Description |
|-------|------|-------------|
| `closed` | `bool` | Whether drawer was closed this frame |
| `snap_point` | `DrawerSnapPoint` | Current snap point state |

### DrawerSnapPoint

| Variant | Description |
|---------|-------------|
| `Closed` | Fully closed (0%) |
| `Partial(f32)` | Partially open (0.0-1.0) |
| `Full` | Fully open (100%) |

## Features

- **Drag Handle**: Centered muted bar for drag-to-dismiss gesture
- **Gesture Dismissal**: Drag down past 50% or with velocity to close
- **Backdrop Overlay**: 80% opacity black backdrop that fades with drag
- **Rounded Corners**: Top corners rounded like vaul drawer
- **ESC Key**: Press Escape to close
- **Backdrop Click**: Click backdrop to close

## Comparison with Sheet

| Feature | Drawer | Sheet |
|---------|--------|-------|
| Position | Bottom only | Left, Right, Top, Bottom |
| Drag handle | Yes | No |
| Drag to dismiss | Yes | No |
| Rounded corners | Top only | None |
| Use case | Mobile-style actions | Side navigation, forms |

## Dependencies

- `egui = "0.33"`
- Theme colors: `background`, `foreground`, `muted_foreground`, `muted`, `border`
