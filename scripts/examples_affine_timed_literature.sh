#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

## Compile baking examples
bash ./scripts/create_files/compile_full.sh fib 20 baking
bash ./scripts/create_files/compile_full.sh simple_voting 20 baking
bash ./scripts/create_files/compile_full.sh calculator 20 baking
bash ./scripts/create_files/compile_full.sh three_buyers 20 baking
bash ./scripts/create_files/compile_full.sh travel_agency 20 baking
bash ./scripts/create_files/compile_full.sh o_auth 20 baking
bash ./scripts/create_files/compile_full.sh logging 20 baking
bash ./scripts/create_files/compile_full.sh online_wallet 20 baking
bash ./scripts/create_files/compile_full.sh smtp 20 baking
bash ./scripts/create_files/compile_full.sh servo 20 baking
bash ./scripts/create_files/compile_full.sh video_stream 20 baking
bash ./scripts/create_files/compile_full.sh remote_data 20 baking
bash ./scripts/create_files/compile_full.sh circuit_breaker 20 baking

# ## Compile extras examples
# bash ./scripts/create_files/compile_full.sh o_auth_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh o_auth_transport 20 transport_macros_multiple
# bash ./scripts/create_files/compile_full.sh dns_fowler 20 baking
# bash ./scripts/create_files/compile_full.sh dns_fowler_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh dns_imai 20 baking
# bash ./scripts/create_files/compile_full.sh http 20 baking

# ## Compile checking examples
# bash ./scripts/create_files/compile_full.sh video_stream_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh adder_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh basic_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh servo_8257_original_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh servo_8257_fixed_checking 20 baking_checking

# ## Compile interleaved examples
# bash ./scripts/create_files/compile_full.sh circuit_breaker_logging 20 interleaved
# bash ./scripts/create_files/compile_full.sh circuit_breaker_solo 20 interleaved
# bash ./scripts/create_files/compile_full.sh logging_solo 20 interleaved
# bash ./scripts/create_files/compile_full.sh sleeping_barber 20 interleaved

## Compile timed examples
bash ./scripts/create_files/compile_full.sh fib_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh simple_voting_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh calculator_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh three_buyers_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh travel_agency_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh o_auth_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh online_wallet_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh smtp_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh servo_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh logging_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh video_stream_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh remote_data_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh circuit_breaker_timed 20 baking_timed

# ## Compile timed extras examples
# bash ./scripts/create_files/compile_full.sh servo_8257_async_failing 20 baking_timed
# bash ./scripts/create_files/compile_full.sh servo_8257_async_working 20 baking_timed
# bash ./scripts/create_files/compile_full.sh servo_8257_async_fixed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh http_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh non_feasible 20 baking_timed

# ## Compile timed interleaved examples
# bash ./scripts/create_files/compile_full.sh sleeping_barber_timed 20 timed_interleaved
# bash ./scripts/create_files/compile_full.sh circuit_breaker_logging_timed 20 timed_interleaved

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
