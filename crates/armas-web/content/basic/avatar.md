# Avatar

User profile images with initials.

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Avatar::new("JD").show(ui);
    Avatar::new("AB").show(ui);
    Avatar::new("XY").show(ui);
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Avatar::new("XS").size_preset(AvatarSize::XSmall).show(ui);
    Avatar::new("SM").size_preset(AvatarSize::Small).show(ui);
    Avatar::new("MD").size_preset(AvatarSize::Medium).show(ui);
    Avatar::new("LG").size_preset(AvatarSize::Large).show(ui);
    Avatar::new("XL").size_preset(AvatarSize::XLarge).show(ui);
});
```

## Shapes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Avatar::new("CR").show(ui);
    Avatar::new("RS").shape(AvatarShape::Rounded).show(ui);
});
```

## Full Name Initials

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Avatar::new("John Doe").show(ui);
    Avatar::new("Alice Marie Smith").show(ui);
    Avatar::new("Bob").show(ui);
});
```
