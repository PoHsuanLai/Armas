# Toast

Temporary notification messages with auto-dismiss and animations.

## Basic Usage

```demo
let state_id = egui::Id::new("toast_manager_1");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Toast").show(ui).clicked() {
    toasts.toast("Operation completed successfully");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Variants

### Default

```demo
let state_id = egui::Id::new("toast_manager_2");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Default").show(ui).clicked() {
    toasts.toast("This is a notification");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Destructive (Error)

```demo
let state_id = egui::Id::new("toast_manager_5");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Error").show(ui).clicked() {
    toasts.error("An error occurred");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Positions

### Top Right

```demo
let state_id = egui::Id::new("toast_manager_6");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| ToastManager::new().position(ToastPosition::TopRight))
});

if Button::new("Show Top Right").show(ui).clicked() {
    toasts.toast("Top right notification");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Bottom Right

```demo
let state_id = egui::Id::new("toast_manager_9");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| ToastManager::new().position(ToastPosition::BottomRight))
});

if Button::new("Show Bottom Right").show(ui).clicked() {
    toasts.toast("Bottom right notification");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Custom Toast

```demo
let state_id = egui::Id::new("toast_manager_12");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Custom Toast").show(ui).clicked() {
    toasts.custom()
        .message("Custom notification")
        .title("Important")
        .destructive()
        .duration(std::time::Duration::from_secs(5))
        .dismissible(true)
        .show();
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## With Title

```demo
let state_id = egui::Id::new("toast_manager_13");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show With Title").show(ui).clicked() {
    toasts.custom()
        .message("Your profile has been updated")
        .title("Success")
        .show();
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Custom Duration

```demo
let state_id = egui::Id::new("toast_manager_14");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show 10 Second Toast").show(ui).clicked() {
    toasts.custom()
        .message("This toast stays for 10 seconds")
        .duration(std::time::Duration::from_secs(10))
        .show();
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Non-Dismissible

```demo
let state_id = egui::Id::new("toast_manager_15");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Non-Dismissible").show(ui).clicked() {
    toasts.custom()
        .message("This toast auto-dismisses")
        .dismissible(false)
        .show();
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Maximum Toasts

```demo
let state_id = egui::Id::new("toast_manager_16");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| ToastManager::new().max_toasts(3))
});

if Button::new("Show Multiple Toasts").show(ui).clicked() {
    toasts.toast("First notification");
    toasts.toast("Second notification");
    toasts.toast("Third notification");
    toasts.toast("Fourth replaces first");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## API Reference

### ToastManager

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.position()` | `ToastPosition` | `TopRight` | Toast position |
| `.max_toasts()` | `usize` | `5` | Maximum visible toasts |
| `.toast()` | `&str` | - | Add default toast |
| `.error()` | `&str` | - | Add error/destructive toast |
| `.custom()` | - | - | Create custom toast |

### ToastBuilder

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.message()` | `&str` | Required | Toast message |
| `.title()` | `&str` | `None` | Toast title |
| `.variant()` | `ToastVariant` | `Default` | Toast variant |
| `.destructive()` | - | - | Make destructive |
| `.color()` | `Color32` | theme | Custom color |
| `.duration()` | `Duration` | `3s` | Display duration |
| `.dismissible()` | `bool` | `true` | Allow manual dismiss |
