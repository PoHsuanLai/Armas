# Popover

Floating panels anchored to elements with smooth animations and positioning.

```demo
let popover_id = ui.id().with("popover_basic");
let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new("my_popover")));
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

```demo
let theme = ui.ctx().armas_theme();
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    // Primary
    let popover_id = ui.id().with("popover_primary");
    let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new("color_primary").color(PopoverColor::Primary)));
    let state_id = ui.id().with("popover_open_primary");
    let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
    let button_response = Button::new("Primary").show(ui);
    if button_response.clicked() { is_open = !is_open; }
    popover = popover.open(is_open);
    let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| { ui.label("Primary color theme"); });
    if response.clicked_outside || response.should_close { is_open = false; }
    ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(popover_id, popover); });
    // Success
    let popover_id = ui.id().with("popover_success");
    let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new("color_success").color(PopoverColor::Success)));
    let state_id = ui.id().with("popover_open_success");
    let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
    let button_response = Button::new("Success").show(ui);
    if button_response.clicked() { is_open = !is_open; }
    popover = popover.open(is_open);
    let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| { ui.label("Success color theme"); });
    if response.clicked_outside || response.should_close { is_open = false; }
    ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(popover_id, popover); });
    // Error
    let popover_id = ui.id().with("popover_error");
    let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new("color_error").color(PopoverColor::Error)));
    let state_id = ui.id().with("popover_open_error");
    let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
    let button_response = Button::new("Error").show(ui);
    if button_response.clicked() { is_open = !is_open; }
    popover = popover.open(is_open);
    let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| { ui.label("Error color theme"); });
    if response.clicked_outside || response.should_close { is_open = false; }
    ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(popover_id, popover); });
});
```

## Styles

```demo
let theme = ui.ctx().armas_theme();
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    // Default
    let popover_id = ui.id().with("popover_style_default");
    let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new("style_default")));
    let state_id = ui.id().with("popover_open_style_default");
    let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
    let button_response = Button::new("Default").show(ui);
    if button_response.clicked() { is_open = !is_open; }
    popover = popover.open(is_open);
    let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| { ui.label("Soft border"); });
    if response.clicked_outside || response.should_close { is_open = false; }
    ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(popover_id, popover); });
    // Elevated
    let popover_id = ui.id().with("popover_style_elevated");
    let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new("style_elevated").style(PopoverStyle::Elevated)));
    let state_id = ui.id().with("popover_open_style_elevated");
    let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
    let button_response = Button::new("Elevated").show(ui);
    if button_response.clicked() { is_open = !is_open; }
    popover = popover.open(is_open);
    let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| { ui.label("Extra padding"); });
    if response.clicked_outside || response.should_close { is_open = false; }
    ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(popover_id, popover); });
    // Flat
    let popover_id = ui.id().with("popover_style_flat");
    let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new("style_flat").style(PopoverStyle::Flat)));
    let state_id = ui.id().with("popover_open_style_flat");
    let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
    let button_response = Button::new("Flat").show(ui);
    if button_response.clicked() { is_open = !is_open; }
    popover = popover.open(is_open);
    let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| { ui.label("No border"); });
    if response.clicked_outside || response.should_close { is_open = false; }
    ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(popover_id, popover); });
});
```

## Positions

```demo
let theme = ui.ctx().armas_theme();
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    for (pos, label) in [
        (PopoverPosition::Bottom, "Bottom"),
        (PopoverPosition::Top, "Top"),
        (PopoverPosition::Left, "Left"),
        (PopoverPosition::Right, "Right"),
    ] {
        let popover_id = ui.id().with(label);
        let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new(label).position(pos)));
        let state_id = ui.id().with(format!("open_{}", label));
        let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
        let button_response = Button::new(label).show(ui);
        if button_response.clicked() { is_open = !is_open; }
        popover = popover.open(is_open);
        let response = popover.show(ui.ctx(), &theme, button_response.rect, |ui| { ui.label(format!("Appears {}", label.to_lowercase())); });
        if response.clicked_outside || response.should_close { is_open = false; }
        ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(popover_id, popover); });
    }
});
```

## Rich Content

```demo
let popover_id = ui.id().with("popover_rich");
let mut popover = ui.ctx().data_mut(|d| d.get_temp::<Popover>(popover_id).unwrap_or_else(|| Popover::new("rich").width(250.0)));
let state_id = ui.id().with("popover_open_rich");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));
let theme = ui.ctx().armas_theme();
let anchor_response = Button::new("User Info").show(ui);
if anchor_response.clicked() { is_open = !is_open; }
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
if response.clicked_outside || response.should_close { is_open = false; }
ui.ctx().data_mut(|d| { d.insert_temp(state_id, is_open); d.insert_temp(popover_id, popover); });
```
