# Typewriter

Text with typewriter animation effect.

## Basic Usage

```demo
Typewriter::new("Hello World from Armas! This is a character-by-character typewriter effect.")
    .id("typewriter_basic")
    .show(ui);
```

## Custom Speed

```demo
Typewriter::new("Fast typing animation!")
    .id("typewriter_fast")
    .speed(40.0)
    .show(ui);
```

## Without Cursor

```demo
Typewriter::new("This one has no cursor.")
    .id("typewriter_no_cursor")
    .cursor(false)
    .show(ui);
```

## Looping Animation

```demo
Typewriter::new("This animation loops!")
    .id("typewriter_loop")
    .loop_mode(true)
    .loop_delay(1.0)
    .show(ui);
```

## API Reference

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `::new()` | `&str` | - | Create typewriter text |
| `.id()` | `impl Hash` | - | Set unique ID (required for multiple instances) |
| `.speed()` | `f32` | `20.0` | Typing speed (characters per second) |
| `.cursor()` | `bool` | `true` | Show blinking cursor |
| `.cursor_blink_speed()` | `f32` | `2.0` | Cursor blink speed |
| `.loop_mode()` | `bool` | `false` | Enable looping animation |
| `.loop_delay()` | `f32` | `2.0` | Delay before restart (seconds) |

## Dependencies

- `egui = "0.33"`
- Theme colors: `on_surface`
