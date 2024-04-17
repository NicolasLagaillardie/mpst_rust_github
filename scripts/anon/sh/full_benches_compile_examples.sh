#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

# Get date
date

## Compile ampst examples
bash ./scripts/create_files/compile_normal.sh calculator_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh online_wallet_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh smtp_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh simple_voting_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh three_buyers_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh travel_agency_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh o_auth_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh http_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh remote_data_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh servo_ampst 100 baking

## Compile rate-base examples, AMPST version
bash ./scripts/create_files/compile_normal.sh gravity_android_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh pinetime_heart_rate_ampst 100 baking
bash ./scripts/create_files/compile_normal.sh proximity_based_car_key_ampst 100 baking

## Compile timed examples
bash ./scripts/create_files/compile_normal.sh calculator_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh online_wallet_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh smtp_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh simple_voting_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh three_buyers_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh travel_agency_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh o_auth_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh http_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh remote_data_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh servo_timed 100 baking_timed

## Compile rate-base examples, ATMP version
bash ./scripts/create_files/compile_normal.sh gravity_android_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh pinetime_heart_rate_timed 100 baking_timed
bash ./scripts/create_files/compile_normal.sh proximity_based_car_key_timed 100 baking_timed

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
