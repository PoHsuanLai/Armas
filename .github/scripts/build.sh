#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.5.1-alpha
# CI: Build workspace
cargo build --workspace --all-features
__status=$?
if [ "${__status}" != 0 ]; then
    echo "Build failed"
    exit 1
fi
echo "Build succeeded"
