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
bash ./scripts/create_files/compile_full.sh mesh_two_binary 10 baking
bash ./scripts/create_files/compile_full.sh mesh_two_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh mesh_two_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_two_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_two_baking_atmp 10 baking_atmp

### Three
echo "Mesh three"
bash ./scripts/create_files/compile_full.sh mesh_three_binary 10 baking
bash ./scripts/create_files/compile_full.sh mesh_three_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh mesh_three_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_three_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_three_baking_atmp 10 baking_atmp

### Four
echo "Mesh four"
bash ./scripts/create_files/compile_full.sh mesh_four_binary 10 baking
bash ./scripts/create_files/compile_full.sh mesh_four_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh mesh_four_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_four_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_four_baking_atmp 10 baking_atmp

### Five
echo "Mesh five"
bash ./scripts/create_files/compile_full.sh mesh_five_binary 10 baking
bash ./scripts/create_files/compile_full.sh mesh_five_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh mesh_five_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_five_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_five_baking_atmp 10 baking_atmp

### Six
echo "Mesh six"
bash ./scripts/create_files/compile_full.sh mesh_six_binary 10 baking
bash ./scripts/create_files/compile_full.sh mesh_six_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh mesh_six_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_six_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_six_baking_atmp 10 baking_atmp

### Seven
echo "Mesh seven"
bash ./scripts/create_files/compile_full.sh mesh_seven_binary 10 baking
bash ./scripts/create_files/compile_full.sh mesh_seven_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh mesh_seven_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_seven_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_seven_baking_atmp 10 baking_atmp

### Eight
echo "Mesh eight"
bash ./scripts/create_files/compile_full.sh mesh_eight_binary 10 baking
bash ./scripts/create_files/compile_full.sh mesh_eight_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh mesh_eight_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_eight_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh mesh_eight_baking_atmp 10 baking_atmp

## Ring
date

### Two
echo "Ring two"
bash ./scripts/create_files/compile_full.sh ring_two_binary 10 baking
bash ./scripts/create_files/compile_full.sh ring_two_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh ring_two_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh ring_two_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh ring_two_baking_atmp 10 baking_atmp

### Three
echo "Ring three"
bash ./scripts/create_files/compile_full.sh ring_three_binary 10 baking
bash ./scripts/create_files/compile_full.sh ring_three_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh ring_three_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh ring_three_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh ring_three_baking_atmp 10 baking_atmp

### Four
echo "Ring four"
bash ./scripts/create_files/compile_full.sh ring_four_binary 10 baking
bash ./scripts/create_files/compile_full.sh ring_four_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh ring_four_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh ring_four_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh ring_four_baking_atmp 10 baking_atmp

### Five
echo "Ring five"
bash ./scripts/create_files/compile_full.sh ring_five_binary 10 baking
bash ./scripts/create_files/compile_full.sh ring_five_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh ring_five_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh ring_five_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh ring_five_baking_atmp 10 baking_atmp

### Six
echo "Ring six"
bash ./scripts/create_files/compile_full.sh ring_six_binary 10 baking
bash ./scripts/create_files/compile_full.sh ring_six_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh ring_six_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh ring_six_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh ring_six_baking_atmp 10 baking_atmp

### Seven
echo "Ring seven"
bash ./scripts/create_files/compile_full.sh ring_seven_binary 10 baking
bash ./scripts/create_files/compile_full.sh ring_seven_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh ring_seven_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh ring_seven_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh ring_seven_baking_atmp 10 baking_atmp

### Eight
echo "Ring eight"
bash ./scripts/create_files/compile_full.sh ring_nine_binary 10 baking
bash ./scripts/create_files/compile_full.sh ring_nine_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh ring_eight_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh ring_eight_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh ring_eight_baking_atmp 10 baking_atmp
