#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

cd ..
cd scribble-java/
./scribble-dist/target/scribblec.sh -ip scribble-demos/scrib/fib/src -d scribble-demos/scrib/fib/src scribble-demos/scrib/fib/src/fib/Fib.scr -rustapi VideoStreamingProtocol video_stream_generated > /dev/null
cd ..
mv scribble-java/video_stream_generated.rs mpst_rust_github/examples/video_stream_generated.rs
cd mpst_rust_github/
cat scripts/vs_td.txt > examples/video_stream_generated.rs
cargo fmt
cargo run --example=video_stream_generated --features="baking"
