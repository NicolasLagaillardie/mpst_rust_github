#!/bin/sh

# Check Rust code before pushing

set -e

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

cargo fmt --verbose --all -- --check
cargo clippy --all-features --verbose -- -D warnings
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
# find ./examples/. -type f -exec sh -c 'for example in "$@"; do (cargo run --example ${example:13:-3} --features="macros rand") done' argv0 {} +

cargo run --example actyx_os_api --features="macros rand"
cargo run --example actyx_os_logging --features="macros rand"
cargo run --example distributed_calc --features="macros rand"
cargo run --example dns_fowler --features="macros rand"
cargo run --example dns_imai --features="macros rand"
cargo run --example fib --features="macros rand"
cargo run --example o_auth --features="macros rand"
cargo run --example o_auth_2 --features="macros transport rand"
cargo run --example online_wallet --features="macros rand"
cargo run --example simple_voting --features="macros rand"
cargo run --example smtp --features="macros rand"
cargo run --example three_buyers --features="macros rand"
cargo run --example travel_three --features="macros rand"
cargo run --example video_stream --features="macros rand"

# cargo test --verbose --all
# cargo test --verbose --all -- --nocapture
RUST_BACKTRACE=1 cargo test --verbose --all --all-features -- --nocapture
# cargo test --verbose --all-features
# cargo bench --verbose --all
