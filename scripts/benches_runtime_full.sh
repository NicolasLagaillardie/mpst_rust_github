#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

### Clean compiled files
cargo clean
date

# Create folders if they do not exist
mkdir -p save/
mkdir -p save/

# Run all mesh benchmarks
echo "Mesh full bench"
date
cargo bench --bench=benches --all-features -- --verbose
mkdir -p save/criterion/
mv -f target/criterion/ save/
cargo clean

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
# python3 scripts/create_graphs/timed_affine_mesh_ring.py