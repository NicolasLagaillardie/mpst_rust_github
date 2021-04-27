#!/bin/bash

set -e

next=0

# Run over 100 times
for i in {201..1000}
do
    next=$(($i+1))
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' benches/ping_pong.rs
    rm -rf target/release/
    cargo bench --bench ping_pong -- --verbose
done

sed -ier 's/static SIZE: i64 = [0-9]\+;/static SIZE: i64 = 1;/g' benches/ping_pong.rs