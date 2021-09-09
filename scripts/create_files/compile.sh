#!/bin/sh

set -e

# Remove previous benchmarks
rm -rf compile_time/$1*.txt

cargo check --example=$1 --features="$3" || command_failed=1

if [ ${command_failed:-0} -ne 1 ]
then
    # Loop
    for i in {1..$2}
    do
        # Remove previous build
        cargo clean
        # Get time
        ts=$(date +%s%N)
        # Run command
        cargo check --example=$1 --features="$3" 
        # Get difference
        tt=$((($(date +%s%N) - $ts)/1000))
        # Output difference
        printf "check; $tt\n" >> compile_time/$1.txt
    done

    # Loop
    for i in {1..$2}
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

    # RLoop
    for i in {1..$2}
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
fi
