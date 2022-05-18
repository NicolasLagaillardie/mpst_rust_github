#!/bin/sh

# Change Cargo.toml to include all examples, benchmarks and tests.

# Stop upon any error
set -e

# Load the new Cargo.toml configuration
cat scripts/full_cargo.toml > Cargo.toml

# Create the new ping_pong benches
./scripts/create_files/create_ping_pong_benches.sh 500

# Replace the number of loops to 10000
sed -ier 's,sample_size([0-9]\+);,sample_size(10000);,g' benches/ping_pong.rs
rm -rf benches/ping_pong.rser
