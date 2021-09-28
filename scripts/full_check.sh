#!/bin/sh

# Check Rust code before pushing

set -e

rm -rf cfsm/
./scripts/clean_all.sh
#################
# echo "cargo fmt started"
# cargo fmt
# echo "cargo fmt completed"
# echo "cargo check started"
# cargo check
# echo "cargo check completed"
# echo "cargo check --all started"
# cargo check --all
# echo "cargo check --all completed"
# echo "cargo check --tests started"
# cargo check --tests
# echo "cargo check --tests completed"
# echo "cargo check --examples started"
# cargo check --examples
# echo "cargo check --examples completed"
# echo "cargo check --benches started"
# cargo check --benches
# echo "cargo check --benches completed"
# echo "cargo test --verbose --all started"
# cargo test --verbose --all
# echo "cargo test --verbose --all completed"
# echo "cargo test --verbose --all -- --nocapture started"
# cargo test --verbose --all -- --nocapture
# echo "cargo test --verbose --all -- --nocapture completed"
# echo "cargo test --verbose --all --no-default-features --no-run started"
# cargo test --verbose --all --no-default-features --no-run
# echo "cargo test --verbose --all --no-default-features --no-run completed"
# echo "cargo clippy started"
# cargo clippy
# echo "cargo clippy completed"
# echo "cargo doc started"
# cargo doc --verbose --workspace --all-features
# echo "cargo doc completed"
#################
cargo fmt --verbose --all -- --check
cargo clippy --all-features --verbose -- -D warnings
cargo doc --all-features
RUST_BACKTRACE=1 cargo check --verbose --all --all-features
RUST_BACKTRACE=1 cargo check --examples --verbose --all-features
RUST_BACKTRACE=1 cargo check --tests --verbose --all-features 
RUST_BACKTRACE=1 cargo check --benches --verbose --all-features
RUST_BACKTRACE=1 cargo doc --verbose --workspace --all-features
# cargo build --verbose --all --all-features
# cargo build --examples --verbose --all-features
# cargo build --tests --verbose --all-features
# cargo build --benches --verbose --all-features
# cargo run --verbose --all-features
# find ./examples/. -type f -exec sh -c 'for example in "$@"; do (cargo run --example ${example:13:-3} --features="macros") done' argv0 {} +
#################
cargo check --tests --examples --benches --bins --all --features="default"
cargo check --tests --examples --benches --bins --all --features="macros_simple"
cargo check --tests --examples --benches --bins --all --features="macros_multiple"
cargo check --tests --examples --benches --bins --all --features="checking"
cargo check --tests --examples --benches --bins --all --features="baking"
cargo check --tests --examples --benches --bins --all --features="baking_interleaved"
cargo check --tests --examples --benches --bins --all --features="transport_tcp"
cargo check --tests --examples --benches --bins --all --features="transport_udp"
cargo check --tests --examples --benches --bins --all --features="transport_http"
cargo check --tests --examples --benches --bins --all --features="transport"
cargo check --tests --examples --benches --bins --all --features="transport_macros_multiple"
cargo check --tests --examples --benches --bins --all --features="full"
cargo check --tests --examples --benches --bins --all --features="macros_multiple checking"
cargo check --tests --examples --benches --bins --all --all-features
#################
# cargo test --verbose --all
# cargo test --verbose --all -- --nocapture
RUST_BACKTRACE=1 cargo test --verbose --all --all-features -- --nocapture
# RUST_BACKTRACE=1 cargo test --verbose --all --all-features -- --nocapture --skip kmc
cargo test --verbose --all --no-default-features --no-run
# cargo test --verbose --all-features
# cargo bench --verbose --all
#################
cargo run --example circuit_breaker --features="macros_multiple"
cargo run --example logging --features="macros_multiple"
cargo run --example circuit_breaker_baking --features="baking"
cargo run --example logging_baking --features="baking"
cargo run --example circuit_breaker_solo --features="baking_interleaved"
cargo run --example logging_solo --features="baking_interleaved"
cargo run --example circuit_breaker_logging_interleaved --features="baking_interleaved"
cargo run --example distributed_calc --features="macros_multiple"
cargo run --example dns_fowler --features="macros_multiple"
cargo run --example dns_imai --features="macros_multiple"
cargo run --example fib --features="macros_multiple"
cargo run --example o_auth --features="macros_multiple"
cargo run --example o_auth_2 --features="transport_macros_multiple"
cargo run --example online_wallet --features="macros_multiple"
cargo run --example simple_voting --features="macros_multiple"
cargo run --example smtp --features="macros_multiple"
cargo run --example three_buyers --features="macros_multiple"
cargo run --example travel_three --features="macros_multiple"
cargo run --example video_stream --features="macros_simple"
#################
echo "done"
