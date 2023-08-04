#!/bin/bash

# Create the ping_pong files examples/ping_pong, for i from 1 to arg

# Stop upon any error
set -e

# Delete previous ping-pong examples
rm -rf examples/ping_pong*.rs

# progress bar function
# prog() {
#     local w=80 p=$1;  shift
#     # create a string of spaces, then change them to dots
#     printf -v dots "%*s" "$(( $p*$w/100 ))" ""; dots=${dots// /.};
#     # print those dots on a fixed-width space plus the percentage etc.
#     printf "\r\e[K|%-*s| %3d %% %s" "$w" "$dots" "$p" "$*";
# }

# Copy from save
cat examples/ping_pong_save/ping_pong_binary_1.rs > examples/ping_pong_binary_1.rs
cat examples/ping_pong_save/ping_pong_mpst_1.rs > examples/ping_pong_mpst_1.rs
cat examples/ping_pong_save/ping_pong_cancel_1.rs > examples/ping_pong_cancel_1.rs
cat examples/ping_pong_save/ping_pong_crossbeam_1.rs > examples/ping_pong_crossbeam_1.rs
cat examples/ping_pong_save/ping_pong_broadcast_cancel_1.rs > examples/ping_pong_broadcast_cancel_1.rs

# Add the next loops
for i in $(eval echo {1..$1})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    echo -ne "Loop created: $i\r"
    #########################
    next=$(($i+1))
    #########################
    cat examples/ping_pong_binary_$i.rs > examples/ping_pong_binary_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' examples/ping_pong_binary_$next.rs
    #########################
    cat examples/ping_pong_mpst_$i.rs > examples/ping_pong_mpst_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' examples/ping_pong_mpst_$next.rs
    #########################
    cat examples/ping_pong_cancel_$i.rs > examples/ping_pong_cancel_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' examples/ping_pong_cancel_$next.rs
    #########################
    cat examples/ping_pong_crossbeam_$i.rs > examples/ping_pong_crossbeam_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' examples/ping_pong_crossbeam_$next.rs
    #########################
    cat examples/ping_pong_broadcast_cancel_$i.rs > examples/ping_pong_broadcast_cancel_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' examples/ping_pong_broadcast_cancel_$next.rs
    #########################
done

find examples/ping_pong/ -name *.rser -delete
find examples/ -name *.rser -delete

cargo fmt --all

echo ''
echo "done"
