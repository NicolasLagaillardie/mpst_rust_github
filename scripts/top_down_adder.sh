#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

cd ../scribble-java/
./scribble-dist/target/scribblec.sh -ip scribble-demos/scrib/fib/src -d scribble-demos/scrib/fib/src scribble-demos/scrib/fib/src/fib/Fib.scr -rustapi Adder Adder_generated
cd ..
mv scribble-java/Adder_generated.rs mpst_rust_github/examples/Adder_generated.rs
cd mpst_rust_github/
cat scripts/tpa.txt
cat scripts/tpa.txt > examples/Adder_generated.rs
cargo fmt