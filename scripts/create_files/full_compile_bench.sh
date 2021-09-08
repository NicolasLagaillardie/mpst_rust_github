#!/bin/bash

# Create the compilation time and benchmarks files for the examples

set -e

sleep 5

### Basic examples
./scripts/create_files/compile.sh o_auth 1 macros_multiple
./scripts/create_files/compile.sh o_auth_2 1 transport_macros_multiple
./scripts/create_files/compile.sh fib 1 macros_multiple
./scripts/create_files/compile.sh simple_voting 1 macros_multiple
./scripts/create_files/compile.sh three_buyer 1 macros_multiple
./scripts/create_files/compile.sh travel_three 1 macros_multiple
./scripts/create_files/compile.sh actyx_os_logging 1 macros_multiple
./scripts/create_files/compile.sh actyx_os_api 1 macros_multiple
./scripts/create_files/compile.sh smtp 1 macros_multiple
./scripts/create_files/compile.sh distributed_calc 1 macros_multiple
./scripts/create_files/compile.sh video_stream 1 macros_simple
./scripts/create_files/compile.sh online_wallet 1 macros_multiple
./scripts/create_files/compile.sh dns_fowler 1 macros_multiple
./scripts/create_files/compile.sh dns_imai 1 macros_multiple

### Ping-pong
for i in $(eval echo {1..1})
do
    ./scripts/create_files/compile.sh ping_pong_binary_$i 1 macros_multiple
    ./scripts/create_files/compile.sh ping_pong_cancel_$i 1 macros_multiple
    ./scripts/create_files/compile.sh ping_pong_broadcast_cancel_$i 1 macros_multiple
    ./scripts/create_files/compile.sh ping_pong_crossbeam_$i 1 macros_multiple
    ./scripts/create_files/compile.sh ping_pong_mpst_$i 1 macros_multiple
done

### Mesh
# Two
./scripts/create_files/compile.sh mesh_two_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_two_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_three_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_two_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_two_mpst 1 macros_multiple

# Three
./scripts/create_files/compile.sh mesh_three_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_three_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_three_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_three_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_three_mpst 1 macros_multiple

# Four
./scripts/create_files/compile.sh mesh_four_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_four_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_four_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_four_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_four_mpst 1 macros_multiple

# Five
./scripts/create_files/compile.sh mesh_five_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_five_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_five_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_five_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_five_mpst 1 macros_multiple

# Six
./scripts/create_files/compile.sh mesh_six_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_six_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_six_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_six_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_six_mpst 1 macros_multiple

# Seven
./scripts/create_files/compile.sh mesh_seven_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_seven_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_seven_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_seven_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_seven_mpst 1 macros_multiple

# Eight
./scripts/create_files/compile.sh mesh_eight_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_eight_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_eight_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_eight_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_eight_mpst 1 macros_multiple

# Nine
./scripts/create_files/compile.sh mesh_nine_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_nine_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_nine_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_nine_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_nine_mpst 1 macros_multiple

# Ten
./scripts/create_files/compile.sh mesh_ten_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_ten_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_ten_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_ten_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_ten_mpst 1 macros_multiple

# Eleven
./scripts/create_files/compile.sh mesh_eleven_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_eleven_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_eleven_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_eleven_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_eleven_mpst 1 macros_multiple

# Twenty
./scripts/create_files/compile.sh mesh_twenty_binary 1 macros_multiple
./scripts/create_files/compile.sh mesh_twenty_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_twenty_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh mesh_twenty_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh mesh_twenty_mpst 1 macros_multiple

### Ring
# Two
./scripts/create_files/compile.sh ring_two_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_two_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_two_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_two_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_two_mpst 1 macros_multiple

# Three
./scripts/create_files/compile.sh ring_three_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_three_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_three_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_three_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_three_mpst 1 macros_multiple

# Four
./scripts/create_files/compile.sh ring_four_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_four_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_four_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_four_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_four_mpst 1 macros_multiple

# Five
./scripts/create_files/compile.sh ring_five_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_five_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_five_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_five_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_five_mpst 1 macros_multiple

# Six
./scripts/create_files/compile.sh ring_six_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_six_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_six_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_six_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_six_mpst 1 macros_multiple

# Seven
./scripts/create_files/compile.sh ring_seven_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_seven_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_seven_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_seven_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_seven_mpst 1 macros_multiple

# Eight
./scripts/create_files/compile.sh ring_eight_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_eight_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_eight_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_eight_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_eight_mpst 1 macros_multiple

# Nine
./scripts/create_files/compile.sh ring_nine_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_nine_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_nine_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_nine_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_nine_mpst 1 macros_multiple

# Ten
./scripts/create_files/compile.sh ring_ten_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_ten_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_ten_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_ten_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_ten_mpst 1 macros_multiple

# Eleven
./scripts/create_files/compile.sh ring_eleven_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_eleven_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_eleven_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_eleven_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_eleven_mpst 1 macros_multiple

# Twenty
./scripts/create_files/compile.sh ring_twenty_binary 1 macros_multiple
./scripts/create_files/compile.sh ring_twenty_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_twenty_broadcast_cancel 1 macros_multiple
./scripts/create_files/compile.sh ring_twenty_crossbeam 1 macros_multiple
./scripts/create_files/compile.sh ring_twenty_mpst 1 macros_multiple

# Run the benchmarks
cargo bench --bench main --features="macros_multiple" -- --verbose
cargo bench --bench ping_pong --features="macros_multiple"  -- --verbose
