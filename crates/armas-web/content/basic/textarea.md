# Textarea

Multi-line text input field with validation states.

```demo
let mut text = String::new();
Textarea::new("Enter your message").show(ui, &mut text);
```

## With Label

```demo
let mut text = String::new();
Textarea::new("Type here...").label("Description").show(ui, &mut text);
```

## Variants

```demo
ui.vertical(|ui| {
    let mut default = String::new();
    Textarea::new("Default").show(ui, &mut default);
    let mut outlined = String::new();
    Textarea::new("Outlined").variant(InputVariant::Outlined).show(ui, &mut outlined);
});
```

## With Character Limit

```demo
let mut text = String::new();
Textarea::new("Enter bio").label("Bio").max_chars(200).show(ui, &mut text);
```

## Validation States

```demo
ui.vertical(|ui| {
    let mut valid = String::from("Great feedback!");
    Textarea::new("Enter feedback").state(InputState::Success).helper_text("Thank you for your feedback").show(ui, &mut valid);
    let mut invalid = String::new();
    Textarea::new("Enter message").state(InputState::Error).helper_text("Message is required").show(ui, &mut invalid);
});
```
