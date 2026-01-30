#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.5.1-alpha
# CI: Run clippy lints
cargo clippy --workspace --all-features --all-targets -- -D warnings
__status=$?
if [ "${__status}" != 0 ]; then
    echo "Clippy found warnings"
    exit 1
fi
echo "Clippy passed"
