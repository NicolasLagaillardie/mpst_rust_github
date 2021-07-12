#!/bin/bash

# Create the expanded version of the examples

set -e

sleep 5

### Basic examples
cargo expand --example o_auth > expand/o_auth.txt
cargo expand --example o_auth_2 > expand/o_auth_2.txt
cargo expand --example fib > expand/fib.txt
cargo expand --example simple_voting_three > expand/simple_voting_three.txt
cargo expand --example three_buyer > expand/three_buyer.txt
cargo expand --example travel_three > expand/travel_three.txt
cargo expand --example actyx_os_logging > expand/actyx_os_logging.txt
cargo expand --example actyx_os_api > expand/actyx_os_api.txt
cargo expand --example smtp > expand/smtp.txt
cargo expand --example distributed_calc > expand/distributed_calc.txt
cargo expand --example video_stream > expand/video_stream.txt
cargo expand --example online_wallet > expand/online_wallet.txt
cargo expand --example dns_fowler > expand/dns_fowler.txt
cargo expand --example dns_imai > expand/dns_imai.txt

### Ping-pong
cargo expand --example ping_pong_binary > expand/ping_pong_binary.txt
cargo expand --example ping_pong_cancel > expand/ping_pong_cancel.txt
cargo expand --example ping_pong_broadcast_cancel > expand/ping_pong_broadcast_cancel.txt
cargo expand --example ping_pong_crossbeam > expand/ping_pong_crossbeam.txt
cargo expand --example ping_pong_mpst > expand/ping_pong_mpst.txt

### Mesh
# Two
cargo expand --example mesh_two_binary > expand/mesh_two_binary.txt
cargo expand --example mesh_two_cancel > expand/mesh_two_cancel.txt
cargo expand --example mesh_three_broadcast_cancel > expand/mesh_three_broadcast_cancel.txt
cargo expand --example mesh_two_crossbeam > expand/mesh_two_crossbeam.txt
cargo expand --example mesh_two_mpst > expand/mesh_two_mpst.txt

# Three
cargo expand --example mesh_three_binary > expand/mesh_three_binary.txt
cargo expand --example mesh_three_cancel > expand/mesh_three_cancel.txt
cargo expand --example mesh_three_broadcast_cancel > expand/mesh_three_broadcast_cancel.txt
cargo expand --example mesh_three_crossbeam > expand/mesh_three_crossbeam.txt
cargo expand --example mesh_three_mpst > expand/mesh_three_mpst.txt

# Four
cargo expand --example mesh_four_binary > expand/mesh_four_binary.txt
cargo expand --example mesh_four_cancel > expand/mesh_four_cancel.txt
cargo expand --example mesh_four_broadcast_cancel > expand/mesh_four_broadcast_cancel.txt
cargo expand --example mesh_four_crossbeam > expand/mesh_four_crossbeam.txt
cargo expand --example mesh_four_mpst > expand/mesh_four_mpst.txt

# Five
cargo expand --example mesh_five_binary > expand/mesh_five_binary.txt
cargo expand --example mesh_five_cancel > expand/mesh_five_cancel.txt
cargo expand --example mesh_five_broadcast_cancel > expand/mesh_five_broadcast_cancel.txt
cargo expand --example mesh_five_crossbeam > expand/mesh_five_crossbeam.txt
cargo expand --example mesh_five_mpst > expand/mesh_five_mpst.txt

# Six
cargo expand --example mesh_six_binary > expand/mesh_six_binary.txt
cargo expand --example mesh_six_cancel > expand/mesh_six_cancel.txt
cargo expand --example mesh_six_broadcast_cancel > expand/mesh_six_broadcast_cancel.txt
cargo expand --example mesh_six_crossbeam > expand/mesh_six_crossbeam.txt
cargo expand --example mesh_six_mpst > expand/mesh_six_mpst.txt

