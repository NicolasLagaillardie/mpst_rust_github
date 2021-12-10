#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

# Compile protocols
## Ping-pong
for i in $(eval echo {1..$1})
do
    ./scripts/create_files/compile.sh ping_pong_binary_$i 10 baking
    ./scripts/create_files/compile.sh ping_pong_crossbeam_$i 10 baking
    ./scripts/create_files/compile.sh ping_pong_baking_cancel_$i 10 baking
done

# Run the benchmarks
cargo bench --bench ping_pong --features="baking"  -- --verbose

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
python scripts/ping_pong_mesh_ring.py