# Flip Words

Animated text that cycles through a list of words with smooth flip transitions.

## Basic Usage

```demo
let words = vec!["Fast", "Powerful", "Beautiful"];
let mut flip = FlipWords::new(words);
flip.show(ui);
```

## Custom Duration

```demo
let words = vec!["Quick", "Smooth", "Elegant"];
let mut flip = FlipWords::new(words)
    .duration(3.0);  // 3 seconds per word

flip.show(ui);
```

## Different Flip Styles

```demo
// Vertical flip (default)
let mut flip = FlipWords::new(vec!["One", "Two", "Three"])
    .style(FlipStyle::Vertical);
flip.show(ui);

// Horizontal flip
let mut flip = FlipWords::new(vec!["Left", "Right"])
    .style(FlipStyle::Horizontal);
flip.show(ui);

// Simple fade
let mut flip = FlipWords::new(vec!["Fade", "In", "Out"])
    .style(FlipStyle::Fade);
flip.show(ui);
```

## Custom Styling

```demo
use egui::Color32;

let words = vec!["Large", "Bold", "Colorful"];
let mut flip = FlipWords::new(words)
    .font_size(48.0)
    .color(Color32::WHITE)
    .highlight(Color32::from_rgb(100, 200, 255));

flip.show(ui);
```

## Complete Example

```demo
use egui::Color32;

let words = vec![
    "Lightning Fast",
    "Beautifully Designed",
    "Highly Customizable",
    "Production Ready",
];

let mut flip = FlipWords::new(words)
    .duration(2.5)
    .style(FlipStyle::Vertical)
    .font_size(32.0)
    .color(Color32::from_gray(220))
    .highlight(theme.primary());

flip.show(ui);
```

## API Reference

### FlipWords

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(words)` | `Vec<impl Into<String>>` | - | Create with words to cycle through |
| `.duration()` | `f32` | `2.5` | Time per word in seconds |
| `.style()` | `FlipStyle` | `Vertical` | Set flip transition style |
| `.font_size()` | `f32` | `24.0` | Set font size |
| `.color()` | `Color32` | `WHITE` | Set text color |
| `.highlight()` | `Color32` | `None` | Set highlight color for active word |
| `.show(&mut Ui)` | - | - | Show animation and update state |

### FlipStyle

| Variant | Description |
|---------|-------------|
| `Vertical` | Vertical flip effect (rotate on X axis) |
| `Horizontal` | Horizontal flip effect (rotate on Y axis) |
| `Fade` | Simple crossfade transition |

## Animation Behavior

- Words automatically cycle in order
- Transition starts 0.5s before word change
- Smooth easing with `CubicInOut` curve
- Continuous animation loop
- Auto-repaints for smooth frames

## State Management

The component maintains internal state:
- Current word index
- Animation timer
- Flip animation progress

You need to store the `FlipWords` instance as mutable to maintain animation state across frames.

## Dependencies

- `egui = "0.33"`
- Animation system: `Animation`, `EasingFunction`
- Theme colors: Uses provided colors (not theme-dependent by default)
