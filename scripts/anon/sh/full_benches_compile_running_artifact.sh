#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

# Get date
date

# Saving Cargo.toml
cat Cargo.toml > scripts/anon/toml/save_cargo.toml

# Increase the sample size to 100,000
# find ./benches/ -type f | xargs sed -i 's/10000)/100000)/g'
find ./benches/ -type f | xargs sed -i 's/100000)/100)/g'

# Updating Cargo.toml
cat scripts/anon/toml/full_cargo.toml > Cargo.toml

# Create folders if they do not exist
mkdir -p save/
rm -rf save/*

cargo clean

## Compile and run examples
bash ./scripts/anon/sh/full_benches_compile_examples.sh

# Create graph for the compile and run benchmarks for the example protocols
python3 scripts/anon/python/full/examples_extra_literature_affine_atmp_check_build_release_atmp_ampst.py

cargo clean

## Compile mesh and ring
bash ./scripts/anon/sh/full_benches_compile_mesh_ring.sh

# Create graph for the compile benchmarks for the mesh protocols
python3 scripts/anon/python/full/mesh_compile.py

# Create graph for the compile benchmarks for the ring protocols
python3 scripts/anon/python/full/ring_compile.py

cargo clean

## Run mesh and ring
bash ./scripts/anon/sh/full_benches_runtime_mesh_ring.sh

# Create graph for the runtime benchmarks for the mesh protocols
python3 scripts/anon/python/full/mesh_bench.py

# Create graph for the runtime benchmarks for the ring protocols
python3 scripts/anon/python/full/ring_bench.py

cargo clean

# Resetting Cargo.toml
cat scripts/anon/toml/save_cargo.toml > Cargo.toml
