#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

### Clean compiled files
cargo clean
date

# Create folders if they do not exist
mkdir -p save/
rm -rf save/*

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

# Run all ping-pong benchmarks
echo "Ping-pong full bench"
date
mkdir -p save/ping_pong/
# cargo bench --bench="ping_pong_*" --all-features -- --verbose
bash ./scripts/benches_ping_pong_one_by_one.sh 250
find . -name "*.svg" -delete
find target/ -name "raw.csv" -delete
find target/ -name "benchmark.json" -delete
find target/ -name "tukey.json" -delete
find target/ -name "index.html" -delete
find target/ -name "sample.json" -delete
mv -f target/criterion/* save/ping_pong/
echo "Ping-pong full bench weight"
du -s -m
cargo clean
