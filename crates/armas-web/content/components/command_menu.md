# Command Menu

Searchable command palette (Cmd+K style) with keyboard navigation.

## Basic Usage

```demo
let commands = vec![
    Command::new("new-file", "New File"),
    Command::new("open-file", "Open File"),
    Command::new("save", "Save"),
];

let mut menu = CommandMenu::new(commands);

let response = menu.show(ui);

if let Some(command_id) = response.executed_command {
    match command_id.as_str() {
        "new-file" => { /* handle new file */ },
        "open-file" => { /* handle open */ },
        "save" => { /* handle save */ },
        _ => {}
    }
}
```

## With Icons and Descriptions

```demo
let commands = vec![
    Command::new("copy", "Copy")
        .with_icon("📋")
        .with_description("Copy selected text")
        .with_shortcut("Cmd+C"),

    Command::new("paste", "Paste")
        .with_icon("📄")
        .with_description("Paste from clipboard")
        .with_shortcut("Cmd+V"),

    Command::new("cut", "Cut")
        .with_icon("✂️")
        .with_description("Cut selected text")
        .with_shortcut("Cmd+X"),
];

let mut menu = CommandMenu::new(commands);
menu.show(ui);
```

## With Categories

```demo
let commands = vec![
    Command::new("new", "New File")
        .with_category("File")
        .with_icon("📄"),

    Command::new("open", "Open File")
        .with_category("File")
        .with_icon("📁"),

    Command::new("find", "Find")
        .with_category("Edit")
        .with_icon("🔍"),
];

let mut menu = CommandMenu::new(commands);
menu.show(ui);
```

## Custom Trigger Key

```demo
use egui::{Key, Modifiers};

let commands = vec![Command::new("search", "Search")];

let mut menu = CommandMenu::new(commands)
    .with_trigger(Key::P, Modifiers::COMMAND);

menu.show(ui);
```

## Manual Control

```demo
let commands = vec![Command::new("test", "Test Command")];
let mut menu = CommandMenu::new(commands);

// Open programmatically
if ui.button("Open Menu").clicked() {
    menu.open();
}

// Check state
if menu.is_open() {
    menu.show(ui);
}
```

## API Reference

### CommandMenu

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(commands)` | `Vec<Command>` | - | Create menu with commands |
| `.with_trigger()` | `(Key, Modifiers)` | `(K, Cmd)` | Set trigger key combination |
| `.open()` | - | - | Open the menu |
| `.close()` | - | - | Close the menu |
| `.toggle()` | - | - | Toggle menu open/closed |
| `.is_open()` | - | - | Check if menu is open |
| `.show()` | `&mut Ui` | - | Show menu and handle interactions |

### Command

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new(id, name)` | `(&str, &str)` | - | Create command with ID and name |
| `.with_description()` | `&str` | - | Set description text |
| `.with_icon()` | `&str` | - | Set icon (emoji or text) |
| `.with_shortcut()` | `&str` | - | Set keyboard shortcut display |
| `.with_category()` | `&str` | - | Set category for grouping |

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
