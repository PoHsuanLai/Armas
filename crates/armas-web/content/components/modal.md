# Modal

Overlay dialogs for focused user interactions with backdrop and smooth animations.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Modal").show(ui).clicked() {
    // Set modal state to open
    let state_id = egui::Id::new("modal_1").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut modal = Modal::new("modal_1")
    .title("Confirmation");

modal.show(ui.ctx(), &theme, |ui| {
    ui.label("Are you sure you want to continue?");
});
```

## Sizes

### Small

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Small Modal").show(ui).clicked() {
    let state_id = egui::Id::new("modal_2").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut modal = Modal::new("modal_2")
    .title("Small Dialog")
    .size(ModalSize::Small);

modal.show(ui.ctx(), &theme, |ui| {
    ui.label("400x300 modal window");
});
```

### Medium

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Medium Modal").show(ui).clicked() {
    let state_id = egui::Id::new("modal_3").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut modal = Modal::new("modal_3")
    .title("Medium Dialog")
    .size(ModalSize::Medium);

modal.show(ui.ctx(), &theme, |ui| {
    ui.label("600x400 modal window");
});
```

### Large

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Large Modal").show(ui).clicked() {
    let state_id = egui::Id::new("modal_4").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut modal = Modal::new("modal_4")
    .title("Large Dialog")
    .size(ModalSize::Large);

modal.show(ui.ctx(), &theme, |ui| {
    ui.label("800x500 modal window");
});
```

### Full Screen

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Full Screen Modal").show(ui).clicked() {
    let state_id = egui::Id::new("modal_5").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut modal = Modal::new("modal_5")
    .title("Full Screen")
    .size(ModalSize::FullScreen);

modal.show(ui.ctx(), &theme, |ui| {
    ui.label("95% of screen size");
});
```

### Custom Size

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Custom Size Modal").show(ui).clicked() {
    let state_id = egui::Id::new("modal_6").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut modal = Modal::new("modal_6")
    .title("Custom Size")
    .size(ModalSize::Custom(500.0, 350.0));

modal.show(ui.ctx(), &theme, |ui| {
    ui.label("Custom dimensions");
});
```

## Without Title

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Modal Without Title").show(ui).clicked() {
    let state_id = egui::Id::new("modal_7").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut modal = Modal::new("modal_7")
    .size(ModalSize::Small);

modal.show(ui.ctx(), &theme, |ui| {
    ui.label("Modal without title bar");
});
```

## Non-Closable

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Non-Closable Modal").show(ui).clicked() {
    let state_id = egui::Id::new("modal_8").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut modal = Modal::new("modal_8")
    .title("Important")
    .closable(false);

modal.show(ui.ctx(), &theme, |ui| {
    ui.label("Cannot close with ESC or backdrop");
});
```

## Confirmation Dialog

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Confirmation Dialog").show(ui).clicked() {
    let state_id = egui::Id::new("confirm_dialog").with("modal_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let state_id = egui::Id::new("confirm_dialog").with("modal_state");
let mut is_open = ui.ctx().data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

let response = confirm_dialog(
    ui.ctx(),
    &theme,
    &mut is_open,
    "Delete Item",
    "Are you sure you want to delete this item?"
);

ui.ctx().data_mut(|d| d.insert_temp(state_id, is_open));

match response {
    ConfirmResponse::Confirm => {
        // Handle confirm
    }
    ConfirmResponse::Cancel => {
        // Handle cancel
    }
    ConfirmResponse::None => {}
}
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.title()` | `&str` | `None` | Modal title |
| `.size()` | `ModalSize` | `Medium` | Modal dimensions |
| `.closable()` | `bool` | `true` | Allow ESC/backdrop close |
| `.backdrop_blur()` | `bool` | `true` | Blur backdrop |

## Helper Functions

| Function | Description |
|----------|-------------|
| `dialog()` | Simple dialog with title and content |
| `confirm_dialog()` | Dialog with Confirm/Cancel buttons |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `on_surface`, `outline`
- Animation system with cubic easing
- Card component for styling
