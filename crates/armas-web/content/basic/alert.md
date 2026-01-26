# Alert

Inline alert messages with icons and optional titles.

```demo
Alert::new("This is an informational alert").show(ui, &theme);
```

## Destructive

```demo
Alert::new("Something went wrong").destructive().show(ui, &theme);
```

## With Title

```demo
Alert::new("Your changes have been saved to the server").title("Success").show(ui, &theme);
```

## Dismissible

```demo
Alert::new("Click the X to dismiss this alert").dismissible(true).show(ui, &theme);
```

## Custom Color

```demo
Alert::new("Alert with custom color").color(Color32::from_rgb(100, 200, 150)).show(ui, &theme);
```
