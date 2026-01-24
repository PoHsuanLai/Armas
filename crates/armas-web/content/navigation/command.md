# Command

A command palette for search and quick actions. Styled to match shadcn/ui command.

## Basic Usage

```demo
let mut cmd = Command::new();

ui.label("Press Cmd+K to open");

let response = cmd.show(ui, |cmd| {
    cmd.item("new-file", "New File");
    cmd.item("open-file", "Open File");
    cmd.item("save", "Save");
});

if let Some(id) = response.executed {
    ui.label(format!("Executed: {}", id));
}
```

## With Icons and Shortcuts

```demo
let mut cmd = Command::new();

ui.label("Press Cmd+K to open");

let response = cmd.show(ui, |cmd| {
    cmd.item("copy", "Copy").icon("📋").shortcut("⌘C");
    cmd.item("paste", "Paste").icon("📄").shortcut("⌘V");
    cmd.item("cut", "Cut").icon("✂️").shortcut("⌘X");
});

if let Some(id) = response.executed {
    ui.label(format!("Executed: {}", id));
}
```

## With Groups

```demo
let mut cmd = Command::new();

ui.label("Press Cmd+K to open");

let response = cmd.show(ui, |cmd| {
    cmd.group("File");
    cmd.item("new", "New File").icon("📄");
    cmd.item("open", "Open File").icon("📂");

    cmd.separator();

    cmd.group("Edit");
    cmd.item("find", "Find").icon("🔍").shortcut("⌘F");
});

if let Some(id) = response.executed {
    ui.label(format!("Executed: {}", id));
}
```

## Custom Trigger Key

```demo
use egui::{Key, Modifiers};

let mut cmd = Command::new()
    .trigger(Key::P, Modifiers::COMMAND);

ui.label("Press Cmd+P to open");

let response = cmd.show(ui, |cmd| {
    cmd.item("search", "Search");
});

if let Some(id) = response.executed {
    ui.label(format!("Executed: {}", id));
}
```

## API Reference

### Command

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create command palette |
| `.trigger()` | `(Key, Modifiers)` | `(K, Cmd)` | Trigger key combination |
| `.placeholder()` | `&str` | `"Type a command or search..."` | Search placeholder |
| `.show()` | closure | - | Show and render |

### CommandBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.item()` | `id, label` | Add a command item |
| `.group()` | `heading` | Add a group heading |
| `.separator()` | - | Add a separator |

### CommandItemBuilder (chainable from .item())

| Method | Type | Description |
|--------|------|-------------|
| `.icon()` | `&str` | Icon |
| `.shortcut()` | `&str` | Keyboard shortcut display |

### CommandResponse

| Field | Type | Description |
|-------|------|-------------|
| `executed` | `Option<String>` | ID of executed command |
| `is_open` | `bool` | Whether the palette is currently open |
| `changed` | `bool` | Whether open state changed this frame |

## Keyboard Shortcuts

- `Cmd+K` (or custom): Toggle menu
- `Escape`: Close menu
- `Arrow Up/Down`: Navigate items
- `Enter`: Execute selected item
