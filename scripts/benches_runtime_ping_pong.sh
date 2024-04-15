#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

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

# Create clean save folder if it does not exist
mkdir -p save/ping_pong/
rm -rf save/ping_pong/*

# Create clean save ping_pong if it does not exist
mkdir -p benches/ping_pong/
rm -rf benches/ping_pong/*

# Remove lines between "Ping-Pong start" and "Ping-Pong end" to Cargo.toml
sed -i '/^######### Ping-Pong start/,/^\######### Ping-Pong end/{/^######### Ping-Pong start/!{/^\######### Ping-Pong end/!d;};}' Cargo.toml

# Copy from save
PATH_BENCH='benches/ping_pong_save'
cat $PATH_BENCH/ping_pong_crossbeam_1.rs > benches/ping_pong/ping_pong_crossbeam.rs
cat $PATH_BENCH/ping_pong_binary_1.rs > benches/ping_pong/ping_pong_binary.rs
cat $PATH_BENCH/ping_pong_mpst_1.rs > benches/ping_pong/ping_pong_mpst.rs
cat $PATH_BENCH/ping_pong_baking_mpst_1.rs > benches/ping_pong/ping_pong_baking_mpst.rs
cat $PATH_BENCH/ping_pong_baking_ampst_1.rs > benches/ping_pong/ping_pong_baking_ampst.rs
cat $PATH_BENCH/ping_pong_baking_atmp_1.rs > benches/ping_pong/ping_pong_baking_atmp.rs

# Add to Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_crossbeam'"\nharness = false\npath = "'benches/ping_pong/ping_pong_crossbeam.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_binary'"\nharness = false\npath = "'benches/ping_pong/ping_pong_binary.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_mpst'"\nharness = false\npath = "'benches/ping_pong/ping_pong_mpst.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_mpst'"\nharness = false\npath = "'benches/ping_pong/ping_pong_baking_mpst.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_ampst'"\nharness = false\npath = "'benches/ping_pong/ping_pong_baking_ampst.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'ping_pong_baking_atmp'"\nharness = false\npath = "'benches/ping_pong/ping_pong_baking_atmp.rs'"\nrequired-features = ["'full'"],g' Cargo.toml

# Copy ping_pong benches i and create ping_pong benches i+1
for i in $(eval echo {0..$END})
do

    echo "Test"

    # prog "$((i/$(( $1 / 100 ))))" still working...
    NEXT=$(($i+1))
    # Modify content files
    ## Crossbeam
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' benches/ping_pong/ping_pong_crossbeam.rs
    ## Binary
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' benches/ping_pong/ping_pong_binary.rs
    ## MPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' benches/ping_pong/ping_pong_mpst.rs
    ## Baking MPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' benches/ping_pong/ping_pong_baking_mpst.rs
    ## Baking AMPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' benches/ping_pong/ping_pong_baking_ampst.rs
    ## Baking ATMP
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$NEXT"';,g' benches/ping_pong/ping_pong_baking_atmp.rs
    # Clean unusued files
    find benches/ -name *.rser -delete
    # Benchmark
    cargo bench --bench="ping_pong_*" --all-features -- --verbose
    # Clean unusued files
    rm -rf target/criterion/report/
    find . -name "*.svg" -delete
    find target/ -name "raw.csv" -delete
    find target/ -name "benchmark.json" -delete
    find target/ -name "tukey.json" -delete
    find target/ -name "index.html" -delete
    find target/ -name "sample.json" -delete
    # Move result to save folder
    mv -f target/criterion/* save/ping_pong/
    # Clean built files
    cargo clean
    #
    echo -ne "Loop $NEXT done\n"
done

# Clean ping_pong folder
rm -rf benches/ping_pong/*

# Remove lines between "Ping-Pong start" and "Ping-Pong end" to Cargo.toml
sed -i '/^######### Ping-Pong start/,/^\######### Ping-Pong end/{/^######### Ping-Pong start/!{/^\######### Ping-Pong end/!d;};}' Cargo.toml

# Echo done
date
echo "Done"
