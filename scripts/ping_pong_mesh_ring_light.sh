#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

# Compile protocols

## Mesh
### Two
./scripts/create_files/compile.sh mesh_two_binary 10 baking
./scripts/create_files/compile.sh mesh_two_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_two_baking_cancel_inline 10 baking

### Three
./scripts/create_files/compile.sh mesh_three_binary 10 baking
./scripts/create_files/compile.sh mesh_three_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_three_baking_cancel_inline 10 baking

### Four
./scripts/create_files/compile.sh mesh_four_binary 10 baking
./scripts/create_files/compile.sh mesh_four_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_four_baking_cancel_inline 10 baking

### Five
./scripts/create_files/compile.sh mesh_five_binary 10 baking
./scripts/create_files/compile.sh mesh_five_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_five_baking_cancel_inline 10 baking

## Ring
### Two
./scripts/create_files/compile.sh ring_two_binary 10 baking
./scripts/create_files/compile.sh ring_two_crossbeam 10 baking
./scripts/create_files/compile.sh ring_two_baking_cancel_inline 10 baking

### Three
./scripts/create_files/compile.sh ring_three_binary 10 baking
./scripts/create_files/compile.sh ring_three_crossbeam 10 baking
./scripts/create_files/compile.sh ring_three_baking_cancel_inline 10 baking

### Four
./scripts/create_files/compile.sh ring_four_binary 10 baking
./scripts/create_files/compile.sh ring_four_crossbeam 10 baking
./scripts/create_files/compile.sh ring_four_baking_cancel_inline 10 baking

### Five
./scripts/create_files/compile.sh ring_five_binary 10 baking
./scripts/create_files/compile.sh ring_five_crossbeam 10 baking
./scripts/create_files/compile.sh ring_five_baking_cancel_inline 10 baking

# Run the benchmarks
cargo bench --bench ping_pong --features="baking"  -- --verbose
cargo bench --bench mesh --features="baking"  -- --verbose
cargo bench --bench ring --features="baking"  -- --verbose

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
python3 scripts/create_graphs/ping_pong_mesh_ring.py