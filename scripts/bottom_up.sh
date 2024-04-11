#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

cat scripts/vs_bu.txt > examples/video_stream_generated.rs
cargo fmt
cargo run --example=video_stream_generated --features="baking_checking"
