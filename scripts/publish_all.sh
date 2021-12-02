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

# final publish
cargo publish --all-features
echo "done"