# Alert

Inline alert messages with icons and optional titles.

```demo
Alert::new("This is an informational alert").show(ui);
```

## Destructive

```demo
Alert::new("Something went wrong").destructive().show(ui);
```

## With Title

```demo
Alert::new("Your changes have been saved to the server").title("Success").show(ui);
```

## Dismissible

```demo
Alert::new("Click the X to dismiss this alert").dismissible(true).show(ui);
```

## Custom Color

```demo
Alert::new("Alert with custom color").color(Color32::from_rgb(100, 200, 150)).show(ui);
```
