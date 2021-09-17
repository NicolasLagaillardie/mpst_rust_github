#!/bin/sh

# Publish all workspaces

# Exit if error
set -e

# mpst_seq
cd mpst_seq/mpst_seq_proc/
cargo publish --all-features
sleep 5
cd ..
cargo publish --all-features
sleep 5
cd ..

# mpst_seq_baking
cd mpst_seq_baking/mpst_seq_proc_baking/
cargo publish --all-features
sleep 5
cd ..
cargo publish --all-features
sleep 5
cd ..

# mpst_seq_checking
cd mpst_seq_checking/mpst_seq_proc_checking/
cargo publish --all-features
sleep 5
cd ..
cargo publish --all-features
sleep 5
cd ..

# mpst_seq_http
cd mpst_seq_http/mpst_seq_proc_http/
cargo publish --all-features
sleep 5
cd ..
cargo publish --all-features
sleep 5
cd ..

# mpst_seq_interleaved
cd mpst_seq_interleaved/mpst_seq_proc_interleaved/
cargo publish --all-features
sleep 5
cd ..
cargo publish --all-features
sleep 5
cd ..

# mpst_seq_macros_simple
cd mpst_seq_macros_simple/mpst_seq_proc_macros_simple/
cargo publish --all-features
sleep 5
cd ..
cargo publish --all-features
sleep 5
cd ..

# mpst_seq_macros_multiple
cd mpst_seq_macros_multiple/mpst_seq_proc_macros_multiple/
cargo publish --all-features
sleep 5
cd ..
cargo publish --all-features
sleep 5
cd ..

# final publish
cargo publish --all-features
echo "done"