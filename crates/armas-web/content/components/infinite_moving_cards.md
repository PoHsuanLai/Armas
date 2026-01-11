# Infinite Moving Cards

Horizontally scrolling cards with infinite loop.

## Basic Usage

```demo
let cards = vec![
    MovingCard::new("Card 1", "First card content"),
    MovingCard::new("Card 2", "Second card content"),
    MovingCard::new("Card 3", "Third card content"),
];

let mut carousel = InfiniteMovingCards::new(cards);
carousel.show(ui);
```

## With Authors

```demo
let cards = vec![
    MovingCard::new("Great Product", "This is amazing!")
        .author("John Doe"),
    MovingCard::new("Highly Recommend", "Five stars!")
        .author("Jane Smith"),
];

let mut carousel = InfiniteMovingCards::new(cards);
carousel.show(ui);
```

## Custom Speed and Direction

```demo
use egui::Direction;

let cards = vec![
    MovingCard::new("Fast", "Fast scroll"),
    MovingCard::new("Right to Left", "Direction demo"),
];

let mut carousel = InfiniteMovingCards::new(cards)
    .speed(ScrollSpeed::Fast)
    .direction(Direction::RightToLeft)
    .pause_on_hover(false);
carousel.show(ui);
```

## Custom Card Styling

```demo
use egui::Color32;

let cards = vec![
    MovingCard::new("Custom", "Styled card")
        .background_color(Color32::from_rgb(40, 40, 50))
        .text_color(Color32::from_gray(230)),
];

let mut carousel = InfiniteMovingCards::new(cards)
    .card_size(300.0, 180.0)
    .spacing(15.0);
carousel.show(ui);
```

## API Reference

### InfiniteMovingCards

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(cards)` | `Vec<MovingCard>` | - | Create infinite cards carousel |
| `.card_size(width, height)` | `(f32, f32)` | `(350.0, 200.0)` | Set card dimensions |
| `.spacing()` | `f32` | `20.0` | Space between cards |
| `.direction()` | `Direction` | `LeftToRight` | Scroll direction |
| `.speed()` | `ScrollSpeed` | `Normal` | Scroll speed |
| `.pause_on_hover()` | `bool` | `true` | Pause on hover |
| `.show(&mut Ui)` | - | - | Show the carousel |

### MovingCard

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(title, subtitle)` | `(&str, &str)` | - | Create card |
| `.author()` | `&str` | `None` | Set author/source |
| `.background_color()` | `Color32` | `Color32::from_rgb(40, 40, 50)` | Set background color |
| `.text_color()` | `Color32` | `Color32::from_gray(230)` | Set text color |

### ScrollSpeed

| Variant | Pixels/Second | Description |
|---------|---------------|-------------|
| `Slow` | 80 | Slow scroll speed |
| `Normal` | 120 | Normal scroll speed |
| `Fast` | 200 | Fast scroll speed |

## Dependencies

- `egui = "0.33"`
