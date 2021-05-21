#!/bin/sh

set -e

cargo check
cargo check --all
cargo check --tests
cargo check --examples
cargo check --benches