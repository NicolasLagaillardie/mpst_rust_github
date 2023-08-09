#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

END=1

# Check if there is one argument at least
if [ -z "$1" ]
then
    echo "No argument supplied"
    exit 2
else
    END=$1
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

### Clean compiled files
cargo clean
date

# Create clean compile_time folder if it does not exist
mkdir -p compile_time/ping_pong/
rm -rf compile_time/ping_pong/*

# Create clean save ping_pong if it does not exist
mkdir -p examples/ping_pong/
rm -rf examples/ping_pong/*

# Remove lines between "Ping-Pong start" and "Ping-Pong end" to Cargo.toml
sed -i '/^######### Ping-Pong start/,/^\######### Ping-Pong end/{/^######### Ping-Pong start/!{/^\######### Ping-Pong end/!d;};}' Cargo.toml

# Copy from save
PATH_BENCH='examples/ping_pong_save'
cat $PATH_BENCH/ping_pong_crossbeam_1.rs > examples/ping_pong/ping_pong_crossbeam.rs
cat $PATH_BENCH/ping_pong_binary_1.rs > examples/ping_pong/ping_pong_binary.rs
cat $PATH_BENCH/ping_pong_mpst_1.rs > examples/ping_pong/ping_pong_mpst.rs
cat $PATH_BENCH/ping_pong_baking_mpst_1.rs > examples/ping_pong/ping_pong_baking_mpst.rs
cat $PATH_BENCH/ping_pong_baking_ampst_1.rs > examples/ping_pong/ping_pong_baking_ampst.rs
cat $PATH_BENCH/ping_pong_baking_timed_1.rs > examples/ping_pong/ping_pong_baking_timed.rs

# Add to Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'ping_pong_crossbeam'"\npath = "'examples/ping_pong/ping_pong_crossbeam.rs'"\nrequired-features = ["'default'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'ping_pong_binary'"\npath = "'examples/ping_pong/ping_pong_binary.rs'"\nrequired-features = ["'default'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'ping_pong_mpst'"\npath = "'examples/ping_pong/ping_pong_mpst.rs'"\nrequired-features = ["'macros_multiple'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'ping_pong_baking_mpst'"\npath = "'examples/ping_pong/ping_pong_baking_mpst.rs'"\nrequired-features = ["'baking'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'ping_pong_baking_ampst'"\npath = "'examples/ping_pong/ping_pong_baking_ampst.rs'"\nrequired-features = ["'baking'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'ping_pong_baking_timed'"\npath = "'examples/ping_pong/ping_pong_baking_timed.rs'"\nrequired-features = ["'baking_timed'"],g' Cargo.toml

# Copy ping_pong examples i and create ping_pong examples i+1
for i in $(eval echo {0..$END})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    NEXT=$(($i+1))
    # Modify content files
    ## Crossbeam
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' examples/ping_pong/ping_pong_crossbeam.rs
    ## Binary
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' examples/ping_pong/ping_pong_binary.rs
    ## MPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' examples/ping_pong/ping_pong_mpst.rs
    ## Baking MPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' examples/ping_pong/ping_pong_baking_mpst.rs
    ## Baking AMPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' examples/ping_pong/ping_pong_baking_ampst.rs
    ## Baking ATMP
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' examples/ping_pong/ping_pong_baking_timed.rs
    # Clean unusued files
    find examples/ -name *.rser -delete
    # Benchmark
    bash ./scripts/create_files/compile_full.sh ping_pong_crossbeam 10 default ping_pong_crossbeam_$NEXT
    bash ./scripts/create_files/compile_full.sh ping_pong_binary 10 default ping_pong_binary_$NEXT
    bash ./scripts/create_files/compile_full.sh ping_pong_mpst 10 macros_multiple ping_pong_mpst_$NEXT
    bash ./scripts/create_files/compile_full.sh ping_pong_baking_mpst 10 baking ping_pong_baking_mpst_$NEXT
    bash ./scripts/create_files/compile_full.sh ping_pong_baking_ampst 10 baking ping_pong_baking_ampst_$NEXT
    bash ./scripts/create_files/compile_full.sh ping_pong_baking_timed 10 baking_timed ping_pong_baking_timed_$NEXT
    
    # Clean built files
    cargo clean
    #
    echo -ne "Loop $NEXT done\n"
done

# Clean ping_pong folder
rm -rf examples/ping_pong/*

# Remove lines between "Ping-Pong start" and "Ping-Pong end" to Cargo.toml
sed -i '/^######### Ping-Pong start/,/^\######### Ping-Pong end/{/^######### Ping-Pong start/!{/^\######### Ping-Pong end/!d;};}' Cargo.toml

# Echo done
date
echo "Done"
