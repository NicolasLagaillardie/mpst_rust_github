#!/bin/bash

# Create the compilation time and benchmarks files for the examples

set -e

sleep 5

### Basic examples
./scripts/create_files/compile.sh o_auth 100
./scripts/create_files/compile.sh o_auth_2 100
./scripts/create_files/compile.sh fib 100
./scripts/create_files/compile.sh simple_voting 100
./scripts/create_files/compile.sh three_buyer 100
./scripts/create_files/compile.sh travel_three 100
./scripts/create_files/compile.sh actyx_os_logging 100
./scripts/create_files/compile.sh actyx_os_api 100
./scripts/create_files/compile.sh smtp 100
./scripts/create_files/compile.sh distributed_calc 100
./scripts/create_files/compile.sh video_stream 100
./scripts/create_files/compile.sh online_wallet 100
./scripts/create_files/compile.sh dns_fowler 100
./scripts/create_files/compile.sh dns_imai 100

### Ping-pong
for i in $(eval echo {1..100})
do
    ./scripts/create_files/compile.sh ping_pong_binary_$i 100
    ./scripts/create_files/compile.sh ping_pong_cancel_$i 100
    ./scripts/create_files/compile.sh ping_pong_broadcast_cancel_$i 100
    ./scripts/create_files/compile.sh ping_pong_crossbeam_$i 100
    ./scripts/create_files/compile.sh ping_pong_mpst_$i 100
done

### Mesh
Two
./scripts/create_files/compile.sh mesh_two_binary 100
./scripts/create_files/compile.sh mesh_two_cancel 100
./scripts/create_files/compile.sh mesh_three_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_two_crossbeam 100
./scripts/create_files/compile.sh mesh_two_mpst 100

# Three
./scripts/create_files/compile.sh mesh_three_binary 100
./scripts/create_files/compile.sh mesh_three_cancel 100
./scripts/create_files/compile.sh mesh_three_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_three_crossbeam 100
./scripts/create_files/compile.sh mesh_three_mpst 100

# Four
./scripts/create_files/compile.sh mesh_four_binary 100
./scripts/create_files/compile.sh mesh_four_cancel 100
./scripts/create_files/compile.sh mesh_four_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_four_crossbeam 100
./scripts/create_files/compile.sh mesh_four_mpst 100

# Five
./scripts/create_files/compile.sh mesh_five_binary 100
./scripts/create_files/compile.sh mesh_five_cancel 100
./scripts/create_files/compile.sh mesh_five_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_five_crossbeam 100
./scripts/create_files/compile.sh mesh_five_mpst 100

# Six
./scripts/create_files/compile.sh mesh_six_binary 100
./scripts/create_files/compile.sh mesh_six_cancel 100
./scripts/create_files/compile.sh mesh_six_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_six_crossbeam 100
./scripts/create_files/compile.sh mesh_six_mpst 100

# Seven
./scripts/create_files/compile.sh mesh_seven_binary 100
./scripts/create_files/compile.sh mesh_seven_cancel 100
./scripts/create_files/compile.sh mesh_seven_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_seven_crossbeam 100
./scripts/create_files/compile.sh mesh_seven_mpst 100

# Eight
./scripts/create_files/compile.sh mesh_eight_binary 100
./scripts/create_files/compile.sh mesh_eight_cancel 100
./scripts/create_files/compile.sh mesh_eight_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_eight_crossbeam 100
./scripts/create_files/compile.sh mesh_eight_mpst 100

# Nine
./scripts/create_files/compile.sh mesh_nine_binary 100
./scripts/create_files/compile.sh mesh_nine_cancel 100
./scripts/create_files/compile.sh mesh_nine_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_nine_crossbeam 100
./scripts/create_files/compile.sh mesh_nine_mpst 100

# Ten
./scripts/create_files/compile.sh mesh_ten_binary 100
./scripts/create_files/compile.sh mesh_ten_cancel 100
./scripts/create_files/compile.sh mesh_ten_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_ten_crossbeam 100
./scripts/create_files/compile.sh mesh_ten_mpst 100

# Eleven
./scripts/create_files/compile.sh mesh_eleven_binary 100
./scripts/create_files/compile.sh mesh_eleven_cancel 100
./scripts/create_files/compile.sh mesh_eleven_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_eleven_crossbeam 100
./scripts/create_files/compile.sh mesh_eleven_mpst 100

