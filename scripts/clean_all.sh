#!/bin/sh

# clean all workspace

# Exit if error
set -eou pipefail

cargo clean

for d in *; do
  if [ -d "$d" ]; then         # or:  if test -d "$d"; then
    ( cd "$d" && cargo clean )
  fi
done
