#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

sleep 60s

cargo clean

## Compile and run examples
bash ./scripts/examples_affine_timed_literature.sh

cargo clean

## Compile mesh and ring
bash ./scripts/benches_compile_mesh_ring.sh

cargo clean

## Run esh and ring and ping-pong
bash ./scripts/benches_runtime_mesh_ring_ping_pong.sh