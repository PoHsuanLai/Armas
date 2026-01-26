# Infinite Moving Cards

Horizontally scrolling cards with infinite loop.

```demo
InfiniteMovingCards::new().show(ui, |carousel| {
    carousel.card("Card 1", "First card content");
    carousel.card("Card 2", "Second card content");
    carousel.card("Card 3", "Third card content");
});
```

## With Authors

```demo
InfiniteMovingCards::new().show(ui, |carousel| {
    carousel.card("Great Product", "This is amazing!").author("John Doe");
    carousel.card("Highly Recommend", "Five stars!").author("Jane Smith");
});
```

## Custom Speed and Direction

```demo
use egui::Direction;
InfiniteMovingCards::new().speed(ScrollSpeed::Fast).direction(Direction::RightToLeft).pause_on_hover(false).show(ui, |carousel| {
    carousel.card("Fast", "Fast scroll");
    carousel.card("Right to Left", "Direction demo");
});
```

## Custom Styling

```demo
use egui::Color32;
InfiniteMovingCards::new().card_size(300.0, 180.0).spacing(15.0).show(ui, |carousel| {
    carousel.card("Custom", "Styled card").background_color(Color32::from_rgb(40, 40, 50)).text_color(Color32::from_gray(230));
});
```
