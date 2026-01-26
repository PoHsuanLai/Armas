# Menu

Dropdown menus with keyboard navigation and nested submenus.

```demo
let state_id = ui.id().with("menu_open_basic");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let button_response = Button::new("File").show(ui, &theme);
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
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## With Shortcuts

```demo
let state_id = ui.id().with("menu_shortcuts");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let button_response = Button::new("Edit").show(ui, &theme);
if button_response.clicked() { is_open = !is_open; }
let mut menu = Menu::new("shortcuts").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("Copy").shortcut("Ctrl+C");
    menu.item("Paste").shortcut("Ctrl+V");
    menu.item("Cut").shortcut("Ctrl+X");
});
if response.clicked_outside || response.selected.is_some() { is_open = false; }
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## With Checkboxes

```demo
let state_id = ui.id().with("menu_checkbox");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let checkbox_state_id = ui.id().with("checkbox_states");
let mut states = ui.ctx().data_mut(|d| d.get_temp::<[bool; 3]>(checkbox_state_id).unwrap_or([true, false, true]));
let button_response = Button::new("View").show(ui, &theme);
if button_response.clicked() { is_open = !is_open; }
let mut menu = Menu::new("checkbox").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.checkbox("Show Toolbar", states[0]);
    menu.checkbox("Show Sidebar", states[1]);
    menu.checkbox("Show Status Bar", states[2]);
});
if let Some((idx, new_state)) = response.checkbox_toggled {
    states[idx] = new_state;
}
if response.clicked_outside { is_open = false; }
ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(checkbox_state_id, states); });
```

## With Submenus

```demo
let state_id = ui.id().with("menu_submenu");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let button_response = Button::new("File").show(ui, &theme);
if button_response.clicked() { is_open = !is_open; }
let mut menu = Menu::new("submenu").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("New");
    menu.submenu("Recent Files", |sub| {
        sub.item("document.txt");
        sub.item("image.png");
        sub.item("data.csv");
    });
    menu.separator();
    menu.item("Save");
});
if response.clicked_outside || response.selected.is_some() { is_open = false; }
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```

## Destructive Actions

```demo
let state_id = ui.id().with("menu_destructive");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let button_response = Button::new("Actions").show(ui, &theme);
if button_response.clicked() { is_open = !is_open; }
let mut menu = Menu::new("destructive").open(is_open);
let response = menu.show(ui.ctx(), button_response.rect, |menu| {
    menu.item("Edit");
    menu.item("Duplicate");
    menu.separator();
    menu.item("Delete").destructive();
});
if response.clicked_outside || response.selected.is_some() { is_open = false; }
ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));
```
