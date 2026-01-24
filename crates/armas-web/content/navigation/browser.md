# Browser

A simple file/folder tree component.

## Basic Usage

```rust
let items = vec![
    BrowserItem::folder("src", "/src"),
    BrowserItem::file("main.rs", "/src/main.rs"),
    BrowserItem::file("lib.rs", "/src/lib.rs"),
];

let mut browser = Browser::new()
    .items(items)
    .width(300.0)
    .height(400.0);

let response = browser.show(ui);
if let Some(path) = response.selected {
    println!("Selected: {:?}", path);
}
```

## Live Demo

```demo
let items = vec![
    BrowserItem::folder("src", "/src"),
    BrowserItem::folder("components", "/src/components"),
    BrowserItem::file("button.rs", "/src/components/button.rs"),
    BrowserItem::file("input.rs", "/src/components/input.rs"),
    BrowserItem::file("main.rs", "/src/main.rs"),
    BrowserItem::folder("tests", "/tests"),
    BrowserItem::file("test_button.rs", "/tests/test_button.rs"),
];

let browser_id = ui.id().with("browser_demo");
let mut browser: Browser = ui.ctx().data_mut(|d| {
    d.get_persisted(browser_id).unwrap_or_else(|| {
        Browser::new()
            .items(items.clone())
            .show_lines(true)
            .width(280.0)
            .height(300.0)
    })
});

let response = browser.show(ui);

ui.ctx().data_mut(|d| {
    d.insert_persisted(browser_id, browser);
});

if let Some(path) = response.selected {
    ui.label(format!("Selected: {:?}", path));
}
```

## API Reference

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create new browser |
| `.width(w)` | `f32` | `0.0` (fill) | Set width |
| `.height(h)` | `f32` | `0.0` (fill) | Set height |
| `.items(items)` | `Vec<BrowserItem>` | Empty | Set file/folder items |
| `.root_path(path)` | `String` | `"/"` | Set root path for filtering |
| `.show_lines(bool)` | `bool` | `false` | Show tree connection lines |

### BrowserItem

| Method | Description |
|--------|-------------|
| `::file(name, path)` | Create a file item |
| `::folder(name, path)` | Create a folder item |

### BrowserResponse

| Field | Type | Description |
|-------|------|-------------|
| `response` | `Response` | Standard egui response |
| `selected` | `Option<PathBuf>` | File selected this frame |
| `toggled` | `Option<PathBuf>` | Folder expanded/collapsed this frame |

### Getters

| Method | Return Type | Description |
|--------|-------------|-------------|
| `.selected()` | `Option<&PathBuf>` | Get currently selected file |
| `.is_expanded(path)` | `bool` | Check if folder is expanded |
