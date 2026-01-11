# Menu

Dropdown and context menus with keyboard navigation, icons, and badges.

## Basic Usage

```demo
let mut menu = Menu::new("my_menu")
    .item("New")
    .item("Open")
    .separator()
    .item("Save")
    .item("Exit");

let anchor = ui.button("File").rect;
let theme = ui.ctx().armas_theme();
menu.show(ui.ctx(), &theme, anchor);
```

## With Icons

```demo
let mut menu = Menu::new("icon_menu")
    .add_item(MenuItem::new("New File").icon("📄"))
    .add_item(MenuItem::new("New Folder").icon("📁"))
    .separator()
    .add_item(MenuItem::new("Settings").icon("⚙"))
    .add_item(MenuItem::new("Help").icon("❓"));

let anchor = ui.button("Menu").rect;
let theme = ui.ctx().armas_theme();
menu.show(ui.ctx(), &theme, anchor);
```

## With Shortcuts

```demo
let mut menu = Menu::new("shortcuts")
    .add_item(MenuItem::new("Copy").shortcut("Ctrl+C"))
    .add_item(MenuItem::new("Paste").shortcut("Ctrl+V"))
    .add_item(MenuItem::new("Cut").shortcut("Ctrl+X"))
    .separator()
    .add_item(MenuItem::new("Undo").shortcut("Ctrl+Z"));

let anchor = ui.button("Edit").rect;
let theme = ui.ctx().armas_theme();
menu.show(ui.ctx(), &theme, anchor);
```

## With Badges

```demo
let mut menu = Menu::new("badges")
    .add_item(MenuItem::new("Messages").badge("5", BadgeColor::Primary))
    .add_item(MenuItem::new("Notifications").badge("12", BadgeColor::Error))
    .add_item(MenuItem::new("Updates").badge("New", BadgeColor::Success));

let anchor = ui.button("View").rect;
let theme = ui.ctx().armas_theme();
menu.show(ui.ctx(), &theme, anchor);
```

## With Disabled Items

```demo
let mut menu = Menu::new("disabled")
    .add_item(MenuItem::new("Available"))
    .add_item(MenuItem::new("Unavailable").disabled(true))
    .add_item(MenuItem::new("Another Option"));

let anchor = ui.button("Actions").rect;
let theme = ui.ctx().armas_theme();
menu.show(ui.ctx(), &theme, anchor);
```

## Complex Menu

```demo
let mut menu = Menu::new("complex")
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
    );

let anchor = ui.button("File").rect;
let theme = ui.ctx().armas_theme();
menu.show(ui.ctx(), &theme, anchor);
```

## Custom Position

```demo
let mut menu = Menu::new("positioned")
    .position(PopoverPosition::Bottom)
    .item("Option 1")
    .item("Option 2")
    .item("Option 3");

let anchor = ui.button("Show").rect;
let theme = ui.ctx().armas_theme();
menu.show(ui.ctx(), &theme, anchor);
```

## Custom Width

```demo
let mut menu = Menu::new("wide")
    .width(300.0)
    .item("This is a wide menu item")
    .item("With more space for content");

let anchor = ui.button("Wide Menu").rect;
let theme = ui.ctx().armas_theme();
menu.show(ui.ctx(), &theme, anchor);
```

## Handling Selection

```demo
let mut menu = Menu::new("selection")
    .item("Red")
    .item("Green")
    .item("Blue");

let theme = ui.ctx().armas_theme();
let anchor = ui.button("Choose Color").rect;
let response = menu.show(ui.ctx(), &theme, anchor);

if let Some(index) = response.selected {
    // Handle menu selection
    // index 0 = Red, 1 = Green, 2 = Blue
}
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
