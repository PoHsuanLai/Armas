# Avatar

User profile images with initials, styled like shadcn/ui Avatar.

## Basic Usage

```demo
ui.horizontal(|ui| {
    Avatar::new("JD").show(ui);
    ui.add_space(8.0);
    Avatar::new("AB").show(ui);
    ui.add_space(8.0);
    Avatar::new("XY").show(ui);
});
```

## Sizes

### Using Presets

```demo
ui.horizontal(|ui| {
    Avatar::new("XS").size_preset(AvatarSize::XSmall).show(ui);
    ui.add_space(8.0);
    Avatar::new("SM").size_preset(AvatarSize::Small).show(ui);
    ui.add_space(8.0);
    Avatar::new("MD").size_preset(AvatarSize::Medium).show(ui);
    ui.add_space(8.0);
    Avatar::new("LG").size_preset(AvatarSize::Large).show(ui);
    ui.add_space(8.0);
    Avatar::new("XL").size_preset(AvatarSize::XLarge).show(ui);
});
```

### Custom Size

```demo
ui.horizontal(|ui| {
    Avatar::new("20").size(20.0).show(ui);
    ui.add_space(8.0);
    Avatar::new("40").size(40.0).show(ui);
    ui.add_space(8.0);
    Avatar::new("60").size(60.0).show(ui);
});
```

## Shapes

### Circle (Default)

```demo
Avatar::new("CR").show(ui);
```

### Rounded Square

```demo
Avatar::new("RS").shape(AvatarShape::Rounded).show(ui);
```

## Full Name Initials

Pass a full name to automatically extract initials:

```demo
ui.horizontal(|ui| {
    Avatar::new("John Doe").show(ui);
    ui.add_space(8.0);
    Avatar::new("Alice Marie Smith").show(ui);
    ui.add_space(8.0);
    Avatar::new("Bob").show(ui);
});
```

## Avatar Group

```demo
ui.horizontal(|ui| {
    Avatar::new("A").show(ui);
    Avatar::new("B").show(ui);
    Avatar::new("C").show(ui);
    ui.label("+5");
});
```

## Combined Examples

### User Profile

```demo
ui.horizontal(|ui| {
    Avatar::new("John Doe")
        .size_preset(AvatarSize::Large)
        .show(ui);
    ui.add_space(12.0);
    ui.vertical(|ui| {
        ui.label("John Doe");
        ui.label("john@example.com");
    });
});
```

### Comment Thread

```demo
ui.vertical(|ui| {
    ui.horizontal(|ui| {
        Avatar::new("AM").size_preset(AvatarSize::Small).show(ui);
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.label("Alice Miller");
            ui.label("Great work on this feature!");
        });
    });
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        Avatar::new("BK").size_preset(AvatarSize::Small).show(ui);
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.label("Bob King");
            ui.label("Thanks for the review!");
        });
    });
});
```

## API Reference

### AvatarSize Enum

```rust
pub enum AvatarSize {
    XSmall,      // 24px
    Small,       // 32px (default)
    Medium,      // 40px
    Large,       // 48px
    XLarge,      // 64px
    Custom(f32), // Custom size
}
```

### AvatarShape Enum

```rust
pub enum AvatarShape {
    Circle,  // Circular (default)
    Rounded, // Rounded square
}
```

### Avatar

#### Constructor

```rust
Avatar::new(text: impl Into<String>) -> Self
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.size()` | `f32` | `32.0` | Avatar size in pixels |
| `.size_preset()` | `AvatarSize` | `Small` | Use size preset |
| `.shape()` | `AvatarShape` | `Circle` | Avatar shape |

#### Show Method

```rust
pub fn show(self, ui: &mut Ui) -> Response
```

## shadcn/ui Styling

The Avatar follows shadcn/ui conventions:

- **Default size**: 32px (size-8)
- **Shape**: `rounded-full` for circle
- **Background**: `bg-muted`
- **Text**: `text-muted-foreground`
- **Font size**: 40% of avatar size
- **Initials**: Automatically extracted from name (first letter of each word)
