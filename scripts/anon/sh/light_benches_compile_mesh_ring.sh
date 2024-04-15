#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -eou pipefail

## Clean
cargo clean
date

# Compile protocols

## Mesh
### Two
echo "Mesh two"
bash ./scripts/create_files/compile_full.sh mesh_two_baking_ampst 5 baking
bash ./scripts/create_files/compile_full.sh mesh_two_baking_atmp 5 baking_timed

### Three
echo "Mesh three"
bash ./scripts/create_files/compile_full.sh mesh_three_baking_ampst 5 baking
bash ./scripts/create_files/compile_full.sh mesh_three_baking_atmp 5 baking_timed

### Four
echo "Mesh four"
bash ./scripts/create_files/compile_full.sh mesh_four_baking_ampst 5 baking
bash ./scripts/create_files/compile_full.sh mesh_four_baking_atmp 5 baking_timed

## Ring
date

### Two
echo "Ring two"
bash ./scripts/create_files/compile_full.sh ring_two_baking_ampst 5 baking
bash ./scripts/create_files/compile_full.sh ring_two_baking_atmp 5 baking_timed

### Three
echo "Ring three"
bash ./scripts/create_files/compile_full.sh ring_three_baking_ampst 5 baking
bash ./scripts/create_files/compile_full.sh ring_three_baking_atmp 5 baking_timed

### Four
echo "Ring four"
bash ./scripts/create_files/compile_full.sh ring_four_baking_ampst 5 baking
bash ./scripts/create_files/compile_full.sh ring_four_baking_atmp 5 baking_timed
