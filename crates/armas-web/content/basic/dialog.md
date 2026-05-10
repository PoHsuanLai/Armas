# Dialog

A modal dialog for focused user interactions with backdrop and smooth animations.

```demo
if Button::new("Open Dialog").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_1").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}
let mut dialog = Dialog::new("dialog_1").title("Confirmation");
dialog.show(ui.ctx(), &theme, |ui| {
    ui.label("Are you sure you want to continue?");
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    if Button::new("Small").show(ui).clicked() {
        let state_id = egui::Id::new("dialog_small").with("dialog_state");
        ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
    }
    if Button::new("Medium").show(ui).clicked() {
        let state_id = egui::Id::new("dialog_medium").with("dialog_state");
        ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
    }
    if Button::new("Large").show(ui).clicked() {
        let state_id = egui::Id::new("dialog_large").with("dialog_state");
        ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
    }
});
Dialog::new("dialog_small").title("Small").size(DialogSize::Small).show(ui.ctx(), &theme, |ui| {
    ui.label("384px max width");
});
Dialog::new("dialog_medium").title("Medium").show(ui.ctx(), &theme, |ui| {
    ui.label("512px max width");
});
Dialog::new("dialog_large").title("Large").size(DialogSize::Large).show(ui.ctx(), &theme, |ui| {
    ui.label("672px max width");
});
```

## With Description

```demo
if Button::new("Open").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_desc").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}
Dialog::new("dialog_desc").title("Edit Profile").description("Make changes to your profile here.").show(ui.ctx(), &theme, |ui| {
    ui.label("Content goes here");
});
```

## With Buttons

```demo
if Button::new("Confirm Action").show(ui).clicked() {
    let state_id = egui::Id::new("dialog_buttons").with("dialog_state");
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}
Dialog::new("dialog_buttons").title("Confirm Action").description("Are you sure you want to proceed?").show(ui.ctx(), &theme, |ui| {
    dialog_footer(ui, |ui| {
        if Button::new("Confirm").variant(ButtonVariant::Default).show(ui).clicked() {
            let state_id = egui::Id::new("dialog_buttons").with("dialog_state");
            ui.ctx().data_mut(|d| d.insert_temp(state_id, false));
        }
        if Button::new("Cancel").variant(ButtonVariant::Outline).show(ui).clicked() {
            let state_id = egui::Id::new("dialog_buttons").with("dialog_state");
            ui.ctx().data_mut(|d| d.insert_temp(state_id, false));
        }
    });
});
```

## Non-Closable

```demo
let state_id = egui::Id::new("dialog_nonclosable").with("dialog_state");
if Button::new("Open").show(ui).clicked() {
    ui.ctx().data_mut(|d| d.insert_temp(state_id, true));
}
Dialog::new("dialog_nonclosable").title("Important").closable(false).show(ui.ctx(), &theme, |ui| {
    ui.label("You must click the button to close this.");
    ui.add_space(16.0);
    dialog_footer(ui, |ui| {
        if Button::new("OK").variant(ButtonVariant::Default).show(ui).clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(state_id, false));
        }
    });
});
```
