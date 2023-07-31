#!/bin/sh

# set -e

# Replace toml
cat Cargo.toml > scripts/toml/save_cargo.toml
cat scripts/toml/full_cargo.toml > Cargo.toml

# Clean everything
bash ./scripts/clean_all.sh

# Check all combinations of features
cargo hack --feature-powerset --exclude-no-default-features clippy --all-targets --skip kmc,transport_udp --workspace # -- -D warnings 
cargo hack --feature-powerset --exclude-no-default-features test --all-targets --skip kmc,transport_udp --workspace # -- -D warnings

# Clean everything
bash ./scripts/clean_all.sh

# Reverse toml
cat scripts/toml/save_cargo.toml > Cargo.toml

echo "done"
