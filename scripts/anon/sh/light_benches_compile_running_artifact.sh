#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

# Get date
date

# Saving Cargo.toml
cat Cargo.toml > scripts/anon/toml/save_cargo.toml

# Increase the sample size to 100,000
find ./ -type f | xargs sed -i 's/100000/10000/g'

# Updating Cargo.toml
cat scripts/anon/toml/light_cargo.toml > Cargo.toml

# Create folders if they do not exist
mkdir -p save/
rm -rf save/*

cargo clean

## Compile and run examples
bash ./scripts/anon/sh/light_benches_examples.sh

cargo clean

## Compile mesh and ring
bash ./scripts/anon/sh/light_benches_compile_mesh_ring.sh

cargo clean

## Run mesh and ring
bash ./scripts/anon/sh/light_benches_runtime_mesh_ring.sh

# Run the Python scripts
python3 scripts/create_graphs/mesh_bench.py # Create graph for the runtime benchmarks for the mesh protocols
python3 scripts/create_graphs/mesh_compile.py # Create graph for the compile benchmarks for the mesh protocols
python3 scripts/create_graphs/ring_bench.py # Create graph for the runtime benchmarks for the ring protocols
python3 scripts/create_graphs/ring_compile.py # Create graph for the compile benchmarks for the ring protocols
python3 scripts/create_graphs/examples_extra_literature_affine_timed_check_build_release_timed_ampst.py # Create graph for the compile benchmarks for the example protocols

cargo clean

# Resetting Cargo.toml
cat scripts/anon/toml/save_cargo.toml > Cargo.toml
