# Stepper

Step-by-step progress indicator for multi-step workflows with horizontal and vertical layouts.

## Basic Usage

```demo
let stepper = Stepper::new()
    .add_step(Step::new("Account"))
    .add_step(Step::new("Profile"))
    .add_step(Step::new("Complete"));

let current_step = 1;
stepper.show(ui, current_step);
```

## With Descriptions

```demo
let stepper = Stepper::new()
    .add_step(Step::new("Account").description("Create your account"))
    .add_step(Step::new("Profile").description("Set up your profile"))
    .add_step(Step::new("Complete").description("Finish setup"));

stepper.show(ui, 0);
```

## With Icons

```demo
let stepper = Stepper::new()
    .add_step(Step::new("User").icon("👤"))
    .add_step(Step::new("Settings").icon("⚙"))
    .add_step(Step::new("Done").icon("✓"));

stepper.show(ui, 1);
```

## Vertical Orientation

```demo
let stepper = Stepper::new()
    .orientation(StepperOrientation::Vertical)
    .add_step(Step::new("Start"))
    .add_step(Step::new("Process"))
    .add_step(Step::new("End"));

stepper.show(ui, 1);
```

## Clickable Steps

```demo
let stepper = Stepper::new()
    .clickable(true)
    .add_step(Step::new("Step 1"))
    .add_step(Step::new("Step 2"))
    .add_step(Step::new("Step 3"));

let response = stepper.show(ui, 1);
if let Some(clicked) = response.clicked_step {
    // Handle step click
}
```

## Without Numbers

```demo
let stepper = Stepper::new()
    .show_numbers(false)
    .add_step(Step::new("Planning"))
    .add_step(Step::new("Development"))
    .add_step(Step::new("Testing"))
    .add_step(Step::new("Deploy"));

stepper.show(ui, 2);
```

## Complete Example

```demo
let stepper = Stepper::new()
    .add_step(
        Step::new("Account")
            .description("Create your account")
            .icon("👤")
    )
    .add_step(
        Step::new("Verification")
            .description("Verify your email")
            .icon("✉")
    )
    .add_step(
        Step::new("Profile")
            .description("Complete your profile")
            .icon("📝")
    )
    .add_step(
        Step::new("Done")
            .description("Start using the app")
            .icon("🎉")
    );

stepper.show(ui, 2);
```

## Different Progress States

### First Step

```demo
let stepper = Stepper::new()
    .add_step(Step::new("Step 1"))
    .add_step(Step::new("Step 2"))
    .add_step(Step::new("Step 3"));

stepper.show(ui, 0);
```

### Middle Step

```demo
let stepper = Stepper::new()
    .add_step(Step::new("Step 1"))
    .add_step(Step::new("Step 2"))
    .add_step(Step::new("Step 3"));

stepper.show(ui, 1);
```

### Last Step

```demo
let stepper = Stepper::new()
    .add_step(Step::new("Step 1"))
    .add_step(Step::new("Step 2"))
    .add_step(Step::new("Step 3"));

stepper.show(ui, 2);
```

## API Reference

### Stepper

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `.add_step()` | `Step` | - | Add a step |
| `.orientation()` | `StepperOrientation` | `Horizontal` | Layout direction |
| `.clickable()` | `bool` | `false` | Make steps clickable |
| `.show_numbers()` | `bool` | `true` | Show step numbers |

### Step

| Method | Type | Description |
|--------|------|-------------|
| `.description()` | `&str` | Add description text |
| `.icon()` | `&str` | Add icon (emoji) |

### StepperResponse

| Field | Type | Description |
|-------|------|-------------|
| `clicked_step` | `Option<usize>` | Index of clicked step |

## Dependencies

- `egui = "0.33"`
- Theme colors: `primary`, `surface`, `on_surface`, `outline`
- Badge component for styling
