#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.5.1-alpha
# CI: Check formatting
cargo fmt --all -- --check
__status=$?
if [ "${__status}" != 0 ]; then
    echo "Formatting check failed. Run 'cargo fmt --all' to fix."
    exit 1
fi
echo "Formatting OK"
