#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

# Get date
date

cargo clean

## Compile mpst examples
bash ./scripts/create_files/compile_full.sh fib_mpst 10 baking

## Compile ampst examples
bash ./scripts/create_files/compile_full.sh fib_ampst 10 baking

## Compile timed examples
bash ./scripts/create_files/compile_full.sh fib_timed 10 baking_timed

## Run and save benchmarks
cargo clean
mkdir -p save/examples/
rm -rf save/examples/*
echo "Examples full bench"
cargo bench --bench="example_fib_*" --all-features
find . -name "*.svg" -delete
mv -f target/criterion/* save/examples/
echo "Examples full bench weight"
du -s -m
cargo clean
