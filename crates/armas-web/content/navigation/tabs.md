# Tabs

Tab navigation styled like shadcn/ui Tabs with animated indicator.

## Basic Usage

```demo
let mut tabs = Tabs::new(vec!["Account", "Password"]);
if let Some(new_index) = tabs.show(ui) {
    // Tab changed to new_index
}
```

## Pre-selected Tab

```demo
let mut tabs = Tabs::new(vec!["Overview", "Analytics", "Reports"])
    .active(1);
tabs.show(ui);
```

## Without Animation

```demo
let mut tabs = Tabs::new(vec!["Tab 1", "Tab 2", "Tab 3"])
    .animate(false);
tabs.show(ui);
```

## API Reference

### Tabs

#### Constructor

```rust
Tabs::new(labels: Vec<impl Into<String>>) -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.active()` | `usize` | `0` | Set active tab index |
| `.animate()` | `bool` | `true` | Enable/disable animation |

#### Show Method

```rust
pub fn show(&mut self, ui: &mut Ui) -> Option<usize>
```

Returns `Some(index)` if the active tab changed, `None` otherwise.

## shadcn/ui Styling

The Tabs component follows shadcn/ui conventions:

- **TabsList**: `bg-muted`, `h-9` (36px), `rounded-lg`, `p-[3px]`
- **TabsTrigger**: `text-muted-foreground` inactive, `text-foreground` active
- **Active indicator**: `bg-background` with `rounded-md`
- **Animation**: Smooth indicator transition between tabs
