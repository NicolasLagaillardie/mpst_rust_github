#!/bin/bash

# Create the ping_pong files from benches/ping_pong_all, for i from 1 to arg

# Stop upon any error
set -e

# Delete previous ping-pong examples
rm -rf benches/ping_pong_all/*.rs

# progress bar function
# prog() {
#     local w=80 p=$1;  shift
#     # create a string of spaces, then change them to dots
#     printf -v dots "%*s" "$(( $p*$w/100 ))" ""; dots=${dots// /.};
#     # print those dots on a fixed-width space plus the percentage etc. 
#     printf "\r\e[K|%-*s| %3d %% %s" "$w" "$dots" "$p" "$*"; 
# }

# Copy from save
cat benches/ping_pong_all_save/mod.rs > benches/ping_pong_all/mod.rs
cat benches/ping_pong_all_save/ping_pong_1.rs > benches/ping_pong_all/ping_pong_1.rs
cat benches/ping_pong_all_save/ping_pong_baking_cancel_1.rs > benches/ping_pong_all/ping_pong_baking_cancel_1.rs
cat benches/ping_pong_all_save/ping_pong_cancel_1.rs > benches/ping_pong_all/ping_pong_cancel_1.rs
cat benches/ping_pong_all_save/ping_pong_cancel_broadcast_1.rs > benches/ping_pong_all/ping_pong_cancel_broadcast_1.rs
cat benches/ping_pong_all_save/ping_pong.rs > benches/ping_pong.rs

# Remove the last bracket in ping_pong.rs
sed -ier 's,}//,,g' benches/ping_pong.rs;

echo "Step 1/2"

for i in $(eval echo {1..$1})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    echo -ne "Loop created: $i\r"
    #########################
    next=$(($i+1))
    #########################
    cat benches/ping_pong_all/ping_pong_$i.rs > benches/ping_pong_all/ping_pong_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_$next.rs
    #########################
    cat benches/ping_pong_all/ping_pong_cancel_$i.rs > benches/ping_pong_all/ping_pong_cancel_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_cancel_$next.rs
    #########################
    cat benches/ping_pong_all/ping_pong_cancel_broadcast_$i.rs > benches/ping_pong_all/ping_pong_cancel_broadcast_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_cancel_broadcast_$next.rs
    #########################
    cat benches/ping_pong_all/ping_pong_baking_cancel_$i.rs > benches/ping_pong_all/ping_pong_baking_cancel_$next.rs && sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_cancel_$next.rs
done

echo ''
echo "Step 2/2"
echo ''

for i in $(eval echo {1..$1})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    echo -ne "Loop created: $i\r"
    #########################
    next=$(($i+1))
    #########################
    printf 'pub mod ping_pong_'"$next"';\n' >> benches/ping_pong_all/mod.rs;
    printf '\t\tping_pong_all::ping_pong_'"$next"'::ping_pong_protocol_mpst,\n' >> benches/ping_pong.rs;
    #########################
    printf '\t\tping_pong_all::ping_pong_'"$next"'::ping_pong_protocol_binary,\n' >> benches/ping_pong.rs;
    #########################
    printf '\t\tping_pong_all::ping_pong_'"$next"'::ping_pong_protocol_crossbeam,\n' >> benches/ping_pong.rs;
    #########################
    printf 'pub mod ping_pong_cancel_'"$next"';\n' >> benches/ping_pong_all/mod.rs;
    printf '\t\tping_pong_all::ping_pong_cancel_'"$next"'::ping_pong_protocol_mpst,\n' >> benches/ping_pong.rs;
    #########################
    printf 'pub mod ping_pong_cancel_broadcast_'"$next"';\n' >> benches/ping_pong_all/mod.rs;
    printf '\t\tping_pong_all::ping_pong_cancel_broadcast_'"$next"'::ping_pong_protocol_mpst,\n' >> benches/ping_pong.rs;
    #########################
    printf 'pub mod ping_pong_baking_cancel_'"$next"';\n' >> benches/ping_pong_all/mod.rs;
    printf '\t\tping_pong_all::ping_pong_baking_cancel_'"$next"'::ping_pong_protocol_mpst,\n' >> benches/ping_pong.rs;
done

printf '}' >> benches/ping_pong.rs;
find benches/ -name *.rser -delete

cargo fmt --all

echo ''
echo "done"
