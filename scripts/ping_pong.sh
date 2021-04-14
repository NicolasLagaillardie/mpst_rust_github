#!/bin/bash

set -e

next=0

# Run over 100 times
for i in {1..100}
do
    cargo bench --bench ping_pong -- --verbose 
    next=$(($i+1))
    sed -i -e "s/static SIZE: i64 = $i;/static SIZE: i64 = $next;/g" benches/ping_pong.rs
    rm -rf target/release/
done

sed -i -e "s/static SIZE: i64 = $next;/static SIZE: i64 = 1;/g" benches/ping_pong.rs