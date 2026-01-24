# Stepper

Step-by-step progress indicator for multi-step workflows.

## Basic Usage

```demo
let current_step = 1;
Stepper::new()
    .show(ui, current_step, |stepper| {
        stepper.step("Account");
        stepper.step("Profile");
        stepper.step("Complete");
    });
```

## With Descriptions

```demo
Stepper::new()
    .show(ui, 0, |stepper| {
        stepper.step("Account").description("Create your account");
        stepper.step("Profile").description("Set up your profile");
        stepper.step("Complete").description("Finish setup");
    });
```

## Vertical Orientation

```demo
Stepper::new()
    .orientation(StepperOrientation::Vertical)
    .show(ui, 1, |stepper| {
        stepper.step("Start");
        stepper.step("Process");
        stepper.step("End");
    });
```

## Clickable Steps

```demo
let response = Stepper::new()
    .clickable(true)
    .show(ui, 1, |stepper| {
        stepper.step("Step 1");
        stepper.step("Step 2");
        stepper.step("Step 3");
    });
if let Some(clicked) = response.clicked_step {
    // Handle step click
}
```

## Without Numbers

```demo
Stepper::new()
    .show_numbers(false)
    .show(ui, 2, |stepper| {
        stepper.step("Planning");
        stepper.step("Development");
        stepper.step("Testing");
        stepper.step("Deploy");
    });
```

## API Reference

### Stepper

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.orientation()` | `StepperOrientation` | `Horizontal` | Layout direction |
| `.clickable()` | `bool` | `false` | Make steps clickable |
| `.show_numbers()` | `bool` | `true` | Show step numbers |
| `.show()` | `(usize, closure)` | - | Render with current step |

### StepBuilder (chainable from .step())

| Method | Type | Description |
|--------|------|-------------|
| `.description()` | `&str` | Add description text |
| `.icon()` | `&str` | Add icon |

### StepperResponse

| Field | Type | Description |
|-------|------|-------------|
| `clicked_step` | `Option<usize>` | Index of clicked step |
