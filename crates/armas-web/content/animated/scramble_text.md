# Scramble Text

Text with scramble/decrypt animation effect.

```demo
ScrambleText::new("Secret Message Decoded!").id("scramble_basic").show(ui, &theme);
```

## Fast Speed

```demo
ScrambleText::new("Quick reveal!").id("scramble_fast").speed(3.0).show(ui, &theme);
```

## Custom Character Set

```demo
ScrambleText::new("Binary Code").id("scramble_binary").charset("01").show(ui, &theme);
```

## Looping

```demo
ScrambleText::new("Loop Forever").id("scramble_loop").loop_mode(true).loop_delay(0.5).show(ui, &theme);
```
