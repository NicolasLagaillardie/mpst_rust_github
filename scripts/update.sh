#!/bin/sh

set -eou pipefail

clear
clear
cargo clean
rustup update

if cargo install-update -a ; then
	echo "cargo update successful"
else
	cargo install cargo-update --force
	# cargo install cargo-update --force --features vendored-libgit2 --features vendored-openssl
fi

cargo update --workspace
cargo fmt --all --verbose
cargo clippy --workspace --all-features --all-targets --verbose
cargo +nightly clippy --workspace --all-features --all-targets --verbose
# cargo hack --feature-powerset --clean-per-run clippy --all-targets --workspace -- -D warnings
# cargo hack --feature-powerset --clean-per-run test --all-targets --workspace
TRYBUILD=overwrite cargo test --all-targets --verbose --workspace --all-features
cargo clean
RUSTFLAGS="-Z macro-backtrace" cargo +nightly clippy --workspace --all-targets --all-features --verbose
cargo clean
