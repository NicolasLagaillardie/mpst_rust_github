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

sleep 30m

### Clean compiled files
cargo clean
date

# Create clean save folder if it does not exist
mkdir -p save/ping_pong/
rm -rf save/ping_pong/*

# Create clean save ping_pong_all if it does not exist
mkdir -p benches/ping_pong_all/
rm -rf benches/ping_pong_all/*

# Remove lines between "Ping-Pong start" and "Ping-Pong end" to Cargo.toml
sed -i '/^######### Ping-Pong start/,/^\######### Ping-Pong end/{/^######### Ping-Pong start/!{/^\######### Ping-Pong end/!d;};}' Cargo.toml

# Copy from save
cat benches/ping_pong_all_save/ping_pong_crossbeam_1.rs > benches/ping_pong_all/ping_pong_crossbeam.rs
cat benches/ping_pong_all_save/ping_pong_binary_1.rs > benches/ping_pong_all/ping_pong_binary.rs
cat benches/ping_pong_all_save/ping_pong_mpst_1.rs > benches/ping_pong_all/ping_pong_mpst.rs
cat benches/ping_pong_all_save/ping_pong_baking_mpst_1.rs > benches/ping_pong_all/ping_pong_baking_mpst.rs
cat benches/ping_pong_all_save/ping_pong_baking_ampst_1.rs > benches/ping_pong_all/ping_pong_baking_ampst.rs
cat benches/ping_pong_all_save/ping_pong_baking_timed_1.rs > benches/ping_pong_all/ping_pong_baking_timed.rs

# Add to Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'bench_ping_pong_crossbeam'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_crossbeam.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'bench_ping_pong_binary'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_binary.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'bench_ping_pong_mpst'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_mpst.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'bench_ping_pong_baking_mpst'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_mpst.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'bench_ping_pong_baking_ampst'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_ampst.rs'"\nrequired-features = ["'full'"],g' Cargo.toml
sed -ier 's,######### Ping-Pong start,######### Ping-Pong start\n\n[[bench]]\nname = "'bench_ping_pong_baking_timed'"\nharness = false\npath = "'benches/ping_pong_all/ping_pong_baking_timed.rs'"\nrequired-features = ["'full'"],g' Cargo.toml

# Copy ping_pong benches i and create ping_pong benches i+1
for i in $(eval echo {0..$end})
do
    # prog "$((i/$(( $1 / 100 ))))" still working...
    next=$(($i+1))
    # Modify content files
    ## Crossbeam
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_crossbeam.rs
    ## Binary
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_binary.rs
    ## MPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_mpst.rs
    ## Baking MPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_mpst.rs
    ## Baking AMPST
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_ampst.rs
    ## Baking ATMP
    sed -ier 's,static LOOPS: i64 = [0-9]\+;,static LOOPS: i64 = '"$next"';,g' benches/ping_pong_all/ping_pong_baking_timed.rs
    # Clean unusued files
    find benches/ -name *.rser -delete
    # Benchmark
    cargo bench --bench="bench_ping_pong_*" --all-features -- --verbose
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
    echo -ne "Loop $next done\n"
done

# Clean ping_pong_all folder
rm -rf benches/ping_pong_all/*

# Remove lines between "Ping-Pong start" and "Ping-Pong end" to Cargo.toml
sed -i '/^######### Ping-Pong start/,/^\######### Ping-Pong end/{/^######### Ping-Pong start/!{/^\######### Ping-Pong end/!d;};}' Cargo.toml

# Echo done
date
echo "Done"
