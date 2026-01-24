# Event Timeline

Vertical timeline for displaying chronological events.

## Basic Usage

```demo
EventTimeline::new()
    .show(ui, |timeline| {
        timeline.item("Project Started", "January 2024");
        timeline.item("First Release", "March 2024");
        timeline.item("Version 2.0", "June 2024");
    });
```

## With Time and Highlighting

```demo
EventTimeline::new()
    .dot_size(12.0)
    .show(ui, |timeline| {
        timeline.item("Started", "Project initiated")
            .time("2 hours ago");
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
| `.dot_size()` | `f32` | `12.0` | Dot size |
| `.line_width()` | `f32` | `2.0` | Line width |
| `.item_gap()` | `f32` | `48.0` | Gap between items |
| `.show_lines()` | `bool` | `true` | Show connecting lines |

### EventTimelineItemBuilder (chainable from .item())

| Method | Type | Description |
|--------|------|-------------|
| `.time()` | `&str` | Set timestamp |
| `.icon()` | `&str` | Set icon |
| `.highlighted()` | `bool` | Mark as highlighted |
