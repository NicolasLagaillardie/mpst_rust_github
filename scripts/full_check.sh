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
# echo "cargo check --workspace started"
# cargo check --all
# echo "cargo check --workspace completed"
# echo "cargo check --tests started"
# cargo check --tests
# echo "cargo check --tests completed"
# echo "cargo check --examples started"
# cargo check --examples
# echo "cargo check --examples completed"
# echo "cargo check --benches started"
# cargo check --benches
# echo "cargo check --benches completed"
# echo "cargo test --verbose --workspace started"
# cargo test --verbose --all
# echo "cargo test --verbose --workspace completed"
# echo "cargo test --verbose --workspace -- --nocapture started"
# cargo test --verbose --workspace -- --nocapture
# echo "cargo test --verbose --workspace -- --nocapture completed"
# echo "cargo test --verbose --workspace --no-default-features --no-run started"
# cargo test --verbose --workspace --no-default-features --no-run
# echo "cargo test --verbose --workspace --no-default-features --no-run completed"
# echo "cargo clippy started"
# cargo clippy
# echo "cargo clippy completed"
# echo "cargo doc started"
# cargo doc --verbose --workspace --all-features
# echo "cargo doc completed"
#################
cargo fmt --verbose --all -- --check
cargo clippy --workspace --all-features --all-targets --verbose -- -D warnings
cargo doc --all-features
RUST_BACKTRACE=1 cargo check --verbose --workspace --all-features
RUST_BACKTRACE=1 cargo check --examples --verbose --all-features
RUST_BACKTRACE=1 cargo check --tests --verbose --all-features 
RUST_BACKTRACE=1 cargo check --benches --verbose --all-features
RUST_BACKTRACE=1 cargo doc --verbose --workspace --all-features
# cargo build --verbose --workspace --all-features
# cargo build --examples --verbose --all-features
# cargo build --tests --verbose --all-features
# cargo build --benches --verbose --all-features
# cargo run --verbose --all-features
# find ./examples/. -type f -exec sh -c 'for example in "$@"; do (cargo run --example ${example:13:-3} --features="macros") done' argv0 {} +
#################
cargo check --lib --verbose --workspace --features="default"
cargo check --lib --verbose --workspace --features="macros_simple"
cargo check --lib --verbose --workspace --features="macros_multiple"
cargo check --lib --verbose --workspace --features="checking"
cargo check --lib --verbose --workspace --features="baking"
cargo check --lib --verbose --workspace --features="baking_interleaved"
cargo check --lib --verbose --workspace --features="transport_tcp"
cargo check --lib --verbose --workspace --features="transport_udp"
cargo check --lib --verbose --workspace --features="transport_http"
cargo check --lib --verbose --workspace --features="transport"
cargo check --lib --verbose --workspace --features="transport_macros_multiple"
cargo check --lib --verbose --workspace --features="full"
cargo check --lib --verbose --workspace --features="macros_multiple checking"
cargo check --lib --verbose --workspace --all-features
#################
cargo check --bins --verbose --workspace --features="default"
cargo check --bins --verbose --workspace --features="macros_simple"
cargo check --bins --verbose --workspace --features="macros_multiple"
cargo check --bins --verbose --workspace --features="checking"
cargo check --bins --verbose --workspace --features="baking"
cargo check --bins --verbose --workspace --features="baking_interleaved"
cargo check --bins --verbose --workspace --features="transport_tcp"
cargo check --bins --verbose --workspace --features="transport_udp"
cargo check --bins --verbose --workspace --features="transport_http"
cargo check --bins --verbose --workspace --features="transport"
cargo check --bins --verbose --workspace --features="transport_macros_multiple"
cargo check --bins --verbose --workspace --features="full"
cargo check --bins --verbose --workspace --features="macros_multiple checking"
cargo check --bins --verbose --workspace --all-features
#################
cargo check --tests --verbose --workspace --features="default"
cargo check --tests --verbose --workspace --features="macros_simple"
cargo check --tests --verbose --workspace --features="macros_multiple"
cargo check --tests --verbose --workspace --features="checking"
cargo check --tests --verbose --workspace --features="baking"
cargo check --tests --verbose --workspace --features="baking_interleaved"
cargo check --tests --verbose --workspace --features="transport_tcp"
cargo check --tests --verbose --workspace --features="transport_udp"
cargo check --tests --verbose --workspace --features="transport_http"
cargo check --tests --verbose --workspace --features="transport"
cargo check --tests --verbose --workspace --features="transport_macros_multiple"
cargo check --tests --verbose --workspace --features="full"
cargo check --tests --verbose --workspace --features="macros_multiple checking"
cargo check --tests --verbose --workspace --all-features
#################
cargo check --examples --verbose --workspace --features="default"
cargo check --examples --verbose --workspace --features="macros_simple"
cargo check --examples --verbose --workspace --features="macros_multiple"
cargo check --examples --verbose --workspace --features="checking"
cargo check --examples --verbose --workspace --features="baking"
cargo check --examples --verbose --workspace --features="baking_interleaved"
cargo check --examples --verbose --workspace --features="transport_tcp"
cargo check --examples --verbose --workspace --features="transport_udp"
cargo check --examples --verbose --workspace --features="transport_http"
cargo check --examples --verbose --workspace --features="transport"
cargo check --examples --verbose --workspace --features="transport_macros_multiple"
cargo check --examples --verbose --workspace --features="full"
cargo check --examples --verbose --workspace --features="macros_multiple checking"
cargo check --examples --verbose --workspace --all-features
#################
cargo check --benches --verbose --workspace --features="default"
cargo check --benches --verbose --workspace --features="macros_simple"
cargo check --benches --verbose --workspace --features="macros_multiple"
cargo check --benches --verbose --workspace --features="checking"
cargo check --benches --verbose --workspace --features="baking"
cargo check --benches --verbose --workspace --features="baking_interleaved"
cargo check --benches --verbose --workspace --features="transport_tcp"
cargo check --benches --verbose --workspace --features="transport_udp"
cargo check --benches --verbose --workspace --features="transport_http"
cargo check --benches --verbose --workspace --features="transport"
cargo check --benches --verbose --workspace --features="transport_macros_multiple"
cargo check --benches --verbose --workspace --features="full"
cargo check --benches --verbose --workspace --features="macros_multiple checking"
cargo check --benches --verbose --workspace --all-features
#################
# cargo test --verbose --all
# cargo test --verbose --workspace -- --nocapture
RUST_BACKTRACE=1 cargo test --verbose --workspace --all-features -- --nocapture
# RUST_BACKTRACE=1 cargo test --verbose --workspace --all-features -- --nocapture --skip kmc
cargo test --verbose --workspace --no-default-features --no-run
# cargo test --verbose --all-features
# cargo bench --verbose --all
cargo test --all-targets --workspace --all-features
#################
cargo run --example circuit_breaker --features="macros_multiple"
cargo run --example logging --features="macros_multiple"
cargo run --example circuit_breaker_baking --features="baking"
cargo run --example logging_baking --features="baking"
# cargo run --example circuit_breaker_solo --features="baking_interleaved"
# cargo run --example logging_solo --features="baking_interleaved"
# cargo run --example circuit_breaker_logging_interleaved --features="baking_interleaved"
cargo run --example distributed_calc --features="macros_multiple"
cargo run --example dns_fowler --features="baking"
cargo run --example dns_fowler_checking --features="baking_checking"
cargo run --example dns_imai --features="macros_multiple"
cargo run --example fib --features="macros_multiple"
cargo run --example o_auth --features="baking"
cargo run --example o_auth_checking --features="baking_checking"
cargo run --example o_auth_transport --features="transport_macros_multiple"
cargo run --example online_wallet --features="macros_multiple"
cargo run --example simple_voting --features="macros_multiple"
cargo run --example smtp --features="macros_multiple"
cargo run --example three_buyers --features="macros_multiple"
cargo run --example three_travel --features="macros_multiple"
cargo run --example video_stream --features="baking_checking"
#################
echo "done"
