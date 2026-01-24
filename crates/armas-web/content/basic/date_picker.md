# DatePicker

Calendar date selection with input field, popover calendar, and keyboard navigation.

## Basic Usage

```demo
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("basic_date");
let mut selected_date: Option<Date> = ctx.data(|d| d.get_temp(date_id));

let mut date_picker = DatePicker::new("my_date_picker");

date_picker.show(&ctx, &theme, ui, &mut selected_date);

ctx.data_mut(|d| d.insert_temp(date_id, selected_date));
```

## With Label

```demo
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("label_date");
let mut selected_date: Option<Date> = ctx.data(|d| d.get_temp(date_id));

let mut date_picker = DatePicker::new("birthday").label("Birthday");
date_picker.show(&ctx, &theme, ui, &mut selected_date);

ctx.data_mut(|d| d.insert_temp(date_id, selected_date));
```

## Custom Placeholder

```demo
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("placeholder_date");
let mut selected_date: Option<Date> = ctx.data(|d| d.get_temp(date_id));

let mut date_picker = DatePicker::new("event_date").placeholder("Choose event date...");
date_picker.show(&ctx, &theme, ui, &mut selected_date);

ctx.data_mut(|d| d.insert_temp(date_id, selected_date));
```

## Pre-Selected Date

```demo
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("preselected_date");
let mut selected_date: Option<Date> = ctx.data(|d| {
    d.get_temp(date_id).or_else(|| Some(Date::new(2024, 3, 15).unwrap()))
});

let mut date_picker = DatePicker::new("appointment");
date_picker.show(&ctx, &theme, ui, &mut selected_date);

ctx.data_mut(|d| d.insert_temp(date_id, selected_date));
```

## Today's Date

```demo
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("today_date");
let mut selected_date: Option<Date> = ctx.data(|d| {
    d.get_temp(date_id).or_else(|| Some(Date::today()))
});

let mut date_picker = DatePicker::new("today");
date_picker.show(&ctx, &theme, ui, &mut selected_date);

ctx.data_mut(|d| d.insert_temp(date_id, selected_date));
```

## Handling Changes

```demo
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("changes_date");
let mut selected_date: Option<Date> = ctx.data(|d| d.get_temp(date_id));

let mut date_picker = DatePicker::new("date");
let response = date_picker.show(&ctx, &theme, ui, &mut selected_date);

if response.changed {
    if let Some(date) = selected_date {
        let formatted = date.format(); // YYYY-MM-DD
        ui.label(format!("Selected: {}", formatted));
    }
}

ctx.data_mut(|d| d.insert_temp(date_id, selected_date));
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
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("manual_date");
let mut selected_date: Option<Date> = ctx.data(|d| d.get_temp(date_id));

let mut date_picker = DatePicker::new("manual");
date_picker.show(&ctx, &theme, ui, &mut selected_date);
// User can type "2024-03-15" directly in input field

ctx.data_mut(|d| d.insert_temp(date_id, selected_date));
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
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
ui.vertical(|ui| {
    ui.heading("Event Registration");
    ui.add_space(8.0);

    let event_date_id = ui.id().with("event_date");
    let mut event_date_value: Option<Date> = ctx.data(|d| d.get_temp(event_date_id));
    let mut event_date = DatePicker::new("event")
        .label("Event Date")
        .placeholder("Select date...");
    event_date.show(&ctx, &theme, ui, &mut event_date_value);
    ctx.data_mut(|d| d.insert_temp(event_date_id, event_date_value));

    ui.add_space(8.0);

    let deadline_id = ui.id().with("deadline_date");
    let mut deadline_date: Option<Date> = ctx.data(|d| d.get_temp(deadline_id));
    let mut deadline = DatePicker::new("deadline")
        .label("Registration Deadline")
        .placeholder("Select deadline...");
    deadline.show(&ctx, &theme, ui, &mut deadline_date);
    ctx.data_mut(|d| d.insert_temp(deadline_id, deadline_date));
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
