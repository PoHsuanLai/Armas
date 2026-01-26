# Drawer

Bottom drawer with drag handle and gesture-based dismissal.

```demo
use egui::Id;
let id = Id::new("drawer_demo_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));
if Button::new("Open Drawer").show(ui, &theme).clicked() {
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
let mut drawer = Drawer::new("drawer_demo").open(is_open).title("Edit Profile").description("Make changes to your profile here.");
let theme = ui.ctx().armas_theme();
let response = drawer.show(ui.ctx(), &theme, |ui| {
    if is_open {
        ui.horizontal(|ui| {
            let mut dark_mode = ui.data_mut(|d| d.get_temp::<bool>(Id::new("dark_mode")).unwrap_or(false));
            Toggle::new().id("dark_mode").size(ToggleSize::Medium).show(ui, &mut dark_mode, &theme);
            ui.label("Dark Mode");
            ui.data_mut(|d| d.insert_temp(Id::new("dark_mode"), dark_mode));
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let mut notifications = ui.data_mut(|d| d.get_temp::<bool>(Id::new("notifications")).unwrap_or(true));
            Toggle::new().id("notifications").size(ToggleSize::Medium).show(ui, &mut notifications, &theme);
            ui.label("Enable Notifications");
            ui.data_mut(|d| d.insert_temp(Id::new("notifications"), notifications));
        });
        ui.add_space(16.0);
        Button::new("Save Changes").show(ui, &theme);
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
if Button::new("Open Tall Drawer").show(ui, &theme).clicked() {
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
let mut drawer = Drawer::new("drawer_height").open(is_open).height(500.0).title("Extended Content");
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
if Button::new("No Handle").show(ui, &theme).clicked() {
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
let mut drawer = Drawer::new("drawer_no_handle").open(is_open).show_handle(false).title("No Drag Handle");
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
