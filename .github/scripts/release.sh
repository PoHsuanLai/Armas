#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.5.1-alpha
# Release: Validate version and publish crates to crates.io
declare -r args_0=("$0" "$@")
version_1="${args_0[1]}"
if [ "$([ "_${version_1}" != "_" ]; echo $?)" != 0 ]; then
    echo "Usage: release.sh <version> [dry-run]"
    exit 1
fi
dry_run_2="${args_0[2]}"
command_1="$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')"
__status=$?
cargo_version_3="${command_1}"
echo "Tag version: ${version_1}"
echo "Cargo.toml version: ${cargo_version_3}"
if [ "$([ "_${version_1}" == "_${cargo_version_3}" ]; echo $?)" != 0 ]; then
    echo "ERROR: Tag version does not match Cargo.toml version"
    echo "Update workspace version in Cargo.toml first."
    exit 1
fi
echo "Version validated: ${cargo_version_3}"
echo ""
# 1. armas-icon (no internal deps)
echo "=== Publishing armas-icon ==="
if [ "$([ "_${dry_run_2}" != "_dry-run" ]; echo $?)" != 0 ]; then
    cargo publish --package armas-icon --dry-run
    __status=$?
    if [ "${__status}" != 0 ]; then
        echo "Dry run failed for armas-icon"
        exit 1
    fi
else
    cargo publish --package armas-icon
    __status=$?
    if [ "${__status}" != 0 ]; then
        echo "Publish failed for armas-icon"
        exit 1
    fi
    echo "Waiting 30s for crates.io indexing..."
    sleep 30
    __status=$?
fi
echo "armas-icon done"
echo ""
# 2. armas-basic (depends on armas-icon)
echo "=== Publishing armas-basic ==="
if [ "$([ "_${dry_run_2}" != "_dry-run" ]; echo $?)" != 0 ]; then
    cargo publish --package armas-basic --dry-run
    __status=$?
    if [ "${__status}" != 0 ]; then
        echo "Dry run failed for armas-basic"
        exit 1
    fi
else
    cargo publish --package armas-basic
    __status=$?
    if [ "${__status}" != 0 ]; then
        echo "Publish failed for armas-basic"
        exit 1
    fi
    echo "Waiting 30s for crates.io indexing..."
    sleep 30
    __status=$?
fi
echo "armas-basic done"
echo ""
# 3. armas-audio (depends on armas-basic, armas-icon)
echo "=== Publishing armas-audio ==="
if [ "$([ "_${dry_run_2}" != "_dry-run" ]; echo $?)" != 0 ]; then
    cargo publish --package armas-audio --dry-run
    __status=$?
    if [ "${__status}" != 0 ]; then
        echo "Dry run failed for armas-audio"
        exit 1
    fi
else
    cargo publish --package armas-audio
    __status=$?
    if [ "${__status}" != 0 ]; then
        echo "Publish failed for armas-audio"
        exit 1
    fi
    echo "Waiting 30s for crates.io indexing..."
    sleep 30
    __status=$?
fi
echo "armas-audio done"
echo ""
# 4. armas (umbrella, depends on armas-basic, armas-audio, armas-icon)
echo "=== Publishing armas ==="
if [ "$([ "_${dry_run_2}" != "_dry-run" ]; echo $?)" != 0 ]; then
    cargo publish --package armas --dry-run
    __status=$?
    if [ "${__status}" != 0 ]; then
        echo "Dry run failed for armas"
        exit 1
    fi
else
    cargo publish --package armas
    __status=$?
    if [ "${__status}" != 0 ]; then
        echo "Publish failed for armas"
        exit 1
    fi
fi
echo "armas done"
echo ""
echo "All crates published for version ${cargo_version_3}"
