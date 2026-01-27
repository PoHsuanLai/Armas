# Themes

Armas uses a shadcn/ui inspired theme system with serializable color palettes.

## Built-in Themes

```demo
use egui::Id;

let theme_id = Id::new("selected_theme");
let mut selected = ui.data_mut(|d| d.get_temp::<usize>(theme_id).unwrap_or(0));

ui.horizontal(|ui| {
    if Button::new("Dark")
        .variant(if selected == 0 { ButtonVariant::Default } else { ButtonVariant::Outline })
        .show(ui, &theme)
        .clicked()
    {
        selected = 0;
        ui.data_mut(|d| d.insert_temp(theme_id, selected));
    }

    if Button::new("Light")
        .variant(if selected == 1 { ButtonVariant::Default } else { ButtonVariant::Outline })
        .show(ui, &theme)
        .clicked()
    {
        selected = 1;
        ui.data_mut(|d| d.insert_temp(theme_id, selected));
    }
});

let theme_name = match selected {
    0 => "Dark Theme (Zinc)",
    1 => "Light Theme (Zinc)",
    _ => "Dark Theme",
};

ui.add_space(8.0);
ui.label(format!("Selected: {}", theme_name));
```

## Creating Custom Themes

You can create your own theme by modifying the color palette:

```rust
let mut theme = Theme::dark();

// Customize colors (RGB values)
theme.colors.primary = [100, 150, 255];
theme.colors.secondary = [255, 100, 150];
theme.colors.background = [15, 15, 20];
```

## Theme Colors

All theme colors follow shadcn/ui naming conventions:

**Base Colors:**
- `background` - Default page background
- `foreground` - Default text color

**Surface Colors:**
- `card` / `card_foreground` - Card backgrounds
- `popover` / `popover_foreground` - Popover/dropdown backgrounds

**Brand Colors:**
- `primary` / `primary_foreground` - Primary brand color
- `secondary` / `secondary_foreground` - Secondary brand color
- `accent` / `accent_foreground` - Accent highlights

**Utility Colors:**
- `muted` / `muted_foreground` - Muted/subtle elements
- `destructive` / `destructive_foreground` - Destructive actions (delete, etc.)

**Border & Input:**
- `border` - Default border color
- `input` - Input field borders
- `ring` - Focus ring color

**State Colors:**
- `hover` - Hover state background
- `focus` - Focus state accent

**Chart Colors:**
- `chart_1` through `chart_5` - Data visualization colors

**Sidebar Colors:**
- `sidebar`, `sidebar_foreground`, `sidebar_primary`, `sidebar_accent`, etc.

## Saving and Loading Themes

Themes are serializable using serde. You can save/load them as JSON:

```rust
use std::fs;

// Serialize to JSON
let json = serde_json::to_string_pretty(&theme)?;
fs::write("my_theme.json", json)?;

// Deserialize from JSON
let json = fs::read_to_string("my_theme.json")?;
let theme: Theme = serde_json::from_str(&json)?;
```

## Using Custom Themes

Apply a theme to your egui app using the extension trait:

```rust
use armas::ext::ArmasContextExt;

// Set theme globally
ctx.set_armas_theme(my_custom_theme);

// Get current theme
let theme = ctx.armas_theme();
```
