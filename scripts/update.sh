#!/bin/sh

# set -e

cargo clean
rustup update
cargo install-update -a
cargo update --workspace
cargo fmt --all --verbose
cargo clippy --workspace --all-features --all-targets --verbose
# cargo hack --feature-powerset --exclude-no-default-features clippy -- -D warning
TRYBUILD=overwrite cargo test --all-targets --verbose --workspace --all-features
cargo clean
RUSTFLAGS="-Z macro-backtrace" cargo +nightly clippy --all-targets --all-features --workspace
cargo clean
