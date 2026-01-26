# Toast

Temporary notification messages with auto-dismiss and animations.

```demo
let state_id = egui::Id::new("toast_manager_1");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| d.get_temp(state_id).unwrap_or_else(ToastManager::new));
if Button::new("Show Toast").show(ui).clicked() {
    toasts.toast("Operation completed successfully");
}
toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Variants

```demo
let state_id = egui::Id::new("toast_manager_variants");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| d.get_temp(state_id).unwrap_or_else(ToastManager::new));
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    if Button::new("Default").show(ui).clicked() {
        toasts.toast("This is a notification");
    }
    if Button::new("Error").show(ui).clicked() {
        toasts.error("An error occurred");
    }
});
toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Positions

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    let state_id_tr = egui::Id::new("toast_manager_tr");
    let mut toasts_tr: ToastManager = ui.ctx().data_mut(|d| d.get_temp(state_id_tr).unwrap_or_else(|| ToastManager::new().position(ToastPosition::TopRight)));
    if Button::new("Top Right").show(ui).clicked() {
        toasts_tr.toast("Top right notification");
    }
    toasts_tr.show(ui.ctx());
    ui.ctx().data_mut(|d| d.insert_temp(state_id_tr, toasts_tr));
    let state_id_br = egui::Id::new("toast_manager_br");
    let mut toasts_br: ToastManager = ui.ctx().data_mut(|d| d.get_temp(state_id_br).unwrap_or_else(|| ToastManager::new().position(ToastPosition::BottomRight)));
    if Button::new("Bottom Right").show(ui).clicked() {
        toasts_br.toast("Bottom right notification");
    }
    toasts_br.show(ui.ctx());
    ui.ctx().data_mut(|d| d.insert_temp(state_id_br, toasts_br));
});
```

## With Title

```demo
let state_id = egui::Id::new("toast_manager_title");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| d.get_temp(state_id).unwrap_or_else(ToastManager::new));
if Button::new("Show With Title").show(ui).clicked() {
    toasts.custom().message("Your profile has been updated").title("Success").show();
}
toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```

## Custom Duration

```demo
let state_id = egui::Id::new("toast_manager_duration");
let mut toasts: ToastManager = ui.ctx().data_mut(|d| d.get_temp(state_id).unwrap_or_else(ToastManager::new));
if Button::new("10 Second Toast").show(ui).clicked() {
    toasts.custom().message("This toast stays for 10 seconds").duration(std::time::Duration::from_secs(10)).show();
}
toasts.show(ui.ctx());
ui.ctx().data_mut(|d| d.insert_temp(state_id, toasts));
```
