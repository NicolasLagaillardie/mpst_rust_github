#!/bin/sh

# Stop upon any error
set -e

cargo test --tests --all-features --workspace # Test all tests
cargo test --examples --all-features --workspace # Test all examples
cargo test --benches --all-features --workspace # Test all benchmarks
cargo test --all-targets --all-features --workspace # Test everything in the library
./scripts/top_down.sh
./scripts/bottom_up.sh
./scripts/examples_literature.sh # Will take up to one hour, progress is displayed in the terminal
./scripts/lightweight_library.sh # Set up
./scripts/ping_pong_mesh_ring_light.sh # This will take around 1 hour
./scripts/full_library.sh # set up
# ./scripts/ping_pong_mesh_ring_full.sh # This will take more than 24 hours

## Mesh

### Six
./scripts/create_files/compile.sh mesh_six_binary 10 baking
./scripts/create_files/compile.sh mesh_six_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_six_baking 10 baking

### Seven
./scripts/create_files/compile.sh mesh_seven_binary 10 baking
./scripts/create_files/compile.sh mesh_seven_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_seven_baking 10 baking

### Eight
./scripts/create_files/compile.sh mesh_eight_binary 10 baking
./scripts/create_files/compile.sh mesh_eight_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_eight_baking 10 baking

### Nine
./scripts/create_files/compile.sh mesh_nine_binary 10 baking
./scripts/create_files/compile.sh mesh_nine_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_nine_baking 10 baking

### Ten
./scripts/create_files/compile.sh mesh_ten_binary 10 baking
./scripts/create_files/compile.sh mesh_ten_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_ten_baking 10 baking

### Eleven
./scripts/create_files/compile.sh mesh_eleven_binary 10 baking
./scripts/create_files/compile.sh mesh_eleven_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_eleven_baking 10 baking

### Twenty
./scripts/create_files/compile.sh mesh_twenty_binary 10 baking
./scripts/create_files/compile.sh mesh_twenty_crossbeam 10 baking
./scripts/create_files/compile.sh mesh_twenty_baking 10 baking

## Ring

### Six
./scripts/create_files/compile.sh ring_six_binary 10 baking
./scripts/create_files/compile.sh ring_six_crossbeam 10 baking
./scripts/create_files/compile.sh ring_six_baking 10 baking

### Seven
./scripts/create_files/compile.sh ring_seven_binary 10 baking
./scripts/create_files/compile.sh ring_seven_crossbeam 10 baking
./scripts/create_files/compile.sh ring_seven_baking 10 baking

### Eight
./scripts/create_files/compile.sh ring_eight_binary 10 baking
./scripts/create_files/compile.sh ring_eight_crossbeam 10 baking
./scripts/create_files/compile.sh ring_eight_baking 10 baking

### Nine
./scripts/create_files/compile.sh ring_nine_binary 10 baking
./scripts/create_files/compile.sh ring_nine_crossbeam 10 baking
./scripts/create_files/compile.sh ring_nine_baking 10 baking

### Ten
./scripts/create_files/compile.sh ring_ten_binary 10 baking
./scripts/create_files/compile.sh ring_ten_crossbeam 10 baking
./scripts/create_files/compile.sh ring_ten_baking 10 baking

### Eleven
./scripts/create_files/compile.sh ring_eleven_binary 10 baking
./scripts/create_files/compile.sh ring_eleven_crossbeam 10 baking
./scripts/create_files/compile.sh ring_eleven_baking 10 baking

### Twenty
./scripts/create_files/compile.sh ring_twenty_binary 10 baking
./scripts/create_files/compile.sh ring_twenty_crossbeam 10 baking
./scripts/create_files/compile.sh ring_twenty_baking 10 baking

./scripts/lightweight_library.sh # Set up
./scripts/ping_pong_mesh_ring_light.sh

# ./scripts/ping_pong.sh # For ping-pong protocols
# ##
./scripts/full_library.sh # set up
./scripts/mesh_full.sh # For all mesh protocols
./scripts/ring_full.sh # For all ring protocols
##
./scripts/lightweight_library.sh # Set up
./scripts/mesh_light.sh # For mesh protocols up to five participants
./scripts/ring_light.sh # For ring protocols up to five participants
./scripts/top_down_adder.sh
cargo run --example="adder_generated" --features=backing_checking
