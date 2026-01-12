# Menu

Dropdown and context menus with keyboard navigation, icons, and badges.

## Basic Usage

```demo
let menu_id = ui.id().with("menu_basic");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("my_menu")
            .item("New")
            .item("Open")
            .separator()
            .item("Save")
            .item("Exit")
    })
});

let state_id = ui.id().with("menu_open_basic");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("File").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## With Icons

```demo
let menu_id = ui.id().with("menu_icons");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("icon_menu")
            .add_item(MenuItem::new("New File").icon("📄"))
            .add_item(MenuItem::new("New Folder").icon("📁"))
            .separator()
            .add_item(MenuItem::new("Settings").icon("⚙"))
            .add_item(MenuItem::new("Help").icon("❓"))
    })
});

let state_id = ui.id().with("menu_open_icons");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Menu").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## With Shortcuts

```demo
let menu_id = ui.id().with("menu_shortcuts");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("shortcuts")
            .add_item(MenuItem::new("Copy").shortcut("Ctrl+C"))
            .add_item(MenuItem::new("Paste").shortcut("Ctrl+V"))
            .add_item(MenuItem::new("Cut").shortcut("Ctrl+X"))
            .separator()
            .add_item(MenuItem::new("Undo").shortcut("Ctrl+Z"))
    })
});

let state_id = ui.id().with("menu_open_shortcuts");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Edit").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## With Badges

```demo
let menu_id = ui.id().with("menu_badges");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("badges")
            .add_item(MenuItem::new("Messages").badge("5", BadgeColor::Primary))
            .add_item(MenuItem::new("Notifications").badge("12", BadgeColor::Error))
            .add_item(MenuItem::new("Updates").badge("New", BadgeColor::Success))
    })
});

let state_id = ui.id().with("menu_open_badges");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("View").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## With Disabled Items

```demo
let menu_id = ui.id().with("menu_disabled");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("disabled")
            .add_item(MenuItem::new("Available"))
            .add_item(MenuItem::new("Unavailable").disabled(true))
            .add_item(MenuItem::new("Another Option"))
    })
});

let state_id = ui.id().with("menu_open_disabled");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Actions").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## Complex Menu

```demo
let menu_id = ui.id().with("menu_complex");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("complex")
            .add_item(
                MenuItem::new("New File")
                    .icon("📄")
                    .shortcut("Ctrl+N")
            )
            .add_item(
                MenuItem::new("Open")
                    .icon("📂")
                    .shortcut("Ctrl+O")
            )
            .separator()
            .add_item(
                MenuItem::new("Save")
                    .icon("💾")
                    .shortcut("Ctrl+S")
            )
            .add_item(
                MenuItem::new("Save As")
                    .icon("💾")
                    .disabled(true)
            )
            .separator()
            .add_item(
                MenuItem::new("Exit")
                    .icon("🚪")
                    .shortcut("Ctrl+Q")
            )
    })
});

let state_id = ui.id().with("menu_open_complex");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("File").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## Custom Position

```demo
let menu_id = ui.id().with("menu_positioned");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("positioned")
            .position(PopoverPosition::Bottom)
            .item("Option 1")
            .item("Option 2")
            .item("Option 3")
    })
});

let state_id = ui.id().with("menu_open_positioned");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Show").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## Custom Width

```demo
let menu_id = ui.id().with("menu_wide");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("wide")
            .width(300.0)
            .item("This is a wide menu item")
            .item("With more space for content")
    })
});

let state_id = ui.id().with("menu_open_wide");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Wide Menu").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## Handling Selection

```demo
let menu_id = ui.id().with("menu_selection");
let mut menu = ui.ctx().data_mut(|d| {
    d.get_temp::<Menu>(menu_id).unwrap_or_else(|| {
        Menu::new("selection")
            .item("Red")
            .item("Green")
            .item("Blue")
    })
});

let state_id = ui.id().with("menu_open_selection");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();

let button_response = Button::new("Choose Color").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

menu = menu.open(is_open);
let response = menu.show(ui.ctx(), &theme, button_response.rect);

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

if let Some(index) = response.selected {
    // Handle menu selection
    // index 0 = Red, 1 = Green, 2 = Blue
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(menu_id, menu);
});
```

## API Reference

### Menu

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.item()` | `&str` | - | Add simple item |
| `.add_item()` | `MenuItem` | - | Add custom item |
| `.separator()` | - | - | Add separator |
| `.position()` | `PopoverPosition` | `Bottom` | Menu position |
| `.width()` | `f32` | `220.0` | Menu width |

### MenuItem

| Method | Type | Description |
|--------|------|-------------|
| `.icon()` | `&str` | Add icon (emoji) |
| `.shortcut()` | `&str` | Add keyboard shortcut |
| `.badge()` | `(&str, BadgeColor)` | Add badge |
| `.disabled()` | `bool` | Disable item |

### MenuResponse

| Field | Type | Description |
|-------|------|-------------|
| `selected` | `Option<usize>` | Index of selected item |
| `clicked_outside` | `bool` | Clicked outside menu |

## Keyboard Navigation

- Arrow Up/Down - Navigate items
- Enter - Select highlighted item
- Escape - Close menu

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface_variant`, `on_surface`, `primary`
- Popover component for positioning
- Badge component for badges
