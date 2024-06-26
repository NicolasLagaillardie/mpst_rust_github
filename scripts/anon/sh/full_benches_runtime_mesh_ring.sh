#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

### Clean compiled files
cargo clean

# Run all mesh benchmarks
echo "Mesh full bench"
date
mkdir -p save/mesh/
cargo bench --bench="mesh_*" --all-features -- --verbose
find . -name "*.svg" -delete
find target/ -name "raw.csv" -delete
find target/ -name "benchmark.json" -delete
find target/ -name "tukey.json" -delete
find target/ -name "index.html" -delete
find target/ -name "sample.json" -delete
mv -f target/criterion/* save/mesh/
echo "Mesh full bench weight"
du -s -m
cargo clean

# Run all ring benchmarks
echo "Ring full bench"
date
mkdir -p save/ring/
cargo bench --bench="ring_*" --all-features -- --verbose
find . -name "*.svg" -delete
find target/ -name "raw.csv" -delete
find target/ -name "benchmark.json" -delete
find target/ -name "tukey.json" -delete
find target/ -name "index.html" -delete
find target/ -name "sample.json" -delete
mv -f target/criterion/* save/ring/
echo "Ring full bench weight"
du -s -m
cargo clean
