#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

## Compile binary examples
bash ./scripts/create_files/compile_full.sh example_fib_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_simple_voting_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_calculator_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_three_buyers_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_travel_agency_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_o_auth_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_logging_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_online_wallet_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_smtp_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_servo_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_video_stream_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_remote_data_binary 20 binary
bash ./scripts/create_files/compile_full.sh example_circuit_breaker_binary 20 binary

## Compile baking examples
bash ./scripts/create_files/compile_full.sh example_fib 20 baking
bash ./scripts/create_files/compile_full.sh example_simple_voting 20 baking
bash ./scripts/create_files/compile_full.sh example_calculator 20 baking
bash ./scripts/create_files/compile_full.sh example_three_buyers 20 baking
bash ./scripts/create_files/compile_full.sh example_travel_agency 20 baking
bash ./scripts/create_files/compile_full.sh example_o_auth 20 baking
bash ./scripts/create_files/compile_full.sh example_logging 20 baking
bash ./scripts/create_files/compile_full.sh example_online_wallet 20 baking
bash ./scripts/create_files/compile_full.sh example_smtp 20 baking
bash ./scripts/create_files/compile_full.sh example_servo 20 baking
bash ./scripts/create_files/compile_full.sh example_video_stream 20 baking
bash ./scripts/create_files/compile_full.sh example_remote_data 20 baking
bash ./scripts/create_files/compile_full.sh example_circuit_breaker 20 baking

# ## Compile extras examples
# bash ./scripts/create_files/compile_full.sh example_o_auth_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh example_o_auth_transport 20 transport_macros_multiple
# bash ./scripts/create_files/compile_full.sh example_dns_fowler 20 baking
# bash ./scripts/create_files/compile_full.sh example_dns_fowler_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh example_dns_imai 20 baking
# bash ./scripts/create_files/compile_full.sh example_http 20 baking

# ## Compile checking examples
# bash ./scripts/create_files/compile_full.sh example_video_stream_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh example_adder_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh example_basic_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh example_servo_8257_original_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh example_servo_8257_fixed_checking 20 baking_checking

# ## Compile interleaved examples
# bash ./scripts/create_files/compile_full.sh example_circuit_breaker_logging 20 interleaved
# bash ./scripts/create_files/compile_full.sh example_circuit_breaker_solo 20 interleaved
# bash ./scripts/create_files/compile_full.sh example_logging_solo 20 interleaved
# bash ./scripts/create_files/compile_full.sh example_sleeping_barber 20 interleaved

## Compile timed examples
bash ./scripts/create_files/compile_full.sh example_fib_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_simple_voting_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_calculator_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_three_buyers_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_travel_agency_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_o_auth_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_online_wallet_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_smtp_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_servo_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_logging_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_video_stream_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_remote_data_timed 20 baking_timed
bash ./scripts/create_files/compile_full.sh example_circuit_breaker_timed 20 baking_timed

# ## Compile timed extras examples
# bash ./scripts/create_files/compile_full.sh example_servo_8257_async_failing 20 baking_timed
# bash ./scripts/create_files/compile_full.sh example_servo_8257_async_working 20 baking_timed
# bash ./scripts/create_files/compile_full.sh example_servo_8257_async_fixed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh example_http_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh example_non_feasible 20 baking_timed

# ## Compile timed interleaved examples
# bash ./scripts/create_files/compile_full.sh example_sleeping_barber_timed 20 timed_interleaved
# bash ./scripts/create_files/compile_full.sh example_circuit_breaker_logging_timed 20 timed_interleaved

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
