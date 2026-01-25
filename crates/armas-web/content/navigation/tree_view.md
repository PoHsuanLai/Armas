# Tree View

A hierarchical tree view for displaying nested items like files/folders.

## Basic Usage

```rust
let items = vec![
    TreeItem::folder("src", "/src"),
    TreeItem::file("main.rs", "/src/main.rs"),
    TreeItem::file("lib.rs", "/src/lib.rs"),
];

let mut tree = TreeView::new()
    .items(items)
    .width(300.0)
    .height(400.0);

let response = tree.show(ui);
if let Some(path) = response.selected {
    println!("Selected: {:?}", path);
}
```

## Live Demo

```demo
let items = vec![
    TreeItem::folder("src", "/src"),
    TreeItem::folder("components", "/src/components"),
    TreeItem::file("button.rs", "/src/components/button.rs"),
    TreeItem::file("input.rs", "/src/components/input.rs"),
    TreeItem::file("main.rs", "/src/main.rs"),
    TreeItem::folder("tests", "/tests"),
    TreeItem::file("test_button.rs", "/tests/test_button.rs"),
];

let tree_id = ui.id().with("tree_view_demo");
let mut tree: TreeView = ui.ctx().data_mut(|d| {
    d.get_persisted(tree_id).unwrap_or_else(|| {
        TreeView::new()
            .items(items.clone())
            .show_lines(true)
            .width(280.0)
            .height(300.0)
    })
});

let response = tree.show(ui);

ui.ctx().data_mut(|d| {
    d.insert_persisted(tree_id, tree);
});

if let Some(path) = response.selected {
    ui.label(format!("Selected: {:?}", path));
}
```

## API Reference

### TreeView

#### Constructor

```rust
TreeView::new() -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.width()` | `f32` | `0.0` (fill) | Set width |
| `.height()` | `f32` | `0.0` (fill) | Set height |
| `.items()` | `Vec<TreeItem>` | Empty | Set tree items |
| `.root_path()` | `String` | `"/"` | Set root path for filtering |
| `.show_lines()` | `bool` | `false` | Show tree connection lines |

#### Getters

| Method | Return Type | Description |
|--------|-------------|-------------|
| `.selected()` | `Option<&PathBuf>` | Get currently selected item |
| `.is_expanded()` | `bool` | Check if branch is expanded |

### TreeItem

| Method | Description |
|--------|-------------|
| `::leaf(name, path)` | Create a leaf item |
| `::branch(name, path)` | Create a branch item (expandable) |
| `::file(name, path)` | Alias for leaf |
| `::folder(name, path)` | Alias for branch |

### TreeViewResponse

| Field | Type | Description |
|-------|------|-------------|
| `response` | `Response` | Standard egui response |
| `selected` | `Option<PathBuf>` | Item selected this frame |
| `toggled` | `Option<PathBuf>` | Branch expanded/collapsed this frame |
