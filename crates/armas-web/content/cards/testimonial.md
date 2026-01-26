# Testimonial

Display customer testimonials and reviews with avatars and ratings.

```demo
TestimonialGrid::new().columns(1).show(ui, |grid| {
    grid.testimonial("Amazing product! Changed my workflow completely.", "John Doe", "CEO, Tech Corp").rating(5);
});
```

## With Avatar

```demo
use egui::Color32;
TestimonialGrid::new().columns(1).show(ui, |grid| {
    grid.testimonial("Great experience working with this tool.", "Jane Smith", "Designer").avatar("JS").avatar_color(Color32::from_rgb(100, 150, 255)).rating(4);
});
```

## Testimonial Grid

```demo
use armas::components::TestimonialGrid;
TestimonialGrid::new().columns(3).gap(20.0).show(ui, |grid| {
    grid.testimonial("Great product!", "Alice", "Engineer").rating(5);
    grid.testimonial("Love it!", "Bob", "Designer").rating(5);
    grid.testimonial("Excellent!", "Carol", "Manager").rating(4);
});
```
