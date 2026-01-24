# Menu

Dropdown and context menus with keyboard navigation. Styled to match shadcn/ui dropdown-menu.

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

## Destructive Items

```demo
let state_id = ui.id().with("menu_open_destructive");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("Actions").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("destructive").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("Edit");
    menu.item("Duplicate");
    menu.separator();
    menu.item("Delete").destructive();
});

if response.clicked_outside || response.selected.is_some() {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
});
```

## With Checkboxes

```demo
let state_id = ui.id().with("menu_open_checkbox");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let checkbox_state_id = ui.id().with("checkbox_states");
let mut states = ui.ctx().data_mut(|d| d.get_temp::<[bool; 3]>(checkbox_state_id).unwrap_or([true, false, true]));

let button_response = Button::new("View Options").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("checkbox").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.checkbox("Show Toolbar", states[0]);
    menu.checkbox("Show Sidebar", states[1]);
    menu.checkbox("Show Status Bar", states[2]);
});

if let Some((idx, new_state)) = response.checkbox_toggled {
    states[idx] = new_state;
}

if response.clicked_outside {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(checkbox_state_id, states);
});
```

## With Radio Groups

```demo
let state_id = ui.id().with("menu_open_radio");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let selected_id = ui.id().with("radio_selected");
let mut selected = ui.ctx().data_mut(|d| d.get_temp::<String>(selected_id).unwrap_or("medium".to_string()));

let button_response = Button::new("Font Size").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("radio").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.radio("Small", "size", "small", selected == "small");
    menu.radio("Medium", "size", "medium", selected == "medium");
    menu.radio("Large", "size", "large", selected == "large");
});

if let Some((_, value)) = response.radio_selected {
    selected = value;
}

if response.clicked_outside {
    is_open = false;
}

ui.ctx().data_mut(|d| {
    d.insert_temp(state_id, is_open);
    d.insert_temp(selected_id, selected);
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

## With Submenu

```demo
let state_id = ui.id().with("menu_open_submenu");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let button_response = Button::new("File").show(ui);
if button_response.clicked() {
    is_open = !is_open;
}

let mut menu = Menu::new("submenu_demo").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("New");
    menu.item("Open");
    menu.submenu("Recent Files", |sub| {
        sub.item("document.txt");
        sub.item("image.png");
        sub.item("data.csv");
    });
    menu.separator();
    menu.item("Save");
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
    menu.item("New File").shortcut("Ctrl+N");
    menu.item("Open").shortcut("Ctrl+O");
    menu.submenu("Recent", |sub| {
        sub.item("Project A");
        sub.item("Project B");
    });
    menu.separator();
    menu.item("Save").shortcut("Ctrl+S");
    menu.item("Save As").disabled(true);
    menu.separator();
    menu.item("Delete").destructive().shortcut("Del");
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
| `.width()` | `f32` | `200.0` | Menu width |
| `.show()` | closure | - | Render with closure-based API |

### MenuBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.item()` | `impl Into<String>` | Add menu item |
| `.separator()` | - | Add separator line |
| `.checkbox()` | `(label, checked)` | Add checkbox item |
| `.radio()` | `(label, group, value, selected)` | Add radio item |
| `.submenu()` | `(label, closure)` | Add submenu with nested items |

### MenuItemBuilder (chainable from .item(), .checkbox(), .radio())

| Method | Type | Description |
|--------|------|-------------|
| `.icon()` | `impl Into<String>` | Add icon |
| `.shortcut()` | `impl Into<String>` | Add keyboard shortcut |
| `.disabled()` | `bool` | Disable item |
| `.inset()` | - | Add left padding (align with icon items) |
| `.destructive()` | - | Red destructive styling |

### MenuResponse

| Field | Type | Description |
|-------|------|-------------|
| `selected` | `Option<usize>` | Index of selected item |
| `clicked_outside` | `bool` | Clicked outside menu |
| `checkbox_toggled` | `Option<(usize, bool)>` | Checkbox toggled (index, new state) |
| `radio_selected` | `Option<(String, String)>` | Radio selected (group, value) |
| `is_open` | `bool` | Whether the menu is currently open |

## Keyboard Navigation

- Arrow Up/Down - Navigate items
- Arrow Right - Open submenu (when on submenu item)
- Enter/Space - Select highlighted item
- Escape - Close menu

## Styling (shadcn/ui)

The menu follows shadcn/ui dropdown-menu styling:
- Content: `bg-popover text-popover-foreground border rounded-md p-1 shadow-md`
- Item: `px-2 py-1.5 text-sm rounded-sm`
- Hover: `focus:bg-accent focus:text-accent-foreground`
- Destructive: `text-destructive focus:bg-destructive/10`
- Disabled: `opacity-50`
- Shortcut: `text-muted-foreground text-xs`
- Separator: `bg-border h-px`
- Label: `px-2 py-1.5 text-sm font-medium`
