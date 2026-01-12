# Why Egui?

Egui's core strengths lie in its immediate mode architecture, lightweight footprint, and deep integration capabilities.

It's particularly vital for:
- **Game development** (like with Bevy)
- **Performance-critical native applications**
- **Direct GPU access** without DOM overhead

It offers unparalleled control over rendering and input, making it the ideal choice when low latency and high refresh rates are crucial.

Armas ensures you don't have to compromise on aesthetics to benefit from egui's technical advantages.

## Comparison with Web Frameworks

Frameworks like Dioxus and Leptos are powerful and have their place, often excelling in:
- Web-centric applications
- Cross-platform desktop apps that leverage web views
- Leveraging the extensive web ecosystem

They benefit from mature CSS layout systems (Flexbox, Grid) and robust accessibility features.

**However**, for use cases demanding:
- Deep integration with game loops
- Very high-frequency updates
- Direct GPU access without the overhead of a DOM

...egui's immediate mode paradigm is often a superior fit.

In web-based frameworks, developers can readily utilize existing CSS component libraries. **Armas fills this gap for egui**, offering a similar level of modern component design without straying from egui's performant, native rendering approach.
