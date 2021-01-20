#!/bin/sh

set -e

# Remove previous benchmarks
rm -rf compile_time/$1*.txt

# Run over 70 times
for i in {1..70}
do
# Remove previous build
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
printf "$tt\n" >> compile_time/$1.txt
done