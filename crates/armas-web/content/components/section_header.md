# Section Header

Collapsible section header with arrow indicator (Studio One style).

## Basic Usage

```demo
let mut collapsed = false;

if SectionHeader::new("Sends", collapsed)
    .show(ui)
    .clicked()
{
    collapsed = !collapsed;
}
```

## Expanded State

```demo
let collapsed = false;
SectionHeader::new("Effects", collapsed)
    .show(ui);
// Shows "Effects ▼"
```

## Collapsed State

```demo
let collapsed = true;
SectionHeader::new("Inserts", collapsed)
    .show(ui);
// Shows "Inserts ▶"
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(label, collapsed)` | `(&str, bool)` | - | Create section header |
| `.show(&mut Ui)` | - | - | Show the header |

## Note

This component matches Studio One's "Sends ▼" style collapsible headers. The arrow indicator automatically changes based on the collapsed state.

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`
