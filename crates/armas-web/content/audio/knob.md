# Knob

Polished glazed ceramic knob with sophisticated 3D appearance and rim level indicator. Perfect for audio mixing, synthesizers, and effect controls.

## Basic Usage

```demo
let theme = ui.ctx().armas_theme();
let mut value = 0.5;

Knob::new(value)
    .show(ui, &mut value, &theme);
```

## With Label

```demo
let theme = ui.ctx().armas_theme();
let mut gain = 0.75;

Knob::new(gain)
    .label("Gain")
    .show(ui, &mut gain, &theme);
```

## Different Sizes

```demo
let theme = ui.ctx().armas_theme();
let mut vol = 0.6;

ui.horizontal(|ui| {
    Knob::new(vol)
        .diameter(40.0)
        .label("Small")
        .show(ui, &mut vol, &theme);

    Knob::new(vol)
        .diameter(60.0)
        .label("Medium")
        .show(ui, &mut vol, &theme);

    Knob::new(vol)
        .diameter(80.0)
        .label("Large")
        .show(ui, &mut vol, &theme);
});
```

## Custom Colors

```demo
let theme = ui.ctx().armas_theme();
let mut bass = 0.4;
let mut mid = 0.5;
let mut treble = 0.6;

ui.horizontal(|ui| {
    Knob::new(bass)
        .label("Bass")
        .glow_color(egui::Color32::from_rgb(255, 100, 100))
        .show(ui, &mut bass, &theme);

    Knob::new(mid)
        .label("Mid")
        .glow_color(egui::Color32::from_rgb(100, 255, 100))
        .show(ui, &mut mid, &theme);

    Knob::new(treble)
        .label("Treble")
        .glow_color(egui::Color32::from_rgb(100, 200, 255))
        .show(ui, &mut treble, &theme);
});
```

## Without Value Display

```demo
let theme = ui.ctx().armas_theme();
let mut pan = 0.5;

Knob::new(pan)
    .label("Pan")
    .show_value(false)
    .show(ui, &mut pan, &theme);
```

## Custom Knob Color

```demo
let theme = ui.ctx().armas_theme();
let mut freq = 0.3;

Knob::new(freq)
    .label("Frequency")
    .color(egui::Color32::from_rgb(50, 50, 60))
    .glow_color(egui::Color32::from_rgb(255, 200, 0))
    .show(ui, &mut freq, &theme);
```

## Mixer Strip

```demo
let theme = ui.ctx().armas_theme();
let mut volume = 0.7;
let mut pan = 0.5;
let mut send = 0.3;

ui.vertical(|ui| {
    ui.label("Channel 1");
    ui.add_space(5.0);

    Knob::new(volume)
        .label("Volume")
        .diameter(50.0)
        .show(ui, &mut volume, &theme);

    Knob::new(pan)
        .label("Pan")
        .diameter(50.0)
        .glow_color(egui::Color32::from_rgb(150, 150, 255))
        .show(ui, &mut pan, &theme);

    Knob::new(send)
        .label("Send")
        .diameter(50.0)
        .glow_color(egui::Color32::from_rgb(100, 255, 150))
        .show(ui, &mut send, &theme);
});
```

## EQ Section

```demo
let theme = ui.ctx().armas_theme();
let mut low = 0.5;
let mut low_mid = 0.5;
let mut high_mid = 0.5;
let mut high = 0.5;

ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 20.0;

    Knob::new(low)
        .label("Low")
        .diameter(55.0)
        .glow_color(egui::Color32::from_rgb(255, 80, 80))
        .show(ui, &mut low, &theme);

    Knob::new(low_mid)
        .label("Low Mid")
        .diameter(55.0)
        .glow_color(egui::Color32::from_rgb(255, 180, 80))
        .show(ui, &mut low_mid, &theme);

    Knob::new(high_mid)
        .label("High Mid")
        .diameter(55.0)
        .glow_color(egui::Color32::from_rgb(80, 200, 255))
        .show(ui, &mut high_mid, &theme);

    Knob::new(high)
        .label("High")
        .diameter(55.0)
        .glow_color(egui::Color32::from_rgb(180, 140, 255))
        .show(ui, &mut high, &theme);
});
```

## Detecting Changes

```demo
let theme = ui.ctx().armas_theme();
let mut cutoff = 0.65;

let response = Knob::new(cutoff)
    .label("Cutoff")
    .show(ui, &mut cutoff, &theme);

if response.changed() {
    ui.label(format!("Value: {:.2}", cutoff));
}
```

## API Reference

### Constructor

```rust
Knob::new(value: f32) -> Self
```

Creates a new knob with initial value (0.0-1.0).

### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.diameter()` | `f32` | `60.0` | Knob diameter in pixels |
| `.label()` | `&str` | `None` | Label text above knob |
| `.show_value()` | `bool` | `true` | Show numeric value below knob |
| `.color()` | `Color32` | Silver/white | Knob body color |
| `.glow_color()` | `Color32` | `theme.primary()` | Inner glow indicator color |
| `.angle_range()` | `f32, f32` | `-2.5, 2.5` | Min/max rotation angles in radians |
| `.sensitivity()` | `f32` | `0.005` | Drag sensitivity multiplier |

### Show Method

```rust
pub fn show(self, ui: &mut egui::Ui, value: &mut f32, theme: &Theme) -> Response
```

Returns an `egui::Response` for interaction handling.

## Interactions

- **Drag up/down**: Increase/decrease value
- **Vertical drag**: Natural knob control (up = increase)
- **Smooth response**: Precise value control with adjustable sensitivity
- **Visual feedback**: Glow arc shows current level

## Visual Design

### Glazed Ceramic Appearance
The knob uses 13 sophisticated rendering layers to achieve a realistic glazed ceramic look:

**Depth Layers:**
- **Outer shadow**: 6-layer soft shadow for depth perception
- **Bottom shadow arc**: 6-layer gradient arc for depth
- **Side ambient occlusion**: 4-layer darker edges on the left side

**Ceramic Body:**
- **Base gradient**: Darker bottom (RGB: 185, 190, 200) to lighter top (RGB: 225, 230, 238)
- **Main ceramic color**: Silver-grey (RGB: 210, 215, 222)

**Glass Glaze Coating:**
- **Base glaze layer**: 10-layer large diffuse area where glaze is thickest
- **Mid glaze layer**: 6-layer concentrated bright area
- **Fresnel effect**: 5-layer edge brightness where light catches the curved glass
- **Subsurface scattering**: 4-layer rim glow where light penetrates the glaze

**Highlights:**
- **Primary specular**: 5-layer main light source reflection
- **Secondary specular**: 3-layer additional reflection
- **Sharp highlights**: Intense bright spots for wet glaze look
- **Edge refraction**: 3-layer subtle color shift at edges

### Rim Level Indicator
- **Colored arc**: Shows current level on the outer rim
- **Bright overlay**: White highlight on top for glass effect
- **48-segment rendering**: Smooth arc rendering
- **Custom colors**: Override with `.glow_color()`
- **Very bright rim**: White rim highlight for ceramic glaze appearance

## Use Cases

### Synthesizer Controls

```demo
let theme = ui.ctx().armas_theme();
let mut attack = 0.2;
let mut decay = 0.4;
let mut sustain = 0.7;
let mut release = 0.5;

ui.label("ADSR Envelope");
ui.horizontal(|ui| {
    Knob::new(attack)
        .label("Attack")
        .diameter(50.0)
        .show(ui, &mut attack, &theme);

    Knob::new(decay)
        .label("Decay")
        .diameter(50.0)
        .show(ui, &mut decay, &theme);

    Knob::new(sustain)
        .label("Sustain")
        .diameter(50.0)
        .show(ui, &mut sustain, &theme);

    Knob::new(release)
        .label("Release")
        .diameter(50.0)
        .show(ui, &mut release, &theme);
});
```

### Effect Parameters

```demo
let theme = ui.ctx().armas_theme();
let mut mix = 0.5;
let mut time = 0.6;
let mut feedback = 0.4;

ui.label("Delay");
ui.horizontal(|ui| {
    Knob::new(mix)
        .label("Mix")
        .show(ui, &mut mix, &theme);

    Knob::new(time)
        .label("Time")
        .glow_color(egui::Color32::from_rgb(100, 200, 255))
        .show(ui, &mut time, &theme);

    Knob::new(feedback)
        .label("Feedback")
        .glow_color(egui::Color32::from_rgb(255, 150, 100))
        .show(ui, &mut feedback, &theme);
});
```

### Compact Mixer

```demo
let theme = ui.ctx().armas_theme();
let mut ch1 = 0.7;
let mut ch2 = 0.5;
let mut ch3 = 0.8;

ui.horizontal(|ui| {
    ui.vertical(|ui| {
        ui.label("Ch 1");
        Knob::new(ch1)
            .diameter(45.0)
            .show_value(false)
            .show(ui, &mut ch1, &theme);
    });

    ui.vertical(|ui| {
        ui.label("Ch 2");
        Knob::new(ch2)
            .diameter(45.0)
            .show_value(false)
            .show(ui, &mut ch2, &theme);
    });

    ui.vertical(|ui| {
        ui.label("Ch 3");
        Knob::new(ch3)
            .diameter(45.0)
            .show_value(false)
            .show(ui, &mut ch3, &theme);
    });
});
```

## Performance

- **Efficient rendering**: Minimal overdraw with layered circles
- **Smooth interaction**: Direct drag-to-value mapping
- **No repaints when idle**: Only redraws on interaction
- **Scalable**: Multiple knobs render efficiently

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `on_surface`, `on_surface_variant`
- Minimum version: `armas 0.1.0`
