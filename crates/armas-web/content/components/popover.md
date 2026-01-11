# Popover

Floating panels anchored to elements with smooth animations and automatic positioning.

## Basic Usage

```demo
let mut popover = Popover::new("my_popover");
let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Click me");
if button_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Popover content");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));

```

## Colors

Popover supports multiple color themes.

### Primary

```demo
let popover_id = ui.id().with("popover_primary");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("color_primary").color(PopoverColor::Primary)
    })
});

let state_id = ui.id().with("popover_open_primary");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Primary");
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Primary color theme");
});
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Success

```demo
let popover_id = ui.id().with("popover_success");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("color_success").color(PopoverColor::Success)
    })
});

let state_id = ui.id().with("popover_open_success");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Success");
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Success color theme");
});
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Warning

```demo
let popover_id = ui.id().with("popover_warning");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("color_warning").color(PopoverColor::Warning)
    })
});

let state_id = ui.id().with("popover_open_warning");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Warning");
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Warning color theme");
});
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Error

```demo
let popover_id = ui.id().with("popover_error");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("color_error").color(PopoverColor::Error)
    })
});

let state_id = ui.id().with("popover_open_error");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Error");
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Error color theme");
});
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Info

```demo
let popover_id = ui.id().with("popover_info");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("color_info").color(PopoverColor::Info)
    })
});

let state_id = ui.id().with("popover_open_info");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Info");
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Info color theme");
});
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Styles

Popover comes with multiple visual styles to match different design needs.

### Default Style

```demo
let mut popover = Popover::new("style_default")
    .style(PopoverStyle::Default);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Default");
if button_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Soft border with moderate rounding");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

### Elevated Style

```demo
let mut popover = Popover::new("style_elevated")
    .style(PopoverStyle::Elevated);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Elevated");
if button_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Lighter border, more rounding, extra padding");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

### Bordered Style

```demo
let mut popover = Popover::new("style_bordered")
    .style(PopoverStyle::Bordered);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Bordered");
if button_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Strong border for emphasis");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

### Flat Style

```demo
let mut popover = Popover::new("style_flat")
    .style(PopoverStyle::Flat);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = ui.button("Flat");
if button_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Minimalist, no border");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## Positions

### Bottom

```demo
let mut popover = Popover::new("bottom")
    .position(PopoverPosition::Bottom);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Bottom");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Appears below");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

### Top

```demo
let mut popover = Popover::new("top")
    .position(PopoverPosition::Top);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Top");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Appears above");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

### Left

```demo
let mut popover = Popover::new("left")
    .position(PopoverPosition::Left);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Left");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Appears on left");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

### Right

```demo
let mut popover = Popover::new("right")
    .position(PopoverPosition::Right);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Right");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Appears on right");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

### Auto

```demo
let mut popover = Popover::new("auto")
    .position(PopoverPosition::Auto);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Auto");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Automatically positioned based on space");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## Fixed Width

```demo
let mut popover = Popover::new("fixed_width")
    .width(200.0);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Show");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("This popover has a fixed width of 200px");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## Max Width

```demo
let mut popover = Popover::new("max_width")
    .max_width(400.0);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Show");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Maximum width is 400px");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## Without Arrow

```demo
let mut popover = Popover::new("no_arrow")
    .show_arrow(false);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("No Arrow");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("No arrow indicator");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## Custom Offset

```demo
let mut popover = Popover::new("offset")
    .offset(egui::vec2(0.0, 16.0));

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Offset");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Custom offset from anchor");
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## Rich Content

```demo
let mut popover = Popover::new("rich")
    .width(250.0);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("User Info");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.vertical(|ui| {
        ui.heading("John Doe");
        ui.label("Software Engineer");
        ui.separator();
        ui.label("📧 john@example.com");
        ui.label("📍 San Francisco, CA");
    });
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## With Buttons

```demo
let mut popover = Popover::new("with_buttons")
    .width(200.0);

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = ui.button("Actions");
if anchor_response.clicked() {
    is_open = !is_open;
}
popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.vertical(|ui| {
        if ui.button("Edit").clicked() {
            // Handle edit
        }
        if ui.button("Delete").clicked() {
            // Handle delete
        }
        if ui.button("Share").clicked() {
            // Handle share
        }
    });
});
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## Handling Close

```demo
let mut popover = Popover::new("close_handling");
let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();


let anchor_response = ui.button("Toggle");
if anchor_response.clicked() {
    is_open = !is_open;
}
let response = popover.open(is_open).show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Click outside to close");
});

if response.clicked_outside {
    // Handle outside click
}
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.color()` | `PopoverColor` | `Surface` | Color theme |
| `.style()` | `PopoverStyle` | `Default` | Visual style variant |
| `.position()` | `PopoverPosition` | `Bottom` | Popover position |
| `.offset()` | `Vec2` | `(0, 12)` | Offset from anchor |
| `.width()` | `f32` | `None` | Fixed width |
| `.max_width()` | `f32` | `400.0` | Maximum width |
| `.show_arrow()` | `bool` | `true` | Show arrow indicator |

### PopoverColor

| Variant | Description |
|---------|-------------|
| `Surface` | Default surface color |
| `Primary` | Primary theme color with subtle tint |
| `Success` | Green success color with subtle tint |
| `Warning` | Yellow warning color with subtle tint |
| `Error` | Red error color with subtle tint |
| `Info` | Blue info color with subtle tint |

### PopoverStyle

| Variant | Description |
|---------|-------------|
| `Default` | Soft border (1px), 12px rounding, 16px padding |
| `Elevated` | Light border (0.5px), 16px rounding, 20px padding |
| `Bordered` | Strong border (2px), 8px rounding, 16px padding |
| `Flat` | No border, 8px rounding, 16px padding |

### PopoverResponse

| Field | Type | Description |
|-------|------|-------------|
| `response` | `Response` | Underlying response |
| `clicked_outside` | `bool` | Whether user clicked outside |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `outline`
- Animation system with cubic easing and scale effects
