# Philosophy

## The Problem: Great Tech, Dated Look

Egui is a fantastic immediate mode GUI library for Rust. It is lightweight, fast, and integrates seamlessly with game engines like Bevy, making it excellent for native applications and tools where performance and direct integration are paramount. However, its default styling is strictly utilitarian. Achieving a modern, polished aesthetic using egui’s raw styling API often requires significant effort and custom drawing, feeling like fighting the tool rather than leveraging its strengths.

## The Armas Solution: A Modern Component Library for Egui

Instead of every developer having to reinvent the wheel, **Armas** provides a comprehensive suite of modern, pre-styled components. We handle the complex drawing commands and style configurations behind the scenes, allowing you to just drop in components that look professional and contemporary right out of the box. Armas aims to elevate the visual experience of egui applications, enabling developers to focus on functionality while delivering a polished user interface.

## Q&A

**Why Egui?**
Egui's core strengths lie in its immediate mode architecture, lightweight footprint, and deep integration capabilities, particularly vital for game development (like with Bevy) and performance-critical native applications. It offers unparalleled control over rendering and input, making it the ideal choice when low latency and high refresh rates are crucial. Armas ensures you don't have to compromise on aesthetics to benefit from egui's technical advantages.

**Why not Dioxus or Leptos?**
Frameworks like Dioxus and Leptos are powerful and have their place, often excelling in web-centric applications or cross-platform desktop apps that leverage web views. They benefit from the extensive web ecosystem, including mature CSS layout systems (Flexbox, Grid) and robust accessibility features. However, for use cases demanding deep integration with game loops, very high-frequency updates, or direct GPU access without the overhead of a DOM, egui's immediate mode paradigm is often a superior fit. Moreover, in these web-based frameworks, developers can readily utilize existing CSS component libraries. Armas fills this gap for egui, offering a similar level of modern component design without straying from egui's performant, native rendering approach.

## Attributions

Many of the design principles and component inspirations in Armas are drawn from the excellent work of the following projects:

*   **Shadcn/ui**: A collection of re-usable components for React, built using Radix UI and Tailwind CSS. Its modularity and focus on modern design have been a significant influence.
*   **HeroUI**: An open-source collection of beautiful Tailwind CSS components.
*   **Aceternity UI**: Modern UI components for React, often featuring stunning animations and intricate designs.

We extend our gratitude to these projects for their contribution to the modern web and UI design landscape, which has directly inspired the development of Armas components.
