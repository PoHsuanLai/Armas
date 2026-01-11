# Avatar

Display user profile images with initials or icons.

## Basic Usage

```demo
Avatar::new("JD").show(ui);
ui.add_space(8.0);
Avatar::new("AB").show(ui);
ui.add_space(8.0);
Avatar::new("XY").show(ui);
```

## Sizes

### Extra Small

```demo
Avatar::new("XS")
    .size(AvatarSize::XSmall)
    .show(ui);
```

### Small

```demo
Avatar::new("SM")
    .size(AvatarSize::Small)
    .show(ui);
```

### Medium (Default)

```demo
Avatar::new("MD")
    .size(AvatarSize::Medium)
    .show(ui);
```

### Large

```demo
Avatar::new("LG")
    .size(AvatarSize::Large)
    .show(ui);
```

### Extra Large

```demo
Avatar::new("XL")
    .size(AvatarSize::XLarge)
    .show(ui);
```

### Custom Size

```demo
Avatar::new("128")
    .size(AvatarSize::Custom(128.0))
    .show(ui);
```

## Shapes

### Circle (Default)

```demo
Avatar::new("CR")
    .shape(AvatarShape::Circle)
    .show(ui);
```

### Rounded Square

```demo
Avatar::new("RS")
    .shape(AvatarShape::RoundedSquare)
    .show(ui);
```

### Square

```demo
Avatar::new("SQ")
    .shape(AvatarShape::Square)
    .show(ui);
```

## Custom Colors

```demo
let theme = ui.ctx().armas_theme();
Avatar::new("AM")
    .color(theme.primary())
    .show(ui);
ui.add_space(8.0);
Avatar::new("JD")
    .color(theme.success())
    .show(ui);
ui.add_space(8.0);
Avatar::new("KL")
    .color(theme.error())
    .show(ui);
```

## With Border

```demo
Avatar::new("BD")
    .with_border(true)
    .show(ui);
```

## With Status Badge

### Online (Success)

```demo
Avatar::new("ON")
    .status(BadgeColor::Success)
    .show(ui);
```

### Busy (Warning)

```demo
Avatar::new("BY")
    .status(BadgeColor::Warning)
    .show(ui);
```

### Offline (Neutral)

```demo
Avatar::new("OF")
    .status(BadgeColor::Neutral)
    .show(ui);
```

## Clickable

```demo
let response = Avatar::new("CL")
    .clickable()
    .show(ui);

if response.clicked() {
    // Handle click
}
```

## Avatar Group

```demo
ui.horizontal(|ui| {
    Avatar::new("A1")
        .size(AvatarSize::Small)
        .show(ui);
    Avatar::new("A2")
        .size(AvatarSize::Small)
        .show(ui);
    Avatar::new("A3")
        .size(AvatarSize::Small)
        .show(ui);
    ui.label("+5");
});
```

## Combined Examples

### User Profile

```demo
ui.horizontal(|ui| {
    Avatar::new("JD")
        .size(AvatarSize::Large)
        .status(BadgeColor::Success)
        .show(ui);

    ui.vertical(|ui| {
        ui.label("John Doe");
        ui.label("Online");
    });
});
```

### Team Members

```demo
ui.horizontal(|ui| {
    let theme = ui.ctx().armas_theme();

    Avatar::new("AM")
        .color(theme.primary())
        .with_border(true)
        .show(ui);
    ui.add_space(8.0);

    Avatar::new("BK")
        .color(theme.success())
        .with_border(true)
        .show(ui);
    ui.add_space(8.0);

    Avatar::new("CL")
        .color(theme.info())
        .with_border(true)
        .show(ui);
});
```

### Different Initials Lengths

```demo
ui.horizontal(|ui| {
    Avatar::new("A").show(ui);
    ui.add_space(8.0);
    Avatar::new("AB").show(ui);
    ui.add_space(8.0);
    Avatar::new("ABC").show(ui);
});
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.size()` | `AvatarSize` | `Medium` | Avatar size preset |
| `.shape()` | `AvatarShape` | `Circle` | Avatar shape |
| `.color()` | `Color32` | Auto | Custom background color |
| `.text_color()` | `Color32` | White | Custom text color |
| `.with_border()` | `bool` | `false` | Show border |
| `.clickable()` | - | `false` | Enable click interaction |
| `.status()` | `BadgeColor` | `None` | Show status badge |

## Sizes

- `AvatarSize::XSmall` - 24px
- `AvatarSize::Small` - 32px
- `AvatarSize::Medium` - 48px (default)
- `AvatarSize::Large` - 64px
- `AvatarSize::XLarge` - 96px
- `AvatarSize::Custom(f32)` - Custom size

## Shapes

- `AvatarShape::Circle` - Circular (default)
- `AvatarShape::RoundedSquare` - Rounded corners
- `AvatarShape::Square` - Sharp corners

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `success`, `error`, `info`, `surface`
