#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

cat scripts/vs_bu.txt > examples/video_stream_full.rs
cargo fmt
cargo run --example=video_stream_full --features="baking_checking"