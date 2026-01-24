# Floating Window

Draggable, resizable container with glassmorphic styling. Perfect for creating flexible UI overlays, dialogs, and floating panels with modern aesthetics.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();

let mut window = FloatingWindow::new("My Window")
    .id("demo_window")
    .style(FloatingWindowStyle::Glass)
    .width(400.0)
    .height(300.0);

let response = window.show(ui.ctx(), &theme, |ui| {
    ui.label("This is a floating window!");
    ui.add_space(8.0);
    ui.label("You can drag it around and resize it.");
});

if response.closed {
    ui.label("Window was closed");
}
```

## Glass Style

```demo
let theme = ui.ctx().armas_theme();

let window = FloatingWindow::new("Glassmorphic Window")
    .id("glass_window")
    .style(FloatingWindowStyle::Glass)
    .opacity(0.8)
    .glow_intensity(0.5)
    .width(350.0)
    .height(250.0);

let _response = window.show(ui.ctx(), &theme, |ui| {
    ui.heading("Glassmorphic Design");
    ui.add_space(8.0);
    ui.label("Semi-transparent background with subtle glow effects");
    ui.add_space(8.0);
    ui.label("Perfect for modern, sleek interfaces");
});
```

## Surface Style

```demo
let theme = ui.ctx().armas_theme();

let window = FloatingWindow::new("Surface Window")
    .id("surface_window")
    .style(FloatingWindowStyle::Surface)
    .width(350.0)
    .height(250.0);

let _response = window.show(ui.ctx(), &theme, |ui| {
    ui.heading("Material Design 3");
    ui.add_space(8.0);
    ui.label("Standard Material Design surface styling");
    ui.add_space(8.0);
    ui.label("Clean and professional appearance");
});
```

## Elevated Style

```demo
let theme = ui.ctx().armas_theme();

let window = FloatingWindow::new("Elevated Window")
    .id("elevated_window")
    .style(FloatingWindowStyle::Elevated)
    .width(350.0)
    .height(250.0);

let _response = window.show(ui.ctx(), &theme, |ui| {
    ui.heading("Elevated Appearance");
    ui.add_space(8.0);
    ui.label("With drop shadow for depth perception");
    ui.add_space(8.0);
    ui.label("Creates visual hierarchy");
});
```

## Custom Styling

```demo
let theme = ui.ctx().armas_theme();

let window = FloatingWindow::new("Customized Window")
    .id("custom_window")
    .style(FloatingWindowStyle::Glass)
    .opacity(0.9)
    .glow_intensity(0.7)
    .corner_radius(24.0)
    .inner_margin(20.0)
    .width(380.0)
    .height(280.0);

let _response = window.show(ui.ctx(), &theme, |ui| {
    ui.heading("Custom Configuration");
    ui.add_space(8.0);
    ui.label("Adjustable opacity and glow");
    ui.label("Custom corner radius");
    ui.label("Custom inner margin/padding");
});
```

## With Controls

```demo
let theme = ui.ctx().armas_theme();

let window = FloatingWindow::new("Control Panel")
    .id("control_window")
    .style(FloatingWindowStyle::Glass)
    .width(380.0)
    .height(320.0);

let _response = window.show(ui.ctx(), &theme, |ui| {
    ui.heading("Settings");
    ui.separator();
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Volume:");
        let (_resp, volume) = Fader::new(0.7)
            .size(120.0, 200.0)
            .show(ui);
        ui.label(format!("{:.0}%", volume * 100.0));
    });

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Brightness:");
        let (_resp, brightness) = Fader::new(0.8)
            .size(120.0, 200.0)
            .show(ui);
        ui.label(format!("{:.0}%", brightness * 100.0));
    });

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if Button::new("Apply").show(ui).clicked() {
            ui.label("Settings applied!");
        }
        if Button::new("Reset").show(ui).clicked() {
            ui.label("Reset to defaults");
        }
    });
});
```

## Multiple Windows

```demo
let theme = ui.ctx().armas_theme();

ui.label("Multiple floating windows:");
ui.add_space(8.0);

let window1 = FloatingWindow::new("Window 1")
    .id("multi_window_1")
    .style(FloatingWindowStyle::Glass)
    .width(300.0)
    .height(200.0);

let _response1 = window1.show(ui.ctx(), &theme, |ui| {
    ui.heading("First Window");
    ui.label("Windows can be positioned independently");
});

let window2 = FloatingWindow::new("Window 2")
    .id("multi_window_2")
    .style(FloatingWindowStyle::Surface)
    .width(300.0)
    .height(200.0);

let _response2 = window2.show(ui.ctx(), &theme, |ui| {
    ui.heading("Second Window");
    ui.label("Each with different styling");
});

