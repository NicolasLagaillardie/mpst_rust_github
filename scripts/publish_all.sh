#!/bin/sh

# Publish all workspaces

# Exit if error
set -e

# mpst_seq_proc
cd mpst_seq_proc/
cargo publish --all-features --dry-run
cargo publish --all-features
sleep 5
cd ..

# mpst_seq
cd mpst_seq/
cargo publish --all-features --dry-run
cargo publish --all-features
sleep 5
cd ..

# final publish
cargo publish --all-features --dry-run
cargo publish --all-features
echo "done"