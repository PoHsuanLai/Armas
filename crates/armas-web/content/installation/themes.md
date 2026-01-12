# Themes

Armas uses a Material Design inspired theme system with serializable color palettes.

## Built-in Themes

```demo
use egui::Id;

let theme_id = Id::new("selected_theme");
let mut selected = ui.data_mut(|d| d.get_temp::<usize>(theme_id).unwrap_or(0));

ui.horizontal(|ui| {
    if Button::new("Dark")
        .variant(if selected == 0 { ButtonVariant::Filled } else { ButtonVariant::Outlined })
        .show(ui)
        .clicked()
    {
        selected = 0;
        ui.data_mut(|d| d.insert_temp(theme_id, selected));
    }

    if Button::new("Light")
        .variant(if selected == 1 { ButtonVariant::Filled } else { ButtonVariant::Outlined })
        .show(ui)
        .clicked()
    {
        selected = 1;
        ui.data_mut(|d| d.insert_temp(theme_id, selected));
    }

    if Button::new("Nord")
        .variant(if selected == 2 { ButtonVariant::Filled } else { ButtonVariant::Outlined })
        .show(ui)
        .clicked()
    {
        selected = 2;
        ui.data_mut(|d| d.insert_temp(theme_id, selected));
    }
});

let theme_name = match selected {
    0 => "Dark Theme",
    1 => "Light Theme",
    2 => "Nord Theme",
    _ => "Dark Theme",
};

ui.add_space(8.0);
ui.label(format!("Selected: {}", theme_name));
```

## Creating Custom Themes

You can create your own theme by modifying the color palette:

```rust
let mut theme = Theme::dark();

// Customize colors
theme.colors.primary = [100, 150, 255];
theme.colors.secondary = [255, 100, 150];
theme.colors.background = [15, 15, 20];
```

## Theme Colors

All theme colors follow Material Design naming:

- **Brand**: `primary`, `secondary`
- **Surfaces**: `background`, `surface`, `surface_variant`
- **Text**: `on_background`, `on_surface`, `on_surface_variant`
- **Borders**: `outline`, `outline_variant`
- **States**: `hover`, `focus`
- **Semantic**: `error`, `warning`, `success`, `info`

## Saving and Loading Themes

Themes are serializable and can be saved/loaded:

```rust
// Save theme
theme.save_to_file("my_theme.json")?;

// Load theme
let theme = Theme::load_from_file("my_theme.json")?;
```
