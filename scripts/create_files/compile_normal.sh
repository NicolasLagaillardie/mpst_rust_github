#!/bin/bash

set -eou pipefail

# Remove previous benchmarks
rm -rf compile_time/$1.txt

cargo check --example=$1 --features="$3" || command_failed=1

END=$2

# Loop
for (( i=1; i<=$END; i++ ))
do
    # Remove previous build
    cargo clean
    # Get time
    ts=$(date +%s%N)
    # Run command
    cargo build --example=$1 --features="$3"
    # Get difference
    tt=$((($(date +%s%N) - $ts)/1000))
    # Output difference
    printf "build; $tt\n" >> compile_time/$1.txt
done
