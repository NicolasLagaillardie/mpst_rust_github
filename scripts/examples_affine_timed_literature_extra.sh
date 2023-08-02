#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

# Get date
date

# ## Compile crossbeam examples
# bash ./scripts/create_files/compile_full.sh calculator_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh circuit_breaker_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh fib_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh logging_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh o_auth_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh online_wallet_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh remote_data_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh servo_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh simple_voting_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh smtp_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh three_buyers_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh travel_agency_crossbeam 20 default
# bash ./scripts/create_files/compile_full.sh video_stream_crossbeam 20 default

# ## Compile binary examples
# bash ./scripts/create_files/compile_full.sh calculator_binary 20 default
# bash ./scripts/create_files/compile_full.sh circuit_breaker_binary 20 default
# bash ./scripts/create_files/compile_full.sh fib_binary 20 default
# bash ./scripts/create_files/compile_full.sh logging_binary 20 default
# bash ./scripts/create_files/compile_full.sh o_auth_binary 20 default
# bash ./scripts/create_files/compile_full.sh online_wallet_binary 20 default
# bash ./scripts/create_files/compile_full.sh remote_data_binary 20 default
# bash ./scripts/create_files/compile_full.sh servo_binary 20 default
# bash ./scripts/create_files/compile_full.sh simple_voting_binary 20 default
# bash ./scripts/create_files/compile_full.sh smtp_binary 20 default
# bash ./scripts/create_files/compile_full.sh three_buyers_binary 20 default
# bash ./scripts/create_files/compile_full.sh travel_agency_binary 20 default
# bash ./scripts/create_files/compile_full.sh video_stream_binary 20 default

# ## Compile basic examples
# bash ./scripts/create_files/compile_full.sh video_stream_basic 20 macros_simple
# bash ./scripts/create_files/compile_full.sh circuit_breaker_basic 20 macros_multiple
# bash ./scripts/create_files/compile_full.sh logging_basic 20 macros_multiple

# ## Compile mpst examples
# bash ./scripts/create_files/compile_full.sh fib_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh circuit_breaker_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh logging_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh calculator_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh dns_fowler_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh dns_fowler_checking_mpst 20 baking_checking
# bash ./scripts/create_files/compile_full.sh dns_imai_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh o_auth_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh o_auth_checking_mpst 20 baking_checking
# bash ./scripts/create_files/compile_full.sh o_auth_transport_mpst 20 transport_macros_multiple
# bash ./scripts/create_files/compile_full.sh online_wallet_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh simple_voting_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh smtp_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh three_buyers_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh travel_agency_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh servo_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh remote_data_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh http_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh video_stream_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh servo_8257_original_mpst 20 baking
# bash ./scripts/create_files/compile_full.sh servo_8257_fixed_mpst 20 baking

# ## Compile ampst examples
# bash ./scripts/create_files/compile_full.sh fib_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh circuit_breaker_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh logging_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh calculator_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh dns_fowler_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh dns_fowler_checking_ampst 20 baking_checking
# bash ./scripts/create_files/compile_full.sh dns_imai_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh o_auth_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh o_auth_checking_ampst 20 baking_checking
# bash ./scripts/create_files/compile_full.sh o_auth_transport_ampst 20 transport_macros_multiple
# bash ./scripts/create_files/compile_full.sh online_wallet_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh simple_voting_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh smtp_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh three_buyers_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh travel_agency_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh servo_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh remote_data_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh http_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh video_stream_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh servo_8257_original_ampst 20 baking
# bash ./scripts/create_files/compile_full.sh servo_8257_fixed_ampst 20 baking

# ## Compile checking examples
# bash ./scripts/create_files/compile_full.sh video_stream_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh adder_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh basic_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh servo_8257_original_checking 20 baking_checking
# bash ./scripts/create_files/compile_full.sh servo_8257_fixed_checking 20 baking_checking

# ## Compile interleaved examples
# bash ./scripts/create_files/compile_full.sh circuit_breaker_logging_interleaved 20 interleaved
# bash ./scripts/create_files/compile_full.sh circuit_breaker_solo 20 interleaved
# bash ./scripts/create_files/compile_full.sh logging_solo 20 interleaved
# bash ./scripts/create_files/compile_full.sh sleeping_barber_interleaved 20 interleaved

# ## Compile interleaved timed examples
# bash ./scripts/create_files/compile_full.sh sleeping_barber_interleaved_timed 20 baking_timed_interleaved
# bash ./scripts/create_files/compile_full.sh circuit_breaker_logging_interleaved_timed 20 baking_timed_interleaved

# ## Compile timed examples
# bash ./scripts/create_files/compile_full.sh fib_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh servo_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh servo_8257_async_failing_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh servo_8257_async_working_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh servo_8257_async_fixed_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh simple_voting_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh calculator_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh o_auth_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh online_wallet_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh smtp_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh three_buyers_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh travel_agency_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh remote_data_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh http_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh non_feasible_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh circuit_breaker_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh logging_timed 20 baking_timed
# bash ./scripts/create_files/compile_full.sh video_stream_timed 20 baking_timed

## Run and save benchmarks
cargo clean
rm -rf save/example/*
echo "Examples full bench"
cargo bench --bench="example_*" --all-features -- --verbose
find . -name "*.svg" -delete
mv -f target/criterion/* save/examples/
echo "Examples full bench weight"
du -s -m
cargo clean
