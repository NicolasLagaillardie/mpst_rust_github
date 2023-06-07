#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

### Clean compiled files
cargo clean
date

# Create folders if they do not exist
mkdir -p save/

# Run all mesh benchmarks
echo "Mesh full bench"
date
mkdir -p save/mesh/
cargo bench --bench="mesh_*" --all-features -- --verbose
mv -f target/criterion/* save/mesh/
cargo clean

# Run all ring benchmarks
echo "Ring full bench"
date
mkdir -p save/ring/
cargo bench --bench="ring_*" --all-features -- --verbose
mv -f target/criterion/* save/ring/
cargo clean

# Run all ring benchmarks
echo "Ping-pong full bench"
date
mkdir -p save/ping_pong/
cargo bench --bench="ping_pong_*" --all-features -- --verbose
mv -f target/criterion/* save/ping_pong/
cargo clean

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
# python3 scripts/create_graphs/timed_affine_mesh_ring.py