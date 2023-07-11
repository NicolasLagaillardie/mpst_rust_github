#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

## Compile baking examples
bash ./scripts/create_files/compile_full.sh fib 1 baking
bash ./scripts/create_files/compile_full.sh simple_voting 1 baking
bash ./scripts/create_files/compile_full.sh distributed_calc 1 baking
bash ./scripts/create_files/compile_full.sh three_buyers 1 baking
bash ./scripts/create_files/compile_full.sh three_travel 1 baking
bash ./scripts/create_files/compile_full.sh o_auth 1 baking
bash ./scripts/create_files/compile_full.sh online_wallet 1 baking
bash ./scripts/create_files/compile_full.sh smtp 1 baking
bash ./scripts/create_files/compile_full.sh servo 1 baking
bash ./scripts/create_files/compile_full.sh logging 1 baking
bash ./scripts/create_files/compile_full.sh video_stream 1 baking
bash ./scripts/create_files/compile_full.sh remote_data 1 baking
bash ./scripts/create_files/compile_full.sh circuit_breaker 1 baking

# ## Compile extras examples
# bash ./scripts/create_files/compile_full.sh o_auth_checking 1 baking_checking
# bash ./scripts/create_files/compile_full.sh o_auth_transport 1 transport_macros_multiple
# bash ./scripts/create_files/compile_full.sh dns_fowler 1 baking
# bash ./scripts/create_files/compile_full.sh dns_fowler_checking 1 baking_checking
# bash ./scripts/create_files/compile_full.sh dns_imai 1 baking
# bash ./scripts/create_files/compile_full.sh http 1 baking

# ## Compile checking examples
# bash ./scripts/create_files/compile_full.sh video_stream 1 baking_checking
# bash ./scripts/create_files/compile_full.sh adder 1 baking_checking
# bash ./scripts/create_files/compile_full.sh basic 1 baking_checking
# bash ./scripts/create_files/compile_full.sh servo_8257_original 1 baking_checking
# bash ./scripts/create_files/compile_full.sh servo_8257_fixed 1 baking_checking

# ## Compile interleaved examples
# bash ./scripts/create_files/compile_full.sh circuit_breaker_logging 1 interleaved
# bash ./scripts/create_files/compile_full.sh circuit_breaker_solo 1 interleaved
# bash ./scripts/create_files/compile_full.sh logging_solo 1 interleaved
# bash ./scripts/create_files/compile_full.sh sleeping_barber 1 interleaved

## Compile timed examples
bash ./scripts/create_files/compile_full.sh fib_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh simple_voting_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh distributed_calc_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh three_buyers_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh three_travel_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh o_auth_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh online_wallet_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh smtp_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh servo_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh logging_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh video_stream_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh remote_data_timed 1 baking_timed
bash ./scripts/create_files/compile_full.sh circuit_breaker_timed 1 baking_timed

# ## Compile timed extras examples
# bash ./scripts/create_files/compile_full.sh servo_8257_async_failing 1 baking_timed
# bash ./scripts/create_files/compile_full.sh servo_8257_async_working 1 baking_timed
# bash ./scripts/create_files/compile_full.sh servo_8257_async_fixed 1 baking_timed
# bash ./scripts/create_files/compile_full.sh http_timed 1 baking_timed
# bash ./scripts/create_files/compile_full.sh non_feasible 1 baking_timed

# ## Compile timed interleaved examples
# bash ./scripts/create_files/compile_full.sh sleeping_barber_timed 1 timed_interleaved
# bash ./scripts/create_files/compile_full.sh circuit_breaker_logging_timed 1 timed_interleaved

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
