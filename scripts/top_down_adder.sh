#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

cd ../scribble-java/
./scribble-dist/target/scribblec.sh -ip scribble-demos/scrib/fib/src -d scribble-demos/scrib/fib/src scribble-demos/scrib/fib/src/fib/Fib.scr -rustapi Adder adder_generated > /dev/null
cd ..
mv scribble-java/adder_generated.rs mpst_rust_github/examples/adder_generated.rs
cd mpst_rust_github/
cat scripts/tpa.txt > examples/adder_generated.rs
cargo fmt