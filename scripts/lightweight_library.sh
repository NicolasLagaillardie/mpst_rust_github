#!/bin/sh

# Change Cargo.toml to include the most lightweight library possible.

# Stop upon any error
set -e

cat scripts/lightweight_cargo.toml > Cargo.toml
./scripts/create_files/create_ping_pong_benches.sh 50