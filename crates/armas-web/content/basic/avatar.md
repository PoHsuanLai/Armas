# Avatar

User profile images with initials.

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Avatar::new("JD").show(ui, &theme);
    Avatar::new("AB").show(ui, &theme);
    Avatar::new("XY").show(ui, &theme);
});
```

## Sizes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Avatar::new("XS").size_preset(AvatarSize::XSmall).show(ui, &theme);
    Avatar::new("SM").size_preset(AvatarSize::Small).show(ui, &theme);
    Avatar::new("MD").size_preset(AvatarSize::Medium).show(ui, &theme);
    Avatar::new("LG").size_preset(AvatarSize::Large).show(ui, &theme);
    Avatar::new("XL").size_preset(AvatarSize::XLarge).show(ui, &theme);
});
```

## Shapes

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Avatar::new("CR").show(ui, &theme);
    Avatar::new("RS").shape(AvatarShape::Rounded).show(ui, &theme);
});
```

## Full Name Initials

```demo
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 8.0;
    Avatar::new("John Doe").show(ui, &theme);
    Avatar::new("Alice Marie Smith").show(ui, &theme);
    Avatar::new("Bob").show(ui, &theme);
});
```
