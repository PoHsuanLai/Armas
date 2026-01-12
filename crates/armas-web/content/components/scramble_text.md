# Scramble Text

Text with scramble/decrypt animation effect. Characters randomly scramble and gradually reveal the target text.

## Basic Usage

```demo
ScrambleText::new("Secret Message Decoded!")
    .with_id("scramble_basic")
    .show(ui);
```

## Fast Speed

```demo
ScrambleText::new("Quick reveal!")
    .with_id("scramble_fast")
    .with_speed(3.0)
    .show(ui);
```

## Custom Character Set

```demo
ScrambleText::new("Binary Code")
    .with_id("scramble_binary")
    .with_charset("01")
    .show(ui);
```

## Looping Animation

```demo
ScrambleText::new("Loop Forever")
    .with_id("scramble_loop")
    .with_loop(true)
    .with_loop_delay(0.5)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `&str` | - | Create scramble text |
| `.with_id()` | `impl Hash` | - | Set unique ID (required for multiple instances) |
| `.with_speed()` | `f32` | `2.0` | Animation speed (progress per second) |
| `.with_charset()` | `String` | symbols+letters+numbers | Custom character set for scrambling |
| `.with_frame_interval()` | `f32` | `0.05` | Seconds between character changes |
| `.with_loop()` | `bool` | `false` | Enable looping animation |
| `.with_loop_delay()` | `f32` | `1.0` | Delay before restart (seconds) |

## Dependencies

- `egui = "0.33"`
- Theme colors: `on_surface`
