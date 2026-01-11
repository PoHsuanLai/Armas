# Animations

## Spotlight

```demo
let mut spotlight = Spotlight::new(&theme).radius(150.0).smoothing(0.15);
spotlight.show(ui, &theme, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);
        ui.label("Spotlight Effect");
    });
});
```

## Aurora

```demo
let mut aurora = Aurora::new();
aurora.show(ui, &theme, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);
        ui.label("Aurora Background");
    });
});
```

## Vortex

```demo
let mut vortex = Vortex::new();
vortex.show(ui, &theme, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);
        ui.label("Vortex Background");
    });
});
```

## Wavy Background

```demo
let mut wavy = WavyBackground::new(ui.available_width(), 200.0);
wavy.show(ui, |ui| {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);
        ui.label("Wavy Background");
    });
});
```
