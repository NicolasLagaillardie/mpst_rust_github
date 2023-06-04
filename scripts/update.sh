#!/bin/sh

set -e

rustup update
cargo clean
cargo install-update -a
cargo update --workspace
cargo fmt --all
cargo +nightly clippy --all-targets --all-features --workspace

# Test everything
echo "Testing features"
#
echo "default"
cargo test --all-targets --verbose --workspace --features="default"
#
echo "macros_simple"
cargo test --all-targets --verbose --workspace --features="macros_simple"
#
echo "macros_multiple"
cargo test --all-targets --verbose --workspace --features="macros_multiple"
#
echo "interleaved"
cargo test --all-targets --verbose --workspace --features="interleaved"
#
echo "checking"
cargo test --all-targets --verbose --workspace --features="checking"
#
echo "macros_multiple checking"
cargo test --all-targets --verbose --workspace --features="macros_multiple checking"
#
echo "baking"
cargo test --all-targets --verbose --workspace --features="baking"
#
echo "baking_timed"
cargo test --all-targets --verbose --workspace --features="baking_timed"
#
echo "baking_interleaved"
cargo test --all-targets --verbose --workspace --features="baking_interleaved"
#
echo "baking_checking"
cargo test --all-targets --verbose --workspace --features="baking_checking"
#
echo "transport_tcp"
cargo test --all-targets --verbose --workspace --features="transport_tcp"
#
echo "transport_udp"
cargo test --all-targets --verbose --workspace --features="transport_udp"
#
echo "transport_http"
cargo test --all-targets --verbose --workspace --features="transport_http"
#
echo "transport"
cargo test --all-targets --verbose --workspace --features="transport"
#
echo "transport_macros_multiple"
cargo test --all-targets --verbose --workspace --features="transport_macros_multiple"
#
echo "affine_timed"
cargo test --all-targets --verbose --workspace --features="affine_timed"
#
echo "full_without_checking"
cargo test --all-targets --verbose --workspace --features="full_without_checking"
#
echo "full"
cargo test --all-targets --verbose --workspace --features="full"
#
echo "all-features"
cargo test --all-targets --verbose --workspace --all-features