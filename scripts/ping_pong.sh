#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

# Compile protocols

# Run the benchmarks
cargo bench --bench ping_pong --features="baking"  -- --verbose

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
python3 scripts/ping_pong_mesh_ring.py