# Command Menu

Searchable command palette (Cmd+K style) with keyboard navigation.

## Basic Usage

```demo
let mut menu = CommandMenu::new();

ui.label("Press Cmd+K to open menu");

let response = menu.show(ui, |menu| {
    menu.command("new-file", "New File");
    menu.command("open-file", "Open File");
    menu.command("save", "Save");
});

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}
```

## With Icons and Descriptions

```demo
let mut menu = CommandMenu::new();

ui.label("Press Cmd+K to open menu");

let response = menu.show(ui, |menu| {
    menu.command("copy", "Copy")
        .icon("📋")
        .description("Copy selected text")
        .shortcut("Cmd+C");

    menu.command("paste", "Paste")
        .icon("📄")
        .description("Paste from clipboard")
        .shortcut("Cmd+V");

    menu.command("cut", "Cut")
        .icon("✂️")
        .description("Cut selected text")
        .shortcut("Cmd+X");
});

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}
```

## With Categories

```demo
let mut menu = CommandMenu::new();

ui.label("Press Cmd+K to open menu");

let response = menu.show(ui, |menu| {
    menu.category("File");
    menu.command("new", "New File")
        .icon("📄");

    menu.command("open", "Open File")
        .icon("📁");

    menu.category("Edit");
    menu.command("find", "Find")
        .icon("🔍");
});

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}
```

## Custom Trigger Key

```demo
use egui::{Key, Modifiers};

let mut menu = CommandMenu::new()
    .trigger(Key::P, Modifiers::COMMAND);

ui.label("Press Cmd+P to open menu");

let response = menu.show(ui, |menu| {
    menu.command("search", "Search");
});

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}
```

## API Reference

### CommandMenu

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create menu |
| `.trigger()` | `(Key, Modifiers)` | `(K, Cmd)` | Set trigger key combination |
| `.placeholder()` | `&str` | `"Type a command..."` | Set search placeholder text |
| `.show()` | closure | - | Show menu with closure-based API |

### CommandMenuBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.command()` | `(&str, &str)` | Add command with ID and name |
| `.category()` | `&str` | Add category header |

### CommandBuilder (chainable from .command())

| Method | Type | Description |
|--------|------|-------------|
| `.description()` | `&str` | Set description text |
| `.icon()` | `&str` | Set icon (emoji or text) |
| `.shortcut()` | `&str` | Set keyboard shortcut display |

### CommandMenuResponse

| Field | Type | Description |
|-------|------|-------------|
| `executed_command` | `Option<String>` | ID of executed command, if any |

## Keyboard Shortcuts

- `Cmd+K` (or custom): Toggle menu
- `Escape`: Close menu
- `Arrow Up/Down`: Navigate commands
- `Enter`: Execute selected command
- Type to search and filter commands

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `primary`, `hover`, `outline`
