#!/bin/bash

set -e

# Create folder if it does not exist
mkdir -p compile_time/

# Remove previous benchmarks
rm -rf compile_time/$1*.txt

cargo check --example=$1 --features="$3" || command_failed=1

END=$2

# Loop check
for (( i=1; i<=$END; i++ ))
do
    # Remove previous build
    cargo clean
    # Get time
    ts=$(date +%s%N)
    # Run command
    cargo check --example=$1 --features="$3"
    # Get difference in ms
    tt=$((($(date +%s%N) - $ts)/1000))
    # Output difference
    printf "check; $tt\n" >> compile_time/$1.txt
done

# Loop build
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

# Loop build --release
for (( i=1; i<=$END; i++ ))
do
    # Remove previous build
    cargo clean
    # Get time
    ts=$(date +%s%N)
    # Run command
    cargo build --release --example=$1 --features="$3"
    # Get difference
    tt=$((($(date +%s%N) - $ts)/1000))
    # Output difference
    printf "release; $tt\n" >> compile_time/$1.txt
done
# fi
