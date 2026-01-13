# Event Timeline

Vertical timeline component for displaying chronological events and milestones.

## Basic Usage

```demo
EventTimeline::new()
    .show(ui, |timeline| {
        timeline.item("Project Started", "January 2024");
        timeline.item("First Release", "March 2024");
        timeline.item("Version 2.0", "June 2024");
    });
```

## With Icons

```demo
EventTimeline::new()
    .show(ui, |timeline| {
        timeline.item("Launched", "Day 1").icon("🚀");
        timeline.item("First User", "Day 2").icon("👤");
        timeline.item("100 Users", "Day 7").icon("🎉");
    });
```

## With Time and Highlighting

```demo
EventTimeline::new()
    .dot_size(12.0)
    .show(ui, |timeline| {
        timeline.item("Started", "Project initiated")
            .time("2 hours ago")
            .icon("🚀");
        timeline.item("In Progress", "Working on features")
            .time("1 hour ago")
            .highlighted(true);
        timeline.item("Complete", "All done!")
            .time("Just now");
    });
```

## API Reference

### EventTimeline

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create timeline |
| `.dot_size()` | `f32` | `12.0` | Set dot size |
| `.line_width()` | `f32` | `2.0` | Set line width |
| `.item_gap()` | `f32` | `48.0` | Gap between items |
| `.show_lines()` | `bool` | `true` | Show connecting lines |
| `.show()` | closure | - | Render with closure-based API |

### EventTimelineBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.item()` | `(&str, &str)` | Add item with title and description |

### EventTimelineItemBuilder (chainable from .item())

| Method | Type | Description |
|--------|------|-------------|
| `.time()` | `&str` | Set timestamp |
| `.icon()` | `&str` | Set icon/emoji |
| `.icon_color()` | `Color32` | Set icon color |
| `.highlighted()` | `bool` | Mark as highlighted/active |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `outline`, `on_surface`
