#!/bin/sh

# Create the compilation time and benchmarks files for the examples

# Stop upon any error
set -e

## Clean
cargo clean

# Compile protocols

## Mesh
### Two
echo "Mesh two"
./scripts/create_files/compile_normal.sh mesh_two_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_two_baking_timed 10 baking_timed

### Three
echo "Mesh three"
./scripts/create_files/compile_normal.sh mesh_three_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_three_baking_timed 10 baking_timed

### Four
echo "Mesh four"
./scripts/create_files/compile_normal.sh mesh_four_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_four_baking_timed 10 baking_timed

### Five
echo "Mesh five"
./scripts/create_files/compile_normal.sh mesh_five_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_five_baking_timed 10 baking_timed

### Six
echo "Mesh six"
./scripts/create_files/compile_normal.sh mesh_six_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_six_baking_timed 10 baking_timed

### Seven
echo "Mesh seven"
./scripts/create_files/compile_normal.sh mesh_seven_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_seven_baking_timed 10 baking_timed

### Eight
echo "Mesh eight"
./scripts/create_files/compile_normal.sh mesh_eight_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_eight_baking_timed 10 baking_timed

### Nine
echo "Mesh nine"
./scripts/create_files/compile_normal.sh mesh_nine_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_nine_baking_timed 10 baking_timed

### Ten
echo "Mesh ten"
./scripts/create_files/compile_normal.sh mesh_ten_baking 10 baking
./scripts/create_files/compile_normal.sh mesh_ten_baking_timed 10 baking_timed

## Ring
### Two
echo "Ring two"
./scripts/create_files/compile_normal.sh ring_two_baking 10 baking
./scripts/create_files/compile_normal.sh ring_two_baking_timed 10 baking_timed

### Three
echo "Ring three"
./scripts/create_files/compile_normal.sh ring_three_baking 10 baking
./scripts/create_files/compile_normal.sh ring_three_baking_timed 10 baking_timed

### Four
echo "Ring four"
./scripts/create_files/compile_normal.sh ring_four_baking 10 baking
./scripts/create_files/compile_normal.sh ring_four_baking_timed 10 baking_timed

### Five
echo "Ring five"
./scripts/create_files/compile_normal.sh ring_five_baking 10 baking
./scripts/create_files/compile_normal.sh ring_five_baking_timed 10 baking_timed

### Six
echo "Ring six"
./scripts/create_files/compile_normal.sh ring_six_baking 10 baking
./scripts/create_files/compile_normal.sh ring_six_baking_timed 10 baking_timed

### Seven
echo "Ring seven"
./scripts/create_files/compile_normal.sh ring_seven_baking 10 baking
./scripts/create_files/compile_normal.sh ring_seven_baking_timed 10 baking_timed

### Eight
echo "Ring eight"
./scripts/create_files/compile_normal.sh ring_eight_baking 10 baking
./scripts/create_files/compile_normal.sh ring_eight_baking_timed 10 baking_timed

### Nine
echo "Ring nine"
./scripts/create_files/compile_normal.sh ring_nine_baking 10 baking
./scripts/create_files/compile_normal.sh ring_nine_baking_timed 10 baking_timed

### Ten
echo "Ring ten"
./scripts/create_files/compile_normal.sh ring_ten_baking 10 baking
./scripts/create_files/compile_normal.sh ring_ten_baking_timed 10 baking_timed

### Clean compiled files
cargo clean

# Run the affine ring benchmarks
echo "Ring affine bench"
cargo bench --bench="ring_affine" --features="affine_timed" -- --verbose
mv target/criterion/ save/criterion/affine_ring/
cargo clean

# Run the affine meshbenchmarks
echo "Mesh affine bench"
cargo bench --bench="mesh_affine" --features="affine_timed" -- --verbose
mv target/criterion/ save/criterion/affine_mesh/
cargo clean

# Run the timed ring benchmarks
echo "Ring timed bench"
cargo bench --bench="ring_timed" --features="affine_timed" -- --verbose
mv target/criterion/ save/criterion/timed_ring/
cargo clean

# Run the timed meshbenchmarks
echo "Mesh timed bench"
cargo bench --bench="mesh_timed" --features="affine_timed" -- --verbose
mv target/criterion/ save/criterion/timed_mesh/
cargo clean

## Concatenate all results in the results/ping_pong_mesh_ring.csv file
python3 scripts/create_graphs/timed_affine_mesh_ring.py