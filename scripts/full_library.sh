#!/bin/sh

# Change Cargo.toml to include all examples, benchmarks and tests.

# Stop upon any error
set -e

cat scripts/full_cargo.toml > Cargo.toml
./scripts/create_files/create_ping_pong_benches 500
