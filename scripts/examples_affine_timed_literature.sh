#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

## Compile basic examples
bash ./scripts/create_files/compile_normal.sh three_buyers 100 affine_timed
bash ./scripts/create_files/compile_normal.sh distributed_calc 100 affine_timed
bash ./scripts/create_files/compile_normal.sh three_travel 100 affine_timed
bash ./scripts/create_files/compile_normal.sh simple_voting 100 affine_timed
bash ./scripts/create_files/compile_normal.sh online_wallet 100 affine_timed
bash ./scripts/create_files/compile_normal.sh o_auth 100 affine_timed
bash ./scripts/create_files/compile_normal.sh smtp 100 affine_timed

## Compile timed examples
bash ./scripts/create_files/compile_normal.sh three_buyers_timed 100 affine_timed
bash ./scripts/create_files/compile_normal.sh distributed_calc_timed 100 affine_timed
bash ./scripts/create_files/compile_normal.sh three_travel_timed 100 affine_timed
bash ./scripts/create_files/compile_normal.sh simple_voting_timed 100 affine_timed
bash ./scripts/create_files/compile_normal.sh online_wallet_timed 100 affine_timed
bash ./scripts/create_files/compile_normal.sh o_auth_timed 100 affine_timed
bash ./scripts/create_files/compile_normal.sh smtp_timed 100 affine_timed

## Compile timed additional examples
bash ./scripts/create_files/compile_normal.sh remote_data 100 affine_timed
bash ./scripts/create_files/compile_normal.sh servo 100 affine_timed

## Run benchmarks
cargo clean

cargo bench --bench examples_baking --all-features -- --verbose
mv -f target/criterion/ save/criterion/examples/
cargo clean

cargo bench --bench examples_timed --all-features -- --verbose
mv -f target/criterion/ save/criterion/examples_timed/
cargo clean

## Concatenate all results in the results/benchmarks_main_from_literature.csv file
# python3 scripts/create_graphs/examples_literature_affine_timed.py

# cargo clean &&
# cargo bench --bench examples_baking --all-features -- --verbose &&
# mv -f target/criterion/ save/criterion/examples/ &&
# cargo clean &&
# cargo bench --bench examples_timed --all-features -- --verbose &&
# mv -f target/criterion/ save/criterion/examples_timed/ &&
# cargo clean