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
cat benches/ping_pong_all_save/ping_pong_baking_mpst_1.rs > benches/ping_pong_all/ping_pong_baking_mpst_1.rs
cat benches/ping_pong_all_save/ping_pong_baking_ampst_1.rs > benches/ping_pong_all/ping_pong_baking_ampst_1.rs
cat benches/ping_pong_all_save/ping_pong_cancel_1.rs > benches/ping_pong_all/ping_pong_cancel_1.rs
cat benches/ping_pong_all_save/ping_pong_cancel_broadcast_1.rs > benches/ping_pong_all/ping_pong_cancel_broadcast_1.rs
cat benches/ping_pong_all_save/ping_pong.rs > benches/ping_pong.rs

# # Remove the last bracket in ping_pong.rs
# sed -ier 's,}//,,g' benches/ping_pong.rs;

echo "Step 1/2"

# Copy ping_pong benches i and create ping_pong benches i+1
for i in $(eval echo {1..$1})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    echo -ne "Loop created: $i\n"
    #########################
    next=$(($i+1))
    #########################
    cat benches/ping_pong_all/ping_pong_$i.rs > benches/ping_pong_all/ping_pong_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_$next.rs
    #########################
    cat benches/ping_pong_all/ping_pong_cancel_$i.rs > benches/ping_pong_all/ping_pong_cancel_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_cancel_$next.rs
    #########################
    cat benches/ping_pong_all/ping_pong_cancel_broadcast_$i.rs > benches/ping_pong_all/ping_pong_cancel_broadcast_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_cancel_broadcast_$next.rs
    #########################
    cat benches/ping_pong_all/ping_pong_baking_mpst_$i.rs > benches/ping_pong_all/ping_pong_baking_mpst_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_mpst_$next.rs
    #########################
    cat benches/ping_pong_all/ping_pong_baking_ampst_$i.rs > benches/ping_pong_all/ping_pong_baking_ampst_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_ampst_$next.rs
done

echo ''
echo "Step 2/2"
echo ''

# Create ping_pong_mod_(i+1) from ping_pong_mod_i 
for i in $(eval echo {1..$1})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    echo -ne "Loop created: $i\n"
    #########################
    next=$(($i+1))
    #########################
    cat benches/ping_pong_mod_$i.rs > benches/ping_pong_mod_$next.rs
    #########################
    sed -ier 's,name = ping_pong_[0-9]\+;,name = ping_pong_'"$next"';,g' benches/ping_pong_mod_$next.rs
    #########################
    sed -ier 's,ping_pong_all::ping_pong_[0-9]\+::ping_pong_protocol_mpst,ping_pong_all::ping_pong_'"$next"'::ping_pong_protocol_mpst,g' benches/ping_pong_mod_$next.rs
    #########################
    sed -ier 's,ping_pong_all::ping_pong_[0-9]\+::ping_pong_protocol_binary,ping_pong_all::ping_pong_'"$next"'::ping_pong_protocol_binary,g' benches/ping_pong_mod_$next.rs
    #########################
    sed -ier 's,ping_pong_all::ping_pong_[0-9]\+::ping_pong_protocol_crossbeam,ping_pong_all::ping_pong_'"$next"'::ping_pong_protocol_crossbeam,g' benches/ping_pong_mod_$next.rs
    #########################
    sed -ier 's,ping_pong_all::ping_pong_baking_mpst_[0-9]\+::ping_pong_protocol_mpst,ping_pong_all::ping_pong_baking_mpst_'"$next"'::ping_pong_protocol_mpst,g' benches/ping_pong_mod_$next.rs
    #########################
    sed -ier 's,ping_pong_all::ping_pong_baking_ampst_[0-9]\+::ping_pong_protocol_ampst,ping_pong_all::ping_pong_baking_ampst_'"$next"'::ping_pong_protocol_ampst,g' benches/ping_pong_mod_$next.rs
    #########################
    sed -ier 's,ping_pong_all::ping_pong_cancel_[0-9]\+::ping_pong_protocol_mpst,ping_pong_all::ping_pong_cancel_'"$next"'::ping_pong_protocol_mpst,g' benches/ping_pong_mod_$next.rs
    #########################
    sed -ier 's,ping_pong_all::ping_pong_cancel_broadcast_[0-9]\+::ping_pong_protocol_mpst,ping_pong_all::ping_pong_cancel_broadcast_'"$next"'::ping_pong_protocol_mpst,g' benches/ping_pong_mod_$next.rs    
done

printf '}' >> benches/ping_pong.rs;
find benches/ -name *.rser -delete

cargo fmt --all

echo ''
echo "done"
