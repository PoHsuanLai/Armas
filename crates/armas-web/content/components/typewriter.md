# Typewriter

Text with typewriter animation effect.

## Basic Usage

```demo
Typewriter::new("Hello World from Armas! This is a character-by-character typewriter effect.")
    .with_id("typewriter_basic")
    .show(ui);
```

## Custom Speed

```demo
Typewriter::new("Fast typing animation!")
    .with_id("typewriter_fast")
    .with_speed(40.0)
    .show(ui);
```

## Without Cursor

```demo
Typewriter::new("This one has no cursor.")
    .with_id("typewriter_no_cursor")
    .with_cursor(false)
    .show(ui);
```

## Looping Animation

```demo
Typewriter::new("This animation loops!")
    .with_id("typewriter_loop")
    .with_loop(true)
    .with_loop_delay(1.0)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `&str` | - | Create typewriter text |
| `.with_id()` | `impl Hash` | - | Set unique ID (required for multiple instances) |
| `.with_speed()` | `f32` | `20.0` | Typing speed (characters per second) |
| `.with_cursor()` | `bool` | `true` | Show blinking cursor |
| `.with_cursor_blink_speed()` | `f32` | `2.0` | Cursor blink speed |
| `.with_loop()` | `bool` | `false` | Enable looping animation |
| `.with_loop_delay()` | `f32` | `2.0` | Delay before restart (seconds) |

## Dependencies

- `egui = "0.33"`
- Theme colors: `on_surface`
