#!/bin/sh

rustup update
cargo clean
cargo install-update -a
cargo update --workspace
cargo +nightly clippy --all-targets --all-features --workspace

# Test everything
cargo test --all-targets --verbose --workspace --features="default"
cargo test --all-targets --verbose --workspace --features="macros_simple"
cargo test --all-targets --verbose --workspace --features="macros_multiple"
cargo test --all-targets --verbose --workspace --features="interleaved"
cargo test --all-targets --verbose --workspace --features="checking"
cargo test --all-targets --verbose --workspace --features="macros_multiple checking"
cargo test --all-targets --verbose --workspace --features="baking"
cargo test --all-targets --verbose --workspace --features="baking_timed"
cargo test --all-targets --verbose --workspace --features="baking_interleaved"
cargo test --all-targets --verbose --workspace --features="baking_checking"
cargo test --all-targets --verbose --workspace --features="transport_tcp"
cargo test --all-targets --verbose --workspace --features="transport_udp"
cargo test --all-targets --verbose --workspace --features="transport_http"
cargo test --all-targets --verbose --workspace --features="transport"
cargo test --all-targets --verbose --workspace --features="transport_macros_multiple"
cargo test --all-targets --verbose --workspace --features="affine_timed"
cargo test --all-targets --verbose --workspace --features="full_without_checking"
cargo test --all-targets --verbose --workspace --features="full"
cargo test --all-targets --verbose --workspace --all-features