#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

sleep 60s

{
    # Saving Cargo.toml
    cat Cargo.toml > scripts/toml/save_cargo.toml

    # Updating Cargo.toml
    cat scripts/toml/full_cargo.toml > Cargo.toml

    cargo clean

    ## Compile and run examples
    bash ./scripts/examples_affine_timed_literature.sh

    cargo clean

    ## Compile mesh and ring
    bash ./scripts/benches_compile_mesh_ring.sh

    cargo clean

    ## Run mesh and ring and ping-pong
    bash ./scripts/benches_runtime_mesh_ring_ping_pong.sh

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
