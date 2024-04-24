#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

# Get date
date

## Compile ampst examples
bash ./scripts/create_files/compile_normal.sh remote_data_ampst 5 baking
bash ./scripts/create_files/compile_normal.sh http_ampst 5 baking
bash ./scripts/create_files/compile_normal.sh pinetime_heart_rate_ampst 5 baking

## Compile timed examples
bash ./scripts/create_files/compile_normal.sh remote_data_atmp 5 baking_atmp
bash ./scripts/create_files/compile_normal.sh http_atmp 5 baking_atmp
bash ./scripts/create_files/compile_normal.sh pinetime_heart_rate_atmp 5 baking_atmp

## Run and save benchmarks
cargo clean
mkdir -p save/examples/
rm -rf save/examples/*
echo "Examples full bench"
cargo bench --bench="example_*" --all-features
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
