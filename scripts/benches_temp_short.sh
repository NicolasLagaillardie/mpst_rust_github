#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

{
    # Stop upon any error
    set -e

    sleep 30s

    # Get date
    date

    ### Clean compiled files
    cargo clean

    # Run all ping-pong benchmarks
    echo "Ping-pong full bench"
    date
    mkdir -p save/ping_pong/
    rm -rf save/ping_pong/*
    # Run all ping-pong benchmarks
    bash ./scripts/benches_runtime_ping_pong_binary.sh 130
    bash ./scripts/benches_runtime_ping_pong_mpst.sh 50
    bash ./scripts/benches_runtime_ping_pong_crossbeam.sh 250
    echo "Ping-pong full bench weight"

    ### Run
    cargo bench --bench="distrib_calc**" --all-features -- --verbose

    ### Clean compiled files
    bash ./scripts/create_files/compile_full.sh distributed_calculators_crossbeam 10 default
    bash ./scripts/create_files/compile_full.sh distributed_calculators_binary 10 default
    bash ./scripts/create_files/compile_full.sh distributed_calculators_mpst 10 baking
    bash ./scripts/create_files/compile_full.sh distributed_calculators_ampst 10 baking
    bash ./scripts/create_files/compile_full.sh distributed_calculators_timed 10 baking_timed

    ### Clean compiled files
    cargo clean

    # Resetting Cargo.toml
    cat scripts/toml/save_cargo.toml > Cargo.toml

    # Send done email
    bash ./scripts/curl/done_curl.sh
} || {
    # Resetting Cargo.toml
    cat scripts/toml/save_cargo.toml > Cargo.toml

    # Send fail email
    bash ./scripts/curl/fail_curl.sh
}
