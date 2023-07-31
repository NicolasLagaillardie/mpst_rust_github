#!/bin/sh

# set -e

# Replace toml
cat Cargo.toml > scripts/toml/save_cargo.toml
cat scripts/toml/full_cargo.toml > Cargo.toml

# Clean everything
bash ./scripts/clean_all.sh

# Check all combinations of features
cargo hack --feature-powerset --exclude-no-default-features clippy --locked --all-targets --workspace # -- -D warning
cargo hack --feature-powerset --exclude-no-default-features test --locked --all-targets --workspace # -- -D warning

# Clean everything
./scripts/clean_all.sh

# Reverse toml
cat scripts/toml/save_cargo.toml > Cargo.toml
