#!/bin/sh

# Check Rust code before pushing

set -e

rm -rf cfsm/
bash ./scripts/clean_all.sh

# cargo check each feature
cargo check --all-targets --verbose --workspace --no-default-features
sed -n '/^################################### Features$/,/^################################### Doc handling for all-features$/p' Cargo.toml | \
grep -iE '[a-z|_]+ =' | \
grep -iEwo '[a-z|_]+ ' | \
while read -r line ; do
    echo "Processing $line"
    RUSTFLAGS="-Z macro-backtrace" cargo check --all-targets --verbose --workspace --features="$line"
done
RUSTFLAGS="-Z macro-backtrace" cargo check --all-targets --verbose --workspace --all-features

# cargo test each feature
cargo test --all-targets --verbose --workspace --no-default-features -- --skip kmc --skip transport_udp
sed -n '/^################################### Features$/,/^################################### Doc handling for all-features$/p' Cargo.toml | \
grep -iE '[a-z|_]+ =' | \
grep -iEwo '[a-z|_]+ ' | \
while read -r line ; do
    echo "Processing $line"
    RUSTFLAGS="-Z macro-backtrace" cargo test --all-targets --verbose --workspace --features="$line" -- --skip kmc --skip transport_udp
done
RUSTFLAGS="-Z macro-backtrace" cargo test --all-targets --verbose --workspace --all-features -- --skip kmc --skip transport_udp

# Reverse toml
cat scripts/toml/save_cargo.toml > Cargo.toml

echo "done"
