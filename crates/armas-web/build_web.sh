#!/bin/bash
set -e

echo "🔨 Building Armas Web Showcase for WASM..."

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "❌ wasm-bindgen-cli not found. Installing..."
    cargo install wasm-bindgen-cli
fi

# Build for WASM
echo "📦 Compiling to WASM..."
cargo build --target wasm32-unknown-unknown --release

# Generate bindings
echo "🔗 Generating JS bindings..."
wasm-bindgen \
    --target web \
    --out-dir dist \
    --out-name armas_web \
    ../../target/wasm32-unknown-unknown/release/armas_web.wasm

# Copy HTML file
echo "📄 Copying HTML..."
cp index.html dist/

# Optimize WASM (optional, requires wasm-opt from binaryen)
if command -v wasm-opt &> /dev/null; then
    echo "⚡ Optimizing WASM..."
    wasm-opt -Oz -o dist/armas_web_bg.wasm.opt dist/armas_web_bg.wasm
    mv dist/armas_web_bg.wasm.opt dist/armas_web_bg.wasm
else
    echo "ℹ️  wasm-opt not found, skipping optimization (install binaryen for smaller builds)"
fi

echo "✅ Build complete! Output in dist/"
echo "🚀 To serve locally, run: python3 -m http.server -d dist 8080"
