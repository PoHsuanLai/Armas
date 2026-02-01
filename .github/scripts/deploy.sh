#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.5.1-alpha
# Deploy: Build WASM with Trunk and prepare for GitHub Pages
cd crates/armas-web && trunk build --release
__status=$?
if [ "${__status}" != 0 ]; then
    echo "WASM build failed"
    exit 1
fi
echo "WASM build succeeded"
cd ../..
touch dist/.nojekyll
__status=$?
if [ "${__status}" != 0 ]; then
    echo "Failed to create .nojekyll"
    exit 1
fi
echo "Deploy artifacts ready"
