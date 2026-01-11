# Toast

Temporary notification messages with auto-dismiss, animations, and multiple severity levels.

## Basic Usage

```demo
let state_id = egui::Id::new("toast_manager_1");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Info Toast").show(ui).clicked() {
    toasts.info("Operation completed successfully");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Variants

### Info

```demo
let state_id = egui::Id::new("toast_manager_2");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Info").show(ui).clicked() {
    toasts.info("This is an informational message");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Success

```demo
let state_id = egui::Id::new("toast_manager_3");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Success").show(ui).clicked() {
    toasts.success("Changes saved successfully");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Warning

```demo
let state_id = egui::Id::new("toast_manager_4");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Warning").show(ui).clicked() {
    toasts.warning("Please review before continuing");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Error

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
    toasts.info("Top right notification");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Top Center

```demo
let state_id = egui::Id::new("toast_manager_7");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| ToastManager::new().position(ToastPosition::TopCenter))
});

if Button::new("Show Top Center").show(ui).clicked() {
    toasts.info("Top center notification");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Top Left

```demo
let state_id = egui::Id::new("toast_manager_8");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| ToastManager::new().position(ToastPosition::TopLeft))
});

if Button::new("Show Top Left").show(ui).clicked() {
    toasts.info("Top left notification");
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
    toasts.info("Bottom right notification");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Bottom Center

```demo
let state_id = egui::Id::new("toast_manager_10");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| ToastManager::new().position(ToastPosition::BottomCenter))
});

if Button::new("Show Bottom Center").show(ui).clicked() {
    toasts.info("Bottom center notification");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

### Bottom Left

```demo
let state_id = egui::Id::new("toast_manager_11");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(|| ToastManager::new().position(ToastPosition::BottomLeft))
});

if Button::new("Show Bottom Left").show(ui).clicked() {
    toasts.info("Bottom left notification");
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
        .variant(ToastVariant::Warning)
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
        .variant(ToastVariant::Success)
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
        .variant(ToastVariant::Info)
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
        .variant(ToastVariant::Info)
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
    toasts.info("First notification");
    toasts.info("Second notification");
    toasts.info("Third notification");
    toasts.info("Fourth replaces first");
}

toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Multiple Toasts

```demo
let state_id = egui::Id::new("toast_manager_17");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| {
    d.get_temp(state_id).unwrap_or_else(ToastManager::new)
});

if Button::new("Show Multiple Variants").show(ui).clicked() {
    toasts.success("File uploaded");
    toasts.warning("Storage almost full");
    toasts.info("3 new messages");
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
| `.info()` | `&str` | - | Add info toast |
| `.success()` | `&str` | - | Add success toast |
| `.warning()` | `&str` | - | Add warning toast |
| `.error()` | `&str` | - | Add error toast |
| `.custom()` | - | - | Create custom toast |

### ToastBuilder

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.message()` | `&str` | Required | Toast message |
| `.title()` | `&str` | `None` | Toast title |
| `.variant()` | `ToastVariant` | `Info` | Toast severity |
| `.duration()` | `Duration` | `3s` | Display duration |
| `.dismissible()` | `bool` | `true` | Allow manual dismiss |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `success`, `warning`, `error`, `surface_variant`
- Card component for consistent styling
- Badge component for icons
- SpringAnimation for slide-in effects
