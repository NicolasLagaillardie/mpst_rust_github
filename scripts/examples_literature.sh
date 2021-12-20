#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

## Compile basic examples
./scripts/create_files/compile.sh three_buyers 10 macros_multiple
./scripts/create_files/compile.sh distributed_calc 10 macros_multiple
./scripts/create_files/compile.sh travel_three 10 macros_multiple
./scripts/create_files/compile.sh simple_voting 10 macros_multiple
./scripts/create_files/compile.sh online_wallet 10 macros_multiple
./scripts/create_files/compile.sh fib 10 macros_multiple
./scripts/create_files/compile.sh video_stream 10 baking_checking
./scripts/create_files/compile.sh o_auth 10 baking_checking
./scripts/create_files/compile.sh logging_baking 10 baking
./scripts/create_files/compile.sh circuit_breaker_baking 10 baking
./scripts/create_files/compile.sh smtp 10 macros_multiple

## Run benchmarks
cargo bench --bench main --all-features -- --verbose

## Concatenate all results in the results/benchmarks_main_from_literature.csv file
python3 scripts/create_graphs/examples_literature.py