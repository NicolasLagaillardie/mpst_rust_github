#!/bin/sh

# Check Rust code before pushing

set -e

rm -rf cfsm/
./scripts/clean_all.sh

sed -n '/^################################### Features$/,/^################################### Doc handling for all-features$/p' Cargo.toml | grep -iE '[a-z|_]+ =' | grep -iEwo '[a-z|_]+'




cargo check --lib --verbose --workspace --features="macros_multiple checking"
RUST_BACKTRACE=1 cargo check --all-targets --verbose --workspace --all-features
cargo test --verbose --workspace --no-default-features --no-run
cargo test --all-targets --workspace --all-features

echo "done"
