# Browser

File browser for audio files, plugins, presets, and samples. Features search, filtering, folder navigation, and preview capabilities for DAW workflows.

## Basic Usage

**Important**: Store the `Browser` instance in your app state to preserve search query, selection, and tab state across frames.

For proper state persistence, store the Browser in your app struct:

```rust
struct MyApp {
    browser: Browser,
}

impl MyApp {
    fn new() -> Self {
        Self {
            browser: Browser::new()
                .width(300.0)
                .height(600.0),
        }
    }
}

// In your UI code:
let response = self.browser.show(ui);
if let Some(path) = response.selected_file {
    println!("Selected file: {:?}", path);
}
```

## Live Demo

```demo
// Note: In the demo, state persists via egui's memory system.
// In a real app, store Browser in your app struct.
let browser_id = ui.id().with("browser_demo");
let mut browser: Browser = ui.ctx().data_mut(|d| {
    d.get_persisted(browser_id).unwrap_or_else(|| {
        Browser::new()
            .width(300.0)
            .height(500.0)
    })
});

let response = browser.show(ui);

// Save state
ui.ctx().data_mut(|d| {
    d.insert_persisted(browser_id, browser);
});

// Display response info
if let Some(path) = response.selected_file {
    ui.label(format!("Selected: {:?}", path));
}
```

## With Custom Dimensions

```demo
let mut browser = Browser::new()
    .width(400.0)
    .height(600.0);

browser.show(ui);
```

## API Reference

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create new browser |
| `.width(width)` | `f32` | `300.0` | Set browser width |
| `.height(height)` | `f32` | `600.0` | Set browser height |
| `.items(items)` | `Vec<BrowserItem>` | Mock data | Set custom file list |
| `.show(&mut Ui)` | - | - | Show browser, returns `BrowserResponse` |

### Getter Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `.selected_file()` | `Option<&PathBuf>` | Get currently selected file |
| `.current_folder()` | `Option<&PathBuf>` | Get current folder path |
| `.search_query()` | `&str` | Get current search text |
| `.active_tab()` | `BrowserTab` | Get active category tab |
| `.active_filters()` | `&[String]` | Get currently active filter tags |

### BrowserResponse

The response from `show()` contains:

- `response: Response` - Standard egui response
- `selected_file: Option<PathBuf>` - File clicked this frame (if any)
- `selected_folder: Option<PathBuf>` - Folder clicked this frame (if any)
- `tab_changed: Option<BrowserTab>` - New tab (if changed this frame)
- `filters_changed: bool` - Filter tags were toggled this frame

## Features

- **Breadcrumb Navigation**: Navigate folder hierarchy with clickable breadcrumbs
- **Back Button**: Quick navigation to parent folder
- **Search Bar**: Real-time filtering with search query
- **Category Tabs**: Audio, Plugins, Presets, Loops, Samples
- **Filter Chips**: Interactive tag-based filtering (click to toggle)
- **Folder Tree**: Hierarchical folder navigation - click folders to drill down
- **File List**: Scrollable list with metadata display
- **Metadata Display**: BPM, key, sample rate for audio files
- **Favorites**: Star icon for favorited items
- **Selection**: Click to select files

## Filter System

The browser automatically collects and displays filter chips based on tags assigned to items:

- **Dynamic Tags**: Filter chips are generated from item tags
- **Interactive**: Click chips to toggle filters on/off
- **Multi-select**: Multiple filters can be active simultaneously
- **Combined Filtering**: Works together with search for precise results

To use filters in your items:

```rust
BrowserItem {
    name: "Kick_808.wav".to_string(),
    tags: vec!["Kicks".to_string(), "808".to_string(), "Bass".to_string()],
    // ... other fields
}
```

## Folder Navigation

The browser provides intuitive folder navigation:

### Breadcrumb Trail
- Shows current path: "Samples > Drums > Kicks"
- Click any breadcrumb to jump to that folder
- Click "Samples" to return to root

### Back Button
- "Back" appears when inside a folder
- Navigate one level up in the hierarchy

### Folder Drilling
- Click any folder to enter it
- Only shows items in the current folder
- Maintains proper hierarchy filtering

### How It Works
Items are filtered by their parent path. When you navigate into "/samples/Drums/Kicks", only items with that exact parent path are shown. This creates a clean, organized browsing experience.

## Browser Tabs

The browser supports multiple content categories:

- **Audio**: Audio files (.wav, .mp3, .flac, etc.)
- **Plugins**: VST/AU plugins
- **Presets**: Plugin presets and patches
- **Loops**: Audio loops and stems
- **Samples**: One-shot samples and sound effects

## File Metadata

Audio files display relevant metadata:

- **Sample Rate**: 44.1kHz, 48kHz, etc.
- **BPM**: Detected or tagged tempo
- **Key**: Musical key (C, Am, etc.)
- **Duration**: File length in seconds

## Visual Design

The browser follows Armas' aceternity-inspired design:

- Glassmorphic search bar
- Smooth tab transitions
- Card-based file items
- Hover effects with subtle glow
- Selected state with primary color border
- Theme-consistent colors throughout

## Planned Features

- **Drag & Drop**: Drag files to timeline/tracks
- **Preview/Audition**: Click play button to preview audio
- **Waveform Thumbnails**: Visual preview of audio content
- **Folder Tree**: Hierarchical folder navigation
- **Filter Chips**: Quick filters (Kicks, Snares, etc.)
- **Context Menu**: Right-click for file operations
- **Sorting**: Sort by name, date, size, BPM, key
- **Recent Files**: Quick access to recently used files
- **Favorites**: Bookmark frequently used items

## Dependencies

- `egui = "0.33"`
- Internal components: Card, Badge, Input
