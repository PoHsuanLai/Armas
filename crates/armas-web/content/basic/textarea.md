# Textarea

Multi-line text input field with validation states.

```demo
let mut text = String::new();
Textarea::new("Enter your message").id("demo_textarea").show(ui, &mut text);
```

## With Label

```demo
let mut text = String::new();
Textarea::new("Type here...").label("Description").id("demo_textarea_label").show(ui, &mut text);
```

## Variants

```demo
ui.vertical(|ui| {
    let mut default = String::new();
    Textarea::new("Default").id("demo_textarea_default").show(ui, &mut default);
    let mut filled = String::new();
    Textarea::new("Filled").variant(InputVariant::Filled).id("demo_textarea_filled").show(ui, &mut filled);
});
```

## With Character Limit

```demo
let mut text = String::new();
Textarea::new("Enter bio").label("Bio").max_chars(200).id("demo_textarea_charlimit").show(ui, &mut text);
```

## Validation States

```demo
ui.vertical(|ui| {
    let mut valid = String::from("Great feedback!");
    Textarea::new("Enter feedback").state(InputState::Success).helper_text("Thank you for your feedback").id("demo_textarea_success").show(ui, &mut valid);
    let mut invalid = String::new();
    Textarea::new("Enter message").state(InputState::Error).helper_text("Message is required").id("demo_textarea_error").show(ui, &mut invalid);
});
```
