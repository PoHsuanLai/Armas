# Scramble Text

Text with scramble/decrypt animation effect. Characters randomly scramble and gradually reveal the target text.

## Basic Usage

```demo
ScrambleText::new("Secret Message Decoded!")
    .id("scramble_basic")
    .show(ui);
```

## Fast Speed

```demo
ScrambleText::new("Quick reveal!")
    .id("scramble_fast")
    .speed(3.0)
    .show(ui);
```

## Custom Character Set

```demo
ScrambleText::new("Binary Code")
    .id("scramble_binary")
    .charset("01")
    .show(ui);
```

## Looping Animation

```demo
ScrambleText::new("Loop Forever")
    .id("scramble_loop")
    .loop_mode(true)
    .loop_delay(0.5)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `&str` | - | Create scramble text |
| `.id()` | `impl Hash` | - | Set unique ID (required for multiple instances) |
| `.speed()` | `f32` | `2.0` | Animation speed (progress per second) |
| `.charset()` | `String` | symbols+letters+numbers | Custom character set for scrambling |
| `.frame_interval()` | `f32` | `0.05` | Seconds between character changes |
| `.loop_mode()` | `bool` | `false` | Enable looping animation |
| `.loop_delay()` | `f32` | `1.0` | Delay before restart (seconds) |

## Dependencies

- `egui = "0.33"`
- Theme colors: `on_surface`
