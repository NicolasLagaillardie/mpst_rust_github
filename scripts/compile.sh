#!/bin/sh

set -e

# Remove previous benchmarks
rm -rf compile_time/$1*.txt

# Run over 100 times
for i in {1..5}
do
    # Remove previous build
    rm -rf target/*
    # rm -rf target/debug/
    # rm -rf target/doc/
    # rm -rf target/release/
    # rm -rf target/rls/
    # rm -rf target/.rustc_info.json
    # rm -rf target/.rustdoc_fingerprint
    # Get time
    ts=$(date +%s%N)
    # Run command
    cargo build --example=$1
    # Get difference
    tt=$((($(date +%s%N) - $ts)/1000))
    # Output difference
    printf "build; $tt\n" >> compile_time/$1.txt
done

# Run over 100 times
for i in {1..5}
do
    # Remove previous build
    rm -rf target/*
    # rm -rf target/debug/
    # rm -rf target/doc/
    # rm -rf target/release/
    # rm -rf target/rls/
    # rm -rf target/.rustc_info.json
    # rm -rf target/.rustdoc_fingerprint
    # Get time
    ts=$(date +%s%N)
    # Run command
    cargo build --release --example=$1
    # Get difference
    tt=$((($(date +%s%N) - $ts)/1000))
    # Output difference
    printf "release; $tt\n" >> compile_time/$1.txt
done

# Run over 100 times
for i in {1..5}
do
    # Remove previous build
    rm -rf target/*
    # rm -rf target/debug/
    # rm -rf target/doc/
    # rm -rf target/release/
    # rm -rf target/rls/
    # rm -rf target/.rustc_info.json
    # rm -rf target/.rustdoc_fingerprint
    # Get time
    ts=$(date +%s%N)
    # Run command
    cargo check --example=$1
    # Get difference
    tt=$((($(date +%s%N) - $ts)/1000))
    # Output difference
    printf "check; $tt\n" >> compile_time/$1.txt
done
