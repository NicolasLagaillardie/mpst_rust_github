#!/bin/bash

set -e

# cargo bench --bench ping_pong -- --verbose

for i in {1..500}
do
    next=$(($i+1))
    cp benches/ping_pong_all/ping_pong_$i.rs benches/ping_pong_all/ping_pong_$next.rs
    sync
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_$next.rs
    sync
    printf 'pub mod ping_pong_'"$next"';\n' >> benches/ping_pong_all/mod.rs;
    printf '\tping_pong_all::ping_pong_'"$next"'::ping_pong,\n' >> benches/ping_pong.rs;
    #########################
    cp benches/ping_pong_all/ping_pong_cancel_$i.rs benches/ping_pong_all/ping_pong_cancel_$next.rs
    sync
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_cancel_$next.rs
    sync
    printf 'pub mod ping_pong_cancel_'"$next"';\n' >> benches/ping_pong_all/mod.rs;
    printf '\tping_pong_all::ping_pong_cancel_'"$next"'::ping_pong,\n' >> benches/ping_pong.rs;
    #########################
    cp benches/ping_pong_all/ping_pong_cancel_broadcast_$i.rs benches/ping_pong_all/ping_pong_cancel_broadcast_$next.rs
    sync
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_cancel_broadcast_$next.rs
    sync
    printf 'pub mod ping_pong_cancel_broadcast_'"$next"';\n' >> benches/ping_pong_all/mod.rs;
    printf '\tping_pong_all::ping_pong_cancel_broadcast_'"$next"'::ping_pong,\n' >> benches/ping_pong.rs;
    # rm -rf target/release/
    # cargo bench --bench ping_pong -- --verbose
done

printf '}' >> benches/ping_pong.rs;
find benches/ping_pong_all/ -name *.rser -delete

cargo fmt

# sed -ier 's/static SIZE: i64 = [0-9]\+;/static SIZE: i64 = 1;/g' benches/ping_pong.rs