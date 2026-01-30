#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.5.1-alpha
# Helper: Bump workspace version and update inter-crate dependency pins
declare -r args_0=("$0" "$@")
ver_1="${args_0[1]}"
if [ "$([ "_${ver_1}" != "_" ]; echo $?)" != 0 ]; then
    echo "Usage: bump_version.sh <new-version>"
    echo "Example: bump_version.sh 0.2.0"
    exit 1
fi
echo "Bumping all versions to ${ver_1}..."
echo ""
# Update workspace version in root Cargo.toml
sed -i '' 's/^version = "[0-9.]*"/version = "'${ver_1}'"/' Cargo.toml
echo "Updated Cargo.toml (workspace)"
# Update armas/Cargo.toml: armas-icon version
sed -i '' 's/armas-icon = { version = "[0-9.]*"/armas-icon = { version = "'${ver_1}'"/' crates/armas/Cargo.toml
echo "Updated crates/armas/Cargo.toml"
# Update armas-audio/Cargo.toml: armas and armas-icon versions
sed -i '' 's/armas = { version = "[0-9.]*"/armas = { version = "'${ver_1}'"/' crates/armas-audio/Cargo.toml
sed -i '' 's/armas-icon = { version = "[0-9.]*"/armas-icon = { version = "'${ver_1}'"/' crates/armas-audio/Cargo.toml
echo "Updated crates/armas-audio/Cargo.toml"
# Update armas-animated/Cargo.toml: armas version
sed -i '' 's/armas = { version = "[0-9.]*"/armas = { version = "'${ver_1}'"/' crates/armas-animated/Cargo.toml
echo "Updated crates/armas-animated/Cargo.toml"
echo ""
echo "All versions bumped to ${ver_1}"
echo ""
echo "Next steps:"
echo "  1. Update CHANGELOGs in each crate"
echo "  2. git add -A && git commit -m 'Release v${ver_1}'"
echo "  3. git tag v${ver_1}"
echo "  4. git push origin master --tags"
