# Input Group

A text input with leading and trailing addon slots for icons, labels, or buttons.

```demo
let mut text = String::new();
InputGroup::new("ig_demo")
    .leading(|ui| { ui.label("🔍"); })
    .placeholder("Search...")
    .show(ui, &mut text);
```

## URL Input

Addons on both sides.

```demo
let mut text = String::new();
InputGroup::new("ig_url")
    .leading(|ui| { ui.label("https://"); })
    .trailing(|ui| { ui.label(".com"); })
    .placeholder("example")
    .width(350.0)
    .show(ui, &mut text);
```

## Currency Input

Leading label with trailing button.

```demo
let mut text = String::new();
InputGroup::new("ig_currency")
    .leading(|ui| { ui.label("$"); })
    .trailing(|ui| { Button::new("Send").variant(ButtonVariant::Ghost).show(ui); })
    .placeholder("0.00")
    .width(250.0)
    .show(ui, &mut text);
```
