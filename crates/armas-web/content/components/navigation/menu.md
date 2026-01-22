# Menu

Dropdown and context menus with keyboard navigation, icons, and badges.

## Basic Usage

```demo
let state_id = ui.id().with("menu_open_basic");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("File").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("my_menu").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("New");
    menu.item("Open");
    menu.separator();
    menu.item("Save");
    menu.item("Exit");
});

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
});
```

## With Icons

```demo
let state_id = ui.id().with("menu_open_icons");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("Menu").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("icon_menu").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("New File").icon("📄");
    menu.item("New Folder").icon("📁");
    menu.separator();
    menu.item("Settings").icon("⚙");
    menu.item("Help").icon("❓");
});

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
});
```

## With Shortcuts

```demo
let state_id = ui.id().with("menu_open_shortcuts");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("Edit").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("shortcuts").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("Copy").shortcut("Ctrl+C");
    menu.item("Paste").shortcut("Ctrl+V");
    menu.item("Cut").shortcut("Ctrl+X");
    menu.separator();
    menu.item("Undo").shortcut("Ctrl+Z");
});

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
});
```

## With Badges

```demo
let state_id = ui.id().with("menu_open_badges");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("View").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("badges").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("Messages").badge("5", theme.primary());
    menu.item("Notifications").badge("12", theme.destructive());
    menu.item("Updates").badge("New", theme.chart_2());
});

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
});
```

## With Disabled Items

```demo
let state_id = ui.id().with("menu_open_disabled");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("Actions").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("disabled").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("Available");
    menu.item("Unavailable").disabled(true);
    menu.item("Another Option");
});

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
});
```

## Complex Menu

```demo
let state_id = ui.id().with("menu_open_complex");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("File").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("complex").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("New File").icon("📄").shortcut("Ctrl+N");
    menu.item("Open").icon("📂").shortcut("Ctrl+O");
    menu.separator();
    menu.item("Save").icon("💾").shortcut("Ctrl+S");
    menu.item("Save As").icon("💾").disabled(true);
    menu.separator();
    menu.item("Exit").icon("🚪").shortcut("Ctrl+Q");
});

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
});
```

## Handling Selection

```demo
let state_id = ui.id().with("menu_open_selection");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("Choose Color").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("selection").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("Red");
    menu.item("Green");
    menu.item("Blue");
});

if let Some(index) = response.selected {
    // Handle menu selection
    // index 0 = Red, 1 = Green, 2 = Blue
}

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
});
```

## API Reference

### Menu

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `impl Into<Id>` | - | Create menu with ID |
| `.open()` | `bool` | - | Set open state (external control) |
| `.position()` | `PopoverPosition` | `Bottom` | Menu position |
| `.width()` | `f32` | `220.0` | Menu width |
| `.color()` | `PopoverColor` | - | Menu color |
| `.style()` | `PopoverStyle` | - | Menu style |
| `.show()` | closure | - | Render with closure-based API |

### MenuBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.item()` | `&str` | Add menu item |
| `.separator()` | - | Add separator |

### MenuItemBuilder (chainable from .item())

| Method | Type | Description |
|--------|------|-------------|
| `.icon()` | `&str` | Add icon (emoji) |
| `.shortcut()` | `&str` | Add keyboard shortcut |
| `.badge()` | `(&str, Color32)` | Add badge with color |
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
