#!/bin/bash

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

## Clean
cargo clean
date

# Compile protocols

## Mesh
### Two
echo "Mesh two"
bash ./scripts/create_files/compile_full.sh example_mesh_two_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_two_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_two_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_two_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_two_baking_timed 10 baking_timed

### Three
echo "Mesh three"
bash ./scripts/create_files/compile_full.sh example_mesh_three_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_three_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_three_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_three_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_three_baking_timed 10 baking_timed

### Four
echo "Mesh four"
bash ./scripts/create_files/compile_full.sh example_mesh_four_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_four_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_four_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_four_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_four_baking_timed 10 baking_timed

### Five
echo "Mesh five"
bash ./scripts/create_files/compile_full.sh example_mesh_five_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_five_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_five_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_five_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_five_baking_timed 10 baking_timed

### Six
echo "Mesh six"
bash ./scripts/create_files/compile_full.sh example_mesh_six_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_six_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_six_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_six_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_six_baking_timed 10 baking_timed

### Seven
echo "Mesh seven"
bash ./scripts/create_files/compile_full.sh example_mesh_seven_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_seven_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_seven_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_seven_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_seven_baking_timed 10 baking_timed

### Eight
echo "Mesh eight"
bash ./scripts/create_files/compile_full.sh example_mesh_eight_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_eight_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_eight_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_eight_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_eight_baking_timed 10 baking_timed

### Nine
echo "Mesh nine"
bash ./scripts/create_files/compile_full.sh example_mesh_nine_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_nine_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_nine_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_nine_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_nine_baking_timed 10 baking_timed

### Ten
echo "Mesh ten"
bash ./scripts/create_files/compile_full.sh example_mesh_ten_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_ten_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_ten_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_ten_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_mesh_ten_baking_timed 10 baking_timed

## Ring
date

### Two
echo "Ring two"
bash ./scripts/create_files/compile_full.sh example_ring_two_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_two_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_two_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_two_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_two_baking_timed 10 baking_timed

### Three
echo "Ring three"
bash ./scripts/create_files/compile_full.sh example_ring_three_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_three_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_three_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_three_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_three_baking_timed 10 baking_timed

### Four
echo "Ring four"
bash ./scripts/create_files/compile_full.sh example_ring_four_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_four_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_four_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_four_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_four_baking_timed 10 baking_timed

### Five
echo "Ring five"
bash ./scripts/create_files/compile_full.sh example_ring_five_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_five_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_five_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_five_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_five_baking_timed 10 baking_timed

### Six
echo "Ring six"
bash ./scripts/create_files/compile_full.sh example_ring_six_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_six_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_six_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_six_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_six_baking_timed 10 baking_timed

### Seven
echo "Ring seven"
bash ./scripts/create_files/compile_full.sh example_ring_seven_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_seven_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_seven_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_seven_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_seven_baking_timed 10 baking_timed

### Eight
echo "Ring eight"
bash ./scripts/create_files/compile_full.sh example_ring_nine_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_nine_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_eight_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_eight_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_eight_baking_timed 10 baking_timed

### Nine
echo "Ring nine"
bash ./scripts/create_files/compile_full.sh example_ring_eight_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_eight_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_nine_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_nine_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_nine_baking_timed 10 baking_timed

### Ten
echo "Ring ten"
bash ./scripts/create_files/compile_full.sh example_ring_ten_binary 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_ten_crossbeam 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_ten_baking_mpst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_ten_baking_ampst 10 baking
bash ./scripts/create_files/compile_full.sh example_ring_ten_baking_timed 10 baking_timed
