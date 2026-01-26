# Table

A responsive table component with minimal styling and borders.

```demo
table(ui, |rows| {
    header_row(rows, |cells| {
        cell(cells, "Name");
        cell(cells, "Role");
        cell(cells, "Status");
    });
    row(rows, |cells| {
        cell(cells, "Alice");
        cell(cells, "Engineer");
        cell(cells, "Active");
    });
    row(rows, |cells| {
        cell(cells, "Bob");
        cell(cells, "Designer");
        cell(cells, "Away");
    });
    row(rows, |cells| {
        cell(cells, "Charlie");
        cell(cells, "Manager");
        cell(cells, "Active");
    });
});
```

## Basic Usage

```demo
table(ui, |rows| {
    header_row(rows, |cells| {
        cell(cells, "Invoice");
        cell(cells, "Status");
        cell(cells, "Method");
        cell(cells, "Amount");
    });
    row(rows, |cells| {
        cell(cells, "INV001");
        cell(cells, "Paid");
        cell(cells, "Credit Card");
        cell(cells, "$250.00");
    });
    row(rows, |cells| {
        cell(cells, "INV002");
        cell(cells, "Pending");
        cell(cells, "PayPal");
        cell(cells, "$150.00");
    });
    row(rows, |cells| {
        cell(cells, "INV003");
        cell(cells, "Paid");
        cell(cells, "Bank Transfer");
        cell(cells, "$350.00");
    });
});
```

## Multiple Rows

```demo
table(ui, |rows| {
    header_row(rows, |cells| {
        cell(cells, "ID");
        cell(cells, "Name");
        cell(cells, "Value");
    });
    for i in 1..=5 {
        row(rows, |cells| {
            cell(cells, &format!("{}", i));
            cell(cells, &format!("Item {}", i));
            cell(cells, &format!("{:.2}", i as f32 * 12.5));
        });
    }
});
```

## Custom Cell Content

```demo
table(ui, |rows| {
    header_row(rows, |cells| {
        cell(cells, "User");
        cell(cells, "Actions");
    });
    row(rows, |cells| {
        cell_ui(cells, |ui| {
            ui.horizontal(|ui| {
                ui.label("@");
                ui.label("Alice");
            });
        });
        cell_ui(cells, |ui| {
            ui.horizontal(|ui| {
                Button::new("Edit").variant(ButtonVariant::Text).show(ui, &theme);
                Button::new("Delete").variant(ButtonVariant::Text).show(ui, &theme);
            });
        });
    });
    row(rows, |cells| {
        cell_ui(cells, |ui| {
            ui.horizontal(|ui| {
                ui.label("@");
                ui.label("Bob");
            });
        });
        cell_ui(cells, |ui| {
            ui.horizontal(|ui| {
                Button::new("Edit").variant(ButtonVariant::Text).show(ui, &theme);
                Button::new("Delete").variant(ButtonVariant::Text).show(ui, &theme);
            });
        });
    });
});
```

## With Badges

```demo
table(ui, |rows| {
    header_row(rows, |cells| {
        cell(cells, "Project");
        cell(cells, "Status");
        cell(cells, "Priority");
    });
    row(rows, |cells| {
        cell(cells, "Website Redesign");
        cell_ui(cells, |ui| {
            Badge::new("In Progress").color(theme.chart_4()).show(ui, &theme);
        });
        cell_ui(cells, |ui| {
            Badge::new("High").destructive().show(ui, &theme);
        });
    });
    row(rows, |cells| {
        cell(cells, "API Migration");
        cell_ui(cells, |ui| {
            Badge::new("Completed").color(theme.chart_2()).show(ui, &theme);
        });
        cell_ui(cells, |ui| {
            Badge::new("High").destructive().show(ui, &theme);
        });
    });
    row(rows, |cells| {
        cell(cells, "Documentation");
        cell_ui(cells, |ui| {
            Badge::new("Pending").color(theme.chart_3()).show(ui, &theme);
        });
        cell_ui(cells, |ui| {
            Badge::new("Low").color(theme.chart_4()).show(ui, &theme);
        });
    });
});
```
