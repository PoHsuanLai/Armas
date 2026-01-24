# Dialog

Overlay dialogs for focused user interactions with backdrop and smooth animations. Styled to match shadcn/ui Dialog conventions.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Dialog").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_1").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut dialog = Dialog::new("dialog_1")
    .title("Confirmation");

dialog.show(ui.ctx(), &theme, |ui| {
    ui.label("Are you sure you want to continue?");
});
```

## Sizes

### Small

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Small Dialog").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_2").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut dialog = Dialog::new("dialog_2")
    .title("Small Dialog")
    .size(DialogSize::Small);

dialog.show(ui.ctx(), &theme, |ui| {
    ui.label("384px max width");
});
```

### Medium

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Medium Dialog").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_3").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut dialog = Dialog::new("dialog_3")
    .title("Medium Dialog")
    .size(DialogSize::Medium);

dialog.show(ui.ctx(), &theme, |ui| {
    ui.label("512px max width (default)");
});
```

### Large

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Large Dialog").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_4").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut dialog = Dialog::new("dialog_4")
    .title("Large Dialog")
    .size(DialogSize::Large);

dialog.show(ui.ctx(), &theme, |ui| {
    ui.label("672px max width");
});
```

### Custom Size

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Custom Size Dialog").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_5").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut dialog = Dialog::new("dialog_5")
    .title("Custom Size")
    .size(DialogSize::Custom(500.0));

dialog.show(ui.ctx(), &theme, |ui| {
    ui.label("Custom max width");
});
```

## With Description

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Dialog with Description").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_6").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut dialog = Dialog::new("dialog_6")
    .title("Edit Profile")
    .description("Make changes to your profile here.");

dialog.show(ui.ctx(), &theme, |ui| {
    ui.label("Content goes here");
});
```

## With Footer Buttons

```demo
let theme = ui.ctx().armas_theme();

if Button::new("Open Dialog with Buttons").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_7").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut dialog = Dialog::new("dialog_7")
    .title("Confirm Action")
    .description("Are you sure you want to proceed?");

dialog.show(ui.ctx(), &theme, |ui| {
    dialog_footer(ui, |ui| {
        if Button::new("Confirm").variant(ButtonVariant::Filled).show(ui).clicked() {
            let state_id = egui::Id::new("dialog_7").with("dialog_state");
            ui.ctx().data_mut(|d| d.insert_temp(state_id, false));
        }
        if Button::new("Cancel").variant(ButtonVariant::Outlined).show(ui).clicked() {
            let state_id = egui::Id::new("dialog_7").with("dialog_state");
            ui.ctx().data_mut(|d| d.insert_temp(state_id, false));
        }
    });
});
```

## Non-Closable

Setting `closable(false)` prevents closing via ESC or backdrop click. User must interact with buttons.

```demo
let theme = ui.ctx().armas_theme();

let state_id = egui::Id::new("dialog_8").with("dialog_state");

if Button::new("Open Non-Closable Dialog").show(ui).clicked() {
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}

let mut dialog = Dialog::new("dialog_8")
    .title("Important")
    .closable(false);

dialog.show(ui.ctx(), &theme, |ui| {
    ui.label("You must click the button to close this.");
    ui.add_space(16.0);
    dialog_footer(ui, |ui| {
        if Button::new("OK").variant(ButtonVariant::Filled).show(ui).clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(state_id, false));
        }
    });
});
```

## API Reference

### Dialog

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.title()` | `&str` | `None` | Dialog title |
| `.description()` | `&str` | `None` | Dialog description (muted text) |
| `.size()` | `DialogSize` | `Medium` | Dialog max width |
| `.closable()` | `bool` | `true` | Allow ESC/backdrop close |
| `.open()` | `bool` | - | External open state control |

### DialogSize

| Variant | Max Width | shadcn equivalent |
|---------|-----------|-------------------|
| `Small` | 384px | sm:max-w-sm |
| `Medium` | 512px | sm:max-w-lg (default) |
| `Large` | 672px | sm:max-w-2xl |
| `ExtraLarge` | 896px | sm:max-w-4xl |
| `FullScreen` | 100% - 32px | - |
| `Custom(f32)` | Custom | - |

### Helper Functions

| Function | Description |
|----------|-------------|
| `dialog_footer(ui, content)` | Right-aligned footer for buttons |
