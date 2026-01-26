# Scrolling Banner

Infinite scrolling banner/marquee component.

```demo
ScrollingBanner::new().show(ui, |ui, _index| {
    ui.label("Breaking News  •  Armas UI Library Released  •  60+ Components  •  ");
});
```

## Custom Speed and Direction

```demo
ScrollingBanner::new().speed(100.0).direction(ScrollDirection::Right).show(ui, |ui, _index| {
    ui.label("Fast scrolling to the right  →  →  →  ");
});
```

## Without Fade

```demo
ScrollingBanner::new().show_fade(false).pause_on_hover(false).show(ui, |ui, _index| {
    ui.label("No fade, no pause on hover  •  Continuous scrolling  •  ");
});
```

## Multiple Items

```demo
ScrollingBanner::new().gap(50.0).show(ui, |ui, _index| {
    ui.label("Item 1");
    ui.add_space(20.0);
    ui.label("•");
    ui.add_space(20.0);
    ui.label("Item 2");
    ui.add_space(20.0);
    ui.label("•");
    ui.add_space(20.0);
    ui.label("Item 3");
});
```
