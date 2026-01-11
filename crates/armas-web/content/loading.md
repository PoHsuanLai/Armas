# Loading

## Spinner

```demo
let mut spinner = Spinner::new().size(40.0);
spinner.show(ui);
```

## Loading Dots

```demo
let mut dots = LoadingDots::new();
dots.show(ui);
```

## Skeleton

```demo
Skeleton::new(300.0, 20.0).show(ui);
ui.add_space(8.0);
Skeleton::new(250.0, 20.0).show(ui);
ui.add_space(8.0);
Skeleton::new(200.0, 20.0).show(ui);
```

## Circular Progress

```demo
let mut progress = CircularProgress::new();
progress.show(ui);
```
