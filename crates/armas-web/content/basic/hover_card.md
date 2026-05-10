# Hover Card

A card that appears when hovering over a trigger element, with configurable open/close delays.

```demo
let trigger = ui.button("@shadcn");
let mut card = HoverCard::new("hover_demo");
card.show(ui.ctx(), &trigger, |ui| {
    ui.label(egui::RichText::new("@shadcn").strong().size(14.0));
    ui.label("The creator of shadcn/ui and taxonomy.");
    ui.label(egui::RichText::new("Joined December 2021").size(12.0));
});
```

## Custom Delays

```demo
let trigger = ui.button("Quick Hover");
let mut card = HoverCard::new("hover_fast")
    .open_delay(0.3)
    .close_delay(0.1);
card.show(ui.ctx(), &trigger, |ui| {
    ui.label("This card opens faster (0.3s) and closes faster (0.1s).");
});
```

## Custom Position and Width

```demo
let trigger = ui.button("User Profile");
let mut card = HoverCard::new("hover_right")
    .position(PopoverPosition::Right)
    .width(280.0);
card.show(ui.ctx(), &trigger, |ui| {
    ui.label(egui::RichText::new("John Doe").strong());
    ui.label("Software Engineer");
    ui.label("San Francisco, CA");
});
```
