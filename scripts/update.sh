#!/bin/sh

set -e

cargo clean
rustup update
cargo install-update -a
cargo update --workspace
cargo fmt --all
cargo +nightly clippy --all-targets --all-features --workspace
cargo check --all-targets --verbose --workspace --all-features
cargo test --all-targets --verbose --workspace --all-features
cargo clean

# # Check everything
# echo "Checking features"
# #
# echo "Checking default"
# cargo check --all-targets --verbose --workspace --features="default"
# cargo clean
# #
# echo "Checking macros_simple"
# cargo check --all-targets --verbose --workspace --features="macros_simple"
# cargo clean
# #
# echo "Checking macros_multiple"
# cargo check --all-targets --verbose --workspace --features="macros_multiple"
# cargo clean
# #
# echo "Checking interleaved"
# cargo check --all-targets --verbose --workspace --features="interleaved"
# cargo clean
# #
# echo "Checking checking"
# cargo check --all-targets --verbose --workspace --features="checking"
# cargo clean
# #
# echo "Checking macros_multiple checking"
# cargo check --all-targets --verbose --workspace --features="macros_multiple checking"
# cargo clean
# #
# echo "Checking baking"
# cargo check --all-targets --verbose --workspace --features="baking"
# cargo clean
# #
# echo "Checking baking_timed"
# cargo check --all-targets --verbose --workspace --features="baking_timed"
# cargo clean
# #
# echo "Checking baking_interleaved"
# cargo check --all-targets --verbose --workspace --features="baking_interleaved"
# cargo clean
# #
# echo "Checking baking_timed_interleaved"
# cargo check --all-targets --verbose --workspace --features="baking_timed_interleaved"
# cargo clean
# #
# echo "Checking baking_checking"
# cargo check --all-targets --verbose --workspace --features="baking_checking"
# cargo clean
# #
# echo "Checking transport_tcp"
# cargo check --all-targets --verbose --workspace --features="transport_tcp"
# cargo clean
# #
# echo "Checking transport_udp"
# cargo check --all-targets --verbose --workspace --features="transport_udp"
# cargo clean
# #
# echo "Checking transport_http"
# cargo check --all-targets --verbose --workspace --features="transport_http"
# cargo clean
# #
# echo "Checking transport"
# cargo check --all-targets --verbose --workspace --features="transport"
# cargo clean
# #
# echo "Checking transport_macros_multiple"
# cargo check --all-targets --verbose --workspace --features="transport_macros_multiple"
# cargo clean
# #
# echo "Checking affine_timed"
# cargo check --all-targets --verbose --workspace --features="affine_timed"
# cargo clean
# #
# echo "Checking full_without_checking"
# cargo check --all-targets --verbose --workspace --features="full_without_checking"
# cargo clean
# #
# echo "Checking full"
# cargo check --all-targets --verbose --workspace --features="full"
# cargo clean
# #
# echo "Checking all-features"
# cargo check --all-targets --verbose --workspace --all-features
# cargo clean

# # Test everything
# echo "Testing features"
# #
# echo "Testing default"
# cargo test --all-targets --verbose --workspace --features="default"
# cargo clean
# #
# echo "Testing macros_simple"
# cargo test --all-targets --verbose --workspace --features="macros_simple"
# cargo clean
# #
# echo "Testing macros_multiple"
# cargo test --all-targets --verbose --workspace --features="macros_multiple"
# cargo clean
# #
# echo "Testing interleaved"
# cargo test --all-targets --verbose --workspace --features="interleaved"
# cargo clean
# #
# echo "Testing checking"
# cargo test --all-targets --verbose --workspace --features="checking"
# cargo clean
# #
# echo "Testing macros_multiple checking"
# cargo test --all-targets --verbose --workspace --features="macros_multiple checking"
# cargo clean
# #
# echo "Testing baking"
# cargo test --all-targets --verbose --workspace --features="baking"
# cargo clean
# #
# echo "Testing baking_timed"
# cargo test --all-targets --verbose --workspace --features="baking_timed"
# cargo clean
# #
# echo "Testing baking_interleaved"
# cargo test --all-targets --verbose --workspace --features="baking_interleaved"
# cargo clean
# #
# echo "Testing baking_timed_interleaved"
# cargo test --all-targets --verbose --workspace --features="baking_timed_interleaved"
# cargo clean
# #
# echo "Testing baking_checking"
# cargo test --all-targets --verbose --workspace --features="baking_checking"
# cargo clean
# #
# echo "Testing transport_tcp"
# cargo test --all-targets --verbose --workspace --features="transport_tcp"
# cargo clean
# #
# echo "Testing transport_udp"
# cargo test --all-targets --verbose --workspace --features="transport_udp"
# cargo clean
# #
# echo "Testing transport_http"
# cargo test --all-targets --verbose --workspace --features="transport_http"
# cargo clean
# #
# echo "Testing transport"
# cargo test --all-targets --verbose --workspace --features="transport"
# cargo clean
# #
# echo "Testing transport_macros_multiple"
# cargo test --all-targets --verbose --workspace --features="transport_macros_multiple"
# cargo clean
# #
# echo "Testing affine_timed"
# cargo test --all-targets --verbose --workspace --features="affine_timed"
# cargo clean
# #
# echo "Testing full_without_checking"
# cargo test --all-targets --verbose --workspace --features="full_without_checking"
# cargo clean
# #
# echo "Testing full"
# cargo test --all-targets --verbose --workspace --features="full"
# cargo clean
# #
# echo "Testing all-features"
# cargo test --all-targets --verbose --workspace --all-features
# cargo clean
