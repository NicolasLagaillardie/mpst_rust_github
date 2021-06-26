#!/bin/sh

set -e

echo "cargo fmt started"
cargo fmt
echo "cargo fmt completed"
echo "cargo check started"
cargo check
echo "cargo check completed"
echo "cargo check --all started"
cargo check --all
echo "cargo check --all completed"
echo "cargo check --tests started"
cargo check --tests
echo "cargo check --tests completed"
echo "cargo check --examples started"
cargo check --examples
echo "cargo check --examples completed"
echo "cargo check --benches started"
cargo check --benches
echo "cargo check --benches completed"
echo "cargo test --verbose --all started"
cargo test --verbose --all
echo "cargo test --verbose --all completed"
echo "cargo test --verbose --all -- --nocapture started"
cargo test --verbose --all -- --nocapture
echo "cargo test --verbose --all -- --nocapture completed"
echo "cargo test --verbose --all --no-default-features --no-run started"
cargo test --verbose --all --no-default-features --no-run
echo "cargo test --verbose --all --no-default-features --no-run completed"
echo "cargo clippy started"
cargo clippy
echo "cargo clippy completed"