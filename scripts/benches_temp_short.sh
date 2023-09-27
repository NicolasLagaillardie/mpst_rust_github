#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

{
    # Stop upon any error
    set -e

    # Disable internet
    # nmcli networking off

    sleep 60s

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
    bash ./scripts/benches_runtime_ping_pong_mpst.sh 50
    echo "Ping-pong full bench weight"

    ### Clean compiled files
    cargo clean

    cargo bench --benches --all-features --workspace
    mv target/criterion/* save/

    # Enable internet
    # nmcli networking on

    # Get data
    date

    # Send done email
    bash ./scripts/curl/done_curl.sh
} || {
    # Enable internet
    # nmcli networking on

    # Send fail email
    bash ./scripts/curl/fail_curl.sh
}
