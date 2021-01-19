#!/bin/sh

set -e

# Get the name of the Rust file in example
# read -p 'Name of the example file: ' NAME

rm -rf target/release/
rm -rf target/debug/
rm -rf target/rls/
rm -rf target/.rustc_info.json

# Get time
ts=$(date +%s%N)

# Run command
cargo build --example=$1

# Get difference
tt=$((($(date +%s%N) - $ts)/1000))

# Output difference
printf "$1\n$tt\n" >> compile_time.txt