# Seven
cargo expand --example mesh_seven_binary > expand/mesh_seven_binary.txt
cargo expand --example mesh_seven_cancel > expand/mesh_seven_cancel.txt
cargo expand --example mesh_seven_broadcast_cancel > expand/mesh_seven_broadcast_cancel.txt
cargo expand --example mesh_seven_crossbeam > expand/mesh_seven_crossbeam.txt
cargo expand --example mesh_seven_mpst > expand/mesh_seven_mpst.txt

# Eight
cargo expand --example mesh_eight_binary > expand/mesh_eight_binary.txt
cargo expand --example mesh_eight_cancel > expand/mesh_eight_cancel.txt
cargo expand --example mesh_eight_broadcast_cancel > expand/mesh_eight_broadcast_cancel.txt
cargo expand --example mesh_eight_crossbeam > expand/mesh_eight_crossbeam.txt
cargo expand --example mesh_eight_mpst > expand/mesh_eight_mpst.txt

# Nine
cargo expand --example mesh_nine_binary > expand/mesh_nine_binary.txt
cargo expand --example mesh_nine_cancel > expand/mesh_nine_cancel.txt
cargo expand --example mesh_nine_broadcast_cancel > expand/mesh_nine_broadcast_cancel.txt
cargo expand --example mesh_nine_crossbeam > expand/mesh_nine_crossbeam.txt
cargo expand --example mesh_nine_mpst > expand/mesh_nine_mpst.txt

# Ten
cargo expand --example mesh_ten_binary > expand/mesh_ten_binary.txt
cargo expand --example mesh_ten_cancel > expand/mesh_ten_cancel.txt
cargo expand --example mesh_ten_broadcast_cancel > expand/mesh_ten_broadcast_cancel.txt
cargo expand --example mesh_ten_crossbeam > expand/mesh_ten_crossbeam.txt
cargo expand --example mesh_ten_mpst > expand/mesh_ten_mpst.txt

# Eleven
cargo expand --example mesh_eleven_binary > expand/mesh_eleven_binary.txt
cargo expand --example mesh_eleven_cancel > expand/mesh_eleven_cancel.txt
cargo expand --example mesh_eleven_broadcast_cancel > expand/mesh_eleven_broadcast_cancel.txt
cargo expand --example mesh_eleven_crossbeam > expand/mesh_eleven_crossbeam.txt
cargo expand --example mesh_eleven_mpst > expand/mesh_eleven_mpst.txt

# Twenty
cargo expand --example mesh_twenty_binary > expand/mesh_twenty_binary.txt
cargo expand --example mesh_twenty_cancel > expand/mesh_twenty_cancel.txt
cargo expand --example mesh_twenty_broadcast_cancel > expand/mesh_twenty_broadcast_cancel.txt
cargo expand --example mesh_twenty_crossbeam > expand/mesh_twenty_crossbeam.txt
cargo expand --example mesh_twenty_mpst > expand/mesh_twenty_mpst.txt

### Ring
# Two
cargo expand --example ring_two_binary > expand/ring_two_binary.txt
cargo expand --example ring_two_cancel > expand/ring_two_cancel.txt
cargo expand --example ring_two_broadcast_cancel > expand/ring_two_broadcast_cancel.txt
cargo expand --example ring_two_crossbeam > expand/ring_two_crossbeam.txt
cargo expand --example ring_two_mpst > expand/ring_two_mpst.txt

# Three
cargo expand --example ring_three_binary > expand/ring_three_binary.txt
cargo expand --example ring_three_cancel > expand/ring_three_cancel.txt
cargo expand --example ring_three_broadcast_cancel > expand/ring_three_broadcast_cancel.txt
cargo expand --example ring_three_crossbeam > expand/ring_three_crossbeam.txt
cargo expand --example ring_three_mpst > expand/ring_three_mpst.txt

