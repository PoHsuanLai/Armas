# Card Stack

Stacked cards with 3D perspective and interactive animations.

```demo
CardStack::new(400.0, 250.0).id("card_stack_basic").show(ui, |stack| {
    stack.card("Card 1", "First card in the stack").color(egui::Color32::from_rgb(59, 130, 246));
    stack.card("Card 2", "Second card with different color").color(egui::Color32::from_rgb(147, 51, 234));
    stack.card("Card 3", "Third card in the rotation").color(egui::Color32::from_rgb(236, 72, 153));
});
```

## Custom Timing

```demo
CardStack::new(400.0, 250.0).id("card_stack_timing").rotation_interval(3.0).transition_duration(0.8).show(ui, |stack| {
    stack.card("Fast Rotation", "This stack rotates every 3 seconds").color(egui::Color32::from_rgb(34, 197, 94));
    stack.card("Smooth Transition", "With a smooth 0.8s transition").color(egui::Color32::from_rgb(251, 146, 60));
    stack.card("Auto-Play", "Automatically cycles through cards").color(egui::Color32::from_rgb(168, 85, 247));
});
```

## Manual Control

```demo
CardStack::new(400.0, 250.0).id("card_stack_manual").auto_rotate(false).show(ui, |stack| {
    stack.card("Click Me", "Click anywhere to advance").color(egui::Color32::from_rgb(239, 68, 68));
    stack.card("Manual Control", "You control the rotation").color(egui::Color32::from_rgb(234, 179, 8));
    stack.card("Interactive", "Click to see the next card").color(egui::Color32::from_rgb(14, 165, 233));
});
```
