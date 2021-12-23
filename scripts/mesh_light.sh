#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

# Compile protocols

## Mesh
### Two
./scripts/create_files/compile.sh mesh_two_binary 10 baking
./scripts/create_files/compile.sh mesh_two_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_two_baking 10 baking

### Three
./scripts/create_files/compile.sh mesh_three_binary 10 baking
./scripts/create_files/compile.sh mesh_three_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_three_baking 10 baking

### Four
./scripts/create_files/compile.sh mesh_four_binary 10 baking
./scripts/create_files/compile.sh mesh_four_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_four_baking 10 baking

### Five
./scripts/create_files/compile.sh mesh_five_binary 10 baking
./scripts/create_files/compile.sh mesh_five_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_five_baking 10 baking

# Run the benchmarks
cargo bench --bench mesh --features="baking" -- --verbose

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
python3 scripts/create_graphs/ping_pong_mesh_ring.py