ui.label("Drag and resize to reposition");
```

## API Reference

### Constructor

```rust
FloatingWindow::new(title: impl Into<String>) -> Self
```

Creates a new floating window with a title.

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.id()` | `impl Into<Id>` | `"floating_window"` | Unique identifier for state persistence |
| `.style()` | `FloatingWindowStyle` | `Glass` | Visual style variant |
| `.opacity()` | `f32` | `0.7` | Glass background opacity (0.0-1.0) |
| `.glow_intensity()` | `f32` | `0.3` | Glow border intensity (0.0-1.0) |
| `.width()` | `f32` | `400.0` | Window width in pixels |
| `.height()` | `f32` | `300.0` | Window height in pixels |
| `.closable()` | `bool` | `true` | Whether window can be closed |
| `.open()` | `bool` | `true` | Initial open state |
| `.initial_pos()` | `Pos2` | Centered | Fixed initial position |
| `.corner_radius()` | `f32` | `theme.spacing.corner_radius` | Corner radius override |
| `.inner_margin()` | `f32` | `theme.spacing.md` | Inner padding override |

### Show Method

```rust
pub fn show<R>(
    self,
    ctx: &egui::Context,
    theme: &Theme,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> FloatingWindowResponse<R>
```

Renders the floating window and returns a response.

### Response

```rust
pub struct FloatingWindowResponse<R> {
    pub closed: bool,        // Whether window was closed this frame
    pub inner: Option<R>,    // Result from content closure
}
```

## Style Variants

### Glass (Recommended)
- **Appearance**: Glassmorphic with semi-transparent background and glow border
- **Best for**: Modern, sleek interfaces with visual layering
- **Customizable**: Opacity and glow intensity

### Surface
- **Appearance**: Standard Material Design 3 surface
- **Best for**: Professional, clean interfaces
- **Customizable**: Corner radius and padding

### Elevated
- **Appearance**: Surface with drop shadow
- **Best for**: Creating visual hierarchy and depth
- **Customizable**: Corner radius and padding

## Features

- **Draggable**: Click and drag the title bar to reposition
- **Resizable**: Drag window edges to resize
- **Collapsible**: Collapse window to just the title bar
- **Scrollable**: Content area scrolls if it exceeds window height
- **State Persistence**: Window position and size are remembered across frames
- **Theme Integration**: Uses Armas theme colors and spacing
- **Multiple Variants**: Glass, Surface, and Elevated styles

## Interactions

- **Click and drag title bar**: Reposition window
- **Drag window edges**: Resize window
- **Click collapse button**: Minimize/expand window
- **Click close button**: Close window (if enabled)
- **Scroll in content**: Navigate long content

## Use Cases

### Modal Dialog
```demo
let theme = ui.ctx().armas_theme();

let dialog = FloatingWindow::new("Confirm Action")
    .id("confirm_dialog")
    .style(FloatingWindowStyle::Glass)
    .width(320.0)
    .height(200.0)
    .open(true);

let _response = dialog.show(ui.ctx(), &theme, |ui| {
    ui.heading("Are you sure?");
    ui.add_space(16.0);
    ui.label("This action cannot be undone.");
    ui.add_space(16.0);

    ui.horizontal(|ui| {
        if Button::new("Cancel").show(ui).clicked() {
            ui.label("Cancelled");
        }
        if Button::new("Confirm").show(ui).clicked() {
            ui.label("Confirmed!");
        }
    });
});
```

### Settings Panel
```demo
let theme = ui.ctx().armas_theme();

let settings = FloatingWindow::new("Preferences")
    .id("settings_panel")
    .style(FloatingWindowStyle::Surface)
    .width(400.0)
    .height(350.0);

let _response = settings.show(ui.ctx(), &theme, |ui| {
    ui.heading("Settings");
    ui.separator();

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 12.0;

        ui.label("Display Options:");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Transparency:");
            let (_resp, opacity) = Fader::new(0.8)
                .size(120.0, 180.0)
                .show(ui);
            ui.label(format!("{:.0}%", opacity * 100.0));
        });

        ui.add_space(16.0);

        if Button::new("Save Settings").show(ui).clicked() {
            ui.label("✓ Settings saved");
        }
    });
});
```

## Performance Notes

- Windows maintain their own state independently
- State persists across frames using egui's memory system
- Rendering is efficient with minimal overdraw
- Multiple windows render without significant performance impact

## Dependencies

- `egui = "0.33"`
- Armas theme system
- egui::Window for dragging/resizing

## Best Practices

1. **Use Unique IDs**: Always provide a unique ID for state persistence
2. **Choose Appropriate Style**: Use Glass for modern design, Surface for professional, Elevated for hierarchy
3. **Responsive Content**: Make window content responsive to window size
4. **Accessibility**: Include keyboard navigation in window controls
5. **Performance**: Avoid heavy computations in the content closure
