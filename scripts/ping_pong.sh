#!/bin/bash

set -e

cargo bench --bench ping_pong -- --verbose

# for i in {1..600}
# do
#     next=$(($i+1))
#     cp benches/ping_pong_all/ping_pong_$i.rs benches/ping_pong_all/ping_pong_$next.rs
#     sync
#     sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_$next.rs
#     sync
#     printf 'pub mod ping_pong_'"$next"';\n' >> benches/ping_pong_all/mod.rs;
#     printf '\tping_pong_all::ping_pong_'"$next"'::ping_pong,\n' >> benches/ping_pong.rs;
#     # rm -rf target/release/
#     # cargo bench --bench ping_pong -- --verbose
# done

# printf '}' >> benches/ping_pong.rs;
# find benches/ping_pong_all/ -name *.rser -delete

# # sed -ier 's/static SIZE: i64 = [0-9]\+;/static SIZE: i64 = 1;/g' benches/ping_pong.rs