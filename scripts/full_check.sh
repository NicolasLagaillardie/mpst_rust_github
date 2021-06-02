#!/bin/sh

set -e

cargo fmt
cargo check
cargo check --all
cargo check --tests
cargo check --examples
cargo check --benches
cargo t
cargo clippy