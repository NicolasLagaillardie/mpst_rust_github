#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

# Get date
date

## Compile crossbeam examples
bash ./scripts/create_files/compile_full.sh calculator_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh circuit_breaker_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh fib_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh logging_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh o_auth_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh online_wallet_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh remote_data_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh servo_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh simple_voting_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh smtp_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh three_buyers_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh travel_agency_crossbeam 10 default
bash ./scripts/create_files/compile_full.sh video_stream_crossbeam 10 default

## Compile binary examples
bash ./scripts/create_files/compile_full.sh calculator_binary 10 default
bash ./scripts/create_files/compile_full.sh circuit_breaker_binary 10 default
bash ./scripts/create_files/compile_full.sh fib_binary 10 default
bash ./scripts/create_files/compile_full.sh logging_binary 10 default
bash ./scripts/create_files/compile_full.sh o_auth_binary 10 default
bash ./scripts/create_files/compile_full.sh online_wallet_binary 10 default
bash ./scripts/create_files/compile_full.sh remote_data_binary 10 default
bash ./scripts/create_files/compile_full.sh servo_binary 10 default
bash ./scripts/create_files/compile_full.sh simple_voting_binary 10 default
bash ./scripts/create_files/compile_full.sh smtp_binary 10 default
bash ./scripts/create_files/compile_full.sh three_buyers_binary 10 default
bash ./scripts/create_files/compile_full.sh travel_agency_binary 10 default
bash ./scripts/create_files/compile_full.sh video_stream_binary 10 default

## Compile basic examples
bash ./scripts/create_files/compile_full.sh video_stream_basic 10 macros_simple
bash ./scripts/create_files/compile_full.sh circuit_breaker_basic 10 macros_multiple
bash ./scripts/create_files/compile_full.sh logging_basic 10 macros_multiple

## Compile mpst examples
bash ./scripts/create_files/compile_full.sh fib_mpst 10 baking
bash ./scripts/create_files/compile_full.sh circuit_breaker_mpst 10 baking
bash ./scripts/create_files/compile_full.sh logging_mpst 10 baking
bash ./scripts/create_files/compile_full.sh calculator_mpst 10 baking
bash ./scripts/create_files/compile_full.sh dns_fowler_mpst 10 baking
bash ./scripts/create_files/compile_full.sh dns_imai_mpst 10 baking
bash ./scripts/create_files/compile_full.sh o_auth_mpst 10 baking
bash ./scripts/create_files/compile_full.sh online_wallet_mpst 10 baking
bash ./scripts/create_files/compile_full.sh simple_voting_mpst 10 baking
bash ./scripts/create_files/compile_full.sh smtp_mpst 10 baking
bash ./scripts/create_files/compile_full.sh three_buyers_mpst 10 baking
bash ./scripts/create_files/compile_full.sh travel_agency_mpst 10 baking
bash ./scripts/create_files/compile_full.sh servo_mpst 10 baking
bash ./scripts/create_files/compile_full.sh remote_data_mpst 10 baking
bash ./scripts/create_files/compile_full.sh http_mpst 10 baking
bash ./scripts/create_files/compile_full.sh video_stream_mpst 10 baking
bash ./scripts/create_files/compile_full.sh servo_8257_original_mpst 10 baking
bash ./scripts/create_files/compile_full.sh servo_8257_fixed_mpst 10 baking

