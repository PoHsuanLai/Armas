# Input

Text input fields with icons and validation states.

```demo
let mut text = String::new();
Input::new("Enter your name").id("input_1").show(ui, &mut text, &theme);
```

## Variants

```demo
ui.vertical(|ui| {
    let mut default = String::new();
    Input::new("Default").id("default").show(ui, &mut default, &theme);
    let mut outlined = String::new();
    Input::new("Outlined").id("outlined").variant(InputVariant::Outlined).show(ui, &mut outlined, &theme);
    let mut filled = String::new();
    Input::new("Filled").id("filled").variant(InputVariant::Filled).show(ui, &mut filled, &theme);
});
```

## With Label

```demo
let mut text = String::new();
Input::new("Enter email").id("labeled").label("Email Address").show(ui, &mut text, &theme);
```

## With Icons

```demo
ui.vertical(|ui| {
    let mut search = String::new();
    Input::new("Search...").id("search").left_icon("*").show(ui, &mut search, &theme);
    let mut username = String::new();
    Input::new("Username").id("username").left_icon("@").right_icon("ok").show(ui, &mut username, &theme);
});
```

## Validation States

```demo
ui.vertical(|ui| {
    let mut valid = String::from("valid@email.com");
    Input::new("Enter email").id("valid").state(InputState::Success).helper_text("Email is valid").show(ui, &mut valid, &theme);
    let mut invalid = String::from("invalid");
    Input::new("Enter email").id("invalid").state(InputState::Error).helper_text("Please enter a valid email").show(ui, &mut invalid, &theme);
});
```

## Password

```demo
let mut password = String::new();
Input::new("Enter password").id("password").label("Password").password(true).left_icon("*").show(ui, &mut password, &theme);
```

## Inline

```demo
let mut name = String::from("Click to edit");
Input::new("").id("inline").variant(InputVariant::Inline).show(ui, &mut name, &theme);
```
