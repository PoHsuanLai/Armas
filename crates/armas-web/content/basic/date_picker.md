# DatePicker

Calendar date selection with button trigger and popover.

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

## With Footer Buttons

Show Today and Clear buttons for quick actions:

```demo
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("footer_date");
let mut selected_date: Option<Date> = ctx.data(|d| d.get_temp(date_id));

let mut date_picker = DatePicker::new("with_footer").show_footer(true);
date_picker.show(&ctx, &theme, ui, &mut selected_date);

ctx.data_mut(|d| d.insert_temp(date_id, selected_date));
```

## Custom Width

```demo
let ctx = ui.ctx().clone();
let theme = ctx.armas_theme();
let date_id = ui.id().with("width_date");
let mut selected_date: Option<Date> = ctx.data(|d| d.get_temp(date_id));

let mut date_picker = DatePicker::new("custom_width").width(200.0);
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

// Format as human-readable
let display = date.format_display(); // "December 25, 2024"

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
        .placeholder("Select deadline...")
        .show_footer(true);
    deadline.show(&ctx, &theme, ui, &mut deadline_date);
    ctx.data_mut(|d| d.insert_temp(deadline_id, deadline_date));
});
```

