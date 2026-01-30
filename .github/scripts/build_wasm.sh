#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.5.1-alpha
# CI: Build WASM with Trunk
cd crates/armas-web && trunk build --release
__status=$?
if [ "${__status}" != 0 ]; then
    echo "WASM build failed"
    exit 1
fi
echo "WASM build succeeded"
