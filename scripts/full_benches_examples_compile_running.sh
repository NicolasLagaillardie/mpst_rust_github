#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

{
    # Stop upon any error
    set -e

    # Get date
    date

    # Saving Cargo.toml
    cat Cargo.toml > scripts/toml/save_cargo.toml

    # Updating Cargo.toml
    cat scripts/toml/full_benches_cargo.toml > Cargo.toml

    # Create folders if they do not exist
    mkdir -p save/
    rm -rf save/*

    cargo clean

    ## Run and save benchmarks
    cargo clean
    mkdir -p save/examples/
    rm -rf save/examples/*
    echo "Examples full bench"
    cargo bench --bench="example_*" --all-features
    find . -name "*.svg" -delete
    mv -f target/criterion/* save/examples/
    echo "Examples full bench weight"
    du -s -m
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
