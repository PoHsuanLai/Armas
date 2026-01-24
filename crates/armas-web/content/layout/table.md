# Table

Styled tables with multiple visual variants. Part of Armas layout module with builder API.

## Basic Usage

```demo
Table::new()
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("Name");
            row.cell("Role");
            row.cell("Status");
        });

        table.row(|row| {
            row.cell("Alice");
            row.cell("Engineer");
            row.cell("Active");
        });

        table.row(|row| {
            row.cell("Bob");
            row.cell("Designer");
            row.cell("Away");
        });

        table.row(|row| {
            row.cell("Charlie");
            row.cell("Manager");
            row.cell("Active");
        });
    });
```

## Striped Style

Alternating row backgrounds for better readability.

```demo
Table::new()
    .style(TableStyle::Striped)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("Product");
            row.cell("Price");
            row.cell("Stock");
        });

        table.row(|row| {
            row.cell("Widget A");
            row.cell("$19.99");
            row.cell("150");
        });

        table.row(|row| {
            row.cell("Widget B");
            row.cell("$29.99");
            row.cell("87");
        });

        table.row(|row| {
            row.cell("Widget C");
            row.cell("$39.99");
            row.cell("203");
        });

        table.row(|row| {
            row.cell("Widget D");
            row.cell("$49.99");
            row.cell("45");
        });
    });
```

## Bordered Style

Full grid with borders around all cells.

```demo
Table::new()
    .style(TableStyle::Bordered)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("Feature");
            row.cell("Basic");
            row.cell("Pro");
        });

        table.row(|row| {
            row.cell("Users");
            row.cell("5");
            row.cell("Unlimited");
        });

        table.row(|row| {
            row.cell("Storage");
            row.cell("10GB");
            row.cell("1TB");
        });

        table.row(|row| {
            row.cell("Support");
            row.cell("Email");
            row.cell("24/7");
        });
    });
```

## Lined Style

Horizontal dividers between rows only.

```demo
Table::new()
    .style(TableStyle::Lined)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("Task");
            row.cell("Status");
            row.cell("Priority");
        });

        table.row(|row| {
            row.cell("Design mockups");
            row.cell("Done");
            row.cell("High");
        });

        table.row(|row| {
            row.cell("Implement API");
            row.cell("In Progress");
            row.cell("High");
        });

        table.row(|row| {
            row.cell("Write tests");
            row.cell("Pending");
            row.cell("Medium");
        });
    });
```

## Compact Mode

Reduced padding for dense data.

```demo
Table::new()
    .style(TableStyle::Striped)
    .compact(true)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("ID");
            row.cell("Name");
            row.cell("Value");
        });

        for i in 1..=5 {
            table.row(|row| {
                row.cell(format!("{}", i));
                row.cell(format!("Item {}", i));
                row.cell(format!("{:.2}", i as f32 * 12.5));
            });
        }
    });
```

## Custom Cell Content

Use closures for rich cell content.

```demo
Table::new()
    .style(TableStyle::Lined)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("User");
            row.cell("Actions");
        });

        table.row(|row| {
            row.cell_ui(|ui| {
                ui.horizontal(|ui| {
                    ui.label("@");
                    ui.label("Alice");
                });
            });

            row.cell_ui(|ui| {
                ui.horizontal(|ui| {
                    Button::new("Edit").variant(ButtonVariant::Text).show(ui);
                    Button::new("Delete").variant(ButtonVariant::Text).show(ui);
                });
            });
        });

        table.row(|row| {
            row.cell_ui(|ui| {
                ui.horizontal(|ui| {
                    ui.label("@");
                    ui.label("Bob");
                });
            });

            row.cell_ui(|ui| {
                ui.horizontal(|ui| {
                    Button::new("Edit").variant(ButtonVariant::Text).show(ui);
                    Button::new("Delete").variant(ButtonVariant::Text).show(ui);
                });
            });
        });
    });
```

## Mixed Content Types

```demo
Table::new()
    .style(TableStyle::Bordered)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("Setting");
            row.cell("Value");
            row.cell("Control");
        });

        table.row(|row| {
            row.cell("Notifications");
            row.cell("Enabled");
            row.cell_ui(|ui| {
                ui.checkbox(&mut true, "");
            });
        });

        table.row(|row| {
            row.cell("Volume");
            row.cell("75%");
            row.cell_ui(|ui| {
                let mut value = 75.0;
                ui.add(egui::Slider::new(&mut value, 0.0..=100.0).show_value(false));
            });
        });

        table.row(|row| {
            row.cell("Theme");
            row.cell("Dark");
            row.cell_ui(|ui| {
                ui.label("Dark");
            });
        });
    });
```

## With Badges

```demo
Table::new()
    .style(TableStyle::Striped)
    .show(ui, |table| {
        table.header_row(|row| {
            row.cell("Project");
            row.cell("Status");
            row.cell("Priority");
        });

        table.row(|row| {
            row.cell("Website Redesign");
            row.cell_ui(|ui| {
                Badge::new("In Progress")
                    .color(theme.chart_4())
                    .show(ui);
            });
            row.cell_ui(|ui| {
                Badge::new("High")
                    .destructive()
                    .show(ui);
            });
        });

        table.row(|row| {
            row.cell("API Migration");
            row.cell_ui(|ui| {
                Badge::new("Completed")
                    .color(theme.chart_2())
                    .show(ui);
            });
            row.cell_ui(|ui| {
                Badge::new("High")
                    .destructive()
                    .show(ui);
            });
        });

        table.row(|row| {
            row.cell("Documentation");
            row.cell_ui(|ui| {
                Badge::new("Pending")
                    .color(theme.chart_3())
                    .show(ui);
            });
            row.cell_ui(|ui| {
                Badge::new("Low")
                    .color(theme.chart_4())
                    .show(ui);
            });
        });
    });
```

## API Reference

### Table

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | - | - | Create new table |
| `.style()` | `TableStyle` | `Default` | Visual style |
| `.compact()` | `bool` | `false` | Reduce padding for dense data |
| `.hoverable()` | `bool` | `false` | Highlight rows on hover |
| `.show()` | `(ui, closure)` | - | Render table with builder |

### TableStyle

| Variant | Description |
|---------|-------------|
| `Default` | Minimal styling with spacing only |
| `Striped` | Alternating row backgrounds (zebra striping) |
| `Bordered` | Full grid with borders around all cells |
| `Lined` | Horizontal dividers between rows only |

### TableBuilder

| Method | Description |
|--------|-------------|
| `.header_row(closure)` | Add header row with cells |
| `.row(closure)` | Add data row with cells |

### RowBuilder

| Method | Description |
|--------|-------------|
| `.cell(text)` | Add simple text cell |
| `.cell_ui(closure)` | Add cell with custom UI |

## Notes

- Table automatically calculates column widths
- Cells can contain any UI elements via `cell_ui()`
- Theme colors are applied automatically
- Headers have bold text by default
- Striped style uses theme surface variant
- Minimum column width is handled automatically
