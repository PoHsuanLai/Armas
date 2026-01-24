# Popover

Floating panels anchored to elements with smooth animations and automatic positioning.

## Basic Usage

```demo
let popover_id = ui.id().with("popover_basic");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("my_popover")
    })
});

let state_id = ui.id().with("popover_open");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Click me").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Popover content");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
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

let button_response = Button::new("Primary").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Primary color theme");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
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

let button_response = Button::new("Success").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Success color theme");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
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

let button_response = Button::new("Warning").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Warning color theme");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
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

let button_response = Button::new("Error").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Error color theme");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
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

let button_response = Button::new("Info").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Info color theme");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Styles

Popover comes with multiple visual styles to match different design needs.

### Default Style

```demo
let popover_id = ui.id().with("popover_style_default");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("style_default").style(PopoverStyle::Default)
    })
});

let state_id = ui.id().with("popover_open_style_default");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Default").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Soft border with moderate rounding");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Elevated Style

```demo
let popover_id = ui.id().with("popover_style_elevated");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("style_elevated").style(PopoverStyle::Elevated)
    })
});

let state_id = ui.id().with("popover_open_style_elevated");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Elevated").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Lighter border, more rounding, extra padding");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Bordered Style

```demo
let popover_id = ui.id().with("popover_style_bordered");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("style_bordered").style(PopoverStyle::Bordered)
    })
});

let state_id = ui.id().with("popover_open_style_bordered");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Bordered").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Strong border for emphasis");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Flat Style

```demo
let popover_id = ui.id().with("popover_style_flat");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("style_flat").style(PopoverStyle::Flat)
    })
});

let state_id = ui.id().with("popover_open_style_flat");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Flat").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| {
    ui.label("Minimalist, no border");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Positions

### Bottom

```demo
let popover_id = ui.id().with("popover_pos_bottom");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("bottom").position(PopoverPosition::Bottom)
    })
});

let state_id = ui.id().with("popover_open_pos_bottom");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Bottom").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Appears below");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Top

```demo
let popover_id = ui.id().with("popover_pos_top");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("top").position(PopoverPosition::Top)
    })
});

let state_id = ui.id().with("popover_open_pos_top");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Top").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Appears above");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Left

```demo
let popover_id = ui.id().with("popover_pos_left");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("left").position(PopoverPosition::Left)
    })
});

let state_id = ui.id().with("popover_open_pos_left");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Left").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Appears on left");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Right

```demo
let popover_id = ui.id().with("popover_pos_right");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("right").position(PopoverPosition::Right)
    })
});

let state_id = ui.id().with("popover_open_pos_right");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Right").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Appears on right");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

### Auto

```demo
let popover_id = ui.id().with("popover_pos_auto");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("auto").position(PopoverPosition::Auto)
    })
});

let state_id = ui.id().with("popover_open_pos_auto");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Auto").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Automatically positioned based on space");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Fixed Width

```demo
let popover_id = ui.id().with("popover_fixed_width");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("fixed_width").width(200.0)
    })
});

let state_id = ui.id().with("popover_open_fixed_width");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Show").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("This popover has a fixed width of 200px");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Max Width

```demo
let popover_id = ui.id().with("popover_max_width");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("max_width").max_width(400.0)
    })
});

let state_id = ui.id().with("popover_open_max_width");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Show").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Maximum width is 400px");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Without Arrow

```demo
let popover_id = ui.id().with("popover_no_arrow");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("no_arrow").show_arrow(false)
    })
});

let state_id = ui.id().with("popover_open_no_arrow");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("No Arrow").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("No arrow indicator");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Custom Offset

```demo
let popover_id = ui.id().with("popover_offset");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("offset").offset(egui::vec2(0.0, 16.0))
    })
});

let state_id = ui.id().with("popover_open_offset");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Offset").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Custom offset from anchor");
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Rich Content

```demo
let popover_id = ui.id().with("popover_rich");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("rich").width(250.0)
    })
});

let state_id = ui.id().with("popover_open_rich");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("User Info").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.vertical(|ui| {
        ui.heading("John Doe");
        ui.label("Software Engineer");
        ui.separator();
        ui.label("john@example.com");
        ui.label("San Francisco, CA");
    });
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## With Buttons

```demo
let popover_id = ui.id().with("popover_with_buttons");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("with_buttons").width(200.0)
    })
});

let state_id = ui.id().with("popover_open_with_buttons");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Actions").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.vertical(|ui| {
        if Button::new("Edit").show(ui).clicked() {
            // Handle edit
        }
        if Button::new("Delete").show(ui).clicked() {
            // Handle delete
        }
        if Button::new("Share").show(ui).clicked() {
            // Handle share
        }
    });
});

if response.clicked_outside || response.should_close {
    is_open = false;
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
```

## Handling Close

```demo
let popover_id = ui.id().with("popover_close_handling");
let mut popover = ui.ctx().data_mut(|d| {
    d.get_temp::<Popover>(popover_id).unwrap_or_else(|| {
        Popover::new("close_handling")
    })
});

let state_id = ui.id().with("popover_open_close_handling");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let anchor_response = Button::new("Toggle").show(ui);
if anchor_response.clicked() {
    is_open = !is_open;
}
popover = popover.open(is_open);
let response = popover.show(ui.ctx(), &theme, anchor_response.rect, |ui| {
    ui.label("Click outside to close");
});

if response.clicked_outside {
    // Handle outside click
}
ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(popover_id, popover);
});
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
