#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

### Clean compiled files
cargo clean

# Run all mesh benchmarks
echo "Mesh full bench"
date
mkdir -p save/examples/
cargo bench --bench="example_*" --all-features -- --verbose
find . -name "*.svg" -delete
find target/ -name "raw.csv" -delete
find target/ -name "benchmark.json" -delete
find target/ -name "tukey.json" -delete
find target/ -name "index.html" -delete
find target/ -name "sample.json" -delete
mv -f target/criterion/* save/examples/
echo "Examples full bench weight"
du -s -m
cargo clean
