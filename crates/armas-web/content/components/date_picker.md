# DatePicker

Calendar date selection with input field, popover calendar, and keyboard navigation.

## Basic Usage

```demo
let mut date_picker = DatePicker::new("my_date_picker");
let mut selected_date = None;

date_picker.show(ui, &mut selected_date);
```

## With Label

```demo
let mut date_picker = DatePicker::new("birthday")
    .label("Birthday");
let mut selected_date = None;

date_picker.show(ui, &mut selected_date);
```

## Custom Placeholder

```demo
let mut date_picker = DatePicker::new("event_date")
    .placeholder("Choose event date...");
let mut selected_date = None;

date_picker.show(ui, &mut selected_date);
```

## Pre-Selected Date

```demo
let mut date_picker = DatePicker::new("appointment");
let mut selected_date = Some(Date::new(2024, 3, 15).unwrap());

date_picker.show(ui, &mut selected_date);
```

## Today's Date

```demo
let mut date_picker = DatePicker::new("today");
let mut selected_date = Some(Date::today());

date_picker.show(ui, &mut selected_date);
```

## Handling Changes

```demo
let mut date_picker = DatePicker::new("date");
let mut selected_date = None;

let response = date_picker.show(ui, &mut selected_date);

if response.changed {
    if let Some(date) = selected_date {
        let formatted = date.format(); // YYYY-MM-DD
        // Handle date change
    }
}
```

## Date Formatting

```demo
let date = Date::new(2024, 12, 25).unwrap();

// Format as YYYY-MM-DD
let formatted = date.format(); // "2024-12-25"

// Get month name
let month = date.month_name(); // "December"

// Get day of week (0=Sunday, 6=Saturday)
let day = date.day_of_week();
```

## Parsing Dates

```demo
// Parse from string
if let Some(date) = Date::parse("2024-03-15") {
    // Use parsed date
}

// Manual input support
let mut date_picker = DatePicker::new("manual");
let mut selected_date = None;
date_picker.show(ui, &mut selected_date);
// User can type "2024-03-15" directly in input field
```

## Working with Dates

```demo
// Create a date
let date = Date::new(2024, 2, 29); // Returns Option (checks validity)

// Get today's date
let today = Date::today();

// Check leap year
let is_leap = Date::is_leap_year(2024); // true

// Get days in month
let days = Date::days_in_month(2024, 2); // 29
```

## Complete Form Example

```demo
ui.vertical(|ui| {
    ui.heading("Event Registration");
    ui.add_space(8.0);

    let mut event_date = DatePicker::new("event")
        .label("Event Date")
        .placeholder("Select date...");
    let mut date = None;
    event_date.show(ui, &mut date);

    ui.add_space(8.0);

    let mut deadline = DatePicker::new("deadline")
        .label("Registration Deadline")
        .placeholder("Select deadline...");
    let mut deadline_date = None;
    deadline.show(ui, &mut deadline_date);
});
```

## Calendar Features

The date picker calendar includes:
- Month/year navigation with arrow buttons
- Weekday headers (Su-Sa)
- Current month days (clickable)
- Previous/next month days (greyed out)
- Today highlighted in turquoise
- Selected date highlighted in primary color
- "Today" quick action button
- "Clear" button to remove selection

## API Reference

### DatePicker

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.placeholder()` | `&str` | "Select a date..." | Input placeholder |
| `.label()` | `&str` | `None` | Label text |

### Date

| Method | Type | Description |
|--------|------|-------------|
| `Date::new(y, m, d)` | `Option<Date>` | Create date (validates) |
| `Date::today()` | `Date` | Get today's date |
| `Date::parse(s)` | `Option<Date>` | Parse YYYY-MM-DD |
| `.format()` | `String` | Format as YYYY-MM-DD |
| `.month_name()` | `&str` | Get month name |
| `.day_of_week()` | `u32` | Get day (0-6) |
| `Date::is_leap_year(y)` | `bool` | Check leap year |
| `Date::days_in_month(y, m)` | `u32` | Days in month |

### DatePickerResponse

| Field | Type | Description |
|-------|------|-------------|
| `changed` | `bool` | Whether date changed |

## Keyboard Support

- Type date directly in YYYY-MM-DD format
- Escape to close calendar

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `on_surface`, `surface_variant`
- Input component for text field
- Popover component for calendar overlay
