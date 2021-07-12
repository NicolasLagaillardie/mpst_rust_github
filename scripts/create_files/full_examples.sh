#!/bin/sh

# Run all the examples

set -e

find ./examples/. -type f -exec sh -c 'for example in "$@"; do (cargo run --example ${example:13:-3}) done' argv0 {} +