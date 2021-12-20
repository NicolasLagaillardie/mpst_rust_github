#!/bin/sh

# Stop upon any error
set -e

cargo test --tests --all-features --workspace # Test all tests
cargo test --examples --all-features --workspace # Test all examples
cargo test --benches --all-features --workspace # Test all benchmarks
cargo test --all-targets --all-features --workspace # Test everything in the library
./scripts/top_down.sh
./scripts/bottom_up.sh
./scripts/examples_literature.sh # Will take up to one hour, progress is displayed in the terminal
./scripts/ping_pong_mesh_ring_light.sh # Set up
./scripts/ping_pong_mesh_ring.sh # This will take around 1 hour
./scripts/full_library.sh # set up
# ./scripts/ping_pong_mesh_ring_full.sh # This will take more than 24 hours
# ./scripts/ping_pong.sh # For ping-pong protocols
# ##
# ./scripts/mesh_full.sh # For all mesh protocols
# ./scripts/ring_full.sh # For all ring protocols
##
./scripts/mesh_light.sh # For mesh protocols up to five participants
./scripts/ring_light.sh # For ring protocols up to five participants
./scripts/top_down_adder.rs
# cargo run --example="Adder_generated" --features=backing_checking
