#!/bin/sh

rustup update && cargo install-update -a && cargo update && cargo +nightly clippy --all-targets --all-features --workspace && cargo test --all-targets --all-features --workspace
