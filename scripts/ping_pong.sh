#!/bin/sh

set -e


# Run over 100 times
for i in {1..100}
do
    cargo bench --bench ping_pong -- --verbose 
    next=$(( $i + 1))
    sed -i -e "s/static SIZE: i64 = $i;/static SIZE: i64 = $next;/g" benches/benchmarks/ping_pong.rs
done