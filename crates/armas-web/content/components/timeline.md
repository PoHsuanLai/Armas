# Timeline

Vertical timeline component for displaying chronological events and milestones.

## Basic Usage

```demo
let items = vec![
    TimelineItem::new("Project Started", "January 2024"),
    TimelineItem::new("First Release", "March 2024"),
    TimelineItem::new("Version 2.0", "June 2024"),
];
let mut timeline = Timeline::new(items);
timeline.show(ui);
```

## With Icons

```demo
let items = vec![
    TimelineItem::new("Launched", "Day 1").icon("🚀"),
    TimelineItem::new("First User", "Day 2").icon("👤"),
    TimelineItem::new("100 Users", "Day 7").icon("🎉"),
];
let mut timeline = Timeline::new(items);
timeline.show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create timeline |
| `.add_item()` | `TimelineItem` | - | Add timeline item |

### TimelineItem

| Method | Type | Description |
|--------|------|-------------|
| `::new()` | `(&str, &str)` | Create item with title and date |
| `.icon()` | `&str` | Set icon/emoji |
| `.completed()` | - | Mark as completed |
| `.active()` | - | Mark as active |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `success`, `surface`
