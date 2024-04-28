#!/bin/sh

# set -eou pipefail

# Replace toml
cat Cargo.toml > scripts/toml/save_cargo.toml
cat scripts/toml/full_cargo.toml > Cargo.toml

# Clean everything
bash ./scripts/clean_all.sh

# Check all combinations of features
cargo hack --feature-powerset --clean-per-run clippy clippy --all-targets --workspace # -- -D warning
cargo hack --feature-powerset --clean-per-run clippy test --all-targets --workspace

# Clean everything
./scripts/clean_all.sh

# Reverse toml
cat scripts/toml/save_cargo.toml > Cargo.toml
