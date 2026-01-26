# Sheet

Slide-out side panels for navigation, settings, or forms.

```demo
use egui::Id;
let id = Id::new("sheet_demo_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));
if Button::new("Open Sheet").show(ui).clicked() {
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
let mut sheet = Sheet::new("sheet_demo").open(is_open).side(SheetSide::Right).size(SheetSize::Medium).title("Settings").description("Make changes to your preferences here.");
let theme = ui.ctx().armas_theme();
let response = sheet.show(ui.ctx(), &theme, |ui| {
    if is_open {
        ui.horizontal(|ui| {
            let mut dark_mode = ui.data_mut(|d| d.get_temp::<bool>(Id::new("sheet_dark_mode")).unwrap_or(false));
            Toggle::new().id("sheet_dark_mode").size(ToggleSize::Medium).show(ui, &mut dark_mode);
            ui.label("Dark Mode");
            ui.data_mut(|d| d.insert_temp(Id::new("sheet_dark_mode"), dark_mode));
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let mut notifications = ui.data_mut(|d| d.get_temp::<bool>(Id::new("sheet_notifications")).unwrap_or(true));
            Toggle::new().id("sheet_notifications").size(ToggleSize::Medium).show(ui, &mut notifications);
            ui.label("Enable Notifications");
            ui.data_mut(|d| d.insert_temp(Id::new("sheet_notifications"), notifications));
        });
        ui.add_space(16.0);
        Button::new("Save Changes").show(ui);
    }
});
if response.closed {
    is_open = false;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
```

## Sides

```demo
use egui::Id;
let id = Id::new("sheet_left_open");
let mut is_open = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));
if Button::new("Open Left Sheet").show(ui).clicked() {
    is_open = true;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
let mut sheet = Sheet::new("sheet_left").open(is_open).side(SheetSide::Left).title("Navigation");
let theme = ui.ctx().armas_theme();
let response = sheet.show(ui.ctx(), &theme, |ui| {
    if is_open {
        Button::new("Home").variant(ButtonVariant::Ghost).full_width(true).show(ui);
        Button::new("Dashboard").variant(ButtonVariant::Ghost).full_width(true).show(ui);
        Button::new("Settings").variant(ButtonVariant::Ghost).full_width(true).show(ui);
    }
});
if response.closed {
    is_open = false;
    ui.data_mut(|d| d.insert_temp(id, is_open));
}
```

## Sizes

```demo
use egui::Id;
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let small_id = Id::new("sheet_small_open");
    let mut small_open = ui.data_mut(|d| d.get_temp::<bool>(small_id).unwrap_or(false));
    if Button::new("Small").show(ui).clicked() {
        small_open = true;
        ui.data_mut(|d| d.insert_temp(small_id, small_open));
    }
    let mut sheet_small = Sheet::new("sheet_small").open(small_open).size(SheetSize::Small).title("Small Sheet");
    let theme = ui.ctx().armas_theme();
    let response = sheet_small.show(ui.ctx(), &theme, |ui| {
        if small_open {
            Badge::new("320px").show(ui);
        }
    });
    if response.closed {
        small_open = false;
        ui.data_mut(|d| d.insert_temp(small_id, small_open));
    }
    let large_id = Id::new("sheet_large_open");
    let mut large_open = ui.data_mut(|d| d.get_temp::<bool>(large_id).unwrap_or(false));
    if Button::new("Large").show(ui).clicked() {
        large_open = true;
        ui.data_mut(|d| d.insert_temp(large_id, large_open));
    }
    let mut sheet_large = Sheet::new("sheet_large").open(large_open).size(SheetSize::Large).title("Large Sheet");
    let response = sheet_large.show(ui.ctx(), &theme, |ui| {
        if large_open {
            Badge::new("540px").show(ui);
        }
    });
    if response.closed {
        large_open = false;
        ui.data_mut(|d| d.insert_temp(large_id, large_open));
    }
});
```
