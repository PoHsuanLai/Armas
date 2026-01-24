# Focus Cards

Grid of cards that highlight on hover with blur effect. General-purpose component that accepts any card content.

## Basic Usage

```demo
FocusCards::new()
    .columns(3)
    .card_size(300.0, 400.0)
    .show(ui, &theme, |cards| {
        cards.card(|ui, theme, _opacity| {
            Card::new()
                .variant(CardVariant::Filled)
                .show(ui, theme, |ui| {
                    ui.heading("Feature 1");
                    ui.label("Description 1");
                });
        });

        cards.card(|ui, theme, _opacity| {
            Card::new()
                .variant(CardVariant::Outlined)
                .show(ui, theme, |ui| {
                    ui.heading("Feature 2");
                    ui.label("Description 2");
                });
        });

        cards.card(|ui, theme, _opacity| {
            Card::new()
                .variant(CardVariant::Elevated)
                .show(ui, theme, |ui| {
                    ui.heading("Feature 3");
                    ui.label("Description 3");
                });
        });
    });
```

## Custom Content

You can put any content in the cards, not just base Cards:

```demo
FocusCards::new()
    .columns(2)
    .card_size(350.0, 450.0)
    .show(ui, &theme, |cards| {
        cards.card(|ui, _theme, _opacity| {
            // Custom content with image
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("Custom Card");
                ui.label("You can add any content here");
                // The opacity parameter lets you adjust your content's appearance
            });
        });

        cards.card(|ui, _theme, _opacity| {
            // Another custom card
            ui.label("Another card with different content");
        });
    });
```

## API Reference

### FocusCards

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create focus cards grid |
| `.columns()` | `usize` | `3` | Number of columns |
| `.card_size()` | `(f32, f32)` | `(300, 400)` | Width and height of each card |
| `.spacing()` | `f32` | `20.0` | Spacing between cards |
| `.show()` | `(&mut Ui, &Theme, closure)` | - | Render with closure-based API |

### FocusCardsBuilder (in closure)

| Method | Type | Description |
|--------|------|-------------|
| `.card()` | `FnOnce(&mut Ui, &Theme, f32)` | Add a card with closure. The `f32` parameter is the opacity factor (0.0-1.0) |

### FocusCardResponse

| Field | Type | Description |
|-------|------|-------------|
| `.response` | `Response` | The overall grid response |
| `.clicked` | `Option<usize>` | Index of clicked card, if any |
| `.hovered` | `Option<usize>` | Index of hovered card, if any |

## Dependencies

- `egui = "0.33"`
- Theme colors: `surface`, `primary`, `outline_variant`
