# Command Menu

Searchable command palette (Cmd+K style) with keyboard navigation.

## Basic Usage

```demo
// Get or create menu from memory
let menu_id = ui.id().with("basic_menu");
let mut menu = ui.data_mut(|d| {
    d.get_temp::<CommandMenu>(menu_id).unwrap_or_else(|| {
        CommandMenu::new(vec![
            Command::new("new-file", "New File"),
            Command::new("open-file", "Open File"),
            Command::new("save", "Save"),
        ])
    })
});

// Button to open menu
if Button::new("Open Menu (or press Cmd+K)").show(ui).clicked() {
    menu.open();
}

let response = menu.show(ui);

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}

// Store back to memory
ui.data_mut(|d| d.insert_temp(menu_id, menu));
```

## With Icons and Descriptions

```demo
let menu_id = ui.id().with("icons_menu");
let mut menu = ui.data_mut(|d| {
    d.get_temp::<CommandMenu>(menu_id).unwrap_or_else(|| {
        CommandMenu::new(vec![
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
        ])
    })
});

if Button::new("Open Menu").show(ui).clicked() {
    menu.open();
}

let response = menu.show(ui);

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}

ui.data_mut(|d| d.insert_temp(menu_id, menu));
```

## With Categories

```demo
let menu_id = ui.id().with("categories_menu");
let mut menu = ui.data_mut(|d| {
    d.get_temp::<CommandMenu>(menu_id).unwrap_or_else(|| {
        CommandMenu::new(vec![
            Command::new("new", "New File")
                .with_category("File")
                .with_icon("📄"),

            Command::new("open", "Open File")
                .with_category("File")
                .with_icon("📁"),

            Command::new("find", "Find")
                .with_category("Edit")
                .with_icon("🔍"),
        ])
    })
});

if Button::new("Open Menu").show(ui).clicked() {
    menu.open();
}

let response = menu.show(ui);

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}

ui.data_mut(|d| d.insert_temp(menu_id, menu));
```

## Custom Trigger Key

```demo
use egui::{Key, Modifiers};

let menu_id = ui.id().with("trigger_menu");
let mut menu = ui.data_mut(|d| {
    d.get_temp::<CommandMenu>(menu_id).unwrap_or_else(|| {
        CommandMenu::new(vec![Command::new("search", "Search")])
            .with_trigger(Key::P, Modifiers::COMMAND)
    })
});

ui.label("Press Cmd+P to open menu");

let response = menu.show(ui);

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}

ui.data_mut(|d| d.insert_temp(menu_id, menu));
```

## Manual Control

```demo
let menu_id = ui.id().with("manual_menu");
let mut menu = ui.data_mut(|d| {
    d.get_temp::<CommandMenu>(menu_id).unwrap_or_else(|| {
        CommandMenu::new(vec![Command::new("test", "Test Command")])
    })
});

// Open programmatically
if Button::new("Open Menu").show(ui).clicked() {
    menu.open();
}

// Close button
if Button::new("Close Menu").show(ui).clicked() {
    menu.close();
}

// Toggle button
if Button::new("Toggle Menu").show(ui).clicked() {
    menu.toggle();
}

// Check state
ui.label(format!("Menu is {}", if menu.is_open() { "open" } else { "closed" }));

let response = menu.show(ui);

if let Some(command_id) = response.executed_command {
    ui.label(format!("Executed: {}", command_id));
}

ui.data_mut(|d| d.insert_temp(menu_id, menu));
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
