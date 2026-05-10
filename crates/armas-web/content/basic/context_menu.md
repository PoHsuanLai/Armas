# Context Menu

Right-click context menus. Same menu content as DropdownMenu, triggered by secondary click.

```demo
let response = ui.allocate_response(egui::vec2(300.0, 100.0), egui::Sense::click());
let rect = response.rect;
ui.painter().rect_filled(rect, 4.0, theme.muted());
ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Right-click here", egui::FontId::proportional(14.0), theme.muted_foreground());
let mut menu = ContextMenu::new("ctx_demo");
let menu_response = menu.show(ui.ctx(), &response, |menu| {
    let _ = menu.item("Cut").shortcut("⌘X");
    let _ = menu.item("Copy").shortcut("⌘C");
    let _ = menu.item("Paste").shortcut("⌘V");
    menu.separator();
    let _ = menu.item("Delete").destructive();
});
```

## With Icons

```demo
let response = ui.allocate_response(egui::vec2(300.0, 80.0), egui::Sense::click());
let rect = response.rect;
ui.painter().rect_filled(rect, 4.0, theme.muted());
ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Right-click for file options", egui::FontId::proportional(14.0), theme.muted_foreground());
let mut menu = ContextMenu::new("ctx_icons");
menu.show(ui.ctx(), &response, |menu| {
    let _ = menu.item("Open").icon("📂");
    let _ = menu.item("Rename").icon("✏️");
    let _ = menu.item("Duplicate").icon("📋");
    menu.separator();
    let _ = menu.item("Move to Trash").icon("🗑️").destructive();
});
```

## With Checkboxes

```demo
let response = ui.allocate_response(egui::vec2(300.0, 80.0), egui::Sense::click());
let rect = response.rect;
ui.painter().rect_filled(rect, 4.0, theme.muted());
ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Right-click for view options", egui::FontId::proportional(14.0), theme.muted_foreground());
let state_id = ui.id().with("ctx_checkbox_states");
let states = ui.ctx().data_mut(|d| d.get_temp::<[bool; 3]>(state_id).unwrap_or([true, false, true]));
let mut menu = ContextMenu::new("ctx_checkbox");
let menu_response = menu.show(ui.ctx(), &response, |menu| {
    menu.checkbox("Show Grid", states[0]);
    menu.checkbox("Show Rulers", states[1]);
    menu.checkbox("Snap to Grid", states[2]);
});
if let Some((idx, new_state)) = menu_response.checkbox_toggled {
    let mut s = states;
    s[idx] = new_state;
    ui.ctx().data_mut(|d| d.insert_temp(state_id, s));
}
```
