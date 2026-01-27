# Select

Searchable dropdown menus with keyboard navigation.

## Basic Usage

```demo
let options = vec![
    SelectOption::new("1", "Option 1"),
    SelectOption::new("2", "Option 2"),
    SelectOption::new("3", "Option 3"),
];

let mut select = Select::new(options).id("select_1");
select.show(ui, &theme);
```

## With Label

```demo
let options = vec![
    SelectOption::new("us", "United States"),
    SelectOption::new("uk", "United Kingdom"),
    SelectOption::new("ca", "Canada"),
];

let mut select = Select::new(options)
    .id("select_2")
    .label("Country");
select.show(ui, &theme);
```

## With Icons

```demo
let options = vec![
    SelectOption::new("home", "Home"),
    SelectOption::new("work", "Work"),
    SelectOption::new("other", "Other"),
];

let mut select = Select::new(options)
    .id("select_3");
select.show(ui, &theme);
```

## With Descriptions

```demo
let options = vec![
    SelectOption::new("free", "Free Plan")
        .description("Perfect for getting started"),
    SelectOption::new("pro", "Pro Plan")
        .description("For professionals and teams"),
    SelectOption::new("enterprise", "Enterprise")
        .description("Advanced features and support"),
];

let mut select = Select::new(options)
    .id("select_4");
select.show(ui, &theme);
```


## With Disabled Options

```demo
let options = vec![
    SelectOption::new("enabled", "Available Option"),
    SelectOption::new("disabled", "Unavailable Option")
        .disabled(true),
    SelectOption::new("another", "Another Option"),
];

let mut select = Select::new(options)
    .id("select_5");
select.show(ui, &theme);
```

## With Pre-Selection

```demo
let options = vec![
    SelectOption::new("red", "Red"),
    SelectOption::new("green", "Green"),
    SelectOption::new("blue", "Blue"),
];

let mut select = Select::new(options)
    .id("select_6")
    .selected("green");
select.show(ui, &theme);
```

## Non-Searchable

```demo
let options = vec![
    SelectOption::new("sm", "Small"),
    SelectOption::new("md", "Medium"),
    SelectOption::new("lg", "Large"),
];

let mut select = Select::new(options)
    .id("select_7")
    .searchable(false);
select.show(ui, &theme);
```

## Custom Width

```demo
let options = vec![
    SelectOption::new("1", "Option 1"),
    SelectOption::new("2", "Option 2"),
];

let mut select = Select::new(options)
    .id("select_8")
    .width(300.0);
select.show(ui, &theme);
```

## Handling Selection

```demo
let options = vec![
    SelectOption::new("1", "Option 1"),
    SelectOption::new("2", "Option 2"),
];

let mut select = Select::new(options)
    .id("select_9");
let response = select.show(ui, &theme);

if response.changed {
    if let Some(value) = response.selected_value {
        // Handle selection change
    }
}
```

