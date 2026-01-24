# Infinite Moving Cards

Horizontally scrolling cards with infinite loop.

## Basic Usage

```demo
InfiniteMovingCards::new()
    .show(ui, |carousel| {
        carousel.card("Card 1", "First card content");
        carousel.card("Card 2", "Second card content");
        carousel.card("Card 3", "Third card content");
    });
```

## With Authors

```demo
InfiniteMovingCards::new()
    .show(ui, |carousel| {
        carousel.card("Great Product", "This is amazing!")
            .author("John Doe");
        carousel.card("Highly Recommend", "Five stars!")
            .author("Jane Smith");
    });
```

## Custom Speed and Direction

```demo
use egui::Direction;

InfiniteMovingCards::new()
    .speed(ScrollSpeed::Fast)
    .direction(Direction::RightToLeft)
    .pause_on_hover(false)
    .show(ui, |carousel| {
        carousel.card("Fast", "Fast scroll");
        carousel.card("Right to Left", "Direction demo");
    });
```

## Custom Card Styling

```demo
use egui::Color32;

InfiniteMovingCards::new()
    .card_size(300.0, 180.0)
    .spacing(15.0)
    .show(ui, |carousel| {
        carousel.card("Custom", "Styled card")
            .background_color(Color32::from_rgb(40, 40, 50))
            .text_color(Color32::from_gray(230));
    });
```

## API Reference

### InfiniteMovingCards

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create infinite cards carousel |
| `.card_size(width, height)` | `(f32, f32)` | `(350.0, 200.0)` | Set card dimensions |
| `.spacing()` | `f32` | `20.0` | Space between cards |
| `.direction()` | `Direction` | `LeftToRight` | Scroll direction |
| `.speed()` | `ScrollSpeed` | `Normal` | Scroll speed |
| `.pause_on_hover()` | `bool` | `true` | Pause on hover |
| `.show()` | closure | - | Render with closure-based API |

### InfiniteMovingCardsBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.card()` | `(&str, &str)` | Add card with title and subtitle |

### CardBuilder (chainable from .card())

| Method | Type | Description |
|--------|------|-------------|
| `.author()` | `&str` | Set author/source |
| `.background_color()` | `Color32` | Set background color |
| `.text_color()` | `Color32` | Set text color |

### ScrollSpeed

| Variant | Pixels/Second | Description |
|---------|---------------|-------------|
| `Slow` | 80 | Slow scroll speed |
| `Normal` | 120 | Normal scroll speed |
| `Fast` | 200 | Fast scroll speed |

## Dependencies

- `egui = "0.33"`
