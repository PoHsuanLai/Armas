# Card Stack

Stacked cards with 3D perspective and interactive animations.

## Basic Usage

```demo
CardStack::new(400.0, 250.0)
    .id("card_stack_basic")  // Unique ID for this demo
    .show(ui, |stack| {
        stack.card("Card 1", "First card in the stack")
            .color(egui::Color32::from_rgb(59, 130, 246));
        stack.card("Card 2", "Second card with different color")
            .color(egui::Color32::from_rgb(147, 51, 234));
        stack.card("Card 3", "Third card in the rotation")
            .color(egui::Color32::from_rgb(236, 72, 153));
    });
```

## With Custom Timing

```demo
CardStack::new(400.0, 250.0)
    .id("card_stack_timing")  // Unique ID for this demo
    .rotation_interval(3.0)  // Rotate every 3 seconds
    .transition_duration(0.8)  // Slower transitions
    .show(ui, |stack| {
        stack.card("Fast Rotation", "This stack rotates every 3 seconds")
            .color(egui::Color32::from_rgb(34, 197, 94));
        stack.card("Smooth Transition", "With a smooth 0.8s transition")
            .color(egui::Color32::from_rgb(251, 146, 60));
        stack.card("Auto-Play", "Automatically cycles through cards")
            .color(egui::Color32::from_rgb(168, 85, 247));
    });
```

## Manual Control

```demo
CardStack::new(400.0, 250.0)
    .id("card_stack_manual")  // Unique ID for this demo
    .auto_rotate(false)  // Disable auto-rotation
    .show(ui, |stack| {
        stack.card("Click Me", "Click anywhere to advance")
            .color(egui::Color32::from_rgb(239, 68, 68));
        stack.card("Manual Control", "You control the rotation")
            .color(egui::Color32::from_rgb(234, 179, 8));
        stack.card("Interactive", "Click to see the next card")
            .color(egui::Color32::from_rgb(14, 165, 233));
    });
```

## API Reference

| Constructor | Parameters | Description |
|-------------|-----------|-------------|
| `::new()` | `(width: f32, height: f32)` | Create new card stack |

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Hash` | `"card_stack"` | **Required for multiple instances** - Set unique ID |
| `.rotation_interval()` | `f32` | `5.0` | Seconds between rotations |
| `.transition_duration()` | `f32` | `0.5` | Duration of card transition |
| `.auto_rotate()` | `bool` | `true` | Enable/disable auto-rotation |
| `.show()` | closure | - | Render with closure-based API |

## CardStackBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.card()` | `(&str, &str)` | Add card with title and description |

## CardBuilder (chainable from .card())

| Method | Type | Description |
|--------|------|-------------|
| `.color()` | `Color32` | Set background color |

## Features

- **Auto-rotation**: Cards automatically cycle
- **Progress indicator**: Visual progress bar showing time until next card
- **Smooth transitions**: Animated card transitions with easing
- **3D stack effect**: Cards appear stacked with depth
- **Manual control**: Optional click-to-advance mode
- **State persistence**: State stored in egui memory (survives recreation)

## Important Notes

⚠️ **Multiple Instances**: When using multiple CardStack components, always set a unique ID with `.id()` to prevent state sharing:

```rust
// ✅ Good - Each has unique ID
CardStack::new(400.0, 250.0).id("stack_1")
CardStack::new(400.0, 250.0).id("stack_2")

// ❌ Bad - Will share state
CardStack::new(400.0, 250.0)  // Uses default "card_stack" ID
CardStack::new(400.0, 250.0)  // Same ID!
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface_variant`, `outline`
