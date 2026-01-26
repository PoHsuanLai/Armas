# Table

Styled tables with multiple visual variants.

```demo
Table::new().show(ui, |table| {
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

## Styles

```demo
ui.vertical(|ui| {
    ui.spacing_mut().item_spacing.y = 24.0;
    ui.label("Striped");
    Table::new().style(TableStyle::Striped).show(ui, |table| {
        table.header_row(|row| {
            row.cell("Product");
            row.cell("Price");
        });
        table.row(|row| {
            row.cell("Widget A");
            row.cell("$19.99");
        });
        table.row(|row| {
            row.cell("Widget B");
            row.cell("$29.99");
        });
    });
    ui.label("Bordered");
    Table::new().style(TableStyle::Bordered).show(ui, |table| {
        table.header_row(|row| {
            row.cell("Feature");
            row.cell("Value");
        });
        table.row(|row| {
            row.cell("Users");
            row.cell("5");
        });
        table.row(|row| {
            row.cell("Storage");
            row.cell("10GB");
        });
    });
    ui.label("Lined");
    Table::new().style(TableStyle::Lined).show(ui, |table| {
        table.header_row(|row| {
            row.cell("Task");
            row.cell("Status");
        });
        table.row(|row| {
            row.cell("Design mockups");
            row.cell("Done");
        });
        table.row(|row| {
            row.cell("Implement API");
            row.cell("In Progress");
        });
    });
});
```

## Compact Mode

```demo
Table::new().style(TableStyle::Striped).compact(true).show(ui, |table| {
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

```demo
Table::new().style(TableStyle::Lined).show(ui, |table| {
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

## With Badges

```demo
Table::new().style(TableStyle::Striped).show(ui, |table| {
    table.header_row(|row| {
        row.cell("Project");
        row.cell("Status");
        row.cell("Priority");
    });
    table.row(|row| {
        row.cell("Website Redesign");
        row.cell_ui(|ui| {
            Badge::new("In Progress").color(theme.chart_4()).show(ui);
        });
        row.cell_ui(|ui| {
            Badge::new("High").destructive().show(ui);
        });
    });
    table.row(|row| {
        row.cell("API Migration");
        row.cell_ui(|ui| {
            Badge::new("Completed").color(theme.chart_2()).show(ui);
        });
        row.cell_ui(|ui| {
            Badge::new("High").destructive().show(ui);
        });
    });
    table.row(|row| {
        row.cell("Documentation");
        row.cell_ui(|ui| {
            Badge::new("Pending").color(theme.chart_3()).show(ui);
        });
        row.cell_ui(|ui| {
            Badge::new("Low").color(theme.chart_4()).show(ui);
        });
    });
});
```
