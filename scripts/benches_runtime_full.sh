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

# Run the full mesh benchmarks
echo "Mesh full bench"
date
cargo bench --bench="mesh_full" --features="full" -- --verbose
mkdir -p save/criterion/mesh/
mv -f target/criterion/ save/criterion/mesh/
cargo clean

# Run the full ring benchmarks
echo "Ring full bench"
date
cargo bench --bench="ring_full" --features="full" -- --verbose
mkdir -p save/criterion/ring/
mv -f target/criterion/ save/criterion/ring/
cargo clean

# Run the baking benchmarks
echo "Examples full bench"
date
cargo bench --bench="examples" --features="full" -- --verbose
mkdir -p save/criterion/examples/
mv -f target/criterion/ save/criterion/examples/
cargo clean

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
# python3 scripts/create_graphs/timed_affine_mesh_ring.py