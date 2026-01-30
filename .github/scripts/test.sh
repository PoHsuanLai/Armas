#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.5.1-alpha
# CI: Run workspace tests
cargo test --workspace --all-features
__status=$?
if [ "${__status}" != 0 ]; then
    echo "Tests failed"
    exit 1
fi
echo "All tests passed"
