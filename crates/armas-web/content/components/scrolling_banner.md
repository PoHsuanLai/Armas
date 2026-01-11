# Scrolling Banner

Infinite scrolling banner/marquee component.

## Basic Usage

```demo
let mut banner = ScrollingBanner::new();
banner.show(ui, |ui, _index| {
    ui.label("Breaking News: Armas UI Library Released!");
    ui.separator();
    ui.label("Now with 60+ components!");
});
```

## Custom Speed and Direction

```demo
let mut banner = ScrollingBanner::new()
    .speed(100.0)
    .direction(ScrollDirection::Right);
banner.show(ui, |ui, _index| {
    ui.label("Fast scrolling to the right");
});
```

## Without Fade Effect

```demo
let mut banner = ScrollingBanner::new()
    .show_fade(false)
    .pause_on_hover(false);
banner.show(ui, |ui, _index| {
    ui.label("No fade, no pause");
});
```

## Vertical Scrolling

```demo
let mut banner = ScrollingBanner::new()
    .direction(ScrollDirection::Up)
    .speed(30.0);
banner.show(ui, |ui, _index| {
    ui.label("Scrolling up");
    ui.separator();
    ui.label("Continuously");
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create scrolling banner |
| `.speed()` | `f32` | `50.0` | Scroll speed (pixels/second) |
| `.direction()` | `ScrollDirection` | `Left` | Scroll direction |
| `.gap()` | `f32` | `32.0` | Gap between repeated content |
| `.pause_on_hover()` | `bool` | `true` | Pause on hover |
| `.show_fade()` | `bool` | `true` | Show fade effect at edges |
| `.fade_width()` | `f32` | `40.0` | Fade width in pixels |
| `.pause()` | - | - | Pause the animation |
| `.resume()` | - | - | Resume the animation |
| `.reset()` | - | - | Reset scroll offset |
| `.show()` | `(&mut Ui, impl Fn(&mut Ui, usize))` | - | Show with content closure |

## ScrollDirection

| Variant | Description |
|---------|-------------|
| `Left` | Scroll from right to left |
| `Right` | Scroll from left to right |
| `Up` | Scroll from bottom to top |
| `Down` | Scroll from top to bottom |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface` (for fade effect)
