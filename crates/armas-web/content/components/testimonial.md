# Testimonial

Display customer testimonials and reviews with avatars and ratings.

## Basic Usage

```demo
use armas::components::{TestimonialItem, TestimonialCard};

let item = TestimonialItem::new(
    "Amazing product! Changed my workflow completely.",
    "John Doe",
    "CEO, Tech Corp"
).rating(5);

TestimonialCard::new(item).show(ui);
```

## With Avatar

```demo
use armas::components::{TestimonialItem, TestimonialCard};
use egui::Color32;

let item = TestimonialItem::new(
    "Great experience working with this tool.",
    "Jane Smith",
    "Designer"
)
.avatar("JS")
.avatar_color(Color32::from_rgb(100, 150, 255))
.rating(4);

TestimonialCard::new(item)
    .width(400.0)
    .show(ui);
```

## Testimonial Grid

```demo
use armas::components::{TestimonialItem, TestimonialGrid};

let testimonials = vec![
    TestimonialItem::new("Great product!", "Alice", "Engineer").rating(5),
    TestimonialItem::new("Love it!", "Bob", "Designer").rating(5),
    TestimonialItem::new("Excellent!", "Carol", "Manager").rating(4),
];

TestimonialGrid::new(testimonials)
    .columns(3)
    .gap(20.0)
    .show(ui);
```

## API Reference

### TestimonialItem

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `(String, String, String)` | - | Create with quote, author, role |
| `.avatar()` | `impl Into<String>` | `None` | Avatar text (emoji/initials) |
| `.avatar_color()` | `Color32` | `primary` | Avatar background color |
| `.rating()` | `u8` | `None` | Star rating (0-5) |

### TestimonialCard

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `TestimonialItem` | - | Create card with item |
| `.width()` | `f32` | `auto` | Card width |
| `.height()` | `f32` | `auto` | Card height |
| `.show_quotes()` | `bool` | `true` | Show quote marks |
| `.hover_effect()` | `bool` | `true` | Enable hover effect |
| `.show_border()` | `bool` | `true` | Show border |

### TestimonialGrid

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `Vec<TestimonialItem>` | - | Create grid with items |
| `.columns()` | `usize` | `auto` | Number of columns |
| `.gap()` | `f32` | `20.0` | Gap between cards |
| `.show_quotes()` | `bool` | `true` | Show quote marks |
| `.hover_effect()` | `bool` | `true` | Enable hover effect |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `primary`, `warning` (for stars)
