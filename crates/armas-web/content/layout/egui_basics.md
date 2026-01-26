# egui Basics

Essential egui layout patterns and utilities.

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 16.0;
    ui.label("Item 1");
    ui.label("Item 2");
    ui.label("Item 3");
});
```

## Horizontal Layout

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 12.0;
    Button::new("Save").variant(ButtonVariant::Filled).show(ui, &theme);
    Button::new("Cancel").variant(ButtonVariant::Outlined).show(ui, &theme);
    Button::new("Delete").variant(ButtonVariant::Text).show(ui, &theme);
});
```

## Grid Layouts

```demo
egui::Grid::new("demo_grid").spacing([16.0, 16.0]).show(ui, |ui| {
    for i in 1..=9 {
        ui.label(format!("Item {}", i));
        if i % 3 == 0 {
            ui.end_row();
        }
    }
});
```

## Form Layouts

```demo
egui::Grid::new("form_grid").spacing([12.0, 8.0]).show(ui, |ui| {
    ui.label("Name:");
    Input::new("Enter name").show(ui, &mut String::new(), &theme);
    ui.end_row();
    ui.label("Email:");
    Input::new("Enter email").show(ui, &mut String::new(), &theme);
    ui.end_row();
    ui.label("Role:");
    ui.label("Administrator");
    ui.end_row();
});
```

## Spacing

```demo
ui.label("Before spacer");
ui.add_space(24.0);
ui.label("After 24px spacer");
ui.add_space(48.0);
ui.label("After 48px spacer");
```

## Max Width

```demo
ui.vertical(|ui| {
    ui.set_max_width(400.0);
    ui.label("This content is constrained to 400px width");
    ui.separator();
    ui.label("Even if the container is wider, this text won't exceed the max width");
});
```

## Centered Layouts

```demo
ui.vertical_centered(|ui| {
    ui.label("Horizontally centered");
    Button::new("Click Me").show(ui, &theme);
});
```

## Custom Rendering

```demo
let (rect, _response) = ui.allocate_exact_size(egui::vec2(200.0, 100.0), egui::Sense::hover());
ui.painter().rect_filled(rect, 4.0, egui::Color32::from_rgb(60, 100, 200));
ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Custom rendering", egui::FontId::default(), egui::Color32::WHITE);
```
