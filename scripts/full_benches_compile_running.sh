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
    cat scripts/toml/full_cargo.toml > Cargo.toml

    # Create folders if they do not exist
    mkdir -p save/
    rm -rf save/*

    cargo clean

    ## Compile and run examples
    bash ./scripts/examples_affine_timed_literature_extra.sh

    cargo clean

    ## Compile mesh and ring
    bash ./scripts/full_benches_compile_mesh_ring.sh

    cargo clean

    ## Run mesh and ring and ping-pong
    bash ./scripts/benches_runtime_mesh_ring_ping_pong.sh

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