## Compile ampst examples
bash ./scripts/create_files/compile_full.sh fib_ampst 10 baking
bash ./scripts/create_files/compile_full.sh circuit_breaker_ampst 10 baking
bash ./scripts/create_files/compile_full.sh logging_ampst 10 baking
bash ./scripts/create_files/compile_full.sh calculator_ampst 10 baking
bash ./scripts/create_files/compile_full.sh dns_fowler_ampst 10 baking
bash ./scripts/create_files/compile_full.sh dns_imai_ampst 10 baking
bash ./scripts/create_files/compile_full.sh o_auth_ampst 10 baking
bash ./scripts/create_files/compile_full.sh online_wallet_ampst 10 baking
bash ./scripts/create_files/compile_full.sh simple_voting_ampst 10 baking
bash ./scripts/create_files/compile_full.sh smtp_ampst 10 baking
bash ./scripts/create_files/compile_full.sh three_buyers_ampst 10 baking
bash ./scripts/create_files/compile_full.sh travel_agency_ampst 10 baking
bash ./scripts/create_files/compile_full.sh servo_ampst 10 baking
bash ./scripts/create_files/compile_full.sh remote_data_ampst 10 baking
bash ./scripts/create_files/compile_full.sh http_ampst 10 baking
bash ./scripts/create_files/compile_full.sh video_stream_ampst 10 baking
bash ./scripts/create_files/compile_full.sh servo_8257_original_ampst 10 baking
bash ./scripts/create_files/compile_full.sh servo_8257_fixed_ampst 10 baking

## Compile checking examples
bash ./scripts/create_files/compile_full.sh dns_fowler_checking_mpst 10 baking_checking
bash ./scripts/create_files/compile_full.sh o_auth_checking_mpst 10 baking_checking
bash ./scripts/create_files/compile_full.sh dns_fowler_checking_ampst 10 baking_checking
bash ./scripts/create_files/compile_full.sh o_auth_checking_ampst 10 baking_checking
bash ./scripts/create_files/compile_full.sh video_stream_checking 10 baking_checking
bash ./scripts/create_files/compile_full.sh adder_checking 10 baking_checking
bash ./scripts/create_files/compile_full.sh basic_checking 10 baking_checking
bash ./scripts/create_files/compile_full.sh servo_8257_original_checking 10 baking_checking
bash ./scripts/create_files/compile_full.sh servo_8257_fixed_checking 10 baking_checking

## Compile interleaved examples
bash ./scripts/create_files/compile_full.sh circuit_breaker_logging_interleaved 10 interleaved
bash ./scripts/create_files/compile_full.sh circuit_breaker_solo 10 interleaved
bash ./scripts/create_files/compile_full.sh logging_solo 10 interleaved
bash ./scripts/create_files/compile_full.sh sleeping_barber_interleaved 10 interleaved

## Compile interleaved timed examples
bash ./scripts/create_files/compile_full.sh sleeping_barber_interleaved_atmp 10 baking_atmp_interleaved
bash ./scripts/create_files/compile_full.sh circuit_breaker_logging_interleaved_atmp 10 baking_atmp_interleaved

## Compile timed examples
bash ./scripts/create_files/compile_full.sh fib_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh servo_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh servo_8257_async_failing_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh servo_8257_async_working_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh servo_8257_async_fixed_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh simple_voting_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh calculator_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh o_auth_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh online_wallet_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh smtp_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh three_buyers_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh travel_agency_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh remote_data_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh http_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh non_feasible_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh circuit_breaker_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh logging_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh video_stream_atmp 10 baking_atmp

## Compile transport examples
bash ./scripts/create_files/compile_full.sh o_auth_transport_mpst 10 transport_macros_multiple
bash ./scripts/create_files/compile_full.sh o_auth_transport_ampst 10 transport_macros_multiple

## Compile rate-base examples, AMPST version
bash ./scripts/create_files/compile_full.sh gravity_android_ampst 10 baking
bash ./scripts/create_files/compile_full.sh pinetime_heart_rate_ampst 10 baking
bash ./scripts/create_files/compile_full.sh proximity_based_car_key_ampst 10 baking

## Compile rate-base examples, ATMP version
bash ./scripts/create_files/compile_full.sh gravity_android_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh pinetime_heart_rate_atmp 10 baking_atmp
bash ./scripts/create_files/compile_full.sh proximity_based_car_key_atmp 10 baking_atmp

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
