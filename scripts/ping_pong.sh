#!/bin/bash

set -e

# progress bar function
prog() {
    local w=80 p=$1;  shift
    # create a string of spaces, then change them to dots
    printf -v dots "%*s" "$(( $p*$w/100 ))" ""; dots=${dots// /.};
    # print those dots on a fixed-width space plus the percentage etc. 
    printf "\r\e[K|%-*s| %3d %% %s" "$w" "$dots" "$p" "$*"; 
}

# cargo bench --bench ping_pong -- --verbose

sed -ier 's,},,g' benches/ping_pong.rs;

for i in {1..500}
do
    prog "$((i/5))" still working...
    #########################
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