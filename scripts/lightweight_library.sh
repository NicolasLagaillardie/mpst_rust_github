#!/bin/sh

# Change Cargo.toml to include the most lightweight library possible.

# Stop upon any error
set -e

# Load the new Cargo.toml configuration
cat scripts/lightweight_cargo.toml > Cargo.toml

# Create the new ping_pong benches
./scripts/create_files/create_ping_pong_benches.sh 50

# Replace the number of loops to 100
sed -ier 's,sample_size([0-9]\+);,sample_size(100);,g' benches/ping_pong.rs
rm -rf benches/ping_pong.rser