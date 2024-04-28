#!/bin/sh

# counts instructions for a standard workload

# Exit if error
set -eou pipefail

# Get the name of the Rust file in example
read -p 'Name of the example file: ' NAME

# Name of the output cachegrind file
OUTFILE="cachegrind/cachegrind.$NAME.`git describe --always --dirty`-`date +%s`"

# Remove previous bin of the Rust file
rm target/release/examples/$NAME || true
rm target/release/examples/$NAME*.d || true

# Build the Rust file
cargo build \
  --example=$NAME \
  --release

# Run Valgrind on the binary and output the requested file
valgrind \
  --tool=cachegrind \
  --cachegrind-out-file="$OUTFILE" \
  ./target/release/examples/$NAME

# Style of the comparison
echo "--------------------------------------------------------------------------------"
echo "change since last run:"
echo "         Ir   I1mr  ILmr          Dr    D1mr    DLmr          Dw    D1mw    DLmw"
echo "--------------------------------------------------------------------------------"

# Command to get the last cachegrind file
LAST=`ls -t cachegrind/cachegrind.$NAME.* | sed -n 2p`

# Run cg_diff and save file
TEST=$(cg_diff $LAST $OUTFILE | tail -1)
printf "$OUTFILE\n$TEST\n" >> cg_diff.txt
