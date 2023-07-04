#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

## Compile basic examples
bash ./scripts/create_files/compile_full.sh circuit_breaker 10 macros_multiple
bash ./scripts/create_files/compile_full.sh logging 10 macros_multiple
bash ./scripts/create_files/compile_full.sh circuit_breaker_baking 10 baking
bash ./scripts/create_files/compile_full.sh logging_baking 10 baking
bash ./scripts/create_files/compile_full.sh distributed_calc 10 macros_multiple
bash ./scripts/create_files/compile_full.sh dns_fowler 10 baking
bash ./scripts/create_files/compile_full.sh dns_fowler_checking 10 baking_checking
bash ./scripts/create_files/compile_full.sh dns_imai 10 macros_multiple
bash ./scripts/create_files/compile_full.sh o_auth 10 baking
bash ./scripts/create_files/compile_full.sh o_auth_checking 10 baking_checking
bash ./scripts/create_files/compile_full.sh o_auth_transport 10 transport_macros_multiple
bash ./scripts/create_files/compile_full.sh online_wallet 10 macros_multiple
bash ./scripts/create_files/compile_full.sh simple_voting 10 macros_multiple
bash ./scripts/create_files/compile_full.sh smtp 10 macros_multiple
bash ./scripts/create_files/compile_full.sh three_buyers 10 macros_multiple
bash ./scripts/create_files/compile_full.sh three_travel 10 macros_multiple
bash ./scripts/create_files/compile_full.sh video_stream_basic 10 macros_simple
bash ./scripts/create_files/compile_full.sh servo 10 baking
bash ./scripts/create_files/compile_full.sh remote_data 10 baking
bash ./scripts/create_files/compile_full.sh http 10 baking

## Compile checking examples
bash ./scripts/create_files/compile_full.sh video_stream 10 baking_checking
bash ./scripts/create_files/compile_full.sh adder 10 baking_checking
bash ./scripts/create_files/compile_full.sh basic 10 baking_checking
bash ./scripts/create_files/compile_full.sh servo_8257_original 10 baking_checking
bash ./scripts/create_files/compile_full.sh servo_8257_fixed 10 baking_checking

## Compile interleaved examples
bash ./scripts/create_files/compile_full.sh circuit_breaker_logging 10 interleaved
bash ./scripts/create_files/compile_full.sh circuit_breaker_solo 10 interleaved
bash ./scripts/create_files/compile_full.sh logging_solo 10 interleaved
bash ./scripts/create_files/compile_full.sh sleeping_barber 10 interleaved

## Compile timed examples
bash ./scripts/create_files/compile_full.sh servo_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh servo_8257_async_failing 10 baking_timed
bash ./scripts/create_files/compile_full.sh servo_8257_async_working 10 baking_timed
bash ./scripts/create_files/compile_full.sh servo_8257_async_fixed 10 baking_timed
bash ./scripts/create_files/compile_full.sh simple_voting_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh distributed_calc_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh o_auth_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh online_wallet_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh smtp_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh three_buyers_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh three_travel_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh remote_data_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh http_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh non_feasible 10 baking_timed
bash ./scripts/create_files/compile_full.sh circuit_breaker_timed 10 baking_timed
bash ./scripts/create_files/compile_full.sh logging_timed 10 baking_timed

## Compile timed interleaved examples
bash ./scripts/create_files/compile_full.sh sleeping_barber_timed 10 timed_interleaved
bash ./scripts/create_files/compile_full.sh circuit_breaker_logging_timed 10 timed_interleaved

## Run benchmarks
cargo clean
echo "Examples full bench"
cargo bench --bench="example_*" --all-features -- --verbose
mkdir -p save/examples/
find . -name "*.svg" -delete
mv -f target/criterion/* save/examples/
echo "Examples full bench weight"
du -s -m
cargo clean

## Concatenate all results in the results/benchmarks_main_from_literature.csv file
# python3 scripts/create_graphs/examples_literature_affine_timed.py

# cargo clean &&
# cargo bench --bench examples_baking --all-features -- --verbose &&
# mv -f target/criterion/ save/criterion/examples/ &&
# cargo clean &&
# cargo bench --bench examples_timed --all-features -- --verbose &&
# mv -f target/criterion/ save/criterion/examples_timed/ &&
# cargo clean