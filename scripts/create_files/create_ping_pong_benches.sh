#!/bin/bash

# Create the ping_pong files from benches/ping_pong_all, for i from 1 to arg

# Stop upon any error
set -e

start=1
end=1

if [ -z "$1" ]
then
    echo "No argument supplied"
    exit 2
elif [ -z "$2" ]
then
    start=1
    end=$1
else
    start=$1
    end=$2
fi

# Check if ping_pong bench in Cargo.toml
if ! grep -Fxq '######### Ping-Pong start' Cargo.toml
then
    echo "Error: ping_pong start bench section is not in Cargo.toml" 1>&2
    exit 2
elif ! grep -Fxq '######### Ping-Pong end' Cargo.toml
then
    echo "Error: ping_pong end bench section is not in Cargo.toml" 1>&2
    exit 3
fi

# Create folder if not present
mkdir -p benches/ping_pong_all/

# Delete previous ping-pong benches
rm -rf benches/ping_pong_all/*.rs

# Remove lines between "Ping-Pong start" and "Ping-Pong end" to Cargo.toml
sed -i '/^######### Ping-Pong start/,/^\######### Ping-Pong end/{/^######### Ping-Pong start/!{/^\######### Ping-Pong end/!d;};}' Cargo.toml

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
cat benches/ping_pong_all_save/ping_pong_crossbeam_1.rs > benches/ping_pong_all/ping_pong_crossbeam_1.rs
cat benches/ping_pong_all_save/ping_pong_binary_1.rs > benches/ping_pong_all/ping_pong_binary_1.rs
cat benches/ping_pong_all_save/ping_pong_mpst_1.rs > benches/ping_pong_all/ping_pong_mpst_1.rs
cat benches/ping_pong_all_save/ping_pong_baking_mpst_1.rs > benches/ping_pong_all/ping_pong_baking_mpst_1.rs
cat benches/ping_pong_all_save/ping_pong_baking_ampst_1.rs > benches/ping_pong_all/ping_pong_baking_ampst_1.rs
cat benches/ping_pong_all_save/ping_pong_baking_timed_1.rs > benches/ping_pong_all/ping_pong_baking_timed_1.rs
cat benches/ping_pong_all_save/ping_pong_cancel_1.rs > benches/ping_pong_all/ping_pong_cancel_1.rs
cat benches/ping_pong_all_save/ping_pong_cancel_broadcast_1.rs > benches/ping_pong_all/ping_pong_cancel_broadcast_1.rs

if [ $start != 1 ]
then
    echo "Updating starting files"

    # Update to start from correct $start
    cat benches/ping_pong_all/ping_pong_crossbeam_1.rs > benches/ping_pong_all/ping_pong_crossbeam_$start.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$start"';,g' benches/ping_pong_all/ping_pong_crossbeam_$start.rs
    cat benches/ping_pong_all/ping_pong_binary_1.rs > benches/ping_pong_all/ping_pong_binary_$start.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$start"';,g' benches/ping_pong_all/ping_pong_binary_$start.rs
    cat benches/ping_pong_all/ping_pong_mpst_1.rs > benches/ping_pong_all/ping_pong_mpst_$start.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$start"';,g' benches/ping_pong_all/ping_pong_mpst_$start.rs
    cat benches/ping_pong_all/ping_pong_baking_mpst_1.rs > benches/ping_pong_all/ping_pong_baking_mpst_$start.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$start"';,g' benches/ping_pong_all/ping_pong_baking_mpst_$start.rs
    cat benches/ping_pong_all/ping_pong_baking_ampst_1.rs > benches/ping_pong_all/ping_pong_baking_ampst_$start.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$start"';,g' benches/ping_pong_all/ping_pong_baking_ampst_$start.rs
    cat benches/ping_pong_all/ping_pong_baking_timed_1.rs > benches/ping_pong_all/ping_pong_baking_timed_$start.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$start"';,g' benches/ping_pong_all/ping_pong_baking_timed_$start.rs
    cat benches/ping_pong_all/ping_pong_cancel_1.rs > benches/ping_pong_all/ping_pong_cancel_$start.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$start"';,g' benches/ping_pong_all/ping_pong_cancel_$start.rs
    cat benches/ping_pong_all/ping_pong_cancel_broadcast_1.rs > benches/ping_pong_all/ping_pong_cancel_broadcast_$start.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$start"';,g' benches/ping_pong_all/ping_pong_cancel_broadcast_$start.rs

    # Add to Cargo.toml
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_crossbeam_"$start"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_crossbeam_"$start".rs'"\nrequired-features = ["'full'"],g' Cargo.toml
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_binary_"$start"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_binary_"$start".rs'"\nrequired-features = ["'full'"],g' Cargo.toml
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_mpst_"$start"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_mpst_"$start".rs'"\nrequired-features = ["'full'"],g' Cargo.toml
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_mpst_"$start"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_mpst_"$start".rs'"\nrequired-features = ["'full'"],g' Cargo.toml
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_ampst_"$start"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_ampst_"$start".rs'"\nrequired-features = ["'full'"],g' Cargo.toml
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_timed_"$start"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_timed_"$start".rs'"\nrequired-features = ["'full'"],g' Cargo.toml
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_cancel_"$start"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_cancel_"$start".rs'"\nrequired-features = ["'full'"],g' Cargo.toml
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_cancel_broadcast_"$start"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_cancel_broadcast_"$start".rs'"\nrequired-features = ["'full'"],g' Cargo.toml
fi

echo "Step 1/2 for bench files"

# Copy ping_pong benches i and create ping_pong benches i+1
for i in $(eval echo {$start..$end})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    next=$(($i+1))
    #########################
    echo -ne "Loop created: $next\n"
    ######################### Crossbeam
    cat benches/ping_pong_all/ping_pong_crossbeam_$i.rs > benches/ping_pong_all/ping_pong_crossbeam_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_crossbeam_$next.rs
    ######################### Binary
    cat benches/ping_pong_all/ping_pong_binary_$i.rs > benches/ping_pong_all/ping_pong_binary_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_binary_$next.rs
    ######################### MPST
    cat benches/ping_pong_all/ping_pong_mpst_$i.rs > benches/ping_pong_all/ping_pong_mpst_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_mpst_$next.rs
    ######################### Baking MPST
    cat benches/ping_pong_all/ping_pong_baking_mpst_$i.rs > benches/ping_pong_all/ping_pong_baking_mpst_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_mpst_$next.rs
    ######################### Baking AMPST
    cat benches/ping_pong_all/ping_pong_baking_ampst_$i.rs > benches/ping_pong_all/ping_pong_baking_ampst_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_ampst_$next.rs
    ######################### Baking ATMP
    cat benches/ping_pong_all/ping_pong_baking_timed_$i.rs > benches/ping_pong_all/ping_pong_baking_timed_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_timed_$next.rs
    ######################### Cancel
    cat benches/ping_pong_all/ping_pong_cancel_$i.rs > benches/ping_pong_all/ping_pong_cancel_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_cancel_$next.rs
    ######################### Cancel Broadcast
    cat benches/ping_pong_all/ping_pong_cancel_broadcast_$i.rs > benches/ping_pong_all/ping_pong_cancel_broadcast_$next.rs
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_cancel_broadcast_$next.rs
done

echo ''
echo "Step 2/2 for Cargo.toml"
echo ''

# Modify Cargo.toml
for i in $(eval echo {$start..$end})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    next=$(($i+1))
    #########################
    echo -ne "Loop created: $next\n"
    ######################### Crossbeam
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_crossbeam_"$next"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_crossbeam_"$next".rs'"\nrequired-features = ["'full'"]\n,g' Cargo.toml
    ######################### Binary
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_binary_"$next"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_binary_"$next".rs'"\nrequired-features = ["'full'"]\n,g' Cargo.toml
    ######################### MPST
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_mpst_"$next"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_mpst_"$next".rs'"\nrequired-features = ["'full'"]\n,g' Cargo.toml
    ######################### Baking MPST
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_mpst_"$next"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_mpst_"$next".rs'"\nrequired-features = ["'full'"]\n,g' Cargo.toml
    ######################### Baking AMPST
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_ampst_"$next"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_ampst_"$next".rs'"\nrequired-features = ["'full'"]\n,g' Cargo.toml
    ######################### Baking ATMP
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_timed_"$next"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_timed_"$next".rs'"\nrequired-features = ["'full'"]\n,g' Cargo.toml
    ######################### Cancel
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_cancel_"$next"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_cancel_"$next".rs'"\nrequired-features = ["'full'"]\n,g' Cargo.toml
    ######################### Cancel Broadcast
    sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_cancel_broadcast_"$next"'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_cancel_broadcast_"$next".rs'"\nrequired-features = ["'full'"]\n,g' Cargo.toml
done

find benches/ -name *.rser -delete
rm -rf Cargo.tomler

cargo fmt --all

echo ''
echo "done"
