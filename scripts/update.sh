#!/bin/sh

set -e

cargo clean
rustup update
cargo install-update -a
cargo update --workspace
cargo fmt --all
cargo +nightly clippy --all-targets --all-features --workspace

# Test everything
echo "Testing features"
#
echo "default"
cargo test --all-targets --verbose --workspace --features="default"
cargo clean
#
echo "macros_simple"
cargo test --all-targets --verbose --workspace --features="macros_simple"
cargo clean
#
echo "macros_multiple"
cargo test --all-targets --verbose --workspace --features="macros_multiple"
cargo clean
#
echo "interleaved"
cargo test --all-targets --verbose --workspace --features="interleaved"
cargo clean
#
echo "checking"
cargo test --all-targets --verbose --workspace --features="checking"
cargo clean
#
echo "macros_multiple checking"
cargo test --all-targets --verbose --workspace --features="macros_multiple checking"
cargo clean
#
echo "baking"
cargo test --all-targets --verbose --workspace --features="baking"
cargo clean
#
echo "baking_timed"
cargo test --all-targets --verbose --workspace --features="baking_timed"
cargo clean
#
echo "baking_interleaved"
cargo test --all-targets --verbose --workspace --features="baking_interleaved"
cargo clean
#
echo "baking_checking"
cargo test --all-targets --verbose --workspace --features="baking_checking"
cargo clean
#
echo "transport_tcp"
cargo test --all-targets --verbose --workspace --features="transport_tcp"
cargo clean
#
echo "transport_udp"
cargo test --all-targets --verbose --workspace --features="transport_udp"
cargo clean
#
echo "transport_http"
cargo test --all-targets --verbose --workspace --features="transport_http"
cargo clean
#
echo "transport"
cargo test --all-targets --verbose --workspace --features="transport"
cargo clean
#
echo "transport_macros_multiple"
cargo test --all-targets --verbose --workspace --features="transport_macros_multiple"
cargo clean
#
echo "affine_timed"
cargo test --all-targets --verbose --workspace --features="affine_timed"
cargo clean
#
echo "full_without_checking"
cargo test --all-targets --verbose --workspace --features="full_without_checking"
cargo clean
#
echo "full"
cargo test --all-targets --verbose --workspace --features="full"
cargo clean
#
echo "all-features"
cargo test --all-targets --verbose --workspace --all-features
cargo clean