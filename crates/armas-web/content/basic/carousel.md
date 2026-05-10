# Carousel

A scrollable content strip with snap-to-item navigation.

```demo
let mut carousel = Carousel::new("carousel_demo").height(120.0);
carousel.show(ui, 5, |ui, index| {
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 6.0, theme.muted());
    ui.centered_and_justified(|ui| {
        ui.label(egui::RichText::new(format!("Slide {}", index + 1)).size(18.0));
    });
});
```

## Multiple Visible Items

```demo
let mut carousel = Carousel::new("carousel_multi")
    .item_basis(0.33)
    .gap(12.0)
    .height(100.0);
carousel.show(ui, 9, |ui, index| {
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 6.0, theme.muted());
    ui.centered_and_justified(|ui| {
        ui.label(format!("{}", index + 1));
    });
});
```

## Without Buttons

```demo
let mut carousel = Carousel::new("carousel_no_buttons")
    .show_buttons(false)
    .height(100.0);
carousel.show(ui, 4, |ui, index| {
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 6.0, theme.muted());
    ui.centered_and_justified(|ui| {
        ui.label(format!("Drag to scroll — Slide {}", index + 1));
    });
});
```