# Twenty
./scripts/create_files/compile.sh mesh_twenty_binary 100
./scripts/create_files/compile.sh mesh_twenty_cancel 100
./scripts/create_files/compile.sh mesh_twenty_broadcast_cancel 100
./scripts/create_files/compile.sh mesh_twenty_crossbeam 100
./scripts/create_files/compile.sh mesh_twenty_mpst 100

### Ring
# Two
./scripts/create_files/compile.sh ring_two_binary 100
./scripts/create_files/compile.sh ring_two_cancel 100
./scripts/create_files/compile.sh ring_two_broadcast_cancel 100
./scripts/create_files/compile.sh ring_two_crossbeam 100
./scripts/create_files/compile.sh ring_two_mpst 100

# Three
./scripts/create_files/compile.sh ring_three_binary 100
./scripts/create_files/compile.sh ring_three_cancel 100
./scripts/create_files/compile.sh ring_three_broadcast_cancel 100
./scripts/create_files/compile.sh ring_three_crossbeam 100
./scripts/create_files/compile.sh ring_three_mpst 100

# Four
./scripts/create_files/compile.sh ring_four_binary 100
./scripts/create_files/compile.sh ring_four_cancel 100
./scripts/create_files/compile.sh ring_four_broadcast_cancel 100
./scripts/create_files/compile.sh ring_four_crossbeam 100
./scripts/create_files/compile.sh ring_four_mpst 100

# Five
./scripts/create_files/compile.sh ring_five_binary 100
./scripts/create_files/compile.sh ring_five_cancel 100
./scripts/create_files/compile.sh ring_five_broadcast_cancel 100
./scripts/create_files/compile.sh ring_five_crossbeam 100
./scripts/create_files/compile.sh ring_five_mpst 100

# Six
./scripts/create_files/compile.sh ring_six_binary 100
./scripts/create_files/compile.sh ring_six_cancel 100
./scripts/create_files/compile.sh ring_six_broadcast_cancel 100
./scripts/create_files/compile.sh ring_six_crossbeam 100
./scripts/create_files/compile.sh ring_six_mpst 100

# Seven
./scripts/create_files/compile.sh ring_seven_binary 100
./scripts/create_files/compile.sh ring_seven_cancel 100
./scripts/create_files/compile.sh ring_seven_broadcast_cancel 100
./scripts/create_files/compile.sh ring_seven_crossbeam 100
./scripts/create_files/compile.sh ring_seven_mpst 100

# Eight
./scripts/create_files/compile.sh ring_eight_binary 100
./scripts/create_files/compile.sh ring_eight_cancel 100
./scripts/create_files/compile.sh ring_eight_broadcast_cancel 100
./scripts/create_files/compile.sh ring_eight_crossbeam 100
./scripts/create_files/compile.sh ring_eight_mpst 100

# Nine
./scripts/create_files/compile.sh ring_nine_binary 100
./scripts/create_files/compile.sh ring_nine_cancel 100
./scripts/create_files/compile.sh ring_nine_broadcast_cancel 100
./scripts/create_files/compile.sh ring_nine_crossbeam 100
./scripts/create_files/compile.sh ring_nine_mpst 100

# Ten
./scripts/create_files/compile.sh ring_ten_binary 100
./scripts/create_files/compile.sh ring_ten_cancel 100
./scripts/create_files/compile.sh ring_ten_broadcast_cancel 100
./scripts/create_files/compile.sh ring_ten_crossbeam 100
./scripts/create_files/compile.sh ring_ten_mpst 100

# Eleven
./scripts/create_files/compile.sh ring_eleven_binary 100
./scripts/create_files/compile.sh ring_eleven_cancel 100
./scripts/create_files/compile.sh ring_eleven_broadcast_cancel 100
./scripts/create_files/compile.sh ring_eleven_crossbeam 100
./scripts/create_files/compile.sh ring_eleven_mpst 100

# Twenty
./scripts/create_files/compile.sh ring_twenty_binary 100
./scripts/create_files/compile.sh ring_twenty_cancel 100
./scripts/create_files/compile.sh ring_twenty_broadcast_cancel 100
./scripts/create_files/compile.sh ring_twenty_crossbeam 100
./scripts/create_files/compile.sh ring_twenty_mpst 100

# Run the benchmarks
cargo bench --bench main -- --verbose
cargo bench --bench ping_pong -- --verbose
