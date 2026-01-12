# Testimonial

Display customer testimonials and reviews with avatars and ratings.

## Basic Usage

```demo
TestimonialGrid::new()
    .columns(1)
    .show(ui, |grid| {
        grid.testimonial(
            "Amazing product! Changed my workflow completely.",
            "John Doe",
            "CEO, Tech Corp"
        )
        .rating(5);
    });
```

## With Avatar

```demo
use egui::Color32;

TestimonialGrid::new()
    .columns(1)
    .show(ui, |grid| {
        grid.testimonial(
            "Great experience working with this tool.",
            "Jane Smith",
            "Designer"
        )
        .avatar("JS")
        .avatar_color(Color32::from_rgb(100, 150, 255))
        .rating(4);
    });
```

## Testimonial Grid

```demo
use armas::components::TestimonialGrid;

TestimonialGrid::new()
    .columns(3)
    .gap(20.0)
    .show(ui, |grid| {
        grid.testimonial("Great product!", "Alice", "Engineer")
            .rating(5);
        grid.testimonial("Love it!", "Bob", "Designer")
            .rating(5);
        grid.testimonial("Excellent!", "Carol", "Manager")
            .rating(4);
    });
```

## API Reference

### TestimonialCard

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(String, String, String)` | - | Create with quote, author, role |
| `.avatar()` | `impl Into<String>` | `None` | Avatar text (emoji/initials) |
| `.avatar_color()` | `Color32` | `primary` | Avatar background color |
| `.rating()` | `u8` | `None` | Star rating (0-5) |
| `.width()` | `f32` | `auto` | Card width |
| `.height()` | `f32` | `auto` | Card height |
| `.show_quotes()` | `bool` | `true` | Show quote marks |
| `.hover_effect()` | `bool` | `true` | Enable hover effect |
| `.show_border()` | `bool` | `true` | Show border |

### TestimonialGrid

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create grid |
| `.columns()` | `usize` | `auto` | Number of columns |
| `.gap()` | `f32` | `20.0` | Gap between cards |
| `.show_quotes()` | `bool` | `true` | Show quote marks |
| `.hover_effect()` | `bool` | `true` | Enable hover effect |
| `.show()` | closure | - | Render with closure-based API |

### TestimonialGridBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.testimonial()` | `(&str, &str, &str)` | Add testimonial with quote, author, role |

### TestimonialBuilder (chainable from .testimonial())

| Method | Type | Description |
|--------|------|-------------|
| `.avatar()` | `&str` | Set avatar text (emoji/initials) |
| `.avatar_color()` | `Color32` | Set avatar background color |
| `.rating()` | `u8` | Set star rating (0-5) |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `primary`, `warning` (for stars)
