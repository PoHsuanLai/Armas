# Armas Web Showcase

Interactive showcase website for the Armas component library, built with egui and compiled to WASM.

## Quick Start

### Run Natively (Desktop)

```bash
cargo run
```

### Build for Web (WASM)

```bash
# Install WASM target (first time only)
rustup target add wasm32-unknown-unknown

# Build and serve
./serve.sh
```

This will:
1. Build the project to WASM
2. Generate JS bindings with wasm-bindgen
3. Start a local server at http://localhost:8080

### Manual Build

```bash
# Just build (output to dist/)
./build_web.sh

# Serve with your preferred server
cd dist && python3 -m http.server 8080
```

## Features

The showcase demonstrates all Armas components:

- ✨ **Animations**: Scrolling banner, spotlight effects
- 🎴 **Cards**: Basic cards, glass panels, gradient cards
- 🔘 **Buttons**: Multiple variants and badges
- ⏳ **Loading**: Spinners, dots, skeletons, progress bars
- 🧭 **Navigation**: Tabs, accordions
- 📊 **Data Display**: Timeline, faders
- 📐 **Layout**: Feature grids, testimonials

## Deployment

To deploy to GitHub Pages or other static hosting:

1. Build for web: `./build_web.sh`
2. Upload the `dist/` folder contents

### GitHub Pages

```bash
# Build
./build_web.sh

# Create gh-pages branch and push dist/ contents
git checkout --orphan gh-pages
git rm -rf .
cp -r crates/armas-web/dist/* .
git add .
git commit -m "Deploy showcase"
git push origin gh-pages --force
```

## Development

The showcase is a single-page application that demonstrates all components interactively. Users can:

- Browse components by category
- See live examples with code snippets
- Switch themes (Ocean, Nord)
- Interact with animated components

## Requirements

- Rust 1.70+
- wasm-bindgen-cli: `cargo install wasm-bindgen-cli`
- (Optional) wasm-opt from binaryen for optimization

## Project Structure

```
armas-web/
├── src/
│   └── main.rs           # Showcase application
├── index.html            # HTML template
├── build_web.sh          # WASM build script
├── serve.sh              # Build and serve locally
└── dist/                 # Build output (gitignored)
```
