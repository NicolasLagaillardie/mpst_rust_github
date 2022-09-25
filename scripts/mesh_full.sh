#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

# Compile protocols

## Mesh
### Two
./scripts/create_files/compile_full.sh mesh_two_binary 10 baking
./scripts/create_files/compile_full.sh mesh_two_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_two_baking 10 baking

### Three
./scripts/create_files/compile_full.sh mesh_three_binary 10 baking
./scripts/create_files/compile_full.sh mesh_three_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_three_baking 10 baking

### Four
./scripts/create_files/compile_full.sh mesh_four_binary 10 baking
./scripts/create_files/compile_full.sh mesh_four_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_four_baking 10 baking

### Five
./scripts/create_files/compile_full.sh mesh_five_binary 10 baking
./scripts/create_files/compile_full.sh mesh_five_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_five_baking 10 baking

### Six
./scripts/create_files/compile_full.sh mesh_six_binary 10 baking
./scripts/create_files/compile_full.sh mesh_six_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_six_baking 10 baking

### Seven
./scripts/create_files/compile_full.sh mesh_seven_binary 10 baking
./scripts/create_files/compile_full.sh mesh_seven_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_seven_baking 10 baking

### Eight
./scripts/create_files/compile_full.sh mesh_eight_binary 10 baking
./scripts/create_files/compile_full.sh mesh_eight_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_eight_baking 10 baking

### Nine
./scripts/create_files/compile_full.sh mesh_nine_binary 10 baking
./scripts/create_files/compile_full.sh mesh_nine_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_nine_baking 10 baking

### Ten
./scripts/create_files/compile_full.sh mesh_ten_binary 10 baking
./scripts/create_files/compile_full.sh mesh_ten_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_ten_baking 10 baking

### Eleven
./scripts/create_files/compile_full.sh mesh_eleven_binary 10 baking
./scripts/create_files/compile_full.sh mesh_eleven_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_eleven_baking 10 baking

### Twenty
./scripts/create_files/compile_full.sh mesh_twenty_binary 10 baking
./scripts/create_files/compile_full.sh mesh_twenty_crossbeam 10 baking
./scripts/create_files/compile_full.sh mesh_twenty_baking 10 baking

# Run the benchmarks
cargo bench --bench mesh --features="baking" -- --verbose

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
python3 scripts/create_graphs/ping_pong_mesh_ring.py