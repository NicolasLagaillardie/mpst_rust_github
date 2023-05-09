#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

### Clean compiled files
cargo clean
date

# Create folders if they do not exist
mkdir -p save/
mkdir -p save/criterion/

# Create the new ping_pong benches
./scripts/create_files/create_ping_pong_benches.sh 500

# Replace the number of loops to 500
sed -ier 's,sample_size([0-9]\+);,sample_size(10000);,g' benches/ping_pong.rs
rm -rf benches/ping_pong.rser

# Run the affine ping pong benchmarks
echo "Ping-pong affine bench"
date
cargo bench --bench ping_pong --features="baking" -- --verbose
mkdir -p save/criterion/affine_ping_pong/
mv -f target/criterion/ save/criterion/affine_ping_pong/
cargo clean

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
# python3 scripts/create_graphs/timed_affine_mesh_ring.py