# Four
cargo expand --example ring_four_binary > expand/ring_four_binary.txt
cargo expand --example ring_four_cancel > expand/ring_four_cancel.txt
cargo expand --example ring_four_broadcast_cancel > expand/ring_four_broadcast_cancel.txt
cargo expand --example ring_four_crossbeam > expand/ring_four_crossbeam.txt
cargo expand --example ring_four_mpst > expand/ring_four_mpst.txt

# Five
cargo expand --example ring_five_binary > expand/ring_five_binary.txt
cargo expand --example ring_five_cancel > expand/ring_five_cancel.txt
cargo expand --example ring_five_broadcast_cancel > expand/ring_five_broadcast_cancel.txt
cargo expand --example ring_five_crossbeam > expand/ring_five_crossbeam.txt
cargo expand --example ring_five_mpst > expand/ring_five_mpst.txt

# Six
cargo expand --example ring_six_binary > expand/ring_six_binary.txt
cargo expand --example ring_six_cancel > expand/ring_six_cancel.txt
cargo expand --example ring_six_broadcast_cancel > expand/ring_six_broadcast_cancel.txt
cargo expand --example ring_six_crossbeam > expand/ring_six_crossbeam.txt
cargo expand --example ring_six_mpst > expand/ring_six_mpst.txt

# Seven
cargo expand --example ring_seven_binary > expand/ring_seven_binary.txt
cargo expand --example ring_seven_cancel > expand/ring_seven_cancel.txt
cargo expand --example ring_seven_broadcast_cancel > expand/ring_seven_broadcast_cancel.txt
cargo expand --example ring_seven_crossbeam > expand/ring_seven_crossbeam.txt
cargo expand --example ring_seven_mpst > expand/ring_seven_mpst.txt

# Eight
cargo expand --example ring_eight_binary > expand/ring_eight_binary.txt
cargo expand --example ring_eight_cancel > expand/ring_eight_cancel.txt
cargo expand --example ring_eight_broadcast_cancel > expand/ring_eight_broadcast_cancel.txt
cargo expand --example ring_eight_crossbeam > expand/ring_eight_crossbeam.txt
cargo expand --example ring_eight_mpst > expand/ring_eight_mpst.txt

# Nine
cargo expand --example ring_nine_binary > expand/ring_nine_binary.txt
cargo expand --example ring_nine_cancel > expand/ring_nine_cancel.txt
cargo expand --example ring_nine_broadcast_cancel > expand/ring_nine_broadcast_cancel.txt
cargo expand --example ring_nine_crossbeam > expand/ring_nine_crossbeam.txt
cargo expand --example ring_nine_mpst > expand/ring_nine_mpst.txt

# Ten
cargo expand --example ring_ten_binary > expand/ring_ten_binary.txt
cargo expand --example ring_ten_cancel > expand/ring_ten_cancel.txt
cargo expand --example ring_ten_broadcast_cancel > expand/ring_ten_broadcast_cancel.txt
cargo expand --example ring_ten_crossbeam > expand/ring_ten_crossbeam.txt
cargo expand --example ring_ten_mpst > expand/ring_ten_mpst.txt

# Eleven
cargo expand --example ring_eleven_binary > expand/ring_eleven_binary.txt
cargo expand --example ring_eleven_cancel > expand/ring_eleven_cancel.txt
cargo expand --example ring_eleven_broadcast_cancel > expand/ring_eleven_broadcast_cancel.txt
cargo expand --example ring_eleven_crossbeam > expand/ring_eleven_crossbeam.txt
cargo expand --example ring_eleven_mpst > expand/ring_eleven_mpst.txt

# Twenty
cargo expand --example ring_twenty_binary > expand/ring_twenty_binary.txt
cargo expand --example ring_twenty_cancel > expand/ring_twenty_cancel.txt
cargo expand --example ring_twenty_broadcast_cancel > expand/ring_twenty_broadcast_cancel.txt
cargo expand --example ring_twenty_crossbeam > expand/ring_twenty_crossbeam.txt
cargo expand --example ring_twenty_mpst > expand/ring_twenty_mpst.txt