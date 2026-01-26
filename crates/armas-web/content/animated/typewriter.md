# Typewriter

Text with typewriter animation effect.

```demo
Typewriter::new("Hello World from Armas! This is a character-by-character typewriter effect.").id("typewriter_basic").show(ui, &theme);
```

## Custom Speed

```demo
Typewriter::new("Fast typing animation!").id("typewriter_fast").speed(40.0).show(ui, &theme);
```

## Without Cursor

```demo
Typewriter::new("This one has no cursor.").id("typewriter_no_cursor").cursor(false).show(ui, &theme);
```

## Looping

```demo
Typewriter::new("This animation loops!").id("typewriter_loop").loop_mode(true).loop_delay(1.0).show(ui, &theme);
```
