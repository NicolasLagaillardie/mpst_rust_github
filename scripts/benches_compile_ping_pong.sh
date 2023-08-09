#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

end=1

# Check if there is one argument at least
if [ -z "$1" ]
then
    echo "No argument supplied"
    exit 2
else
    end=$1
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
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'example_ping_pong_crossbeam'"\npath = "'examples/ping_pong/ping_pong_crossbeam.rs'"\nrequired-features = ["'default'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'example_ping_pong_binary'"\npath = "'examples/ping_pong/ping_pong_binary.rs'"\nrequired-features = ["'default'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'example_ping_pong_mpst'"\npath = "'examples/ping_pong/ping_pong_mpst.rs'"\nrequired-features = ["'macros_multiple'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'example_ping_pong_baking_mpst'"\npath = "'examples/ping_pong/ping_pong_baking_mpst.rs'"\nrequired-features = ["'baking'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'example_ping_pong_baking_ampst'"\npath = "'examples/ping_pong/ping_pong_baking_ampst.rs'"\nrequired-features = ["'baking'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[example]]\nname = "'example_ping_pong_baking_timed'"\npath = "'examples/ping_pong/ping_pong_baking_timed.rs'"\nrequired-features = ["'baking_timed'"],g' Cargo.toml

# Copy ping_pong examples i and create ping_pong examples i+1
for i in $(eval echo {0..$end})
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
    bash ./scripts/create_files/compile_full.sh example_ping_pong_crossbeam 1 default
    bash ./scripts/create_files/compile_full.sh example_ping_pong_binary 1 default
    bash ./scripts/create_files/compile_full.sh example_ping_pong_mpst 1 macros_multiple
    bash ./scripts/create_files/compile_full.sh example_ping_pong_baking_mpst 1 baking
    bash ./scripts/create_files/compile_full.sh example_ping_pong_baking_ampst 1 baking
    bash ./scripts/create_files/compile_full.sh example_ping_pong_baking_timed 1 baking_timed
    # Clean unusued files
    rm -rf target/criterion/report/
    find . -name "*.svg" -delete
    find target/ -name "raw.csv" -delete
    find target/ -name "benchmark.json" -delete
    find target/ -name "tukey.json" -delete
    find target/ -name "index.html" -delete
    find target/ -name "sample.json" -delete
    # Move result to save folder
    mv -f target/criterion/* compile_time/ping_pong/
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
