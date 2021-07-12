#!/bin/bash

# Create the ping_pong files example/ping_pong, for i from 1 to arg

set -e

# progress bar function
prog() {
    local w=80 p=$1;  shift
    # create a string of spaces, then change them to dots
    printf -v dots "%*s" "$(( $p*$w/100 ))" ""; dots=${dots// /.};
    # print those dots on a fixed-width space plus the percentage etc. 
    printf "\r\e[K|%-*s| %3d %% %s" "$w" "$dots" "$p" "$*"; 
}

for i in $(eval echo {1..$1})
do
    prog "$((i/$(( $1 / 100 ))))" still working...
    #########################
    next=$(($i+1))
    cp example/ping_pong/ping_pong_binary_$i.rs example/ping_pong/ping_pong_binary_$next.rs
    sync
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' example/ping_pong/ping_pong_binary_$next.rs
    sync
    #########################
    next=$(($i+1))
    cp example/ping_pong/ping_pong_mpst_$i.rs example/ping_pong/ping_pong_mpst_$next.rs
    sync
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' example/ping_pong/ping_pong_mpst_$next.rs
    sync
    #########################
    next=$(($i+1))
    cp example/ping_pong/ping_pong_cancel_$i.rs example/ping_pong/ping_pong_cancel_$next.rs
    sync
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' example/ping_pong/ping_pong_cancel_$next.rs
    sync
    #########################
    next=$(($i+1))
    cp example/ping_pong/ping_pong_crossbeam_$i.rs example/ping_pong/ping_pong_crossbeam_$next.rs
    sync
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' example/ping_pong/ping_pong_crossbeam_$next.rs
    sync
    #########################
    next=$(($i+1))
    cp example/ping_pong/ping_pong_broadcast_cancel_$i.rs example/ping_pong/ping_pong_broadcast_cancel_$next.rs
    sync
    sed -ier 's,static SIZE: i64 = [0-9]\+;,static SIZE: i64 = '"$next"';,g' example/ping_pong/ping_pong_broadcast_cancel_$next.rs
    sync
done

find example/ping_pong/ -name *.rser -delete

cargo fmt