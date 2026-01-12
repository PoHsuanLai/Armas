# Card Stack

Stacked cards with 3D perspective and interactive animations.

## Basic Usage

```demo
let card_stack = CardStack::new(400.0, 250.0)
    .with_id("card_stack_basic")  // Unique ID for this demo
    .add_card(StackCard {
        title: "Card 1".to_string(),
        description: "First card in the stack".to_string(),
        color: egui::Color32::from_rgb(59, 130, 246),
    })
    .add_card(StackCard {
        title: "Card 2".to_string(),
        description: "Second card with different color".to_string(),
        color: egui::Color32::from_rgb(147, 51, 234),
    })
    .add_card(StackCard {
        title: "Card 3".to_string(),
        description: "Third card in the rotation".to_string(),
        color: egui::Color32::from_rgb(236, 72, 153),
    });
card_stack.show(ui);
```

## With Custom Timing

```demo
let card_stack = CardStack::new(400.0, 250.0)
    .with_id("card_stack_timing")  // Unique ID for this demo
    .with_rotation_interval(3.0)  // Rotate every 3 seconds
    .with_transition_duration(0.8)  // Slower transitions
    .add_card(StackCard {
        title: "Fast Rotation".to_string(),
        description: "This stack rotates every 3 seconds".to_string(),
        color: egui::Color32::from_rgb(34, 197, 94),
    })
    .add_card(StackCard {
        title: "Smooth Transition".to_string(),
        description: "With a smooth 0.8s transition".to_string(),
        color: egui::Color32::from_rgb(251, 146, 60),
    })
    .add_card(StackCard {
        title: "Auto-Play".to_string(),
        description: "Automatically cycles through cards".to_string(),
        color: egui::Color32::from_rgb(168, 85, 247),
    });
card_stack.show(ui);
```

## Manual Control

```demo
let card_stack = CardStack::new(400.0, 250.0)
    .with_id("card_stack_manual")  // Unique ID for this demo
    .with_auto_rotate(false)  // Disable auto-rotation
    .add_card(StackCard {
        title: "Click Me".to_string(),
        description: "Click anywhere to advance".to_string(),
        color: egui::Color32::from_rgb(239, 68, 68),
    })
    .add_card(StackCard {
        title: "Manual Control".to_string(),
        description: "You control the rotation".to_string(),
        color: egui::Color32::from_rgb(234, 179, 8),
    })
    .add_card(StackCard {
        title: "Interactive".to_string(),
        description: "Click to see the next card".to_string(),
        color: egui::Color32::from_rgb(14, 165, 233),
    });
card_stack.show(ui);
```

## API Reference

| Constructor | Parameters | Description |
|-------------|-----------|-------------|
| `::new()` | `(width: f32, height: f32)` | Create new card stack |

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.with_id()` | `impl Hash` | `"card_stack"` | **Required for multiple instances** - Set unique ID |
| `.add_card()` | `StackCard` | - | Add a card to the stack |
| `.with_rotation_interval()` | `f32` | `5.0` | Seconds between rotations |
| `.with_transition_duration()` | `f32` | `0.5` | Duration of card transition |
| `.with_auto_rotate()` | `bool` | `true` | Enable/disable auto-rotation |

## StackCard Fields

| Field | Type | Description |
|-------|------|-------------|
| `title` | `String` | Card title text |
| `description` | `String` | Card description text |
| `color` | `Color32` | Background color |

## Features

- **Auto-rotation**: Cards automatically cycle
- **Progress indicator**: Visual progress bar showing time until next card
- **Smooth transitions**: Animated card transitions with easing
- **3D stack effect**: Cards appear stacked with depth
- **Manual control**: Optional click-to-advance mode
- **State persistence**: State stored in egui memory (survives recreation)

## Important Notes

⚠️ **Multiple Instances**: When using multiple CardStack components, always set a unique ID with `.with_id()` to prevent state sharing:

```rust
// ✅ Good - Each has unique ID
CardStack::new(400.0, 250.0).with_id("stack_1")
CardStack::new(400.0, 250.0).with_id("stack_2")

// ❌ Bad - Will share state
CardStack::new(400.0, 250.0)  // Uses default "card_stack" ID
CardStack::new(400.0, 250.0)  // Same ID!
```

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface_variant`, `outline`
